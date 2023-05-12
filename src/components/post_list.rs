use sycamore::prelude::*;
use crate::model::posts;

#[component]
pub fn PostList<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div {
            ul {
                li {"Post1"}
                li {"Post2"}
                li {"Post3"}
            }

        }
    }
}