use sycamore::prelude::*;
use sycamore_router::{Route, Router, RouterProps};

mod components;

#[derive(Route)]
enum AppRoutes {
    #[to("/")]
    FrontPage,
    #[not_found]
    NotFound,
}

fn main() {
    sycamore::render(|cx| view! { cx,
        div {
            h1 { "Welcome to my blog!" }
        }
    });
}
