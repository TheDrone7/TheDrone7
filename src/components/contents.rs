use super::{
    about::AboutSection, contact::ContactSection, hero::Hero, projects::ProjectSection,
    skills::SkillsSection,
};
use sycamore::prelude::*;

#[component]
pub fn Content<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Hero {}
        AboutSection {}
        ProjectSection {}
        SkillsSection {}
        ContactSection {}
    }
}
