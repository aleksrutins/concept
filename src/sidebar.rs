use dioxus::prelude::*;

#[inline_props]
pub fn Sidebar(cx: Scope) -> Element {
    cx.render(
        rsx! {
            div { "Hello" }
        }
    )
}