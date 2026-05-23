use serde::{Deserialize};
use serde;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MultiLanguageName {
    pub ja: String,
    pub en: String,
}