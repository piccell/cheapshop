use maud::{html, Markup};
use crate::routes::main_page;
use crate::models::items::{Shop, ShopForm};
use rocket::State;
use rocket::response::Redirect;
use rocket::request::Form;
use std::iter::FromIterator;
use crate::models::items::FileStores;

#[get("/")]
pub fn list(store:State<FileStores>) -> Markup {
    let mut entries = Vec::from_iter(store.shops.all::<Shop>().unwrap().clone());
    entries.sort_by(|(_,a), (_,b)| a.name.cmp(&b.name));

    let content = html! {
        div class="row" {
            div class="col s12 l6" {
                div class="card" {
                    div class="card-image" {
                        a href="/shops/0" class="btn-floating halfway-fab blue darken-3" {
                            i class="material-icons" {"add"}
                        }
                    }

                    div class="card-content" {
                        ul class="collection with-header" {
                            li class="collection-header" {
                                h4 {"Liste des magasins"}
                            }
                        @for entry in entries {                       
                            li class="collection-item" {
                                div { (entry.1.name)
                                    a href={"/shops/"(entry.0)} class="secondary-content" {
                                        i class="material-icons" {"edit"}
                                    }                            
                                }
                            }
                        }                
                    }
                }
            }
            }
        }
    };

    main_page::page(content)
}

#[get("/<id>")]
pub fn edit_page(id:String, store:State<FileStores>) -> Markup {
    let shop = {
        if id.eq("0") {
            Shop {name: "".to_owned()}
        }
        else {
            store.shops.get::<Shop>(&id).unwrap()
        }
    };

    // let content = html! {
    //     h5 {"Information magasin"}
    //     div class="row" {
    //         form class="col s12" action="/shops" method="post" {
	// 			@if !id.eq("0") {
	// 				input type="hidden" name="_method" value="put";				
	// 			}
	// 			input type="hidden" name="uuid" value={(id)};
    //             div class="row" {
    //                 div class="input-field col s6" {
    //                     input id="name" type="text" name="name" value={(shop.name)};
    //                     label class="active" for="name" {"Nom du magasin"}
    //                 }
    //             }
    //             div class="row" {
    //                 div class="col s12" {
    //                     a href="/shops" class="btn-flat" {"Annuler"}
    //                     input type="submit" value="Valider" class="btn blue darken-3";                        
    //                 }                    
    //             }
    //         }       
    //     }
    // };

    // main_page::page(content)    
	main_page::item_detail(&id, &shop)
}

#[post("/", data = "<form>")]
pub fn create(form:Form<ShopForm>, store:State<FileStores>) -> Redirect {
	let shop_form = form.into_inner();

	let shop = Shop {
		name: shop_form.name
	};

	store.shops.save(&shop).expect("erreur sauvegarde shops");


    Redirect::to("/shops")
}

#[put("/", data = "<form>")]
pub fn save(form:Form<ShopForm>, store:State<FileStores>) -> Redirect {
	let shop_form = form.into_inner();

	let shop_updated = Shop {
		name: shop_form.name
	};

	store.shops.save_with_id(&shop_updated, &shop_form.uuid).expect("erreur fichier shops");

    Redirect::to("/shops")
}