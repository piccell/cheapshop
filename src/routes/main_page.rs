use crate::models::item_name::ItemName;
use maud::{html, Markup, DOCTYPE};

fn header() -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/materialize/1.0.0/css/materialize.min.css";
            link href="https://fonts.googleapis.com/icon?family=Material+Icons" rel="stylesheet";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
        }
        style {
            r#"
			td {
				text-align: center
			}

			table thead tr th {
				text-align: center
			}

			td a:link {
				color: black ;
			}

			td a:visited {
				color:black;
			}

			.cheappest {
				background-color:green;
				color:white;
			}
			"#
        }
    }
}

fn footer() -> Markup {
    html! {
        script src="https://cdnjs.cloudflare.com/ajax/libs/materialize/1.0.0/js/materialize.min.js"{}
        script { "M.AutoInit();"}
    }
}

pub fn page(content: Markup) -> Markup {
    let menus = vec![
        ("Prix", "/prices"),
        ("Articles", "/articles"),
        ("Magasins", "/shops"),
    ];

    html! {
        (header())
        body {
            nav {
                div class="nav-wrapper orange darken-4" {
                    a href="#!" class="brand-logo" {"Mes courses"}
                    a href="#" data-target="mobile-demo" class="sidenav-trigger" {
                        i class="material-icons" {"menu"}
                    }
                    ul class="right hide-on-med-and-down" {
                        @for m in &menus {
                            li { a href={(m.1)} {(m.0)} }
                        }
                    }
                }
            }
            ul class="sidenav" id="mobile-demo" {
                @for m in &menus {
                    li { a href={(m.1)} {(m.0)} }
                }
            }

            div class="container" {
               (content)
            }
            (footer())
        }
    }
}

pub fn item_detail<T: ItemName>(id: &String, item: &T, title: &str, route: &str) -> Markup {
    html! {
        h5 {(title)}
        div class="row" {
            form class="col s12" action={(route)} method="post" {
                @if !id.eq("0") {
                    input type="hidden" name="_method" value="put";
                }
                input type="hidden" name="uuid" value={(id)}
                div class="row valign-wrapper" {
                    div class="input-field col s6" {
                        input id="name" type="text" name="name" value={(item.name())};
                        label class="active" for="name" {"Nom"}
                    }
                    div class="col s6 input-field" {
                        input type="submit" value="OK" class="btn green accent-4";
                    }
                }
            }
        }
    }
}
