use super::{skill::SkillView, structure::Skill};
use sycamore::prelude::*;

pub fn get_database_skills() -> Vec<Skill> {
    let mut s = Vec::new();

    s.push(Skill {
        label: "PostgreSQL".to_string(),
        icon: ".perseus/static/postgres.png".to_string(),
        proficiency: 80,
    });

    s.push(Skill {
        label: "MySQL".to_string(),
        icon: ".perseus/static/mysql.png".to_string(),
        proficiency: 80,
    });

    s.push(Skill {
        label: "MongoDB".to_string(),
        icon: ".perseus/static/mongodb.png".to_string(),
        proficiency: 80,
    });

    s.push(Skill {
        label: "SQLite".to_string(),
        icon: ".perseus/static/sqlite.png".to_string(),
        proficiency: 80,
    });

    s.push(Skill {
        label: "Redis".to_string(),
        icon: ".perseus/static/redis.png".to_string(),
        proficiency: 60,
    });

    s
}

#[component]
pub fn DatabaseSkills<G: Html>(cx: Scope) -> View<G> {
    let database_skills = create_signal(cx, get_database_skills());

    view! { cx,
        div(class="card w-96 bg-neutral shadow-md border border-primary-focus hover:border-primary hover:shadow-xl m-4") {
            div(class="card-body") {
                div (class="card-title") { "Databases" }
                div (class="py-2 flex flex-col gap-2") {
                    Keyed (
                        iterable=database_skills,
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
