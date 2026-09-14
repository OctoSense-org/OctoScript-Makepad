//! Backend-agnostic Octoscript renderer core.
//!
//! [`build`] evaluates the Octoscript DSL in the **renderer-free** makepad-script VM
//! and returns a [`UiNode`] tree. Backends (ArkUI, makepad, …) turn that tree
//! into their own widgets, which is what makes makepad *one* render backend
//! rather than *the* renderer. Nothing here depends on makepad-platform,
//! makepad-draw, or any widget crate — only on the VM.
//!
//! ```no_run
//! let src = r#"{t:"column", bg: 4278190080, c:[ {t:"text", text:"hi", h: 20} ]}"#;
//! let tree = octoscript_render::build(src, |_vm| {}).unwrap();
//! assert_eq!(tree.kind, octoscript_render::NodeKind::Column);
//! ```

mod eval;
mod l0_helpers;

pub use eval::{add_global_fn, build, num_prop, prop, string_prop};

/// The node model, re-exported.
///
/// It lives in `octoscript-node` now, which depends on nothing at all: a branch
/// point that drags a VM lineage with it cannot be adopted by a host that
/// already has one, and octos-one could not take this crate for exactly that
/// reason. Re-exported so every existing consumer of `octoscript_render::UiNode`
/// keeps working — the split is a packaging change, not an API one.
pub use octoscript_node::{state, Attrs, NodeKind, UiNode};

