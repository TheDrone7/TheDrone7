use super::icons::up::ChevronUp;
use sycamore::prelude::*;

#[component]
pub fn ContactSection<G: Html>(cx: Scope) -> View<G> {
    let email = "h@thedrone7.dev";
    let insta = "thedrone_7";
    let twitr = "TheDrone_7";

    // Discord ID and name#tag
    let dscid = "374886124126208000";
    let dcnam = "@TheDrone7#1624";

    // Extra links
    let replit = "TheDrone7";
    let github = "TheDrone7";

    view! { cx,
        div (class="hero min-h-screen relative bg-base-100", id="contact") {
            div (class="hero-content flex-col max-w-screen-md text-center") {
                h1 (class="text-4xl") {
                    span (class="text-primary-focus") { "Contact" }
                    span { " me" }
                }
                p (class="py-2") {
                    "Want to hire me or just talk to me? "
                    "Feel free to use any of the following options!"
                }
                div (class="grid grid-cols-1 grid-flow-row") {

                }
            }
            div (class="absolute bottom-12 flex-row") {
                a (href="#skills", class="btn btn-circle btn-outline mr-6") {
                    ChevronUp {}
                }
            }
        }
    }
}
