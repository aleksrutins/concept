#![allow(non_snake_case)]

mod content;
mod data;
mod sidebar;

use std::env;

use data::NoteTree;
// main.rs
use dioxus::prelude::*;

use crate::{content::Content, sidebar::Sidebar};

static CFG: AtomRef<NoteTree> = |_| NoteTree::new(env::var("CONCEPT_NOTE_ROOT").unwrap().into());

static STYLES: &str = include_str!("./app.css");

fn main() {
    env::var("CONCEPT_NOTE_ROOT")
        .expect("Please provide the CONCEPT_NOTE_ROOT environment variable.");
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "app-view",
            style {
                vec![STYLES]
            },
            Sidebar {},
            Content {}
        }
    })
}
