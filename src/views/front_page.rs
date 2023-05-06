use sycamore::prelude::*;

use brianjlogan::header::Header;

#[component]
pub fn FrontPage<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Header
    }
}