use super::icons::down::ChevronDown;
use perseus::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn Hero<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div (class="hero bg-base-200 min-h-screen") {
            div (class="hero-content flex-col") {
                img (class="max-w-sm rounded-full shadow-2xl", src=".perseus/static/pfp.png", alt="Display picture") {}
                div (class="text-center mt-4 mb-12") {
                    h1 (class="text-5xl font-bold") { "TheDrone7" }
                    p (class="py-4 text-2xl") {
                        "Full-stack Web Developer"
                    }
                }
                a (href="#about", class="btn btn-circle btn-outline") {
                    ChevronDown {}
                }
            }
        }
    }
}
