use sycamore::prelude::*;

#[component]
pub fn Header<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="header") {
            img(alt="picture of brian", src="images/profile.png",class="profile-photo")
            h1{"BrianJlogan.com"}
            div(class="sub_header"){
                ul {
                    li { "About me"}
                    li { "Tech" }
                    li { "Food" }
                    li { "Coffee" }
                    li { "Resume" }
                    li { "Posts" }
                    li { "Contact Me"}
                }
                a {
                    "Sign-in"
                }
            }

        }
    }
}