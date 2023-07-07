use sycamore::prelude::*;
use mdsycx::{parse,MDSycX};
use crate::model::posts::{get_post};

#[component]
pub async fn PostComponent<G: Html>(cx: Scope<'_>, slug: String) -> View<G> {

        let req = get_post(slug);

        match req.await {
            Ok(value) => {
                match parse::<()>(create_ref(cx, value)) {
                    Ok(parsed) => {
                        view! { cx,
                            div{
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