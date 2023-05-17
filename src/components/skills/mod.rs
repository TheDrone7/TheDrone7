pub mod database;
pub mod general;
pub mod node;
pub mod skill;
pub mod structure;
pub mod style;

use super::icons::{down::ChevronDown, up::ChevronUp};
use database::DatabaseSkills;
use general::GeneralSkills;
use node::NodeSkills;
use style::StyleSkills;
use sycamore::prelude::*;

#[component]
pub fn SkillsSection<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div (class="hero min-h-screen relative bg-base-100", id="skills") {
            div (class="hero-content flex-col max-w-screen-lg 2xl:max-w-screen-2xl pt-24 pb-32") {
                h1 (class="text-4xl") {
                    span { "My " }
                    span (class="text-primary-focus") { "Skills" }
                }
                p (class="py-4 text-center") {
                    "Most of my big works use "
                    span (class="text-secondary") { "TypeScript" }
                    " or "
                    span (class="text-secondary") { "Node.js" }
                    " but, I have a lot more things I am proficient with. "
                    br {}
                    "Here is a more detailed list of my skills and proficiency."
                }
                div(class="grid grid-flow-row lg:grid-cols-2 2xl:grid-cols-3") {
                    NodeSkills {}
                    StyleSkills {}
                    GeneralSkills {}
                    DatabaseSkills {}
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
