use std::{collections::HashMap, result};
use serde::de::Visitor;
use sycamore::prelude::*;
use mdsycx::{parse,MDSycX};
use crate::model::posts::{get_post, get_inverted_index};

#[component]
pub async fn Search<G: Html>(cx: Scope<'_>) -> View<G> {

        let index = get_inverted_index().await.unwrap();
        let search_query = create_signal(cx, String::from(""));
        let search_class = create_signal(cx, String::from("result hidden"));
    
        create_effect(cx, move || {
            if search_query.get().len() > 0 {
                search_class.set(String::from("result"))
            } else {
                search_class.set(String::from("result hidden"))
            }
                
        });
        

        view! { cx,

            div(class="searchbar"){
                input(placeholder="🔎",bind:value=search_query){}
            }
            ul(class=search_class.get()){
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

