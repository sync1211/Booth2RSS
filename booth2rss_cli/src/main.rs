use std::env;

extern crate booth2rss;
use booth2rss::BoothClient;
use booth2rss::errors::BoothRequestError;
use booth2rss::objects::booth_store::BoothStore;

mod argparse;

async fn convert_prices(client: &BoothClient, store: &mut BoothStore, src: &str, tgt: &str) {
    if store.items.is_empty() {
        return;
    }

    // Try to auto-detect the currency string
    let source_currency = store.items
        .first()
        .unwrap()
        .try_detect_currency()
        .unwrap_or(src.to_owned());

    let exchange_res  = client.get_currency_exchange_rate(&source_currency, tgt).await;

    if let Err(e) = &exchange_res {
        eprintln!("ERROR: Unable to get currency exchange rate: {e}");
        return;
    }

    let exchange_rate = exchange_res.unwrap().rate;

    for item in store.items.iter_mut() {
        item.apply_currency_conversion(exchange_rate, tgt);
    }
}

#[tokio::main]
async fn main() {   
    let params = match argparse::parse_arguments(env::args()) {
        Some(p) => p,
        None => return
    };

    let client = booth2rss::BoothClient::with_defaults();
    
    let store_res = client.get_booth_store(
        &params.url,
        params.max_pages,
        params.unblur_nsfw
    ).await;

    if let Ok(mut store) = store_res {
        if params.convert_currency {
            convert_prices(&client, &mut store, "JPY", &params.convert_target).await;
        }

        let rss = store.as_rss(
            !params.include_unavailable,
            !params.allow_nsfw,
            params.vrc_only,
            0
        );

        println!("{rss}");
        return;
    }

    if let Err(e) = store_res {
        match e {
            BoothRequestError::HttpError(status, reason) => eprintln!("ERROR: {} - {}", status, reason),
            e => eprintln!("ERROR: {}", e)
        };
    }
}
