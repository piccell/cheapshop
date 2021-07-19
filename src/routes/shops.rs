use crate::models::file_stores::FileStores;
use crate::models::shop::{Shop, ShopForm};
use crate::views;
use maud::Markup;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use std::iter::FromIterator;

#[get("/")]
pub fn list(store: State<FileStores>) -> Markup {
   use crate::models::item_name::ItemName;

	let mut entries = Vec::from_iter(store.shops.all::<Shop>().unwrap().clone());
	entries.sort_by(|(_, a), (_, b)| a.name_upper().cmp(&b.name_upper()));

	views::shops::list(entries)
}

#[get("/<id>")]
pub fn edit_page(id: String, store: State<FileStores>) -> Markup {
	let shop = {
		if id.eq("0") {
			Shop {
				name: "".to_owned(),
			}
		} else {
			store.shops.get::<Shop>(&id).unwrap()
		}
	};

	views::shops::detail(id, shop, "Fiche magasin", "/shops")
}

#[post("/", data = "<form>")]
pub fn create(form: Form<ShopForm>, store: State<FileStores>) -> Redirect {
	let shop_form = form.into_inner();

	let shop = Shop {
		name: shop_form.name,
	};

	store.shops.save(&shop).expect("erreur sauvegarde shops");

	Redirect::to("/shops")
}

#[put("/", data = "<form>")]
pub fn save(form: Form<ShopForm>, store: State<FileStores>) -> Redirect {
	let shop_form = form.into_inner();

	let shop_updated = Shop {
		name: shop_form.name,
	};

	store
		.shops
		.save_with_id(&shop_updated, &shop_form.uuid)
		.expect("erreur fichier shops");

	Redirect::to("/shops")
}
