use crate::models::item_name::ItemName;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Article {
    pub name: String,
}

impl ItemName for Article {
    fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, FromForm)]
pub struct ArticleForm {
    pub uuid: String,
    pub name: String,
}

pub const UNITS: [(&'static str, f32); 5] = [
    ("g", 1000.0),
    ("kg", 1.0),
    ("ml", 1000.0),
    ("cl", 100.0),
    ("l", 1.0),
];