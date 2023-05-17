use super::{skill::SkillView, structure::Skill};
use sycamore::prelude::*;

pub fn get_general_skills() -> Vec<Skill> {
    let mut s = Vec::new();

    s.push(Skill {
        label: "Rust".to_string(),
        icon: ".perseus/static/rust.png".to_string(),
        proficiency: 90,
    });

    s.push(Skill {
        label: "Python".to_string(),
        icon: ".perseus/static/python.png".to_string(),
        proficiency: 90,
    });

    s.push(Skill {
        label: "Java".to_string(),
        icon: ".perseus/static/java.png".to_string(),
        proficiency: 80,
    });

    s.push(Skill {
        label: "C++".to_string(),
        icon: ".perseus/static/cpp.png".to_string(),
        proficiency: 60,
    });

    s.push(Skill {
        label: "C".to_string(),
        icon: ".perseus/static/c.png".to_string(),
        proficiency: 50,
    });

    s
}

#[component]
pub fn GeneralSkills<G: Html>(cx: Scope) -> View<G> {
    let general_skills = create_signal(cx, get_general_skills());

    view! { cx,
        div(class="card w-4/5 sm:w-96 bg-neutral shadow-md border border-primary-focus hover:border-primary hover:shadow-xl m-4") {
            div(class="card-body") {
                div (class="card-title") { "General-purpose languages" }
                div (class="py-2 flex flex-col gap-2") {
                    Keyed (
                        iterable=general_skills,
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
