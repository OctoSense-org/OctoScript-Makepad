//! File-backed L0 assembly for the design lab and native preview host.
use std::path::Path;

pub struct PreparedCard {
    pub source: String,
    pub tree: octoscript_render::UiNode,
    pub native_components: bool,
}

/// Give preview nodes deterministic, addressable IDs and retain their portable
/// provenance. This is opt-in instrumentation for the beauty host, not an
/// alternative source of measured bounds or control state: those come from Studio.
pub fn inspectable(tree: &mut octoscript_render::UiNode) -> Vec<serde_json::Value> {
    fn visit(
        n: &mut octoscript_render::UiNode,
        path: String,
        parent: Option<String>,
        out: &mut Vec<serde_json::Value>,
    ) {
        let original_id = n.attrs.id.clone();
        let id = format!("beauty_{path}");
        n.attrs.id = Some(id.clone());
        let a = &n.attrs;
        out.push(
            serde_json::json!({"id": id, "parent": parent, "original_id": original_id,
            "kind": format!("{:?}", n.kind), "kit": a.kit, "kit_index": a.kit_index, "text": a.text, "placeholder": a.placeholder,
            "image": a.src, "icon": a.icon_name, "width": a.w, "height": a.h,
            "enabled": a.enabled, "selected": a.selected, "on": a.on, "value": a.value, "value2": a.value2,
            "focused": a.focused, "password": a.password}),
        );
        for (i, child) in n.children.iter_mut().enumerate() {
            visit(child, format!("{path}_{i}"), Some(id.clone()), out);
        }
    }
    let mut out = Vec::new();
    visit(tree, "0".into(), None, &mut out);
    out
}

/// Use the same ordered palette chain as the application. Missing fragments
/// and incomplete realization are errors, never a fallback to another mood.
pub fn prepare(card: &str, data: &serde_json::Value, dir: &Path) -> Result<PreparedCard, String> {
    prepare_with_state(card, data, dir, &octoscript_ui_l0::InstanceStore::default())
}

/// Rebuild the same native Card after a declared event changed instance state.
pub fn prepare_with_state(card: &str, data: &serde_json::Value, dir: &Path,
    state: &octoscript_ui_l0::InstanceStore) -> Result<PreparedCard, String> {
    let report = octoscript_ui_l0::realize_with_state(card, data, state, Default::default());
    let root = report.complete_root()?;
    let mood = octoscript_ui_l0::card_theme(card).unwrap_or_else(|| "dark".into());
    if octoscript_ui_l0::kit_pack::contains(root) {
        if !octoscript_ui_l0::card_theme_axes(card).is_empty() {
            return Err("native kit theme axes require a registered token override".into());
        }
        let path = dir.join("native").join(&mood).join("kit.json");
        let pack: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(&path).map_err(|e|format!("{}: {e}",path.display()))?
        ).map_err(|e|e.to_string())?;
        if pack["theme"] != mood { return Err("kit theme does not match the ledger".into()); }
        let source = octoscript_ui_l0::kit_pack::lower(root, &pack, data)?;
        let tree = crate::design::prepare(&source)?;
        return Ok(PreparedCard { source, tree, native_components: true });
    }
    let read =
        |name: &str| std::fs::read_to_string(dir.join(name)).map_err(|e| format!("{name}: {e}"));
    let mut parts = vec![read("_palette_dark.octoscript")?];
    if mood != "dark" {
        parts.push(read(&format!("_palette_{mood}.octoscript"))?);
    }
    let mut axes = octoscript_ui_l0::card_theme_axes(card);
    let order = [
        "ground", "accent", "radius", "density", "emphasis", "icons", "texture", "depth", "type",
    ];
    axes.sort_by_key(|(axis, _)| order.iter().position(|a| a == axis).unwrap_or(usize::MAX));
    for (axis, value) in axes {
        if matches!(
            value.as_str(),
            "neutral" | "regular" | "none" | "soft" | "sans"
        ) {
            continue;
        }
        let file = if axis == "accent" {
            format!("_axis_accent_{value}_{mood}.octoscript")
        } else {
            format!("_axis_{axis}_{value}.octoscript")
        };
        parts.push(read(&file)?);
    }
    for name in ["_derive_color.octoscript", "_derive.octoscript"] {
        parts.push(read(name)?);
    }
    let pack_path=dir.join("native").join(&mood).join("kit.json");
    if pack_path.is_file() {
        let pack:serde_json::Value=serde_json::from_str(&std::fs::read_to_string(pack_path).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
        parts.push(octoscript_ui_l0::kit_pack::theme_source(&pack));
    }
    parts.push(read("_kit.octoscript")?);
    parts.push(octoscript_ui_l0::kit::lower(&root));
    let source = parts.join("\n");
    // The lab is offline: a source capability without an adapter fails the
    // checked VM. Fixture data belongs in the explicit host data argument.
    let tree = octoscript_render::build(&source, |_| {})
        .ok_or_else(|| "assembled L0 card failed checked Splash evaluation".to_string())?;
    Ok(PreparedCard { source, tree, native_components: false })
}

pub fn texts(node: &octoscript_render::UiNode) -> Vec<String> {
    let mut out = Vec::new();
    fn walk(n: &octoscript_render::UiNode, out: &mut Vec<String>) {
        if let Some(text) = &n.attrs.text {
            if !text.is_empty() {
                out.push(text.clone());
            }
        }
        for child in &n.children {
            walk(child, out);
        }
    }
    walk(node, &mut out);
    out
}

pub fn images(node: &octoscript_render::UiNode) -> Vec<String> {
    let mut out = Vec::new();
    fn walk(n: &octoscript_render::UiNode, out: &mut Vec<String>) {
        if matches!(n.kind,octoscript_render::NodeKind::Image | octoscript_render::NodeKind::Svg) {
            if let Some(src) = &n.attrs.src {
                out.push(src.clone());
            }
        }
        for child in &n.children {
            walk(child, out);
        }
    }
    walk(node, &mut out);
    out
}
