use sycamore::prelude::*;
use mdsycx::{parse,MDSycX};
use crate::model::posts::{get_post};

#[component]
pub fn PostList<G: Html>(cx: Scope) -> View<G> {


    let markdown = get_post(String::from("test.md"));

    let markdown = markdown.await();

    let parsed = parse::<()>(markdown).unwrap();
    
    view! { cx,
        div(class="markdown-container") {
            MDSycX(bind:body=parsed.body())
        }
    }
}