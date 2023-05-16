use super::{about::AboutSection, hero::Hero};
use perseus::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn Content<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Hero {}
        AboutSection {}
    }
}
