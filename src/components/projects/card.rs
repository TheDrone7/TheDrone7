use sycamore::prelude::*;

use super::{link::ProjectLinkButton, stack::ProjectStackIcon};
use crate::projects::structure::Project;

#[derive(Prop)]
pub struct ProjectCardProps {
    pub project: Project,
}

#[component]
pub fn ProjectCard<G: Html>(cx: Scope, props: ProjectCardProps) -> View<G> {
    let stack = create_signal(cx, props.project.stack);
    let links = create_signal(cx, props.project.links);
    view! { cx,
        div(class="card w-4/5 sm:w-96 bg-neutral shadow-md border border-primary-focus hover:border-primary hover:shadow-xl m-4") {
            div(class="card-body") {
                div (class="card-title") { (props.project.title) }
                p (class="py-2") { (props.project.description) }
                div (class="py-2 flex flex-row flex-wrap gap-2") {
                    Keyed (
                        iterable=stack,
                        view=|cx, x| view! { cx,
                            ProjectStackIcon(icon=x)
                        },
                        key=|x| x.clone().name,
                    )
                }
                div (class="card-actions flex-row-reverse") {
                    Keyed (
                        iterable=links,
                        view=|cx, x| view! { cx,
                            ProjectLinkButton(href=x.href, label=x.label)
                        },
                        key=|x| x.clone().href,
                    )
                }
            }
        }
    }
}
