use serde::{Deserialize};

use crate::errors::RequestError;

#[derive(Deserialize)]
pub struct CurrencyExchangeRate {
    // pub date: String,
    // base: String,
    // quote: String,
    pub rate: f32
}

const BASE_URL: &str = "https://api.frankfurter.dev/v2/rate/";


pub async fn get_exchange_rate(client: &reqwest::Client, src: &str, tgt: &str) -> Result<CurrencyExchangeRate, RequestError> {
    let url = format!("{BASE_URL}{src}/{tgt}");

    log::debug!("Requesting URL {url}...");
    let response = match client.get(url).send().await {
        Ok(r) => r,
        Err(e) => return Err(RequestError::NetworkError(e.to_string()))
    };

    let status = response.status();
    if !status.is_success() {
        return Err(RequestError::HttpError(status.as_u16(), status.to_string()));
    }

    let rate = match response.json::<CurrencyExchangeRate>().await {
        Ok(r) => r,
        Err(e) => return Err(RequestError::ParseError(e.to_string()))
    };

    return Ok(rate);
}