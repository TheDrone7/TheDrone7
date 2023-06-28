use crate::projects::structure::ProjectStackItem;
use sycamore::prelude::*;

#[derive(Prop)]
pub struct ProjectStackIconProps {
    pub icon: ProjectStackItem,
}

#[component]
pub fn ProjectStackIcon<G: Html>(cx: Scope, props: ProjectStackIconProps) -> View<G> {
    let img_alt = props.icon.name.clone();
    view! { cx,
        div (class="avatar tooltip", data-tip=props.icon.name) {
            div (class="w-8 rounded-xl") {
                img(src=props.icon.image, alt=img_alt) {}
            }
        }
    }
}
