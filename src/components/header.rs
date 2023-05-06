use sycamore::prelude::*;

#[component]
pub fn Header<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div {
            i{
                "headshot"
            }
            ul {
                li { "About me"}
                li { "Tech" }
                li { "Food" }
                li { "Coffee" }
                li { "Resume" }
                li { "Posts" }
                li { "Contact Me"}
            }
            a {
                "Sign-in"
            }

        }
    }
}