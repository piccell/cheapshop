use maud::{html, Markup};
use crate::routes::main_page;
use crate::models::article::Article;
use rocket::State;
use rocket::response::Redirect;
use rocket::request::Form;
use std::iter::FromIterator;
use crate::models::file_store::FileStores;

#[get("/")]
pub fn list(store:State<FileStores>) -> Markup {
    let mut entries = Vec::from_iter(store.articles.all::<Article>().unwrap().clone());
    entries.sort_by(|(_,a), (_,b)| a.name.cmp(&b.name));

    let content = html! {
        div class="row" {
            div class="col s12 m6" {
                div class="card" {
                    div class="card-image" {
                        a href="/articles/0" class="btn-floating halfway-fab blue darken-3" {
                            i class="material-icons" {"add"}
                        }
                    }

                    div class="card-content" {
                        ul class="collection with-header" {
                            li class="collection-header" {
                                h4 {"Liste des articles"}
                            }
                        @for entry in entries {                       
                            li class="collection-item" {
                                div { (entry.1.name)
                                    a href={"/articles/"(entry.0)} class="secondary-content" {
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
    let method_name = if id.eq("0") {"post"} else {"put"};
    let article = {
        if id.eq("0") {
            Article {name: "".to_owned()}
        }
        else {
            store.articles.get(&id).unwrap()
        }
    };

    let content = html! {
        h5 {"Information article"}
        div class="row" {
            form class="col s12" action="/articles" method=(method_name) {
                div class="row" {
                    div class="input-field col s6" {
                        input id="name" type="text" name="name" value={(article.name)};
                        label class="active" for="name" {"Nom de l'article"}
                    }
                }
                div class="row" {
                    div class="col s12" {
                        a href="/articles" class="btn-flat" {"Annuler"}
                        input type="submit" value="Valider" class="btn blue darken-3";                        
                    }                    
                }
            }       
        }
    };

    main_page::page(content)    
}

#[post("/", data = "<article>")]
pub fn create(article:Form<Article>, store:State<FileStores>) -> Redirect {
    let id = store.articles.save(&article.into_inner());
    dbg!(id.unwrap());

    Redirect::to("/articles")
}
