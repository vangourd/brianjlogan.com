use sycamore::prelude::*;
use mdsycx::{parse,MDSycX};
use crate::model::posts::{get_post};

#[component]
pub async fn PostList<G: Html>(cx: Scope<'_>) -> View<G> {


    let req = get_post(String::from("test.md"));

    let result = req.await;

    match result {
        Ok(value) => {
            let parsed = parse::<()>(create_ref(cx, value)).unwrap();

            view! { cx,
                div(class="markdown-container") {
                    MDSycX(body=parsed.body)
                }
            }
        }
        Err(error) => {
            println!("{error:?}");
            view! { cx,
                table {
                    tbody {
                        (format!("Unable to render post. {error:?}"))
                    }
                }
            }
        }
    }
    
    
}