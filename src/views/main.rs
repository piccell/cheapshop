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
					a href="#!" class="brand-logo" {"Courses"}
					a href="#" data-target="mobile-demo" class="sidenav-trigger" {
						i class="material-icons" {"menu"}
					}
					ul class="right hide-on-med-and-down" {
					@for m in &menus {
						li { a href={(m.1)} {(m.0)} }
					}}
				}
			}
			ul class="sidenav" id="mobile-demo" {
			@for m in &menus {
				li { a href={(m.1)} {(m.0)} }
			}}

			div class="container" {
				(content)
			}
			(footer())
		}
	}
}