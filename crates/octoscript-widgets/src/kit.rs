//! Ported kit components shared with the Octos Makepad backend.
use makepad_widgets;
use makepad_widgets::{Cx, WidgetRef};

fn set_design_selection(root: &WidgetRef, cx: &mut Cx, selected: bool) {
    if let Some(mut widget) = root.borrow_mut::<crate::design::DesignButton>() {
        widget.set_selected(cx, selected);
    }
}

#[path = "kit_shared.rs"]
mod shared;
pub use shared::*;
