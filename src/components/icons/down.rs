use sycamore::prelude::*;

#[component]
pub fn ChevronDown<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        svg (xmlns="http://www.w3.org/2000/svg", fill="currentColor", viewBox="0 96 960 960", class="inline-block w-6 h-6 stroke-current") {
            path (
                stroke-linecap="round",
                stroke-linejoin="round",
                stroke-width="2",
                d="M480 731 220 471l75-75 185 185 185-185 75 75-260 260Z"
            ) {}
        }
    }
}
