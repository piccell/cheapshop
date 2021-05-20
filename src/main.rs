#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod config;
mod routes;
mod models;

use models::file_store;
use jfs::Store;

fn main() {
    let file_store = file_store::FileStores {
                        articles: Store::new("article").unwrap(),
                        shops: Store::new("shop").unwrap()
                    };

    
    rocket::custom(config::from_env())
    .manage(file_store)
    .mount("/articles", routes![        
        routes::articles::list, 
        routes::articles::edit_page,
        routes::articles::create,        
    ])    
    .launch();
}
