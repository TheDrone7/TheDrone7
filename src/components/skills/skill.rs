use super::structure::Skill;
use sycamore::prelude::*;

#[derive(Prop)]
pub struct SkillViewProps {
    skill: Skill,
    bars: &'static str,
}

#[component]
pub fn SkillView<G: Html>(cx: Scope, props: SkillViewProps) -> View<G> {
    let class = format!("progress {} flex-grow", props.bars);
    view! { cx,
        div (class="w-full flex flex-row place-items-center gap-4") {
            div (class="avatar tooltip", data-tip=props.skill.label) {
                div (class="w-6 rounded-xl") {
                    img(src=props.skill.icon) {}
                }
            }
            progress (class=class, value=props.skill.proficiency, max="100") {}
        }
    }
}
