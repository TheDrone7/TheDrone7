use sycamore::prelude::*;

#[derive(Prop)]
pub struct Props {
    pub href: String,
    pub label: String,
}

#[component]
pub fn ProjectLinkButton<G: Html>(cx: Scope, props: Props) -> View<G> {
    view! { cx,
        a (class="btn", href=props.href, target="_blank") {
            (props.label)
        }
    }
}
