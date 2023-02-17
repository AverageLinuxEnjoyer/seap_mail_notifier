use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Subscription {
    // #[serde(default)]
    pub id: i32,
    pub id_user: i32,
    pub min_price: Option<i32>,
    pub max_price: Option<i32>,
    pub title_keywords: Option<Vec<String>>,
    pub desc_keywords: Option<Vec<String>>,
    pub additional_info_keywords: Option<Vec<String>>,
}
