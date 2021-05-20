use jfs::Store;

#[derive(Clone)]
pub struct FileStores {
   pub articles: Store,
   pub shops: Store,
}