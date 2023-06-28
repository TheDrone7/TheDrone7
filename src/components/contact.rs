use super::icons::up::ChevronUp;
use crate::components::icons::logos::{DiscIcon, GHubIcon, InstIcon, MailIcon, ReplIcon, TwitIcon};
use sycamore::prelude::*;

#[component]
pub fn ContactSection<G: Html>(cx: Scope) -> View<G> {
    let email = "h@thedrone7.dev";
    let insta = "thedrone_7";
    let twitr = "TheDrone_7";

    let dscid = "374886124126208000";

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
                div (class="grid grid-cols-1 grid-flow-row gap-4") {
                    a (
                        class="btn btn-success btn-outline btn-wide flex-row justify-start align-center pl-8",
                        href=("mailto:".to_string() + email),
                        target="_blank"
                    ) {
                        MailIcon {}
                        div (class="divider divider-horizontal before:bg-success after:bg-success")
                        span { "Email" }
                    }

                    a (
                        class="btn btn-error btn-outline btn-wide flex-row justify-start align-center pl-8",
                        href=("https://instagram.com/".to_string() + insta),
                        target="_blank"
                    ) {
                        InstIcon {}
                        div (class="divider divider-horizontal before:bg-error after:bg-error")
                        span { "Instagram" }
                    }

                    a (
                        class="btn btn-info btn-outline btn-wide flex-row justify-start align-center pl-8",
                        href=("https://twitter.com/".to_string() + twitr),
                        target="_blank"
                    ) {
                        TwitIcon {}
                        div (class="divider divider-horizontal before:bg-info after:bg-info")
                        span { "Twitter" }
                    }

                    a (
                        class="btn btn-info btn-outline btn-wide flex-row justify-start align-center pl-8",
                        href=("https://discord.com/users/".to_string() + dscid),
                        target="_blank"
                    ) {
                        DiscIcon {}
                        div (class="divider divider-horizontal before:bg-info after:bg-info")
                        span { "Discord" }
                    }
                }
                div (class="divider")
                p (class="mb-4") {
                    "Want to check out more of my work? Check these out!"
                }
                div (class="grid grid-cols-1 grid-flow-row gap-4") {
                    a (
                        class="btn btn-accent btn-outline btn-wide flex-row justify-start align-center pl-8",
                        href=("https://replit.com/@".to_string() + replit),
                        target="_blank"
                    ) {
                        ReplIcon {}
                        div (class="divider divider-horizontal before:bg-accent after:bg-accent")
                        span { "Replit" }
                    }
                    a (
                        class="btn btn-outline btn-wide flex-row justify-start align-center pl-8",
                        href=("https://github.com/".to_string() + github),
                        target="_blank"
                    ) {
                        GHubIcon {}
                        div (class="divider divider-horizontal before:bg-neutral-content after:bg-neutral-content")
                        span { "GitHub" }
                    }
                }
            }
            div (class="absolute bottom-12 flex-row") {
                a (href="#skills", class="btn btn-circle btn-outline mr-6", aria_label="Visit Skills Section") {
                    ChevronUp {}
                }
            }
        }
    }
}
