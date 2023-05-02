use sycamore::prelude::*;

#[component]
pub fn FrontPage<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        h1 { "Welcome to my blog!" }
    }
}