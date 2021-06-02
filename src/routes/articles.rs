use maud::{html, Markup};
use crate::routes::main_page;
use crate::models::items::{Article, ArticleForm, FileStores};
use rocket::State;
use rocket::response::Redirect;
use rocket::request::Form;
use std::iter::FromIterator;

#[get("/")]
pub fn list(store:State<FileStores>) -> Markup {
    let mut entries = Vec::from_iter(store.articles.all::<Article>().unwrap().clone());
    entries.sort_by(|(_,a), (_,b)| a.name.cmp(&b.name));

    let content = html! {
        div class="row" {
            div class="col s12 l6" {
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
    let article = {
        if id.eq("0") {
            Article {name: "".to_owned()}
        }
        else {
            store.articles.get(&id).unwrap()
        }
    };

    main_page::item_detail(&id, &article)
}

#[post("/", data = "<form>")]
pub fn create(form:Form<ArticleForm>, store:State<FileStores>) -> Redirect {
	let article_form = form.into_inner();

	let article = Article {
		name: article_form.name
	};

    store.articles.save(&article).expect("erreur sauvegarde articles");

    Redirect::to("/articles")
}

#[put("/", data = "<form>")]
pub fn save(form:Form<ArticleForm>, store:State<FileStores>) -> Redirect {
	let article_form = form.into_inner();

	let article_updated = Article {
		name: article_form.name
	};

	store.articles.save_with_id(&article_updated, &article_form.uuid).expect("erreur fichier articles");

    Redirect::to("/articles")
}