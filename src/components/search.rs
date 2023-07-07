use std::{collections::HashMap, result};
use sycamore::prelude::*;
use mdsycx::{parse,MDSycX};
use crate::model::posts::{get_post, get_inverted_index};

#[component]
pub async fn Search<G: Html>(cx: Scope<'_>) -> View<G> {

        let search_terms = create_signal(cx, String::new());

        let mut visible = 0;
        create_effect(cx, move || 
            if visible == 0 {
                visible = 1;
            } else {
                visible = 0;
            }
        );

        let mut result: [String; 2] = [
            String::from("result hidden"),
            String::from("result")
        ];
        

        let index = get_inverted_index().await.unwrap();

        view! { cx,

            div(class="searchbar"){
                input(placeholder="🔎",bind:value=search_terms){""}
            }
            ul(class=result[visible]){
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

