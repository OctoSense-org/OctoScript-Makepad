// Makepad Plot - Matplotlib-style plotting library for Makepad 2.0 (Splash)

pub use makepad_widgets;
use makepad_widgets::*;

pub mod charts;
pub mod plot_view;
pub mod script_util;
pub mod types;

pub use charts::bar::*;
pub use charts::circular::*;
pub use charts::field::*;
pub use charts::hierarchy::*;
pub use charts::line::*;
pub use charts::line2::*;
pub use charts::polar::*;
pub use charts::scatter::*;
pub use charts::stack::*;
pub use charts::stats::*;
pub use charts::three_d::*;
pub use plot_view::*;
pub use types::*;

/// Register all makepad-plot widgets and enums with the Splash VM under `mod.plot`.
/// Call after `makepad_widgets::script_mod(vm)`.
pub fn script_mod(vm: &mut ScriptVm) {
    vm.bx.heap.new_module(id!(plot));
    crate::types::script_mod(vm);
    crate::plot_view::script_mod(vm);
    crate::charts::line::script_mod(vm);
    crate::charts::line2::script_mod(vm);
    crate::charts::stack::script_mod(vm);
    crate::charts::bar::script_mod(vm);
    crate::charts::scatter::script_mod(vm);
    crate::charts::circular::script_mod(vm);
    crate::charts::polar::script_mod(vm);
    crate::charts::stats::script_mod(vm);
    crate::charts::field::script_mod(vm);
    crate::charts::three_d::script_mod(vm);
    crate::charts::hierarchy::script_mod(vm);
    crate::charts::grid::script_mod(vm);
}
