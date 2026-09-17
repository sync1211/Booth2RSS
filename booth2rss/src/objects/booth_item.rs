use std::fmt;
use crate::{objects::booth_category::BoothCategory, utils::sanitize_xml};
use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
pub struct BoothItem {
    id: i32,
    name: String,
    category: BoothCategory,
    pub is_adult: bool,
    pub is_end_of_sale: bool,
    pub is_placeholder: bool,
    pub is_sold_out: bool,
    pub is_vrchat: bool,
    pub minimum_stock: Option<i32>,
    pub price: String,
    pub local_price: Option<String>,
    thumbnail_image_urls: Vec<String>,
    url: String
}

impl BoothItem {
    pub fn get_description(&self) -> String {
        let mut content_tags: Vec<&str> = Vec::new();


        if self.is_adult   {
            content_tags.push("[ADULT CONTENT]");
        }

        if self.is_vrchat {
            content_tags.push("[VRChat]");
        }

        if self.is_placeholder {
            content_tags.push("[PLACEHOLDER]");
        }

        let mut description = format!("Category: {}\nPrice: {}", self.category.get_name(), self.price);
        if let Some(lprice) = &self.local_price {
            description.push_str(&format!(" (~ {lprice})"));
        }

        if !content_tags.is_empty() {
            description = format!("{}\n{description}", content_tags.join(" "));
        }

        if self.is_sold_out  {
            description.push_str(" (Sold Out)");
        }

        if self.is_end_of_sale  {
            description.push_str(" (End Of Sale)");
        }

        return sanitize_xml(&description);
	}

    fn get_state_id(&self) -> String {
        let mut state_id = format!("{};{}", self.id, self.price);
        
        if self.is_sold_out {
            state_id.push_str(";EOS");
        }

        if self.is_placeholder {
            state_id.push_str(";PH");
        }

        return state_id;
    }

    pub fn as_rss(&self) -> String {
        let mut display_name = String::new();
        
        if self.is_adult {
            display_name.push_str("🔞 ");
        }
        display_name.push_str(&self.name);

        let display_name_safe = sanitize_xml(&display_name);

        let thumbnail_url = self.thumbnail_image_urls.first().map_or("", |x| x);

        // ID of the current item state
        // Any change in price or availability will be treated as a new entry by RSS readers
        let state_id = self.get_state_id();

        let category_name = self.category.get_name();
        let description = self.get_description();
        let url = &self.url;

        let mut rss = String::new();
        rss.push_str("<item>");
        rss.push_str(&format!("<title>{display_name_safe}</title>"));
        rss.push_str(&format!("<link>{url}</link>"));
        rss.push_str(&format!("<enclosure url=\"{thumbnail_url}\" type=\"image/jpeg\" length=\"0\"/>"));
        rss.push_str("<description>");
        rss.push_str(&description);
        rss.push_str("</description>");
        rss.push_str(&format!("<guid>{state_id}</guid>"));
        rss.push_str(&format!("<category>{category_name}</category>"));
        rss.push_str("</item>");

        return rss;
    }

    pub fn try_detect_currency(&self) -> Option<String> {
        let price = self.price.trim();

        // Extract the last 3 letters of the price (should be three letters)
        let currency = &price[price.len() - 3..];

        if !currency.contains(" ") && !currency.is_empty() && currency.is_ascii() {
            let uppercase = currency.to_ascii_uppercase();
            println!("Detected currency as {uppercase}");
            return Some(uppercase.to_string());
        }

        eprintln!("Failed to detect currency from string '{}'", currency);
        return None;
    }

    pub fn apply_currency_conversion(&mut self, exchange_rate: f32, currency_suffix: &str) -> bool {
        let price_clean = self.price.to_string()
            .replace(",", "")
            .replace(".", "");

        let price_num = &price_clean[0..price_clean.find(" ").unwrap_or(price_clean.len())];
        
        let parse_res = price_num.parse::<f32>();
        if let Err(e) = parse_res {
            eprintln!("Unable to convert '{price_num}' to i32: {e}");
            return false;
        }

        self.local_price = Some(format!("{:.2}{currency_suffix}", parse_res.unwrap() * exchange_rate));
        return true;
    }
}

impl fmt::Display for BoothItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]({})", self.name, self.url)
    }
}
