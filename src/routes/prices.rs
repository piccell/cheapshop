use crate::models::article::Article;
use crate::models::file_stores::FileStores;
use crate::models::price::{self, Price, PriceDeleteForm, PriceForm};
use crate::views;
use maud::Markup;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;

#[get("/")]
pub fn list(store: State<FileStores>) -> Markup {
    let articles = store.get_sorted_articles();
    let shops = store.get_sorted_shops();
    let prices = store.prices.all().unwrap();

    views::prices::list(articles, shops, prices)
}

#[get("/<article_id>")]
pub fn add_price_page(article_id: String, store: State<FileStores>) -> Markup {
    let shops = store.get_sorted_shops();
    let mut shop_select = vec![];
    let article = store.articles.get::<Article>(&article_id).unwrap();

    let article_prices = match store.prices.get::<Vec<Price>>(&article_id) {
        Ok(prices) => prices,
        _ => vec![],
    };

    dbg!(&article_prices);
    for sp in shops {
        dbg!(sp.0.clone());
        if !article_prices.iter().any(|x| x.shop_id == sp.0) {
            shop_select.push(sp);
        }
    }

    views::prices::edit(article, article_id, shop_select)
}

#[get("/edit/<article_id>/<shop_id>")]
pub fn edit_price_page(article_id: String, shop_id: String, store: State<FileStores>) -> Markup {
    let shops = store.get_sorted_shops();

    let article = store.articles.get::<Article>(&article_id).unwrap();

    let price = match store.prices.get::<Vec<Price>>(&article_id) {
        Ok(prices) => prices.into_iter().find(|x| x.shop_id.eq(&shop_id)).unwrap(),
        _ => Price {
            shop_id: shop_id.clone(),
            value: 0,
            unit: "kg".to_string(),
        },
    };

    views::prices::article_edit(article, article_id, price, shop_id, shops)    
}

#[put("/", data = "<form>")]
pub fn save(form: Form<PriceForm>, store: State<FileStores>) -> Redirect {
    if let Ok(article_prices) = store.prices.get::<Vec<Price>>(&form.article_id) {
        let mut prices = article_prices;
        if let Some(idx) = prices.iter().position(|x| *x.shop_id == form.shop_id) {
            if let Ok(price) = form.price.parse::<f32>() {
                let price = price::price_per_unit(price, form.quantity, form.unit.to_string());
                let price = (price * 100.0) as usize;
                prices[idx].value = price;

                store
                    .prices
                    .save_with_id(&prices, &form.article_id)
                    .expect("erreur sauvegarde du prix");
            }
        }
    };

    Redirect::to("/")
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

        Redirect::to(format!("/articles/{}", &form.article_id))
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

            store
                .prices
                .save_with_id(&prices, &form.article_id)
                .expect("erreur sauvegarde du prix");
        }
    };

    Redirect::to("/")
}
