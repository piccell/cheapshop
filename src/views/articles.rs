use crate::models::article::Article;
use crate::models::file_stores::FileStores;
use crate::models::price::Price;
use crate::models::shop::Shop;
use crate::models::item_name::ItemName;
use crate::views::main;
use crate::views::shared;
use maud::{html, Markup};
use rocket::State;
use std::iter::FromIterator;

pub fn list(store: State<FileStores>) -> Markup {
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

    main::page(content)
}

pub fn edit(id: String, store: State<FileStores>) -> Markup {
	let article = {
		if id.eq("0") {
			Article {
				name: "".to_string(),
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

	let nb_shops = store.get_sorted_shops().len();
/*
        let mut items_sorted = Vec::from_iter(items);
        items_sorted.sort_by(|(_, a), (_, b)| a.name_upper().cmp(&b.name_upper()));

        items_sorted
*/
	let priced_shops = prices
		.into_iter()
		.map(|x| (store.shops.get(&x.shop_id).unwrap(), x))        
		.collect::<Vec<(Shop, Price)>>();
    
    let mut shop_sorted = Vec::from_iter(priced_shops);
    shop_sorted.sort_by(|(a, _), (b, _)| a.name_upper().cmp(&b.name_upper()));        

	let form = shared::item_detail(&id, &article, "Fiche article", "/articles");

	let prices = html! {
		@if shop_sorted.len() > 0 {
		table class="striped" {
			tbody {
			@for ps in &shop_sorted {
				tr {
					td { (ps.0.name) }
						td {
							a href={"/prices/edit/"(id)"/"(ps.1.shop_id)} { (format!("{} €/{}", ps.1.value as f32 / 100.0, ps.1.unit)) }
						}
					}
				}
			}}
		}
		@else {
		p {"Aucun prix enregistré"}
		}
	};

	let content = html! {
		div class="row" {
			div class="col s12 m12 l12" {
				(form)
				div class="divider" {}
				@if shop_sorted.len() < nb_shops  && !id.eq("0") {
					a href={"/prices/"(id)} {"+ Ajouter un prix"}
				}
				(prices)
			}
		}
	};

	main::page(content)
}
