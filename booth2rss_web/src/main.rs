use std::time::Duration;
use moka::future::Cache;

use actix_web::{App, HttpResponse, HttpServer, get, http::StatusCode, web, http::header::ContentType};
use serde::Deserialize;

extern crate booth2rss;
use booth2rss::{BoothClient, objects::booth_store::BoothStore, errors::BoothRequestError};

use crate::config_reader::read_config;

mod config_reader;

const CONFIG_PATH: &str = "./config.json";

#[derive(Clone)]
struct AppGlobals {
   store_cache: Cache<String,BoothStore>,
   exc_rate_cache: Cache<String,f32>,
   booth_client: BoothClient,
   fallback_currency_src: String,
   allow_currency_conversion: bool
}

#[derive(Deserialize)]
#[serde(default)]
struct StoreParams {
    url: Option<String>,
    max_pages: u32,
    filter_unavailable: bool,
    unblur_nsfw: bool,
    allow_nsfw: bool,
    vrc_only: bool,
    currency: Option<String>
}

impl Default for StoreParams {
    fn default() -> Self {
        StoreParams {
            url: None,
            max_pages: 10,
            filter_unavailable: true,
            unblur_nsfw: false,
            allow_nsfw: false,
            vrc_only: false,
            currency: None
        }
    }
}


async fn convert_prices(client: &BoothClient, store: &mut BoothStore, fallback_currency: &str, target_currency: &str, rate_cache: &Cache<String,f32>) {
    if store.items.len() == 0 {
        return;
    }

    // Try to auto-detect the currency string
    let source_currency = store.items
        .first()
        .unwrap()
        .try_detect_currency()
        .unwrap_or(fallback_currency.to_owned());

    let key = format!("{source_currency}>{target_currency}");

    let exchange_rate: f32;
    if let Some(cached_rate) = rate_cache.get(&key).await {
        exchange_rate = cached_rate;
    } else {

        let exchange_res  = client.get_currency_exchange_rate(&source_currency, target_currency).await;

        if let Err(e) = &exchange_res {
            eprintln!("ERROR: Unable to get currency exchange rate: {e}");
            return;
        }

        exchange_rate = exchange_res.unwrap().rate;

        // Save exchange rate to cache
        rate_cache.insert(key, exchange_rate).await;
    }

    for item in store.items.iter_mut() {
        item.apply_currency_conversion(exchange_rate, target_currency);
    }
}


#[get("/booth2rss/store")]
async fn get_store(store_data: web::Query<StoreParams>, globals: web::Data<AppGlobals>) -> HttpResponse {
    // Check URL
    let url = match &store_data.url {
        Some(url) => url.to_owned(),
        None => return HttpResponse::UnprocessableEntity().body("No url provided".to_string())
    };

    if url.is_empty() {
        return HttpResponse::UnprocessableEntity().body("No url provided".to_string())
    }

    // Check currency value
    let mut target_currency = None;
    if let Some(ref currency) = store_data.currency {
        if currency.trim().len() != 3 || !currency.is_ascii() || currency.contains(" ") {
            return HttpResponse::UnprocessableEntity().body(format!("Invalid short value for currency: '{currency}'"))
        }
        target_currency = Some(currency);
    }

    let cache_key = format!(
        "{}{}-{}",
        store_data.max_pages,
        store_data.unblur_nsfw,
        url
    );

    // Get value from cache if it's still valid
    let mut store: BoothStore;
    if let Some(cached_store) = globals.store_cache.get(&cache_key).await {
        store = cached_store;
    } else {
        let store_res = globals.booth_client.get_booth_store(&url, store_data.max_pages, store_data.unblur_nsfw).await;

        store = match store_res {
            Ok(s) => s,
            Err(BoothRequestError::HttpError(status_code, reason)) => {
                let status = StatusCode::from_u16(status_code)
                    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

                return HttpResponse::build(status)
                    .body(reason);
            },
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string())
        };

        // Save value to cache
        globals.store_cache.insert(cache_key, store.clone()).await;
    }

    // Apply currency conversion
    if globals.allow_currency_conversion && let Some(target_currency) = target_currency {
        convert_prices(&globals.booth_client, &mut store, &globals.fallback_currency_src, target_currency, &globals.exc_rate_cache).await;
    }

    let store_rss = store.as_rss(store_data.filter_unavailable, !store_data.allow_nsfw, store_data.vrc_only, 15);
    
    return HttpResponse::Ok()
        .content_type(ContentType::xml())
        .body(store_rss);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config_data = read_config(CONFIG_PATH);

    let client = BoothClient::with_defaults();

    let req_cache = Cache::<String, BoothStore>::builder()
        .max_capacity(config_data.cache_size)
        .time_to_live(Duration::from_mins(config_data.cache_minutes))
        .build();

    let exc_cache = Cache::<String, f32>::builder()
        .max_capacity(config_data.currency_conversion_cache_size)
        .time_to_live(Duration::from_mins(config_data.currency_conversion_cache_ttl_minutes))
        .build();

   let globals = AppGlobals {
       store_cache: req_cache,
       exc_rate_cache: exc_cache,
       booth_client: client,
       fallback_currency_src: config_data.currency_fallback,
       allow_currency_conversion: config_data.allow_currency_conversion,
   };

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(globals.clone()))
        .service(get_store)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
