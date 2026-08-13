use std::fmt;
use crate::objects::booth_item::BoothItem;
use crate::utils::sanitize_xml;

pub struct BoothStore {
    name: String,
    nickname: String,
    description: String,
    url: String,
    icon_url: String,
    pub items: Vec<BoothItem>
}

impl BoothStore {
    pub fn new(name: String, nickname: String, description: String, url: &str, icon_url: String, items: Vec<BoothItem>) -> BoothStore {
        return BoothStore {
            name: name,
            nickname: nickname,
            description: description,
            url: url.to_owned(),
            icon_url: icon_url,
            items: items
        }
    }

    pub fn as_rss(&self, filter_unavailable: bool, filter_nsfw: bool, vrc_only: bool, ttl: i32) -> String {

        let name = sanitize_xml(&self.name);
        let nickname = sanitize_xml(&self.nickname);
        let url = &self.url;
        let description = sanitize_xml(&self.description);
        let icon_url = &self.icon_url;
        
        // Assemble RSS
        let mut rss = "<?xml version=\"1.0\" encoding=\"UTF-8\" ?>".to_string();
        rss.push_str("<rss version=\"2.0\">");
        rss.push_str("<channel>");
        rss.push_str(&format!("<title>{name}</title>"));
        rss.push_str(&format!("<link>{url}</link>"));
        rss.push_str(&format!("<description>{description}</description>"));
        rss.push_str("<generator>Booth2RSS (Rust)</generator>");
        rss.push_str("<image>");
        rss.push_str(&format!("<url>{icon_url}</url>"));
        rss.push_str(&format!("<title>{nickname}</title>"));
        rss.push_str(&format!("<link>{icon_url}</link>"));
        rss.push_str("</image>");
        rss.push_str("<category>Store</category>");
        rss.push_str(&format!("<ttl>{ttl}</ttl>"));

        // Add items 
        for item in self.items.iter() {
            // Filter: Unavailable
            let is_unavailable = item.is_sold_out || item.is_end_of_sale;

            if (filter_unavailable && is_unavailable)
                || (filter_nsfw && item.is_adult)
                || (vrc_only && !item.is_vrchat) {
                continue;
            }

            rss.push_str(&item.as_rss());
        }

        rss.push_str("</channel>");
        rss.push_str("</rss>");

        return rss;
    }
}

impl fmt::Display for BoothStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]({})", self.name, self.url)
    }
}
