use perseus::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn Navbar<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
      div (class="w-full navbar bg-base-300") {
        div (class="flex-none lg:hidden") {
          label (for="nav-drawer", class="btn btn-square btn-ghost") {
            svg (xmlns="http://www.w3.org/2000/svg", fill="none", viewBox="0 0 24 24", class="inline-block w-6 h-6 stroke-current") {
              path (stroke-linecap="round", stroke-linejoin="round", stroke-width="2", d="M4 6h16M4 12h16M4 18h16") {}
            }
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
