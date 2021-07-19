use crate::models::article::{Article, ArticleForm};
use crate::models::file_stores::FileStores;
use crate::views;
use maud::Markup;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;

#[get("/")]
pub fn list(store: State<FileStores>) -> Markup {
    views::articles::list(store)
}

#[get("/<id>")]
pub fn edit_page(id: String, store: State<FileStores>) -> Markup {
    views::articles::edit(id, store)
}

#[post("/", data = "<form>")]
pub fn create(form: Form<ArticleForm>, store: State<FileStores>) -> Redirect {
    let form = form.into_inner();

    let article = Article { name: form.name };

    if let Ok(id) = store.articles.save(&article) {
        Redirect::to(format!("/articles/{}", id))
    } else {
        Redirect::to("/articles/0")
    }
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
