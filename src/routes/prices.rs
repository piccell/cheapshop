use crate::models::article::Article;
use crate::models::file_stores::FileStores;
use crate::models::price::{self, Price, PriceDeleteForm, PriceForm};
use crate::routes::main_page;
use maud::{html, Markup};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use std::collections::BTreeMap;

#[get("/")]
pub fn list(store: State<FileStores>) -> Markup {
	 let articles = store.get_sorted_articles();
	 let shops = store.get_sorted_shops();
	 let prices: BTreeMap<String, Vec<Price>> = store.prices.all().unwrap();

	 let content = html! {
			h1 {"Prix par unité"}
			div class="row" {
				 div class="col s12 m10 l8" {
					  table {
							thead {
								 tr {
									  td {}
									  @for shop in &shops {
											th {(shop.1.name)}
									  }
								 }
							}
							tbody {
								 @for art in articles {
									  @let store_ids = shops.iter().map(|x| x.0.clone()).collect::<Vec<String>>();
									  tr {
											th {
												 a href={"/articles/"(art.0)} {(art.1.name)}
											}
											(view_article_price_in_shop(&prices, art, store_ids.clone()))
									  }
								 }
							}
					  }
				 }
			}
	 };

	 main_page::page(content)
}

fn view_article_price_in_shop(
	 prices: &BTreeMap<String, Vec<Price>>,
	 art_id: (String, Article),
	 shops: Vec<String>,
) -> Markup {
	 if let Some(article_prices) = prices.get(&art_id.0) {
		  let min_price = article_prices.iter().map(|x| x.value).min().unwrap_or(0);

		  html! {
				@for shop in shops {
					 @if let Some(p) = article_prices.iter().find(|x| x.shop_id.eq(&shop)) {
						  @let bg_class = if p.value == min_price {"green lighten-3 white-text"} else {""};
						  @let p = p.value as f32 / 100.0;

						  td class={(bg_class)} {
								a href={"/prices/view/"(&art_id.0)"/"(&shop)} { (p.to_string()) }
						  }
					 }
					 @else {
						  td {}
					 }
				}
		  }
	 } else {
		  html! {
				@for _ in shops {
					 td {}
				}
		  }
	 }
}

#[get("/<article_id>")]
pub fn add_price_page(article_id: String, store: State<FileStores>) -> Markup {
	let shops = store.get_sorted_shops();
	let mut shop_select = vec![];
	let article = store.articles.get::<Article>(&article_id).unwrap();

	let article_prices = match store.prices.get::<Vec<Price>>(&article_id) {
		Ok(prices) => prices,
		_ => vec![]
	};

	for sp in shops {
		if !article_prices.iter().any(|x| x.shop_id == sp.0) {
			shop_select.push(sp);
		}
	}
	
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
								@for shop in shop_select {
									option value={(shop.0)} {(shop.1.name)}
								}
							}
						}
					}

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

					button class="btn " type="submit" {"Valider"}				
				}
			}
		}		
	};

	main_page::page(content)
}

#[get("/view/<article_id>/<shop_id>")]
pub fn view_page(article_id: String, shop_id: String, store: State<FileStores>) -> Markup {
	let shops = store.get_sorted_shops();
	
	 let article = store.articles.get::<Article>(&article_id).unwrap();

	 let price: Price = match store.prices.get::<Vec<Price>>(&article_id) {
		  Ok(prices) => prices.into_iter().find(|x| x.shop_id.eq(&shop_id)).unwrap(),
		  _ => Price {
				shop_id: shop_id.clone(),
				value: 0,
				unit: "kg".to_string(),
		  },
	 };

	 let content = html! {
		  h3	{"Prix "(article.name)}
		  div class="row" {
				div class="col s12 m6 l3" {
					 div class="input-field" {
						  form action="/prices" method="post" {
								input type="hidden" name="_method" value="delete";
								div class="row" {
									 div class="col s12" {
										  input type="hidden" name="article_id" value={(article_id)} {}
										  input type="hidden" name="shop_id" value={(shop_id)} {}
										  @if let Some(shop) = shops.iter().find(|x| x.0.eq(&shop_id)) {
												h5 {"à "(shop.1.name)}
										  }
									 }
								}
								div class="row " {
									 div class="col s12 l6" {
										  h4 { (price.euros())"€ / "(price.unit)}
									 }
								}

								button class="btn red" type="submit" {"supprimer"}
						  }
					 }
				}
		  }
	 };

	 main_page::page(content)
}

#[post("/", data = "<form>")]
pub fn create(form: Form<PriceForm>, store: State<FileStores>) -> Redirect {
	let form = form.into_inner();

	if let Ok(price) = form.price.parse::<f32>() {
		let price = price::price_per_unit(price, form.quantity, form.unit.clone());
		let price = (price * 100.0) as usize;

		let new_price = Price {
			shop_id: form.shop_id.clone(),
			value: price,
			unit: form.unit.clone(),
		};

		let mut article_prices: Vec<Price> = match store.prices.get(&form.article_id) {
			Ok(prices) => prices,
			_ => vec![],
		};

		match article_prices.iter().find(|x| x.shop_id.eq(&form.shop_id)) {
			Some(_) => {}
			_ => {
					article_prices.push(new_price);
			}
		}

		store
			.prices
			.save_with_id(&article_prices, &form.article_id)
			.expect("erreur sauvegarde du prix");

		Redirect::to("/")
	} else {
		Redirect::to("/")
	}
}

#[delete("/", data = "<form>")]
pub fn remove(form: Form<PriceDeleteForm>, store: State<FileStores>) -> Redirect {
	let form = form.into_inner();

	if let Ok(article_prices) = store.prices.get::<Vec<Price>>(&form.article_id) {
		let mut prices = article_prices;
		if let Some(idx) = prices.iter().position(|x| *x.shop_id == form.shop_id) {
			prices.remove(idx);
			dbg!(&prices);
			store
					.prices
					.save_with_id(&prices, &form.article_id)
					.expect("erreur sauvegarde du prix");
		}
	};

	Redirect::to("/")
}
