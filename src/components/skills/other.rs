use super::{skill::SkillView, structure::Skill};
use sycamore::prelude::*;

pub fn get_other_skills() -> Vec<Skill> {
    let mut s = Vec::new();

    s.push(Skill {
        label: "GraphQL".to_string(),
        icon: ".perseus/static/graphql.png".to_string(),
        proficiency: 100,
    });

    s.push(Skill {
        label: "GitHub".to_string(),
        icon: ".perseus/static/github.png".to_string(),
        proficiency: 90,
    });

    s.push(Skill {
        label: "Figma".to_string(),
        icon: ".perseus/static/figma.png".to_string(),
        proficiency: 80,
    });

    s.push(Skill {
        label: "Docker".to_string(),
        icon: ".perseus/static/docker.png".to_string(),
        proficiency: 60,
    });

    s
}

#[component]
pub fn OtherSkills<G: Html>(cx: Scope) -> View<G> {
    let other_skills = create_signal(cx, get_other_skills());

    view! { cx,
        div(class="card w-4/5 sm:w-96 bg-neutral shadow-md border border-primary-focus hover:border-primary hover:shadow-xl m-4") {
            div(class="card-body") {
                div (class="card-title") { "Other tools" }
                p { "Tools I use from time-to-time." }
                div (class="py-2 flex flex-col gap-2") {
                    Keyed (
                        iterable=other_skills,
                        view=|cx, x| view! { cx,
                            SkillView (skill=x, bars="progress-secondary")
                        },
                        key=|x| x.clone().label,
                    )
                }
            }
        }
    }
}
