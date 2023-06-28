use sycamore::prelude::*;
use brianjlogan::base::Base;
use brianjlogan::header::Header;
use brianjlogan::footer::Footer;
use brianjlogan::cpost::PostComponent;

#[component]
pub fn PostView<G: Html>(cx: Scope, slug: String) -> View<G> {
    view! { cx,
        Base{
            Header
            PostComponent(slug)
            Footer
        }
    }
}