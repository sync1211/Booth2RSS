use std::fmt;
use rss::{CategoryBuilder, Channel, ChannelBuilder, ImageBuilder};

use crate::objects::booth_item::BoothItem;

#[derive(Clone)]
pub struct BoothStore {
    name: String,
    description: String,
    url: String,
    icon_url: String,
    pub items: Vec<BoothItem>
}

impl BoothStore {
    pub fn new(name: String, description: String, url: &str, icon_url: String, items: Vec<BoothItem>) -> BoothStore {
        return BoothStore {
            name: name,
            description: description,
            url: url.to_owned(),
            icon_url: icon_url,
            items: items
        }
    }

    pub fn as_rss(&self, filter_unavailable: bool, filter_nsfw: bool, vrc_only: bool, ttl: i32) -> Channel {
        // Add items 
        let mut items = Vec::new();
        for item in self.items.iter() {
            // Filter: Unavailable
            let is_unavailable = item.is_sold_out || item.is_end_of_sale;

            if (filter_unavailable && is_unavailable)
                || (filter_nsfw && item.is_adult)
                || (vrc_only && !item.is_vrchat) {
                continue;
            }

            items.push(item.as_rss());
        }

        // Assemble RSS
        let icon = ImageBuilder::default()
            .url(self.icon_url.to_string())
            .link(self.url.to_string())
            .title(self.name.to_string())
            .build();
        let category = CategoryBuilder::default()
            .name("store".to_string())
            .build();
        let channel = ChannelBuilder::default()
            .ttl(ttl.to_string())
            .title(self.name.to_string())
            .link(self.url.to_string())
            .image(icon)
            .items(items)
            .description(self.description.to_string())
            .generator("Booth2RSS".to_string())
            .category(category)
            .build();

        return channel;
    }
}

impl fmt::Display for BoothStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]({})", self.name, self.url)
    }
}
