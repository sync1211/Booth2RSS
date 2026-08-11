use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

use actix_web::{get, web, App, HttpServer, HttpResponse};
use serde::Deserialize;

extern crate booth2rss;

mod cache;
use cache::ResponseCache;

static CACHE: Lazy<Arc<Mutex<ResponseCache>>> = Lazy::new(|| {
    Arc::new(Mutex::new(ResponseCache::with_defaults()))
});

static CLIENT: once_cell::sync::Lazy<reqwest::Client> = once_cell::sync::Lazy::new(|| {
        reqwest::Client::new()
});


#[derive(Deserialize)]
#[serde(default)]
struct StoreParams {
    url: Option<String>,
    limit: i32,
    filter_unavailable: bool,
    unblur_nsfw: bool,
    allow_nsfw: bool,
    vrc_only: bool
}

impl Default for StoreParams {
    fn default() -> Self {
        StoreParams {
            url: None,
            limit: 10,
            filter_unavailable: true,
            unblur_nsfw: false,
            allow_nsfw: false,
            vrc_only: false
        }
    }
}

#[get("/booth2rss/store")]
async fn get_store(store_data: web::Query<StoreParams>) -> HttpResponse {
    let url = match &store_data.url {
        Some(url) => url.to_owned(),
        None => return HttpResponse::UnprocessableEntity().body("No url provided".to_string())
    };

    if url.is_empty() {
        return HttpResponse::UnprocessableEntity().body("No url provided".to_string())
    }
    
    let cache_key = format!(
        "{}{}{}{}{}-{}",
        store_data.limit,
        store_data.unblur_nsfw,
        store_data.filter_unavailable,
        store_data.allow_nsfw,
        store_data.vrc_only,
        url
    );

    // Get value from cache if it's still valid
    if let Ok(cache) = CACHE.lock()
        && let Some(rss) = cache.get_active_value(&cache_key) {
        return HttpResponse::Ok().body(rss.to_owned());
    }

    let store_res = booth2rss::get_booth_store(&CLIENT, &url, store_data.limit, store_data.unblur_nsfw).await;

    let store = match store_res {
        Ok(s) => s,
        Err(status) => {          
            return status;
        }
    };

    let store_rss = store.as_rss(store_data.filter_unavailable, !store_data.allow_nsfw, store_data.vrc_only, 15);

    // Save value to cache
    if let Ok(mut c) = CACHE.lock() {
        c.add_item(cache_key.to_string(), store_rss.to_owned());
    }

    return HttpResponse::Ok().body(store_rss);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(get_store)
    )
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
