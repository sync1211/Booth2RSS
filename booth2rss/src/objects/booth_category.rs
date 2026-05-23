use std::fmt;
use crate::objects::multi_language_name::MultiLanguageName;
use serde::{Deserialize};


#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BoothCategory {
    name: MultiLanguageName,
    url: String
}

impl BoothCategory {
    pub fn get_name(&self) -> String {
        return self.name.en.to_string();
    }
}

impl fmt::Display for BoothCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]({})", self.name.en, self.url)
    }
}