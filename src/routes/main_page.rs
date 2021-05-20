use maud::{html, Markup, DOCTYPE};

fn header() -> Markup {
   html! {
       (DOCTYPE)
       meta charset="utf-8";
       link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/materialize/1.0.0/css/materialize.min.css";
       link href="https://fonts.googleapis.com/icon?family=Material+Icons" rel="stylesheet";
   }
}

fn footer() -> Markup {
   html! {
       script src="https://cdnjs.cloudflare.com/ajax/libs/materialize/1.0.0/js/materialize.min.js";
   }
}

pub fn page(content: Markup) -> Markup {
    html! {
        (header())
        body {           
            nav {
                div class="nav-wrapper orange darken-4" {
                    a href="#!" class="brand-logo" {
                        "Je fais mes courses"
                    }
                }
            }

            div class="container" {
               (content)
            }
        }
        (footer())
    }
}