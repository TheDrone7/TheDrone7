use super::icons::hamburger::HamburgerIcon;
use perseus::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn Navbar<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div (class="w-full navbar fixed top-0 bg-base-300") {
            div (class="flex-none lg:hidden") {
                label (for="nav-drawer", class="btn btn-square btn-ghost") {
                    HamburgerIcon {}
                }
            }
            div (class="flex-1 px-2 mx-2") {
                a (href="/", class="btn btn-ghost normal-case text-xl") { "TheDrone7" }
            }
            div (class="flex-none hidden lg:block") {
                ul (class="menu menu-horizontal") {
                    li {
                        a (href="#about") { "About" }
                    }
                    li {
                        a (href="#contact") { "Contact" }
                    }
                    li {
                        a (href="#projects") { "Projects" }
                    }
                    li {
                        a (href="#skills") { "Skills" }
                    }
                }
            }
        }
    }
}
