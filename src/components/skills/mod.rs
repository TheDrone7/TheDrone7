use super::icons::{down::ChevronDown, up::ChevronUp};
use sycamore::prelude::*;

#[component]
pub fn SkillsSection<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div (class="hero min-h-screen relative bg-base-100", id="skills") {
            div (class="hero-content flex-col max-w-screen-lg pt-24 pb-32") {
                h1 (class="text-4xl") {
                    span { "My " }
                    span (class="text-primary-focus") { "Skills" }
                }

            }
            div (class="absolute bottom-12 flex-row") {
                a (href="#projects", class="btn btn-circle btn-outline mr-6") {
                    ChevronUp {}
                }
                a (href="#contact", class="btn btn-circle btn-outline") {
                    ChevronDown {}
                }
            }
        }
    }
}
