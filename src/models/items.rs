use serde::{Deserialize, Serialize};

pub trait ItemName {
    fn name(&self) -> String;
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Article {
    pub name: String
}

impl ItemName for Article {
    fn name(&self) -> String { self.name.clone() }
}

#[derive(Debug, FromForm)]
pub struct ArticleForm {
    pub uuid:String,
    pub name: String
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Shop {
    pub name: String
}

impl ItemName for Shop {
    fn name(&self) -> String { self.name.clone() }
}

#[derive(Debug, FromForm)]
pub struct ShopForm {
    pub uuid:String,
    pub name: String
}



use jfs::Store;

#[derive(Clone)]
pub struct FileStores {
   pub articles: Store,
   pub shops: Store,
}