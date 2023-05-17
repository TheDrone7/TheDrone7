use sycamore::prelude::*;

#[component]
pub fn Drawer<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div (class="drawer-side") {
            label (for="nav-drawer", class="drawer-overlay") {}
            ul (class="menu p-4 w-80 bg-base-100") {
                li {
                    a (href="#about") { "About" }
                }
                li {
                    a (href="#projects") { "Projects" }
                }
                li {
                    a (href="#skills") { "Skills" }
                }
                li {
                    a (href="#contact") { "Contact" }
                }
            }
        }
    }
}
