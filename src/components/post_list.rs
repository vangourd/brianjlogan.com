use std::collections::HashMap;

use sycamore::prelude::*;
use mdsycx::{parse,MDSycX};
use crate::model::{posts::{get_post}, index::get_inverted_index};

#[component]
pub async fn PostList<G: Html>(cx: Scope<'_>) -> View<G> {

        let index = get_inverted_index().await.unwrap();

        let map: HashMap<String, String> = serde_json::from_str(&index).unwrap();

        view! { cx,
            table {
                tbody {
                    (format!("{:?}", map))
                }
            }
        }

}