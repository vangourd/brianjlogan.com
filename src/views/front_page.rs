use sycamore::prelude::*;

use brianjlogan::header::Header;
use brianjlogan::post_list::PostList;

#[component]
pub fn FrontPage<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Header
        PostList
    }
}