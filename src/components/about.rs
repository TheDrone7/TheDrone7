use super::icons::{down::ChevronDown, up::ChevronUp};
use sycamore::prelude::*;

use chrono::{DateTime, Utc};

fn yts(y: u32) -> String {
    match y {
        1 => "1st".to_string(),
        2 => "2nd".to_string(),
        3 => "3rd".to_string(),
        _ => format!("{}th", y),
    }
}

#[component]
pub fn AboutSection<G: Html>(cx: Scope) -> View<G> {
    // Calculate age
    let birthday =
        DateTime::parse_from_str("2.11.2002 7:00 am +0530", "%d.%m.%Y %H:%M %P %z").unwrap();
    let now = Utc::now();
    let age = now.years_since(birthday.into()).unwrap();

    // Current city
    let city = "Ottawa, Canada";

    // Degree progress
    let start_day =
        DateTime::parse_from_str("1.5.2021 4:00 am +0000", "%d.%m.%Y %H:%M %P %z").unwrap();
    let deg_year = now.years_since(start_day.into()).unwrap();
    let deg = if deg_year > 4 {
        "graduate from".to_string()
    } else {
        format!("{} year student at", yts(deg_year))
    };

    view! { cx,
        div (class="hero min-h-screen relative bg-base-100", id="about") {
            div (class="hero-content flex-col max-w-screen-md text-center") {
                h1 (class="text-4xl") {
                    span (class="text-primary-focus") { "About" }
                    span { " me" }
                }

                p (class="my-4") {
                    "Hello! I am "
                    span(class="text-secondary") { "Harmeet" }
                    " (pronounced [ h uh r - m ee t ]), a " (age) " year old "
                    span(class="text-secondary") { "full stack web developer" }
                    ", currently living in "
                    span(class="text-secondary") { (city) }
                }

                p (class="my-4") {
                    "I am currently a " (deg) " "
                    span(class="text-secondary") { "Carleton University" }
                    " Bachelor of Computer Science program "
                    "majoring in "
                    span(class="text-secondary") { "Computer and Internet Security." }
                }

                p (class="my-4") {
                    "I often code for fun, exploring new or different technologies "
                    "in fact, this very website was written while I was doing exactly that!"
                }

                p (class="my-4") {
                    "Apart from coding, I enjoy gaming, cooking, going on walks, and blasting music. "
                    "I am also fairly active on discord engaging with different communities such as "
                    "the Replit community or the Riot Games Third Party Developers community."
                }
            }
            div (class="absolute bottom-12 flex-row") {
                a (href="/", class="btn btn-circle btn-outline mr-6", aria_label="Visit Landing Section") {
                    ChevronUp {}
                }
                a (href="#projects", class="btn btn-circle btn-outline", aria_label="Visit Projects Section") {
                    ChevronDown {}
                }
            }
        }
    }
}
