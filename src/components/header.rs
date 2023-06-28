use sycamore::prelude::*;
use crate::components::search::Search;

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
                    a(href="/tag/programming"){"programming"}
                    a(href="/tag/devops"){"devops"}
                    a(href="/tag/food"){"food"}
                    a(href="/tag/biking"){"biking"}
                    a(href="https://docs.google.com/forms/d/e/1FAIpQLSdB3IfeDbLuTPJk6YsyNZBubMvMOwgtA4fl6qKQJCUyssjD8Q/viewform"){"contact"}
                    a(href="/post/resume"){"resume"}
                }
                
            }
            Search
        }
    }
}