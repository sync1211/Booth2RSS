use std::env;

extern crate booth2rss;

mod argparse;

#[tokio::main]
async fn main() {   
    let params = match argparse::parse_arguments(env::args()) {
        Some(p) => p,
        None => return
    };

    let client = reqwest::Client::new();
    let store_res = booth2rss::get_booth_store(
        &client,
        &params.url,
        params.max_pages,
        params.unblur_nsfw
    ).await;

    let result = match store_res {
        Ok(store) => store.as_rss(
            !params.include_unavailable,
            !params.allow_nsfw,
            params.vrc_only,
            params.limit
        ),
        Err(response) => format!("ERROR: {} - {:#?}", response.status().as_u16(), response.body())
    };

    println!("{}", result);
}
