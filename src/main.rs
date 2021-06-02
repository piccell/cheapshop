#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod config;
mod routes;
mod models;

use models::items;
use jfs::Store;

fn main() {
    let file_store = items::FileStores {
                        articles: Store::new("articles").unwrap(),
                        shops: Store::new("shops").unwrap()
                    };

    
    rocket::custom(config::from_env())
    .manage(file_store)
    .mount("/articles", routes![        
        routes::articles::list, 
        routes::articles::edit_page,
        routes::articles::create,        
        routes::articles::save,                
    ])    
    .mount("/shops", routes![        
        routes::shops::list, 
        routes::shops::edit_page,
        routes::shops::create,        
        routes::shops::save,
    ])        
    .launch();
}
