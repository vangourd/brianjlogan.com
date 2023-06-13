use sycamore::prelude::*;

#[component]
pub fn Footer<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="footer"){
           p {
            "This site was written in Rustlang!\n"
            img(src="https://rustacean.net/assets/rustacean-flat-happy.png")br{}
            a(href="https://sycamore-rs.netlify.app/"){
                "using the Sycamore Framework"
                img(src="https://sycamore-rs.netlify.app/logo.svg")
            }
            br{}br{}
            "Read my post about how I built it."
           }
        }
    }
}