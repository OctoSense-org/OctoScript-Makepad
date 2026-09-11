//! Render the **reference** catalog's own screens through this backend.
//!
//! The Octoscript-Android catalog is the reference rendering — the same DSL, the
//! same VM, but walked into real `com.google.android.material.*` views. Pointing
//! this backend at those exact files is the only way to compare the two
//! renderers on equal terms: same input, so every difference is ours.
//!
//! Usage: `cargo run -p octoscript-makepad --example refcheck -- <path/to/octoscript/dir>`
//! Defaults to the sibling checkout.

use octoscript_render::makepad_script::makepad_live_id::*;
use octoscript_render::makepad_script::*;
use std::collections::BTreeMap;

const DEFAULT_DIR: &str = "~/home/Octoscript-Android/catalog/rust/octoscript";

/// `S(key)` / `N(key, dflt)` — the reference reads its widget state through
/// these. Nothing here is stateful, so they answer with the defaults.
fn register_state(vm: &mut ScriptVm) {
    let f_s = octoscript_render::add_global_fn(vm, &[(live_id!(k), ScriptValue::NIL)], |vm, _a| {
        vm.bx.heap.new_string_from_str("")
    });
    vm.set_injected_global(live_id!(S), f_s);

    let f_n = octoscript_render::add_global_fn(
        vm,
        &[(live_id!(k), ScriptValue::NIL), (live_id!(d), ScriptValue::NIL)],
        |vm, a| {
            let d = octoscript_render::num_prop(vm, a, live_id!(d)).unwrap_or(0.0);
            ScriptValue::from_f64(d)
        },
    );
    vm.set_injected_global(live_id!(N), f_n);
}

fn count(node: &octoscript_render::UiNode) -> usize {
    1 + node.children.iter().map(count).sum::<usize>()
}

fn main() {
    let dir = std::env::args().nth(1).unwrap_or_else(|| DEFAULT_DIR.into());
    let kit = std::fs::read_to_string(format!("{dir}/kit.octoscript")).unwrap_or_default();

    let mut screens: Vec<String> = std::fs::read_dir(&dir)
        .expect("reference octoscript dir")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|f| f.ends_with(".octoscript") && f != "kit.octoscript")
        .collect();
    screens.sort();

    let mut ok = 0usize;
    let mut failed: Vec<String> = Vec::new();
    let mut nodes_total = 0usize;
    let mut widgets: BTreeMap<String, usize> = BTreeMap::new();

    for file in &screens {
        let name = file.trim_end_matches(".octoscript").to_string();
        let body = std::fs::read_to_string(format!("{dir}/{file}")).unwrap_or_default();
        let src = format!("{kit}\n{body}");
        match octoscript_render::build(&src, register_state) {
            Some(tree) => {
                let n = count(&tree);
                nodes_total += n;
                let ui = octoscript_makepad::to_makepad_ui(&tree);
                for w in [
                    "RoundedView",
                    "RoundedShadowView",
                    "Label",
                    "Button",
                    "CheckBox",
                    "RadioButton",
                    "Toggle",
                    "Slider",
                    "TextInput",
                    "ScrollYView",
                ] {
                    let c = ui.matches(&format!("{w} {{")).count();
                    if c > 0 {
                        *widgets.entry(w.to_string()).or_default() += c;
                    }
                }
                if std::env::var("DUMP").ok().as_deref() == Some(name.as_str()) {
                    println!("{ui}");
                }
                println!("  {name:20} {n:4} nodes  {:6} chars of dialect", ui.len());
                ok += 1;
            }
            None => {
                println!("  {name:20}   -- did not build");
                failed.push(name);
            }
        }
    }

    println!(
        "\n{ok}/{} reference screens build   ({nodes_total} nodes)",
        screens.len()
    );
    if !failed.is_empty() {
        println!("failed: {}", failed.join(" "));
    }
    println!("\nwidgets emitted:");
    for (w, c) in &widgets {
        println!("  {w:20} {c}");
    }
}
