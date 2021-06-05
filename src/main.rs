#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod config;
mod models;
mod routes;

use jfs::Store;
use models::file_stores::FileStores;

fn main() {
    let file_store = FileStores {
        articles: Store::new("articles").unwrap(),
        shops: Store::new("shops").unwrap(),
        prices: Store::new("prices").unwrap(),
    };

    rocket::custom(config::from_env())
        .manage(file_store)
        .mount("/", routes![routes::prices::list,])
        .mount("/prices", routes![routes::prices::edit_page,])
        .mount(
            "/articles",
            routes![
                routes::articles::list,
                routes::articles::edit_page,
                routes::articles::create,
                routes::articles::save,
            ],
        )
        .mount(
            "/shops",
            routes![
                routes::shops::list,
                routes::shops::edit_page,
                routes::shops::create,
                routes::shops::save,
            ],
        )
        .launch();
}
