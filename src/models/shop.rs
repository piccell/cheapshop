use crate::models::item_name::ItemName;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Shop {
    pub name: String,
}

impl ItemName for Shop {
    fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, FromForm)]
pub struct ShopForm {
    pub uuid: String,
    pub name: String,
}
