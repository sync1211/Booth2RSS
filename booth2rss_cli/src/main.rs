use std::env;

extern crate booth2rss;
use booth2rss::errors::BoothRequestError;

mod argparse;

#[tokio::main]
async fn main() {   
    let params = match argparse::parse_arguments(env::args()) {
        Some(p) => p,
        None => return
    };

    let client = booth2rss::Booth2RSSClient::with_defaults();
    let store_res = client.get_booth_store(
        &params.url,
        params.max_pages,
        params.unblur_nsfw
    ).await;

    let result = match store_res {
        Ok(store) => store.as_rss(
            !params.include_unavailable,
            !params.allow_nsfw,
            params.vrc_only,
            0
        ),
        Err(BoothRequestError::HttpError(status, reason)) => format!("ERROR: {} - {}", status, reason),
        Err(e) => format!("ERROR: {:?}", e)
    };

    println!("{}", result);
}
