use perseus::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn Hero<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div (class="hero bg-base-200 place-items-start") {
            div (class="hero-content flex-col lg:flex-row") {
                img (class="max-w-sm rounded-full shadow-2xl", src=".perseus/static/pfp.png", alt="Display picture") {}
                div (class="pl-12") {
                    h1 (class="text-5xl font-bold") { "TheDrone7" }
                    p (class="py-4 text-2xl") {
                        "Full-stack Web Developer"
                    }
                }
            }
        }
    }
}
