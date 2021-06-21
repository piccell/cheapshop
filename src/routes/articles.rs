use crate::models::article::{Article, ArticleForm};
use crate::models::file_stores::FileStores;
use crate::models::price::Price;
use crate::models::shop::Shop;
use crate::routes::main_page;
use maud::{html, Markup};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use std::iter::FromIterator;

#[get("/")]
pub fn list(store: State<FileStores>) -> Markup {
    use crate::models::item_name::ItemName;

    let mut entries = Vec::from_iter(store.articles.all::<Article>().unwrap().clone());
    entries.sort_by(|(_, a), (_, b)| a.name_upper().cmp(&b.name_upper()));

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
pub fn edit_page(id: String, store: State<FileStores>) -> Markup {
    let article = {
        if id.eq("0") {
            Article {
                name: "".to_owned(),
            }
        } else {
            store.articles.get(&id).unwrap()
        }
    };

    let prices: Vec<Price> = {
        if id.eq("0") {
            vec![]
        } else {
            match store.prices.get(&id) {
                Ok(prices) => prices,
                _error => vec![],
            }
        }
    };

    let priced_shops = prices
        .iter()
        .map(|x| (store.shops.get(&x.shop_id).unwrap(), x.value))
        .collect::<Vec<(Shop, usize)>>();

    let form = main_page::item_detail(&id, &article, "Fiche article", "/articles");

    let prices = html! {
        @if priced_shops.len() > 0 {
            table class="striped" {
                tbody {
                    @for ps in priced_shops {
                        tr {
                            td { (ps.0.name) }
                            td { (format!("{} euros",ps.1 as f32 / 100.0)) }
                        }
                    }
                }
            }
        }
        @else {
            p {"Aucun prix enregistré"}
        }
    };

    let content = html! {
        div class="row" {
            div class="col s12 l6" {
                (form)
                div class="divider" {}
                a href={"/prices/"(id)} {"Ajouter un prix"}
                (prices)
            }
        }
    };

    main_page::page(content)
}

#[post("/", data = "<form>")]
pub fn create(form: Form<ArticleForm>, store: State<FileStores>) -> Redirect {
    let form = form.into_inner();

    let article = Article { name: form.name };

    store
        .articles
        .save(&article)
        .expect("erreur sauvegarde articles");

    Redirect::to("/articles")
}

#[put("/", data = "<form>")]
pub fn save(form: Form<ArticleForm>, store: State<FileStores>) -> Redirect {
    let form = form.into_inner();

    let article_updated = Article { name: form.name };

    store
        .articles
        .save_with_id(&article_updated, &form.uuid)
        .expect("erreur fichier articles");

    Redirect::to("/articles")
}
