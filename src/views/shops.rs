use maud::{html, Markup};
use crate::models::shop::Shop;
use crate::views;

pub fn list(entries:Vec<(String, Shop)>) -> Markup {
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
							}}
						}
					}
				}
			}
		}
	};

	views::main::page(content)
}

pub fn detail(id:String, shop:Shop, title:&str, route:&str) -> Markup {
	let form = views::shared::item_detail(&id, &shop, title, route);

	views::main::page(form)	
}