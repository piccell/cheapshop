use maud::{html, Markup};
use crate::models::article::Article;
use crate::models::shop::Shop;
use crate::models::price::Price;
use crate::views::main;
use std::collections::BTreeMap;

pub fn list(
	articles:Vec<(String, Article)>,
	shops:Vec<(String, Shop)>,
	prices:BTreeMap<String, Vec<Price>>
) -> Markup {
	let content = html! {
		h1 {"Prix"}
		div class="row" {
			div class="col s12 m7 l8" {
				table {
					tbody {
					@for art in articles {
						tr {
							th {
								a href={"/articles/"(art.0)} {(art.1.name)}
							}
							(view_article_price_in_shop(&prices, art, &shops))
						}}
					}
				}
			}
		}
  	};

  main::page(content)
}

fn view_article_price_in_shop(
	prices: &BTreeMap<String, Vec<Price>>,
	art_id: (String, Article),
	shops: &Vec<(String, Shop)>,
) -> Markup {
	if let Some(article_prices) = prices.get(&art_id.0) {
		if let Some(min_price) = article_prices.iter().min_by_key(|p| &p.value) {
			if let Some(cheap_shop) = shops.iter().find(|x| x.0.eq(&min_price.shop_id)) {
				let p = min_price.value as f32 / 100.0;

				html! {
					td {
						(cheap_shop.1.name) br;
						(p.to_string())" €/"(min_price.unit)
						}
				}
			} else {
				html! {
					td;
				}
			}
		} else {
			html! {
				td;
			}
		}
	} else {
		html! {
			td;
		}
	}
}

pub fn article_list(article:Article, article_id:String, shops:Vec<(String, Shop)>) -> Markup {
	let content = html! {
		h3	{"Prix "(article.name)}
		div class="row" {
			 div class="col s12 m6 l3" {
				  form action="/prices" method="post" {
						input type="hidden" name="article_id" value={(article_id)};
						div class="row" {
							 div class="col s12 input-field" {
								  select name="shop_id" {
										option value="" disabled selected {"Choisir un magasin"}
										@for shop in shops {
											 option value={(shop.0)} {(shop.1.name)}
										}
								  }
							 }
						}
	
						(article_pricing_html())
						button class="btn " type="submit" {"Valider"}
				  }
			 }
		}
	};
	
	main::page(content)
}

pub fn article_edit(
	article: Article, 
	article_id: String,
	price: Price,
	shop_id: String,
	shops: Vec<(String, Shop)>
) -> Markup {
	let content = html! {
		h5	{ a href="/" { { icon class="large material-icons" {"arrow_back"} (article.name)} } }
		div class="row" {
			 div class="col s12 m6 l3" {
				  div class="input-field" {
						form action="/prices" method="post" {
							input type="hidden" name="_method" value="put";
							input type="hidden" name="article_id" value={(article_id)};
							input type="hidden" name="shop_id" value={(shop_id)};

							div class="row" {
								div class="col s12" {
								@if let Some(shop) = shops.iter().find(|x| x.0.eq(&shop_id)) {
									h5 {"Prix pour "(shop.1.name)}
									h5 {(price.euros())" €/"(price.unit)}
								}}
							}
							(article_pricing_html())
							div class="row" {
								div class="col s6 l6" {
									form action="/prices" method="post" {
										input type="hidden" name="_method" value="delete";
										input type="hidden" name="article_id" value={(article_id)} {}
										input type="hidden" name="shop_id" value={(shop_id)} {}

										button class="btn red" type="submit" {"supprimer"}
									}
								}
								div class="col s6 l6" {
									button class="btn" type="submit" {"modifier"}
								}
							}
						}
				  	}
			 	}
			}
  		};

  	main::page(content)	
}

fn article_pricing_html() -> Markup {
	html! {
		 div class="row" {
			  div class="col s12 input-field" {
					input type="text" id="price" placeholder="prix" class="validate" name="price";
					label for="price" {"Prix"}
			  }
		 }
		 div class="row" {
			  div class="col s8 input-field" {
					input type="text" id="quantity" placeholder="quantité" class="validate" name="quantity";
					label for="quantity" {"Quantité"}
			  }

			  div class="col s4 input-field" {
					select name="unit" {
						 optgroup label="poids" {
							  option value="kg" {"Kg"}
							  option value="g" {"g"}
						 }
						 optgroup label="volume" {
							  option value="l" {"l"}
							  option value="cl" {"cl"}
							  option value="ml" {"ml"}
						 }
					}
					label {"Unité"}
			  }
		 }
	}
}
