mod utils;
use url::Url;
use substring::Substring;

mod currency_exchange;
use currency_exchange::get_exchange_rate;

use crate::currency_exchange::convert_item_price;
use crate::objects::booth_item::BoothItem;
use crate::objects::booth_store::BoothStore;

pub mod errors;
use errors::BoothRequestError;

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

// Last page detection
const LAST_PAGE_SKIP: &str= "=";
const LAST_PAGE_START_STRING: &str= "<a class=\"nav-item last-page\" href=\"/items?";

// Store
const STORE_NAME_START: &str="<span class=\"shop-name-label display_title\">";
const STORE_NAME_END: &str = "</span>";

const STORE_NICK_START: &str = "<a class=\"nav\" title=\"Home\" href=\"/\">";
const STORE_NICK_END: &str = "</a>";

const STORE_DESC_START: &str = "<div class=\"description\"><div class=\"booth-description\"><div class=\"autolink u-mb-[0-9]+\"><div>";
const STORE_DESC_END: &str = "</div>";

const STORE_ICON_START: &str = "<div class=\"avatar-image\" style=\"background-image: url(";
const STORE_ICON_END: &str = ")";

// Item
const ITEM_DATA_START: &str = "data-item=\"";
const ITEM_DATA_END: &str = "\"";

#[derive(Clone)]
pub struct BoothClient {
    client: reqwest::Client,
    convert_currency: bool,
    currency_target: String,
    currency_source: String
}


impl BoothClient {
    pub fn with_defaults() -> BoothClient {
        return BoothClient::with_client(
            reqwest::Client::builder()
            .user_agent(SELF_USER_AGENT)
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .unwrap_or_default()
        );
    }

    pub fn with_client(client: reqwest::Client) -> BoothClient {
        return BoothClient{
            client,
            convert_currency: true,
            currency_target: "EUR".to_string(),
            currency_source: "JPY".to_string(),
        };
    }

    pub fn set_currency_conversion_options(&mut self, enabled: bool, source: &str, target: &str) {
        self.convert_currency = enabled;
        self.currency_source = source.to_string();
        self.currency_target = target.to_string();
    }

    pub async fn get_booth_store(&self, url: &str, max_pages: i32, unblur_nsfw: bool) -> Result<BoothStore, BoothRequestError> {
    
        // Url checks
        let mut url_obj = match Url::parse(url) {
            Ok(url) => url,
            Err(e) => return Err(BoothRequestError::InvalidUrl(e.to_string()))

        };
        
        match url_obj.domain() {
            Some(domain) => if !domain.ends_with(".booth.pm") {
                return Err(BoothRequestError::NotBoothUrl())
            },
            None => return Err(BoothRequestError::InvalidUrl("Missing domain!".to_string()))
        };

        let url_path = url_obj.path();
        if !(url_path.contains("items") || url_path.contains("item_lists")) {
            url_obj.set_path(&format!("{url_path}items"));
        }

        let mut page_count: Option<i32> = None;
        let mut items: Vec<BoothItem> = Vec::new();

        // Get exchange rate for currency conversion
        //TODO: Make target currency configurable!
        let exchange_res  = get_exchange_rate(&self.client, &self.currency_source, &self.currency_target).await;
        
        let exchange_rate = match &exchange_res {
            Ok(exc) => Some(exc.rate),
            Err(_) => None
        };

        if let Err(e) = &exchange_res {
            eprintln!("ERROR: Unable to get currency exchange rate: {e}");
        }

        let mut i = 1;
        loop {
            println!("Fetching page {}/{:#?}...", i, page_count);
            url_obj.set_query(Some(&format!("page={}", i)));

            let result = get_page(&self.client, &url_obj, unblur_nsfw).await;

            let content = match result {
                Ok(response) => response,
                Err(status) => return Err(status),
            };

            // Detect number of total pages
            if page_count.is_none() {
                page_count = get_page_count_from_content(&content);
                
                if let Some(pc) = page_count {
                    println!("Detected maximum page count {}", pc);
                }
            }
        
        
            // Get items from content
            let new_items = get_items_from_content(&content, exchange_rate);
            let new_items_iter = new_items.into_iter();
            
            // Add new items to item list
            let mut new_items_count = 0;
            for new_item in new_items_iter {
                //println!("Got: {new_item}");
                items.push(new_item);
                new_items_count += 1;
            }
            println!("New items: {}", new_items_count);

            // Exit condition
            if new_items_count == 0 || i >= max_pages || (!page_count.is_none() &&  i >= page_count.unwrap()) {
                println!("Last page reached!");
                url_obj.set_query(None);
                return Ok(create_store_from_content(&content, url_obj.as_ref(), items));
            }

            i += 1;
        }
    }
}




