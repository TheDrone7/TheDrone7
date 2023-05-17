use super::{about::AboutSection, hero::Hero, projects::ProjectSection};
use perseus::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn Content<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Hero {}
        AboutSection {}
        ProjectSection {}
    }
}
