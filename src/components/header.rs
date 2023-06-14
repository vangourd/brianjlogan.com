use sycamore::prelude::*;

#[component]
pub fn Header<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="header") {
            div(class="top_header"){
                img(alt="picture of brian", src="/images/profile.png",class="profile-photo")
                h1{"brianjlogan.com"}
            }
            nav{
                ul{
                    li{"programming"}
                    li{"dev-sec-ops"}
                    li{"food"}
                    li{"biking"}
                    a(href="https://docs.google.com/forms/d/e/1FAIpQLSdB3IfeDbLuTPJk6YsyNZBubMvMOwgtA4fl6qKQJCUyssjD8Q/viewform"){"contact"}
                    li{"resume"}
                }
                input(placeholder="🔎"){""}
            }

        }
    }
}