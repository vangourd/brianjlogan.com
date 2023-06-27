
use sycamore::prelude::*;
use sycamore_router::{Route, Router, HistoryIntegration};
use views::{front_page::FrontPage, not_found::NotFound,post::Post};

mod views;

#[derive(Route)]
enum AppRoutes {
    #[to("/")]
    FrontPage,
    #[to("/post/<slug>")]
    Post {slug: String},
    #[not_found]
    NotFound,
}

fn main() {
    sycamore::render(|cx| view! { cx,
            Router(
                integration=HistoryIntegration::new(),
                view=|cx,route: &ReadSignal<AppRoutes>| {
                    view! { cx,
                        div(class="app") {
                            (match route.get().as_ref() {
                                AppRoutes::FrontPage => view! { cx,
                                    Post((String::from("frontpage")))
                                },
                                AppRoutes::NotFound => view! { cx,
                                    NotFound
                                },
                                AppRoutes::Post{slug} => view! { cx, 
                                    Post(slug.to_owned())
                                }
                            })
                        }
                    }
                }
            )   
        }
    )
}