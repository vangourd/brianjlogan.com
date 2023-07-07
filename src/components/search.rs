use std::{collections::HashMap, result};
use sycamore::prelude::*;
use mdsycx::{parse,MDSycX};
use crate::model::{posts::{get_post}, index::get_inverted_index};

#[component]
pub async fn Search<G: Html>(cx: Scope<'_>) -> View<G> {

        let value = create_signal(cx, String::new());

        let result_class = String::from("hidden");
        // let index = get_inverted_index().await.unwrap();

        // let map: HashMap<String, String> = serde_json::from_str(&index).unwrap();

        // User inputs a query into search box.
        // Once a query is available 
        // Run a rank method using the query against the index.

        view! { cx,

            div(class="searchbar"){
                input(placeholder="🔎",bind:value=value){""}
            }
            ul(class="result hidden"){
                h2{"Found x results:"}
                li(href="/post/"){
                    a(href="/post/"){"Post: My Resume"}
                }
                li(href="/tag/"){
                    a(href="/tag/"){"Posts tagged: Rustlang"}
                }
            }
            
        }

}

