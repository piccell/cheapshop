use serde::{Deserialize, Serialize};
use crate::models::item_name::ItemName;

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
