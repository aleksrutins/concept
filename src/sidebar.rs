use std::{rc::Rc, sync::Arc};

use crate::{data::NoteTree, CFG};
use dioxus::prelude::*;

pub fn Sidebar(cx: Scope) -> Element {
    let note_tree = use_atom_ref(&cx, CFG);
    let walker = cx.use_hook(|_| note_tree.read().walk().into_iter().filter_map(Result::ok));
    cx.render(
        rsx! {
            div {
                walker.map(|item| {
                    let item_name = item.path().file_name().map(|item| item.to_str()).flatten()?.to_string();
                    Some(rsx!(
                        a {
                            onclick: move |_evt| {
                                note_tree.write().open(item.path());
                            },
                            "{item_name}"
                        }
                    ))
                })
            }
        }
    )
}
