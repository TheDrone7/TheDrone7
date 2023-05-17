use sycamore::prelude::*;

#[derive(Prop)]
pub struct Props {
    pub href: String,
    pub label: String,
}

#[component]
pub fn ProjectLinkButton<G: Html>(cx: Scope, props: Props) -> View<G> {
    let primary = props.label == "View".to_string();
    let class = if (primary) {
        "btn btn-secondary"
    } else {
        "btn btn-ghost"
    };
    view! { cx,
        a (class=class, href=props.href, target="_blank") {
            (props.label)
        }
    }
}
