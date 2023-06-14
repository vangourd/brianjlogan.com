use sycamore::prelude::*;

#[component]
pub fn Header<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="header") {
            div(class="top_header"){
                img(alt="picture of brian", src="images/profile.png",class="profile-photo")
                h1{"brianjlogan.com"}
            }
            nav{
                ul{
                    li{"programming"}
                    li{"dev-sec-ops"}
                    li{"food"}
                    li{"biking"}
                    li{"contact"}
                    li{"resume"}
                }
                input(placeholder="🔎"){""}
            }

        }
    }
}