use std::time::Duration;
use moka::future::Cache;

use actix_web::{App, HttpResponse, HttpServer, get, http::StatusCode, web, http::header::ContentType};
use serde::Deserialize;

extern crate booth2rss;
use booth2rss::{BoothClient, objects::booth_store::BoothStore, errors::BoothRequestError};

//mod cache;
//use cache::ResponseCache;

use crate::config_reader::read_config;

mod config_reader;

const CONFIG_PATH: &str = "./config.json";

//#[derive(Clone)]
//struct AppGlobals {
//    store_cache: Cache<String,String>, //TODO: Cache store, not string!
//    exc_rate_cache: Cache<String,f32>,
//    booth_client: BoothClient,
//    currency_src: String,
//    currency_tgt: String,
//    convert_currency: bool
//}

#[derive(Deserialize)]
#[serde(default)]
struct StoreParams {
    url: Option<String>,
    max_pages: u32,
    filter_unavailable: bool,
    unblur_nsfw: bool,
    allow_nsfw: bool,
    vrc_only: bool
}

impl Default for StoreParams {
    fn default() -> Self {
        StoreParams {
            url: None,
            max_pages: 10,
            filter_unavailable: true,
            unblur_nsfw: false,
            allow_nsfw: false,
            vrc_only: false
        }
    }
}


pub fn convert_item_price(price: &str, exchange_rate: f32, target_currency: &str) -> Option<String> {
    let price_clean = price.to_string()
        .replace(",", "")
        .replace(".", "");

    let price_num = &price_clean[0..price_clean.find(" ").unwrap_or(price_clean.len())];
    
    return match price_num.parse::<f32>() {
        Ok(i) => Some(format!("{:.2}{target_currency}", i * exchange_rate)),
        Err(e) => {
            println!("Unable to convert {price_num} to i32: {e}");
            return None;
        }
    };
}

async fn convert_price(client: &web::Data<BoothClient>, store: &mut BoothStore, src: String, tgt: String, rate_cache: web::Data<Cache<String,f32>>) {
    let key = format!("{src}>{tgt}");

    let exchange_rate: f32;
    if let Some(cached_rate) = rate_cache.get(&key).await {
        exchange_rate = cached_rate;
    } else {

        let exchange_res  = client.get_currency_exchange_rate(&src, &tgt).await;

        if let Err(e) = &exchange_res {
            eprintln!("ERROR: Unable to get currency exchange rate: {e}");
            return;
        }

        exchange_rate = exchange_res.unwrap().rate;

        // Save exchange rate to cache
        rate_cache.insert(key, exchange_rate).await;
    }

    for item in store.items.iter_mut() {
        item.local_price = convert_item_price(&item.price, exchange_rate, &tgt);
    }
}


#[get("/booth2rss/store")]
async fn get_store(store_data: web::Query<StoreParams>, client: web::Data<booth2rss::BoothClient>, cache: web::Data<Cache<String, BoothStore>>, exc_cache: web::Data<Cache<String,f32>>) -> HttpResponse {
    let url = match &store_data.url {
        Some(url) => url.to_owned(),
        None => return HttpResponse::UnprocessableEntity().body("No url provided".to_string())
    };

    if url.is_empty() {
        return HttpResponse::UnprocessableEntity().body("No url provided".to_string())
    }

    let cache_key = format!(
        "{}{}-{}",
        store_data.max_pages,
        store_data.unblur_nsfw,
        url
    );

    // Get value from cache if it's still valid
    let mut store: BoothStore;
    if let Some(cached_store) = cache.get(&cache_key).await {
        store = cached_store;
    } else {
        let store_res = client.get_booth_store(&url, store_data.max_pages, store_data.unblur_nsfw).await;

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
        cache.insert(cache_key, store.clone()).await;
    }

    // Apply currency conversion
    //TODO: Pass config values
    convert_price(&client, &mut store, "JPY".to_string(), "EUR".to_string(), exc_cache).await;

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

    let exc_cache = Cache::<String, String>::builder()
        .max_capacity(config_data.cache_size)
        .time_to_live(Duration::from_mins(60)) //TODO: make This configurable!
        .build();

//    let globals = AppGlobals {
//        store_cache: req_cache,
//        exc_rate_cache: exc_cache,
//        booth_client: client,
//        currency_src: config_data.currency_source,
//        currency_tgt: config_data.currency_target,
//        convert_currency: config_data.convert_currency
//    };
//TODO: Add target currency as a web parameter

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(client.clone()))
        .app_data(web::Data::new(req_cache.clone()))
        .app_data(web::Data::new(exc_cache.clone()))
        .service(get_store)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