/// Re-exported so backends and hosts can name VM types (for capability
/// registration) without taking their own makepad-script dependency/version.
pub use makepad_script;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigation_controls_resolve_all_helpers() {
        let source = [
            include_str!("../../../components/l0/_palette_dark.octoscript"),
            include_str!("../../../components/l0/_derive.octoscript"),
            include_str!("../../../components/l0/_kit.octoscript"),
            "{t: \"column\", c: l0_mapcontrols(\"all\", [], 0)}",
        ].join("\n");
        let tree = build(&source, |_| {}).expect("map controls evaluate as a complete tree");
        fn actions(node: &UiNode, out: &mut Vec<String>) {
            if let Some(action) = &node.attrs.action { out.push(action.clone()); }
            for child in &node.children { actions(child, out); }
        }
        let mut found = Vec::new();
        actions(&tree, &mut found);
        assert_eq!(found, ["zoomin", "zoomout", "recenter"]);
    }

    #[test]
    fn native_payload_json_preserves_nested_escapes_and_controls() {
        let controls: String = (0u8..32).map(char::from).collect();
        let target = serde_json::json!({"k":"root/Field#0","e":"input","v":format!("{controls}\\\" 🦀 x\",\"e\":\"reset")}).to_string();
        let mut vm = makepad_script::ScriptVmBase::new();
        for input in ["\\".to_string(), "\t\0".to_string(), controls, target] {
            let value = vm.heap.new_string_from_str(&input);
            let mut encoded = String::new();
            vm.heap.to_json_inner(value, &mut encoded);
            assert_eq!(serde_json::from_str::<String>(&encoded).unwrap(), input);
        }
    }

    #[test]
    fn walks_a_simple_tree() {
        // A column with two text children — the DSL is real script, evaluated
        // by the VM (note the arithmetic colour, since hex literals are 0 here).
        let src = r#"
            fn argb(a,r,g,b){ return ((a*256+r)*256+g)*256+b }
            {t:"column", bg: argb(255,20,20,20), pad: 12, c: [
                {t:"text", text:"Octoscript", size: 20, weight: 7, color: argb(255,255,255,255), w: 120, h: 28},
                {t:"text", text:"on the shared VM", size: 14, color: argb(255,200,200,200), w: 200, h: 20},
            ]}
        "#;
        let tree = build(src, |_vm| {}).expect("evaluates");
        assert_eq!(tree.kind, NodeKind::Column);
        assert_eq!(tree.attrs.pad, Some(12.0));
        assert_eq!(tree.attrs.bg, Some(0xFF141414));
        assert_eq!(tree.children.len(), 2);
        assert_eq!(tree.children[0].kind, NodeKind::Text);
        assert_eq!(tree.children[0].attrs.text.as_deref(), Some("Octoscript"));
        assert_eq!(tree.children[0].attrs.weight, Some(7));
        assert_eq!(tree.count(), 3);
    }

    #[test]
    fn loops_and_helpers_run_in_the_vm() {
        // Proves the tree is *computed*, not literal: a while-loop builds rows.
        let src = r#"
            let kids = []
            let i = 0
            while i < 3 { kids.push({t:"row", h: 40, c: [ {t:"text", text:"row " + i, w: 80, h: 20} ]}); i = i + 1 }
            {t:"column", c: kids}
        "#;
        let tree = build(src, |_vm| {}).expect("evaluates");
        assert_eq!(tree.children.len(), 3);
        assert_eq!(
            tree.children[2].children[0].attrs.text.as_deref(),
            Some("row 2")
        );
    }

    #[test]
    fn unknown_root_tag_is_none() {
        assert!(build(r#"{t:"nope"}"#, |_vm| {}).is_none());
    }
}

#[cfg(test)]
mod boundary_tests {
    use super::*;
    use makepad_script::makepad_live_id::LiveId;

    #[test]
    fn failures_cannot_be_followed_by_a_successful_tree() {
        assert!(build("undefined_call()\n{t:\"text\", text:\"tail\"}", |_| {}).is_none());
        assert!(build("while true {}", |_| {}).is_none());
        assert!(build("{t:\"column\", c:[{t:\"text\"},{t:\"unknown\"}]}", |_| {}).is_none());
        assert!(build("{t:\"column\", c:42}", |_| {}).is_none());
    }

    #[test]
    fn depth_and_width_limits_return_complete_trees_or_failure() {
        let nested = |n| {
            format!(
                "{}{{t:\"text\", text:\"END\"}}{}",
                "{t:\"column\", c:[".repeat(n),
                "]}".repeat(n)
            )
        };
        assert_eq!(build(&nested(40), |_| {}).unwrap().count(), 41);
        assert!(build(&nested(octoscript_node::MAX_TREE_DEPTH + 1), |_| {}).is_none());
        let wide = "let c = []\nlet i = 0\nwhile i < 65536 { c.push({t:\"text\"}); i = i + 1 }\n{t:\"column\", c:c}";
        assert!(build(wide, |_| {}).is_none());
    }

    #[test]
    fn arithmetic_and_numeric_attributes_preserve_missing() {
        for expr in [
            "sys.l0_math(\"/\", 4, 0)",
            "sys.l0_math(\"%\", 4, 0)",
            "sys.l0_math(\"+\", sys.num(\"—\"), 2)",
            "sys.l0_math(\"*\", 1e308, 1e308)",
            "sys.l0_math(\"/\", 1, sys.l0_math(\"*\", 1e308, 1e308))",
        ] {
            let node = build(&format!("{{t:\"text\", text:{expr}}}"), |_| {}).unwrap();
            assert_eq!(node.attrs.text.as_deref(), Some("—"), "{expr}");
        }
        let node = build(
            "{t:\"text\", text:42, variant:2, w:sys.num(\"—\"), h:1e100}",
            |_| {},
        )
        .unwrap();
        assert_eq!(node.attrs.text.as_deref(), Some("42"));
        assert_eq!(node.attrs.variant.as_deref(), Some("2"));
        assert_eq!(node.attrs.w, None);
        assert_eq!(node.attrs.h, None);
    }

    #[test]
    fn pure_helpers_preserve_an_injected_capability_object() {
        let node = build(r#"{t:"text", text:sys.answer}"#, |vm| {
            let sys = vm.bx.heap.new_object();
            vm.bx.heap.set_value_def(
                sys,
                makepad_script::id!(answer).into(),
                makepad_script::ScriptValue::from_f64(42.0),
            );
            vm.set_injected_global(makepad_script::id!(sys), sys.into());
        })
        .unwrap();
        assert_eq!(node.attrs.text.as_deref(), Some("42"));
    }

    #[test]
    fn runtime_string_encoder_round_trips_external_text() {
        let input = "quote \" slash \\ newline\nUnicode 🦀\0 x\",\"e\":\"reset\",\"v\":\"";
        let node = build("{t:\"text\", text:sys.json_string(input)}", |vm| {
            let value = vm.bx.heap.new_string_from_str(input);
            vm.set_injected_global(makepad_script::id!(input), value);
        })
        .unwrap();
        let decoded: String = serde_json::from_str(node.attrs.text.as_ref().unwrap()).unwrap();
        assert_eq!(decoded, input);
    }
}