pub async fn get_page(client: &reqwest::Client, url: &Url, allow_adult: bool) -> Result<String, BoothRequestError> {
    let mut builder = client.get(url.to_string())
        .header(reqwest::header::ACCEPT_LANGUAGE, ACCEPTED_LANGUAGE);

    if allow_adult {
        builder = builder.header(reqwest::header::COOKIE, ADULT_COOKIE);
    }

    // Request data
    let result = builder.send().await;

    let response = match result {
        Ok(resp) => resp,
        Err(e) => return Err(BoothRequestError::NetworkError(format!("Request failed: {e}")))
    };

    let status = response.status();
    println!("Status: {status}");

    if !status.is_success() {
        return Err(BoothRequestError::HttpError(
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
        Err(e) => Err(BoothRequestError::ParseError(format!("Error reading response text: {:?}", e)))
    }
}

fn create_store_from_content(content: &String, store_url: &str, items: Vec<BoothItem>) -> BoothStore {
    let nickname = match utils::get_value_between_snippets(content, STORE_NICK_START, STORE_NICK_END) {
        Some(name) => name,
        None => "(parse error)".to_string()
    };

    let name = match utils::get_value_between_snippets(content, STORE_NAME_START, STORE_NAME_END) {
        Some(name) => name,
        None => nickname.clone()
    };

    let description = match utils::get_value_between_snippets(content, STORE_DESC_START, STORE_DESC_END) {
        Some(desc) => desc,
        None => "(not found)".to_string()
    };

    let icon_url = match utils::get_value_between_snippets(content, STORE_ICON_START, STORE_ICON_END) {
        Some(desc) => desc,
        None => "https://booth.pm/favicon.ico".to_string()
    };

    return BoothStore::new(
        name,
        nickname,
        description,
        store_url,
        icon_url,
        items
    );
}

fn get_page_count_from_content(content: &String) -> Option<i32> {
    let mut page_string = utils::get_value_between_snippets(content, LAST_PAGE_START_STRING, "\"")?; 

    // Cut off parts before number
    let page_num_index = utils::index_of(&page_string, LAST_PAGE_SKIP, 0);
    if let Some(index) = page_num_index {
        page_string = page_string.substring(index + 1, page_string.len()).to_string();
    }

    // Cut off parts after number
    let param_index = utils::index_of(&page_string, "&", 0);
    if let Some(index) = param_index {
        page_string = page_string.substring(0, index).to_string();
    }

    let param_index = utils::index_of(&page_string, "\"", 0);
    if let Some(index) = param_index {
        page_string = page_string.substring(0, index).to_string();
    }

    // Convert to string (page_string is hopefully a valid integer now)
    match page_string.parse::<i32>() {
        Ok(page_count) => return Some(page_count),
        Err(error) => {
            eprintln!("Unable to get page count: Could not parse '{}' as i32: {}", page_string, error);
            return None;
        }
    };
}

fn get_items_from_content(content: &String, exchange_rate: Option<f32>) -> Vec<BoothItem> {
    let mut item_list: Vec<BoothItem> = Vec::new();
    let mut offset  = 0;
    let mut item_result;

    loop {
        // Find item data string
        (item_result, offset) = utils::get_value_between_snippets_offset(content, ITEM_DATA_START, ITEM_DATA_END, offset);

        let item_data_string = match item_result { 
            Some(data) => data.replace("&quot;", "\""),
            None => return item_list
        };

        // Deserialize
        let item_result = serde_json::from_str::<BoothItem>(item_data_string.trim());

        let mut item = match item_result {
            Ok(i) => i,
            Err(e) => {
                eprintln!("Unable to deserialize item: {}", e);
                continue;
            }
        };

        if let Some(rate) = exchange_rate {
            item.local_price = convert_item_price(&item.price, rate, &self.currency_target);
        }

        item_list.push(item);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_page_count_from_content() {
        let input = "<li><a class=\"nav-item last-page\" href=\"/items?page=5\"><i class=\"icon-angle-double-right no-margin s-1x\"></i></a></li>".to_string();
        let expected = Some(5);

        let result = get_page_count_from_content(&input);

        assert_eq!(result, expected);
    }
}
