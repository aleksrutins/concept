use dioxus::prelude::*;

pub fn Content(cx: Scope) -> Element {
    let note_tree = use_atom_ref(&cx, crate::CFG);
    let content = note_tree.read().render_current().unwrap_or("No note open".into());
    cx.render(rsx! {
        div {
            class: "content",
            dangerous_inner_html: "{content}"
        }
    })
}