use super::{skill::SkillView, structure::Skill};
use sycamore::prelude::*;

pub fn get_style_skills() -> Vec<Skill> {
    let mut s = Vec::new();

    s.push(Skill {
        label: "Tailwind CSS".to_string(),
        icon: ".perseus/static/tailwind.png".to_string(),
        proficiency: 90,
    });

    s.push(Skill {
        label: "EmotionCSS".to_string(),
        icon: ".perseus/static/emotion.png".to_string(),
        proficiency: 90,
    });

    s.push(Skill {
        label: "PostCSS".to_string(),
        icon: ".perseus/static/postcss.png".to_string(),
        proficiency: 70,
    });

    s.push(Skill {
        label: "CSS3".to_string(),
        icon: ".perseus/static/css.png".to_string(),
        proficiency: 60,
    });

    s.push(Skill {
        label: "Sass".to_string(),
        icon: ".perseus/static/sass.png".to_string(),
        proficiency: 40,
    });

    s
}

#[component]
pub fn StyleSkills<G: Html>(cx: Scope) -> View<G> {
    let style_skills = create_signal(cx, get_style_skills());

    view! { cx,
        div(class="card w-96 bg-neutral shadow-md border border-secondary-focus hover:border-secondary hover:shadow-xl m-4") {
            div(class="card-body") {
                div (class="card-title") { "Styling tools" }
                div (class="py-2 flex flex-col gap-2") {
                    Keyed (
                        iterable=style_skills,
                        view=|cx, x| view! { cx,
                            SkillView (skill=x, bars="progress-primary")
                        },
                        key=|x| x.clone().label,
                    )
                }
                p (class="") {
                    "I am not a designer, but I am proficient at implementing designs."
                }
            }
        }
    }
}
