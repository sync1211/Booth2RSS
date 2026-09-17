use serde::{Deserialize};

#[derive(Deserialize)]
pub struct CurrencyExchangeRate {
    // pub date: String,
    // base: String,
    // quote: String,
    pub rate: f32
}

const BASE_URL: &str = "https://api.frankfurter.dev/v2/rate/";


pub async fn get_exchange_rate(client: &reqwest::Client, src: &str, tgt: &str) -> Result<CurrencyExchangeRate, String> {
    let url = format!("{BASE_URL}{src}/{tgt}");

    let response = match client.get(url).send().await {
        Ok(r) => r,
        Err(e) => return Err(e.to_string())
    };

    if ! response.status().is_success() {
        return Err(response.status().to_string());
    }

    let rate = match response.json::<CurrencyExchangeRate>().await {
        Ok(r) => r,
        Err(e) => return Err(e.to_string())        
    };


    return Ok(rate);
}
