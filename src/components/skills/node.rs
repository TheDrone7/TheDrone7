use super::{skill::SkillView, structure::Skill};
use sycamore::prelude::*;

pub fn get_node_skills() -> Vec<Skill> {
    let mut s = Vec::new();

    s.push(Skill {
        label: "Express".to_string(),
        icon: ".perseus/static/express.png".to_string(),
        proficiency: 100,
    });

    s.push(Skill {
        label: "Vue.JS".to_string(),
        icon: ".perseus/static/vue.png".to_string(),
        proficiency: 100,
    });

    s.push(Skill {
        label: "Node.JS".to_string(),
        icon: ".perseus/static/nodejs.png".to_string(),
        proficiency: 90,
    });

    s.push(Skill {
        label: "Next.JS".to_string(),
        icon: ".perseus/static/next.png".to_string(),
        proficiency: 90,
    });

    s.push(Skill {
        label: "Nuxt.JS".to_string(),
        icon: ".perseus/static/nuxt.png".to_string(),
        proficiency: 90,
    });

    s.push(Skill {
        label: "TypeScript".to_string(),
        icon: ".perseus/static/typescript.png".to_string(),
        proficiency: 80,
    });

    s.push(Skill {
        label: "React".to_string(),
        icon: ".perseus/static/react.png".to_string(),
        proficiency: 80,
    });

    s
}

#[component]
pub fn NodeSkills<G: Html>(cx: Scope) -> View<G> {
    let node_skills = create_signal(cx, get_node_skills());

    view! { cx,
        div(class="card w-96 bg-neutral shadow-md border border-primary-focus hover:border-primary hover:shadow-xl m-4") {
            div(class="card-body") {
                div (class="card-title") { "JavaScript /TS" }
                div (class="py-2 flex flex-col gap-2") {
                    Keyed (
                        iterable=node_skills,
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
