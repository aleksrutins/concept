mod sidebar;

// main.rs
use dioxus::prelude::*;

use crate::sidebar::Sidebar;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.use_hook(|_| {
    });
    cx.render(rsx!{
        div {
            Sidebar {}
        }
    })
}
