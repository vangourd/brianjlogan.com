use sycamore::prelude::*;

#[component]
pub fn Header<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="header") {
            img(alt="picture of brian", src="images/profile.png",class="profile-photo")
            h1{"BrianJlogan.com"}
            div(class="sub_header"){
                p{
                "let's talk: " 
                    strong{a(href="#"){"everything "}}
                    a{"tech food biking"}
                    input(placeholder="search"){""}
            }
            }

        }
    }
}