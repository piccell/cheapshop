use crate::models::item_name::ItemName;
use maud::{html, Markup};

pub fn item_detail<T: ItemName>(id: &String, item: &T, title: &str, route: &str) -> Markup {
	html! {
		h5	{ a href="/" { { icon class="large material-icons" {"arrow_back"} (title)} } }
		div class="row" {
			form class="col s12" action={(route)} method="post" {
			@if !id.eq("0") {
				input type="hidden" name="_method" value="put";
			}
				input type="hidden" name="uuid" value={(id)}
				div class="row valign-wrapper" {
					div class="input-field col s10 m4" {
						input id="name" type="text" name="name" value={(item.name())};
						label class="active" for="name" {"Nom"}
					}
					div class="col s2 input-field" {
					@if id.eq("0") {
						input type="submit" value="OK" class="btn green accent-4";
					}
					@else {
						input type="submit" value="Ok" class="btn orange accent-4";						
					}}
				}
			}	
		}
	}
}