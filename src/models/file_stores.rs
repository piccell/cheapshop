use crate::models::article::Article;
use crate::models::item_name::ItemName;
use crate::models::shop::Shop;
use jfs::Store;
use std::collections::BTreeMap;
use std::iter::FromIterator;

#[derive(Clone)]
pub struct FileStores {
    pub articles: Store,
    pub shops: Store,
    pub prices: Store,
}

impl FileStores {
    pub fn get_sorted_articles(&self) -> Vec<(String, Article)> {
        let items: BTreeMap<String, Article> = self.articles.all().unwrap();
        let mut items_sorted = Vec::from_iter(items);
        items_sorted.sort_by(|(_, a), (_, b)| a.name_upper().cmp(&b.name_upper()));

        items_sorted
    }

    pub fn get_sorted_shops(&self) -> Vec<(String, Shop)> {
        let items: BTreeMap<String, Shop> = self.shops.all().unwrap();
        let mut items_sorted = Vec::from_iter(items);
        items_sorted.sort_by(|(_, a), (_, b)| a.name_upper().cmp(&b.name_upper()));

        items_sorted
    }
}
