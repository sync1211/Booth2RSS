pub mod utils;
use url::Url;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::sync::LazyLock;

pub mod currency_exchange;
use currency_exchange::get_exchange_rate;
use currency_exchange::CurrencyExchangeRate;

use crate::objects::booth_item::BoothItem;
use crate::objects::booth_store::BoothStore;

pub mod errors;
use errors::RequestError;

pub mod objects {
    pub mod booth_item;
    pub mod booth_store;
    pub mod booth_category;
    pub mod multi_language_name;
}

// Request
const SELF_USER_AGENT: &str = "Booth2Rss";
const ADULT_COOKIE: &str = "adult=t";
const ACCEPTED_LANGUAGE: &str = "en-US";


// Selectors (lazy init to save resources)
static NICKNAME_SELECTOR: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("[title='Home']").unwrap()
});
static NAME_SELECTOR: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse(".shop-name-label").unwrap()
});
static DESCRIPTION_SELECTOR: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse(".booth-description").unwrap()
});
static ICON_SELECTOR: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse(".avatar-image").unwrap()
});

static LAST_PAGE_SELECTOR: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse(".last-page").unwrap()
});

static ITEM_SELECTOR: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("[data-item]").unwrap()
});



#[derive(Clone)]
pub struct BoothClient {
    client: reqwest::Client
}


impl BoothClient {
    pub fn with_defaults() -> BoothClient {
        return BoothClient::with_client(
            reqwest::Client::builder()
            .user_agent(SELF_USER_AGENT)
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .unwrap()
        );
    }

    pub fn with_client(client: reqwest::Client) -> BoothClient {
        return BoothClient{
            client
        };
    }

    pub async fn get_booth_store(&self, url: &str, max_pages: u32, unblur_nsfw: bool) -> Result<BoothStore, RequestError> {
    
        // Url checks
        let mut url_obj = match Url::parse(url) {
            Ok(url) => url,
            Err(e) => return Err(RequestError::InvalidUrl(e.to_string()))

        };
        
        match url_obj.domain() {
            Some(domain) => if !domain.ends_with(".booth.pm") {
                return Err(RequestError::NotBoothUrl())
            },
            None => return Err(RequestError::InvalidUrl("Missing domain!".to_string()))
        };

        let url_path = url_obj.path();
        if !(url_path.contains("items") || url_path.contains("item_lists")) {
            url_obj.set_path(&format!("{url_path}items"));
        }

        let mut page_count: Option<u32> = None;
        let mut items: HashSet<BoothItem> = HashSet::new();

        let mut i = 1;
        loop {
            log::info!("Fetching page {}/{:#?}...", i, page_count.unwrap_or_default());
            url_obj.set_query(Some(&format!("page={}", i)));

            let result = get_page(&self.client, &url_obj, unblur_nsfw).await;

            let content = match result {
                Ok(response) => response,
                Err(status) => return Err(status),
            };

            let document = Html::parse_document(&content);

            // Detect number of total pages
            if page_count.is_none() {
                page_count = get_max_page_count(&document);
                
                if let Some(pc) = page_count {
                    log::info!("Detected maximum page count: {}", pc);
                }
            }
        
            // Get items from content
            let new_items = get_items_from_content(&document);
            let new_items_iter = new_items.into_iter();
            
            // Add new items to item list
            let mut new_items_count = 0;
            for new_item in new_items_iter {
                if !items.insert(new_item) {
                    break; // Duplicate item -> we are reading the same page twice
                }
                new_items_count += 1;
            }
            log::info!("New items: {}", new_items_count);

            // Exit condition
            if new_items_count == 0 || i >= max_pages || (!page_count.is_none() &&  i >= page_count.unwrap()) {
                log::debug!("Last page reached!");
                url_obj.set_query(None);
                return Ok(create_store_from_content(&document, url_obj.as_ref(), items));
            }

            i += 1;
        }
    }

    pub async fn get_currency_exchange_rate(&self, src: &str, tgt: &str) -> Result<CurrencyExchangeRate, RequestError> {
        return get_exchange_rate(&self.client, src, tgt).await; 
    }
}




