use std::collections::HashMap;
use sycamore::prelude::*;
use mdsycx::{parse,MDSycX};
use crate::model::{posts::{get_post}, index::get_inverted_index};

#[component]
pub async fn Search<G: Html>(cx: Scope<'_>) -> View<G> {

        let value = create_signal(cx, String::new());

        // let index = get_inverted_index().await.unwrap();

        // let map: HashMap<String, String> = serde_json::from_str(&index).unwrap();

        // User inputs a query into search box.
        // Once a query is available 
        // Run a rank method using the query against the index.

        view! { cx,
            input(placeholder="🔎",bind:value=value){""}
        }

}

