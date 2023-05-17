pub mod card;
pub mod link;
pub mod stack;

use super::icons::{down::ChevronDown, up::ChevronUp};
use crate::projects::get_project_list;
use card::ProjectCard;
use sycamore::prelude::*;

#[component]
pub fn ProjectSection<G: Html>(cx: Scope) -> View<G> {
    let projects_list = create_signal(cx, get_project_list());

    view! { cx,
        div (class="hero min-h-screen relative bg-base-100", id="projects") {
            div (class="hero-content flex-col max-w-screen-lg 2xl:max-w-screen-2xl pt-24 pb-32") {
                h1 (class="text-4xl") {
                    span { "My " }
                    span (class="text-primary-focus") { "Projects" }
                }
                div(class="grid grid-flow-row lg:grid-cols-2 2xl:grid-cols-3") {
                    Keyed (
                        iterable=projects_list,
                        view=|cx, x| view! { cx,
                            ProjectCard(project = x)
                        },
                        key=|x| x.clone().title,
                    )
                }
            }
            div (class="absolute bottom-12 flex-row") {
                a (href="#about", class="btn btn-circle btn-outline mr-6") {
                    ChevronUp {}
                }
                a (href="#skills", class="btn btn-circle btn-outline") {
                    ChevronDown {}
                }
            }
        }
    }
}
