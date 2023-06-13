use sycamore::prelude::*;

#[derive(Prop)]
pub struct BaseComponentProps<'a, G: Html> {
    children: Children<'a, G>,
}

#[component]
pub fn Base<'a, G: Html>(cx: Scope<'a>, props: BaseComponentProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);

    view! { cx,
        div(class="container"){
            (children)
        }
    }
}