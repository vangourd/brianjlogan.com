use sycamore::prelude::*;
use mdsycx::{parse,MDSycX};
use crate::model::posts::{get_post};

#[component]
pub async fn Post<G: Html>(cx: Scope<'_>) -> View<G> {

        let post = String::from("test.md");

        let req = get_post(post);

        match req.await {
            Ok(value) => {
                match parse::<()>(create_ref(cx, value)) {
                    Ok(parsed) => {
                        view! { cx,
                            div (class="post") {
                                MDSycX(body=parsed.body)
                            }
                        }
                    }
                    Err(error) => {
                        println!("{error:?}");
                        view! { cx,
                            table {
                                tbody {
                                    (format!("Unable to render post."))
                                }
                            }
                        }
                    }
                } 
            }
            Err(error) => {
                println!("{error:?}");
                view! { cx,
                    table {
                        tbody {
                            (format!("Unable to fetch post."))
                        }
                    }
                }
            }
        }
    
    
}