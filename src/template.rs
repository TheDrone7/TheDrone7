use crate::components::{contents::Content, drawer::Drawer, navbar::Navbar};
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "IndexPageStateRx")]
struct IndexPageState {
    greeting: String,
}

#[auto_scope]
fn index_page<G: Html>(cx: Scope, state: &IndexPageStateRx) -> View<G> {
    view! { cx,
        nav (class="drawer") {
            input (id="nav-drawer", type="checkbox", class="drawer-toggle") {}
            div (class="drawer-content flex flex-col") {
               Navbar {}
                div (class="page-content") {
                    Content {}
                }
            }
            Drawer {}
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope, _props: IndexPageState) -> View<SsrNode> {
    view! { cx,
        title { "TheDrone7" }
    }
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> IndexPageState {
    IndexPageState {
        greeting: "Hello world!".to_string(),
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .head_with_state(head)
        .build()
}
