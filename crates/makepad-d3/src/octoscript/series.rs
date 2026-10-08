//! `d3.series(f, n [, t])` — a data series computed by the Splash kernel JIT.
//!
//! Takes the document's own `fn(i)` (an element kernel body writing
//! `out[i]`), compiles it once through `vm_kernel` with everything it
//! reaches in the document — helper fns, captured numbers — and runs it
//! as native code over all `n` elements, instead of the VM interpreting
//! the fn once per point. The result comes back as an ordinary script
//! array, so it feeds any chart's `data:` or `set_data(...)` unchanged.
//!
//! ```octoscript,ignore
//! let dx = 0.001
//! fn wave(x) { return sin(x * 3.0) * 0.8 + sin(x * 37.0) * 0.1 }
//! fn gen(i) { out[i] = wave(float(i) * dx) + t * 0.0 }
//! ui.chart.set_data(d3.series(gen, 100000))
//! // animated: d3.series(gen, 100000, frame_time) — same compiled kernel,
//! // only the `t` parameter changes per call.
//! ```
//!
//! The kernel subset is typed Splash: numbers, vectors, bounded loops and
//! helper fns. A fn outside it (objects, strings, ui access) is refused
//! with the compiler's message at the document's line.

// The `script_mod!` macro generates a public registration function that
// cannot carry a doc comment.
#![allow(missing_docs)]

use std::cell::RefCell;
use std::sync::Arc;

use makepad_script_compute::kernel::{Kernel, MathMode};
use makepad_script_compute::vm_kernel::{self, Decl, Entry, VmKernel};
use makepad_script_compute::Backend;
use makepad_script::script_err_invalid_args;
use makepad_widgets::*;

/// The most elements one `d3.series` call computes. Generous for charts
/// (a 4k display shows ~4k points per pixel column) while bounding what a
/// sandboxed body can ask of the host in one call.
const MAX_SERIES: usize = 2_000_000;

/// Compiled kernels kept per registered fn value; cleared when it grows
/// past this (an app animating a handful of series stays fully cached).
const CACHE_CAP: usize = 16;

/// Elements from which a run is split across threads (an element kernel's
/// result does not depend on the split).
const PARALLEL_FROM: usize = 65536;

pub fn make_series_fn(vm: &mut ScriptVm) -> ScriptValue {
    let cache: RefCell<Vec<(ScriptObject, Arc<Kernel>)>> = RefCell::new(Vec::new());
    let mut native = vm.bx.code.native.borrow_mut();
    native
        .add_fn(&mut vm.bx.heap, script_args_def!(f = NIL, n = NIL, t = NIL), move |vm, args| series_call(vm, args, &cache))
        .into()
}

fn series_call(vm: &mut ScriptVm, args: ScriptObject, cache: &RefCell<Vec<(ScriptObject, Arc<Kernel>)>>) -> ScriptValue {
    let f = script_value!(vm, args.f);
    let n = script_value!(vm, args.n);
    let t = script_value!(vm, args.t).as_number().unwrap_or(0.0) as f32;
    let Some(entry) = f.as_object() else {
        return script_err_invalid_args!(vm.bx.threads.cur_ref().trap, "d3.series takes (fn(i), n [, t])");
    };
    let Some(n) = n.as_number().filter(|n| *n >= 0.0) else {
        return script_err_invalid_args!(vm.bx.threads.cur_ref().trap, "d3.series takes (fn(i), n [, t]) with n >= 0");
    };
    let n = (n as usize).min(MAX_SERIES);

    let cached = cache.borrow().iter().find(|(e, _)| *e == entry).map(|(_, k)| k.clone());
    let kernel = match cached {
        Some(kernel) => kernel,
        None => {
            let vk = VmKernel {
                decls: vec![
                    Decl::Output { name: "out".into(), ty: "f32".into(), stride: None, offset: None, buffer: None },
                    Decl::Param { name: "t".into(), default: 0.0, range: None },
                ],
                entry: Entry::Element,
                entry_fn: entry,
                math: MathMode::Fast,
                uses: Vec::new(),
                bind: Vec::new(),
            };
            match vm_kernel::compile(vm, &vk, &[], Backend::Native, &[]) {
                Ok((kernel, _source)) => {
                    let mut cache = cache.borrow_mut();
                    if cache.len() >= CACHE_CAP {
                        cache.clear();
                    }
                    cache.push((entry, kernel.clone()));
                    kernel
                }
                Err(errors) => {
                    let message = errors.first().map(|e| e.message.clone()).unwrap_or_else(|| "kernel compile failed".into());
                    return script_err_invalid_args!(vm.bx.threads.cur_ref().trap, "d3.series: {}", message);
                }
            }
        }
    };

    let mut out = vec![0f32; n];
    if n > 0 {
        let mut call = kernel.call();
        call.set_param("t", t);
        if let Err(e) = call.output("out", &mut out) {
            return script_err_invalid_args!(vm.bx.threads.cur_ref().trap, "d3.series: {:?}", e);
        }
        let threads = if n >= PARALLEL_FROM { std::thread::available_parallelism().map(|p| p.get()).unwrap_or(1).min(8) } else { 1 };
        let run = if threads > 1 { call.run_parallel(n, threads) } else { call.run(n) };
        drop(call);
        if let Err(e) = run {
            return script_err_invalid_args!(vm.bx.threads.cur_ref().trap, "d3.series: {:?}", e);
        }
    }

    let array = vm.bx.heap.new_array();
    for v in &out {
        vm.bx.heap.array_push_unchecked(array, ScriptValue::from_f32(*v));
    }
    array.into()
}

script_mod! {
    mod.d3.series = #(make_series_fn(vm))
}
