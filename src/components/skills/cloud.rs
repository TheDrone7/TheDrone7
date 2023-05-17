use super::{skill::SkillView, structure::Skill};
use sycamore::prelude::*;

pub fn get_cloud_skills() -> Vec<Skill> {
    let mut s = Vec::new();

    s.push(Skill {
        label: "Replit".to_string(),
        icon: ".perseus/static/replit.png".to_string(),
        proficiency: 100,
    });

    s.push(Skill {
        label: "Supabase".to_string(),
        icon: ".perseus/static/supabase.png".to_string(),
        proficiency: 90,
    });

    s.push(Skill {
        label: "Firebase".to_string(),
        icon: ".perseus/static/firebase.png".to_string(),
        proficiency: 90,
    });

    s.push(Skill {
        label: "Google Cloud Platform".to_string(),
        icon: ".perseus/static/gcp.png".to_string(),
        proficiency: 70,
    });

    s.push(Skill {
        label: "Amazon Web Services".to_string(),
        icon: ".perseus/static/aws.png".to_string(),
        proficiency: 70,
    });

    s
}

#[component]
pub fn CloudSkills<G: Html>(cx: Scope) -> View<G> {
    let cloud_skills = create_signal(cx, get_cloud_skills());

    view! { cx,
        div(class="card w-4/5 sm:w-96 bg-neutral shadow-md border border-primary-focus hover:border-primary hover:shadow-xl m-4") {
            div(class="card-body") {
                div (class="card-title") { "Cloud Platforms" }
                div (class="py-2 flex flex-col gap-2") {
                    Keyed (
                        iterable=cloud_skills,
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