pub async fn get_page(client: &reqwest::Client, url: &Url, allow_adult: bool) -> Result<String, RequestError> {
    let mut builder = client.get(url.to_string())
        .header(reqwest::header::ACCEPT_LANGUAGE, ACCEPTED_LANGUAGE);

    if allow_adult {
        builder = builder.header(reqwest::header::COOKIE, ADULT_COOKIE);
    }

    // Request data
    log::debug!("Sending get request to {url}...");
    let result = builder.send().await;

    let response = match result {
        Ok(resp) => resp,
        Err(e) => return Err(RequestError::NetworkError(format!("Request failed: {e}")))
    };

    let status = response.status();
    log::info!("Request finished with status {status}");

    if !status.is_success() {
        return Err(RequestError::HttpError(
            status.as_u16(),
            status
                .canonical_reason()
                .unwrap_or("Unknown Error")
                .to_string()
            )
        );
    }

    return match response.text().await {
        Ok(response_text) => Ok(response_text),
        Err(e) => Err(RequestError::ParseError(format!("Error reading response text: {:?}", e)))
    }
}

fn get_element_text(document: &Html, selector: &Selector) -> Option<String> {
    let element_option = document.select(selector).next();
    
    return Some(element_option?.text().collect::<Vec<_>>().join("\n"));
}

fn get_background_image(document: &Html, selector: &Selector) -> Option<String> {
    let element_option = document.select(selector).next();
    let element_style = element_option?.attr("style")?;
    
    let background_url = element_style
        .split("(").last()?
        .strip_suffix(")")?.to_string();

    return Some(background_url);
}

fn create_store_from_content(document: &Html, store_url: &str, items: HashSet<BoothItem>) -> BoothStore {
    let nickname = get_element_text(document, &NICKNAME_SELECTOR).unwrap_or("(parse error)".to_string());
    let name = get_element_text(document,  &NAME_SELECTOR).unwrap_or(nickname.clone());
    let description = get_element_text(document,  &DESCRIPTION_SELECTOR).unwrap_or("(not found)".to_string());
    let icon_url = get_background_image(document,  &ICON_SELECTOR).unwrap_or("https://booth.pm/favicon.ico".to_string());

    return BoothStore::new(
        name,
        description,
        store_url,
        icon_url,
        Vec::from_iter(items)
    );
}

fn get_max_page_count(document: &Html) -> Option<u32> {
    let last_page_element_result = document.select(&LAST_PAGE_SELECTOR).next();
    let last_page_element = last_page_element_result?;

    // Get page number from "href" attribute
    let last_page_href = last_page_element.attr("href").unwrap_or_default();
    let last_page_str = last_page_href.split("=").last().unwrap_or_default();
        
    // Convert to u32
    let parse_result = last_page_str.parse::<u32>();
    if let Err(parse_err) = parse_result {
        log::warn!("Unable to get page count: Could not parse '{}' as i32: {}", last_page_str, parse_err);
        return None;
    }

    return Some(parse_result.unwrap());
}

fn get_items_from_content(document: &Html) -> Vec<BoothItem> {
    let mut item_list: Vec<BoothItem> = Vec::new();

    let item_elements = document.select(&ITEM_SELECTOR);

    for element in item_elements {
        let item_data_opt = element.attr("data-item");
        if item_data_opt.is_none() {
            continue;
        }

        // Deserialize
        let item_result = serde_json::from_str::<BoothItem>(item_data_opt.unwrap());

        let item = match item_result {
            Ok(i) => i,
            Err(e) => {
                log::error!("Unable to deserialize item: {}", e);
                continue;
            }
        };

        item_list.push(item);
    }

    return item_list;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_page_count_from_content() {
        let input = Html::parse_fragment("<li><a class=\"nav-item last-page\" href=\"/items?page=5\"><i class=\"icon-angle-double-right no-margin s-1x\"></i></a></li>");
        let expected = Some(5);

        let result = get_max_page_count(&input);

        assert_eq!(result, expected);
    }
}
