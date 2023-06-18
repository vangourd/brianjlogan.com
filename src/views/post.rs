use sycamore::prelude::*;
use brianjlogan::base::Base;
use brianjlogan::header::Header;
use brianjlogan::footer::Footer;

#[component]
pub fn Post<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Base{
            Header
            p{"This is a post"}
            Footer
        }
        
    }
}