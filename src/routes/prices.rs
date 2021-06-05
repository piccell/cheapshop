use crate::models::file_stores::FileStores;
use crate::models::shop::Shop;
use crate::routes::main_page;
use maud::{html, Markup};
use rocket::State;
use std::collections::BTreeMap;

#[get("/")]
pub fn list(store: State<FileStores>) -> Markup {
    let content = html! {
       h1 {"Prices"}
    };

    main_page::page(content)
}

#[get("/<id>")]
pub fn edit_page(id: String, store: State<FileStores>) -> Markup {
    use std::iter::FromIterator;
	use crate::models::item_name::ItemName;

    let shops: BTreeMap<String, Shop> = store.shops.all().unwrap();
    let mut sorted = Vec::from_iter(shops);
    sorted.sort_by(|(_, a), (_, b)| a.name_upper().cmp(&b.name_upper()));

    let content = html! {
       div class="input-field col s12" {
          select {
             option value="" disabled selected {"Choisir"}
             @for sh in sorted {
                option value={(sh.0)} {(sh.1.name)}
             }
          }
       }
    };

    main_page::page(content)
}
