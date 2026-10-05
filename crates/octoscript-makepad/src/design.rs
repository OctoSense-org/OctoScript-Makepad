//! Source-measured Splash designs. Geometry is in logical pixels, text sizes
//! in CSS/Sketch pixels (Makepad's text API uses points, hence 72/96).
//! This path preserves explicit design styles; it does not apply a theme,
//! except that inline code in a markdown region takes the theme's code style.
use octoscript_render::{Attrs, NodeKind, UiNode};
use std::fmt::Write;

/// The symbol face every design text style falls back to.
const SYMBOLS: &str = "crate_resource(\"makepad_widgets:resources/jetbrains_mono_variable.ttf\")";

/// The largest advance between the wrapped lines of a markdown region, as a
/// multiple of the font size. The source `line_height` of such a region is
/// the distance between its paragraphs, which can be twice the font size.
const WRAPPED_LINE_PITCH: f32 = 1.4;

/// The line advance of makepad's `TextFlow` at `line_spacing: 1`, as a
/// multiple of the design font size, measured with Inter. A `Label` advances
/// by its family's natural line box instead.
const TEXT_FLOW_LINE_ADVANCE: f32 = 1.1303;

/// The font size and font resource a design text node must state.
fn text_font(a: &Attrs) -> Result<(f32, &str), String> {
    let size = a.size.ok_or("design font size required")?;
    // An empty resource names no font at all.
    let font = a
        .font_src
        .as_deref()
        .filter(|font| !font.is_empty())
        .ok_or("design font resource required")?;
    Ok((size, font))
}

/// The resource expression for a design font. `file:` names an absolute
/// platform path; anything else is a crate resource.
fn font_resource(font: &str) -> Result<String, String> {
    if let Some(path) = font.strip_prefix("file:") {
        if !std::path::Path::new(path).is_absolute() {
            return Err("platform font path must be absolute".into());
        }
        Ok(format!("file_resource({path:?})"))
    } else {
        Ok(format!("crate_resource({font:?})"))
    }
}

/// The natural line box of a bundled design family, as a multiple of the
/// font size. The importer supplies it with the family.
fn natural_line_box(font: &str) -> f32 {
    if font.contains("PlusJakarta") {
        1.26
    } else if font.contains("Poppins") {
        1.5
    } else {
        2478. / 2048.
    }
}

/// The colour emoji face every design text style falls back to.
fn emoji_resource() -> &'static str {
    if cfg!(target_os = "macos") {
        "file_resource(\"/System/Library/Fonts/Apple Color Emoji.ttc\")"
    } else {
        "crate_resource(\"makepad_widgets:resources/NotoColorEmoji.ttf\")"
    }
}

fn design_asset_allowed(src: &str) -> bool {
    if src.starts_with("http://127.0.0.1:") { return true; }
    #[cfg(target_arch = "wasm32")]
    if src.starts_with("http://localhost:") || src.starts_with("http://localhost/") { return true; }
    // The browser host additionally checks against its own iframe origin/base.
    // Native/lab builds retain their loopback-only resource rule.
    #[cfg(target_arch = "wasm32")]
    for base in [
        "https://octosense.org/wasm/service-cards/card-assets/",
        "https://octosense-org.github.io/wasm/service-cards/card-assets/",
        "https://octosense-org.github.io/Octosense-website/wasm/service-cards/card-assets/",
    ] {
        if let Some(path) = src.strip_prefix(base) {
            let parts: Vec<_> = path.split('/').collect();
            if parts.len() == 3 && parts[1] == "assets"
                && parts.iter().all(|part| !part.is_empty() && *part != "." && *part != ".."
                    && part.bytes().all(|c| c.is_ascii_alphanumeric() || matches!(c, b'-' | b'_' | b'.'))) {
                return true;
            }
        }
    }
    false
}

/// Turn template-local child paths into the IDs of this mounted instance.
/// Reject broken bindings before evaluating any Makepad widget source.
pub fn kit_contract(n: &UiNode) -> Result<Option<serde_json::Value>, String> {
    let Some(raw) = &n.attrs.kit else { return Ok(None) };
    let mut config: serde_json::Value = serde_json::from_str(raw).map_err(|e| e.to_string())?;
    if n.kind != NodeKind::Stack || !matches!(config["widget"].as_str(),
        Some("KitButton" | "KitFormField" | "KitTabBar" | "KitBottomNavigation" | "TaskplanProjectCard" | "CamoTrackRow")) {
        return Err("unknown semantic native widget".into());
    }
    fn target<'a>(root: &'a UiNode, path: &serde_json::Value) -> Result<&'a UiNode, String> {
        let path=path.as_array().ok_or("kit binding must be a child path")?;
        if path.is_empty() { return Err("kit binding cannot target itself".into()); }
        let mut node=root;
        for index in path {
            node=node.children.get(index.as_u64().ok_or("invalid kit child index")? as usize)
                .ok_or("kit child path is out of bounds")?;
        }
        Ok(node)
    }
    fn bind(root: &UiNode, path: &mut serde_json::Value, role: &str) -> Result<(), String> {
        let node=target(root,path)?;
        if role=="control" && !matches!(node.kind,NodeKind::Button | NodeKind::Radio) {
            return Err("semantic control requires a native Button or RadioButton".into());
        }
        if role=="input" && node.kind!=NodeKind::Input {
            return Err("semantic field requires native TextInput".into());
        }
        let id=node.attrs.id.as_ref().ok_or("semantic part requires an ID")?;
        *path=serde_json::Value::String(id.clone());
        Ok(())
    }
    for (role,path) in config["bindings"].as_object_mut().ok_or("missing kit bindings")? {
        bind(n,path,role)?;
    }
    if let Some(actions)=config.get_mut("action_bindings").and_then(|v|v.as_object_mut()) {
        for path in actions.values_mut() {bind(n,path,"control")?;}
    }
    let required=match config["widget"].as_str().unwrap() {
        "KitButton"=>Some("control"),"KitFormField"=>Some("input"),
        "TaskplanProjectCard"|"CamoTrackRow"=>Some("title"),_=>None,
    };
    if required.is_some_and(|role|!config["bindings"][role].is_string()) {
        return Err("semantic widget is missing its required native part".into());
    }
    if matches!(config["widget"].as_str(),Some("KitTabBar"|"KitBottomNavigation")) && !config["items"].is_array() {
        return Err("navigation is missing its native items".into());
    }
    if let Some(items)=config.get_mut("items").and_then(|v|v.as_array_mut()) {
        if items.len()<2 {return Err("navigation requires at least two items".into());}
        for item in items {
            bind(n,&mut item["root"],"root")?;
            bind(n,&mut item["control"],"control")?;
            for path in item["paint"].as_array_mut().ok_or("missing item paint bindings")? {
                bind(n,path,"paint")?;
            }
            for path in item["surfaces"].as_array_mut().ok_or("missing item surface bindings")? {
                bind(n,path,"surface")?;
            }
            if let Some(paths)=item.get_mut("indicators").and_then(|v|v.as_array_mut()) {
                for path in paths {bind(n,path,"indicator")?;}
            }
        }
    }
    if let Some(items)=config["items"].as_array() {
        let index=n.attrs.kit_index.ok_or("navigation requires selected_index")?;
        if index < -1 || index as usize>=items.len() && index!=-1 {
            return Err("navigation selected_index is out of bounds".into());
        }
        config["source_selected_index"]=config["selected_index"].clone();
        config["selected_index"]=index.into();
    }
    Ok(Some(config))
}

pub fn prepare(source: &str) -> Result<UiNode, String> {
    octoscript_render::build(source, |_| {})
        .ok_or_else(|| "design failed checked Splash evaluation".into())
}

/// A node's source frame, in the tree's window-local coordinates.
#[derive(Clone, Copy)]
struct Frame {
    x: f64,
    y: f64,
    w: f64,
}

impl Frame {
    fn of(a: &Attrs) -> Self {
        Frame {
            x: a.x.unwrap_or(0.),
            y: a.y.unwrap_or(0.),
            w: a.w.unwrap_or(0.).into(),
        }
    }
}

pub fn to_makepad_ui(tree: &UiNode) -> Result<String, String> {
    // `in_flow` is set for the children of a `row`/`col` stack: the parent's
    // flow places them, so they emit neither `abs_pos` nor a margin.
    // `parent` is the frame of the node's parent; the root's parent is the
    // mount, at the origin and as wide as the root.
    fn emit(
        n: &UiNode,
        out: &mut String,
        flow_origin: Option<(f64, f64)>,
        in_flow: bool,
        parent: Frame,
    ) -> Result<(), String> {
        let a = &n.attrs;
        let scroll_y = n.kind == NodeKind::Stack && a.variant.as_deref() == Some("scroll_y");
        // A `row` or `col` stack lays its children out along one axis, so they
        // follow its size instead of each keeping its measured frame. Measured
        // sources describe every container as a `stack`, so the flow is a stack
        // variant rather than a node kind.
        let flow_dir = match (n.kind, a.variant.as_deref()) {
            (NodeKind::Stack, Some("row")) => Some("Right"),
            (NodeKind::Stack, Some("col")) => Some("Down"),
            _ => None,
        };
        // A `markdown` text node is a region of prose whose body is markdown:
        // makepad's `Markdown` parses it and draws inline code as chips.
        let markdown = n.kind == NodeKind::Text && a.variant.as_deref() == Some("markdown");
        // A fill or fit flag replaces the measured extent with makepad's `Fill`
        // or `Fit`, so a reusable component can fill the width of its slot and
        // take its height from its content. Fill wins over fit on each axis.
        // Without a flag the measured size is emitted, which keeps fixed chrome
        // exact.
        let width = if a.fillw == Some(1) {
            "Fill".to_string()
        } else if a.fitw == Some(1) {
            "Fit".to_string()
        } else {
            a.w.ok_or("design width required")?.to_string()
        };
        let height = if a.fillh == Some(1) {
            "Fill".to_string()
        } else if a.fith == Some(1) {
            "Fit".to_string()
        } else {
            a.h.ok_or("design height required")?.to_string()
        };
        // A non-text node with `alignx: 1` is anchored to its parent's right
        // edge: it keeps its measured gap to that edge, so it follows the
        // parent's width rather than staying at its measured x. On text,
        // `alignx` aligns the run inside the label instead.
        let right_anchor = a.alignx == Some(1.0)
            && !in_flow
            && !matches!(n.kind, NodeKind::Text | NodeKind::Input);
        // Leaf widgets may reuse their Walk for internal text layout. Keep the
        // positioning margin on a wrapper so it cannot be applied twice.
        let wrapped = (flow_origin.is_some() || right_anchor)
            && !in_flow
            && (n.kind != NodeKind::Stack || right_anchor);
        // The node's offset in its parent.
        let left = a.x.unwrap_or(0.) - parent.x;
        let top = a.y.unwrap_or(0.) - parent.y;
        if wrapped {
            if right_anchor {
                // makepad ignores `align` on a child of an Overlay, so a
                // full-width wrapper aligns the node to the right, and the
                // node's right margin keeps its gap to the parent's edge.
                writeln!(
                    out,
                    "View {{width: Fill height: {height} margin: Inset{{top: {top}}} align: Align{{x: 1.0}}"
                )
                .unwrap();
            } else {
                let (x, y) = flow_origin.unwrap();
                writeln!(out, "View {{width: {} height: {} margin: Inset{{left: {} top: {} right: 0 bottom: 0}} flow: Overlay padding: 0 clip_x: false clip_y: false",
                    a.w.ok_or("design width required")?, a.h.ok_or("design height required")?,
                    a.x.unwrap_or(0.) - x, a.y.unwrap_or(0.) - y).unwrap();
            }
        }
        let contract = kit_contract(n)?;
        let glass = matches!(a.variant.as_deref(),Some("glass_surface" | "glass_overlay"));
        let glass_children = n.children.iter().any(|c|matches!(c.attrs.variant.as_deref(),Some("glass_surface" | "glass_overlay" | "glass_svg" | "glass_group")));
        let widget = if let Some(config)=&contract { config["widget"].as_str().unwrap() } else { match n.kind {
            NodeKind::Stack if matches!(a.variant.as_deref(),Some("text_runs" | "text_shadow_label")) => "DesignText",
            NodeKind::Stack if a.variant.as_deref() == Some("pill") => "DesignPill",
            NodeKind::Stack if a.variant.as_deref() == Some("button") => "DesignButton",
            NodeKind::Stack if a.variant.as_deref() == Some("radio") => "DesignRadio",
            NodeKind::Stack if scroll_y => "ScrollYView",
            NodeKind::Stack if glass => "DesignGlassSurface",
            // The importer marks the component that owns the glass and its
            // foreground. Promoting its parent too would move an entire
            // artboard into the overlay and leave the backdrop scene empty.
            NodeKind::Stack if a.variant.as_deref() == Some("glass_group") => "DesignOverlay",
            NodeKind::Stack if a.variant.as_deref()==Some("text_shadow") => "DesignTextShadow",
            NodeKind::Stack if matches!(a.variant.as_deref(),Some("surface" | "ellipse")) => "DesignSurface",
            NodeKind::Stack if a.bg.is_some() => "DesignSurface",
            NodeKind::Stack => "View",
            NodeKind::Text if markdown => "Markdown",
            NodeKind::Text if a.rotation.is_some() => "DesignRotatedLabel",
            NodeKind::Text => "Label",
            NodeKind::Web => "Browser",
            NodeKind::Input => "DesignInput",
            NodeKind::Button => "DesignNativeButton",
            NodeKind::Slider | NodeKind::RangeSlider => "DesignAtroSlider",
            NodeKind::Image => "DesignImage",
            NodeKind::Radio if a.variant.as_deref()==Some("silent") => "KitSelectionControl",
            NodeKind::Radio => "DesignCamoRadio",
            NodeKind::Toggle if a.variant.as_deref()==Some("camo") => "DesignCamoToggle",
            NodeKind::Toggle if a.variant.as_deref()==Some("atro") => "DesignAtroToggle",
            NodeKind::Toggle => "DesignToggle",
            NodeKind::Checkbox if a.variant.as_deref()==Some("atro_pill") => "DesignAtroPill",
            NodeKind::Checkbox if a.variant.as_deref()==Some("camo") => "DesignCamoCheckbox",
            NodeKind::Checkbox => "DesignAtroCheckbox",
            NodeKind::Svg if a.variant.as_deref()==Some("glass_svg") => "DesignGlassSvg",
            NodeKind::Svg => "Svg",
            NodeKind::StockPlot if a.variant.as_deref()==Some("donut") => "mod.plot.DonutChart",
            NodeKind::StockPlot if a.variant.as_deref()==Some("bar") => "mod.plot.BarPlot",
            NodeKind::StockPlot if a.variant.as_deref()==Some("radar") => "mod.plot.RadarChart",
            NodeKind::StockPlot if a.variant.as_deref()==Some("scatter") => "mod.plot.ScatterPlot",
            NodeKind::StockPlot => "mod.plot.LinePlot",
            NodeKind::Progress => "DesignProgressBar",
            _ => return Err(format!("unsupported measured design node: {:?}", n.kind)),
        }};
        if let Some(id) = &a.id {
            if !id.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                return Err("invalid design widget ID".into());
            }
            write!(out, "{id} := ").unwrap();
        }
        writeln!(out, "{widget} {{").unwrap();
        if n.kind == NodeKind::Input {
            for (property, route) in [("on_change", &a.changeto), ("on_return", &a.tapto)] {
                if let Some(target) = route.as_ref().filter(|t| !t.is_empty()) {
                    writeln!(out, "{property}: fn(text) {{ NAV(t: {target:?}, v: text) }}").unwrap();
                }
            }
        } else if n.kind == NodeKind::Button {
            if let Some(target) = a.tapto.as_ref().filter(|t| !t.is_empty()) {
                writeln!(out, "on_click: || {{ NAV(t: {target:?}) }}").unwrap();
            }
        }
        if let Some(config)=&contract {
            writeln!(out,"contract: {:?} glass: {}",config.to_string(),a.variant.as_deref()==Some("glass_group")).unwrap();
        }
        writeln!(out, "width: {width} height: {height}").unwrap();
        // Source frames stay window-local. Inside a scroll viewport, convert
        // them to parent-relative overlay margins so native layout applies the
        // scroll offset and measures the complete content extent.
        if in_flow {
            // The parent's flow places this node.
        } else if wrapped {
            if right_anchor {
                let w = f64::from(a.w.unwrap_or(0.));
                let gap = (parent.w - left - w).max(0.) as f32;
                writeln!(out, "margin: Inset{{right: {gap}}}").unwrap();
            } else {
                writeln!(out, "margin: 0").unwrap();
            }
        } else if let Some((x, y)) = flow_origin {
            writeln!(out, "margin: Inset{{left: {} top: {} right: 0 bottom: 0}}",
                a.x.unwrap_or(0.) - x, a.y.unwrap_or(0.) - y).unwrap();
        } else {
            writeln!(out, "abs_pos: vec2({}, {})", a.x.unwrap_or(0.), a.y.unwrap_or(0.)).unwrap();
        }
        match n.kind {
            NodeKind::Web => {
                let src=a.src.as_ref().ok_or("web document source required")?;
                if !src.starts_with("data:text/html;charset=utf-8;base64,") {
                    return Err("measured web content requires an inline HTML document".into());
                }
                writeln!(out,"backend: BrowserBackend.Native url: {src:?}").unwrap();
            }
            NodeKind::StockPlot => {
                writeln!(out,"demo_data: false clip_plot: false plot_margin: Inset{{left:0 top:0 right:0 bottom:0}} draw_bg.color: #0000 draw_vector.draw_depth: 0").unwrap();
                if a.variant.as_deref()==Some("donut") {
                    writeln!(out,"show_percentages: false show_labels: false").unwrap();
                } else if a.variant.as_deref()==Some("bar") {
                    writeln!(out,"show_grid: false show_ticks: false show_border: false show_category_labels: false show_value_ticks: false").unwrap();
                } else if a.variant.as_deref()==Some("scatter") {
                    writeln!(out,"show_grid: false show_ticks: false show_border: false data_padding: 0").unwrap();
                } else if a.variant.as_deref()==Some("radar") {
                    writeln!(out,"show_grid: false show_ticks: false show_border: false show_radar_grid: false").unwrap();
                } else {
                    writeln!(out,"show_grid: false show_ticks: false show_border: false data_padding: 0 show_points: false").unwrap();
                }
            }
            NodeKind::Progress => {
                writeln!(out,"value: {} draw_bg.track_color: {} draw_bg.fill_color: {}",
                    a.value.unwrap_or(0.),super::hex_rgba(a.bg.unwrap_or(0xffdddddd)),
                    super::hex_rgba(a.color.unwrap_or(0xff22785d))).unwrap();
            }
            NodeKind::Slider | NodeKind::RangeSlider => {
                writeln!(out,"min: 0 max: 1 default: {} draw_bg.ink: {} draw_bg.handle_border: {}",
                    a.value.unwrap_or(0.),super::hex_rgba(a.color.unwrap_or(0xff4c5fef)),a.border.unwrap_or(2.5)).unwrap();
                if let Some(end)=a.value2 {writeln!(out,"range_end: {end}").unwrap();}
            }
            NodeKind::Button => {
                writeln!(out,"enabled: {}",a.enabled.unwrap_or(1)!=0).unwrap();
            }
            NodeKind::Toggle | NodeKind::Checkbox | NodeKind::Radio => {
                if n.kind!=NodeKind::Radio {
                    writeln!(out, "active: {}", a.on.unwrap_or(0) != 0).unwrap();
                }
                if a.variant.as_deref()==Some("camo") {
                    writeln!(out,"draw_bg.ink: {} draw_bg.off_color: {} draw_bg.mark_color: {} draw_bg.corner_radius: {}",
                        super::hex_rgba(a.color.unwrap_or(0xff0077ff)),super::hex_rgba(a.bg.unwrap_or(0x1a000000)),
                        super::hex_rgba(a.bordercolor.unwrap_or(0xffffffff)),a.radius.unwrap_or(3.)).unwrap();
                } else if n.kind==NodeKind::Toggle && a.variant.is_none() {
                    writeln!(out,"draw_bg.ink: {} draw_bg.off_color: {} draw_bg.mark_color: {}",
                        super::hex_rgba(a.color.unwrap_or(0xff1e8eba)),super::hex_rgba(a.bg.unwrap_or(0xff9da4ae)),
                        super::hex_rgba(a.bordercolor.unwrap_or(0xffffffff))).unwrap();
                } else if n.kind==NodeKind::Checkbox && a.variant.as_deref()!=Some("atro_pill") {
                    writeln!(out,"draw_bg.silent: {}",if a.variant.as_deref()==Some("atro_silent") {1.0} else {0.0}).unwrap();
                    writeln!(out,"draw_bg.ink: {} draw_bg.corner_radius: {} draw_bg.mark_color: {}",super::hex_rgba(a.color.unwrap_or(0xff4c5fef)),a.radius.unwrap_or(0.),super::hex_rgba(a.bordercolor.unwrap_or(0xffffffff))).unwrap();
                } else if a.variant.as_deref()==Some("atro_pill") {
                    let checked=a.on.unwrap_or(0)!=0;
                    writeln!(out,"draw_bg.active_color: {} draw_bg.inactive_color: {} draw_bg.mark_color: {}",
                        super::hex_rgba(if checked {a.bg.unwrap_or(0xff4c5fef)} else {0xff4c5fef}),
                        super::hex_rgba(if checked {0xff191922} else {a.bg.unwrap_or(0xff191922)}),
                        super::hex_rgba(a.color.unwrap_or(0xffffffff))).unwrap();
                }
            }
            NodeKind::Stack => {
                if a.variant.as_deref()==Some("text_shadow_label") {
                    writeln!(out,"foreground_only: true").unwrap();
                }
                if a.variant.as_deref()==Some("text_shadow") {
                    writeln!(out,"draw_bg.tint_color: {} draw_bg.sigma: {}",
                        super::hex_rgba(a.color.unwrap_or(0)),a.blur.unwrap_or(0.)*0.5).unwrap();
                }
                if glass {
                    writeln!(out,"draw_bg.corner_radius: {} draw_bg.tint_color: {} draw_bg.overlay_blend: {} draw_bg.blur_level: {} draw_bg.backdrop_tint: {}",
                        a.radius.unwrap_or(0.),super::hex_rgba(a.bg.unwrap_or(0)),
                        if a.variant.as_deref()==Some("glass_overlay") {1.0} else {0.0},
                        a.blur.unwrap_or(1.).max(1.).log2().clamp(0.,6.),super::hex_rgba(a.color.unwrap_or(0))).unwrap();
                }
                if matches!(a.variant.as_deref(),Some("surface" | "ellipse")) {
                    if let Some(bg2)=a.bg2 {writeln!(out,"draw_bg.color2: {} draw_bg.gradient: 1.0",super::hex_rgba(bg2)).unwrap();}
                    writeln!(out,"draw_bg.radius: {} draw_bg.ellipse: {} draw_bg.border_width: {} draw_bg.border_position: {} draw_bg.border_color: {}",
                        a.radius.unwrap_or(0.), if a.variant.as_deref()==Some("ellipse") {1.0} else {0.0},
                        a.border.unwrap_or(0.),a.value.unwrap_or(0.),super::hex_rgba(a.bordercolor.unwrap_or(0))).unwrap();
                }
                if a.variant.as_deref() == Some("radio") {
                    writeln!(out, "active: {}", a.on.unwrap_or(0) != 0).unwrap();
                }
                if a.variant.as_deref() == Some("button") {
                    writeln!(out,"glass: {glass_children}").unwrap();
                    writeln!(
                        out,
                        "enabled: {} cursor: MouseCursor.Hand",
                        a.enabled.unwrap_or(1) != 0
                    )
                    .unwrap();
                }
                if let Some(dir) = flow_dir {
                    let px = a.padx.or(a.pad).unwrap_or(0.);
                    let py = a.pady.or(a.pad).unwrap_or(0.);
                    writeln!(
                        out,
                        "flow: {dir} padding: Inset{{left: {px} top: {py} right: {px} bottom: {py}}}"
                    )
                    .unwrap();
                    if let Some(spacing) = a.spacing {
                        writeln!(out, "spacing: {spacing}").unwrap();
                    }
                    if let Some(bg) = a.bg {
                        // A filled stack is a `DesignSurface`, so the radius
                        // rounds the fill.
                        writeln!(
                            out,
                            "show_bg: true draw_bg.color: {} draw_bg.radius: {}",
                            super::hex_rgba(bg),
                            a.radius.unwrap_or(0.)
                        )
                        .unwrap();
                    }
                    if let Some(selected) = a.selected {
                        writeln!(out, "selected: {}", selected != 0).unwrap();
                    }
                    for c in &n.children {
                        emit(c, out, None, true, Frame::of(a))?;
                    }
                } else {
                    let clip = scroll_y || a.variant.as_deref() == Some("clip");
                    writeln!(
                        out,
                        "flow: Overlay padding: 0 clip_x: {clip} clip_y: {clip}"
                    )
                    .unwrap();
                    if let Some(bg) = a.bg {
                        writeln!(out, "show_bg: true draw_bg.color: {}", super::hex_rgba(bg))
                            .unwrap();
                    }
                    if let Some(selected) = a.selected {
                        writeln!(out, "selected: {}", selected != 0).unwrap();
                    }
                    for c in &n.children {
                        // A right-anchored node no longer sits at its source x,
                        // so its children are placed relative to it.
                        let origin = if scroll_y || flow_origin.is_some() || right_anchor {
                            Some((a.x.unwrap_or(0.), a.y.unwrap_or(0.)))
                        } else {
                            None
                        };
                        emit(c, out, origin, false, Frame::of(a))?;
                    }
                }
            }
            NodeKind::Text if markdown => {
                let (size, font) = text_font(a)?;
                let weight = a.weight.unwrap_or(400);
                // The source `line_height` of a region is the distance between
                // its paragraphs; wrapped lines advance by at most
                // `WRAPPED_LINE_PITCH`. `TextFlow` scales `line_spacing`
                // against its own line advance, not the family's line box.
                let spacing = if a.font_asc.is_some() {
                    1.0
                } else {
                    let pitch = a.line_height.unwrap_or(size * natural_line_box(font));
                    pitch.min(size * WRAPPED_LINE_PITCH) / (size * TEXT_FLOW_LINE_ADVANCE)
                };
                let style = format!(
                    "TextStyle{{font_family: FontFamily{{latin := FontMember{{res: {} asc: 0.04 desc: 0.04 weight: {weight}}} symbols := FontMember{{res: {SYMBOLS} asc: 0 desc: 0 weight: 400}} emoji := FontMember{{res: {} asc: 0 desc: 0}}}} font_size: {} line_spacing: {spacing}}}",
                    font_resource(font)?,
                    emoji_resource(),
                    size * 0.75
                );
                writeln!(out, "body: {:?}", a.text.as_deref().unwrap_or("")).unwrap();
                // `TextFlow` sizes every run by its own `font_size` rather than
                // by the style's, so the label's point size goes here.
                writeln!(out, "font_size: {}", size * 0.75).unwrap();
                let paragraph = a.line_height.unwrap_or(size * 1.45);
                writeln!(out, "paragraph_spacing: {paragraph}").unwrap();
                writeln!(out, "text_style_normal: {style}").unwrap();
                // Inline code is set in the theme's code style, as in makepad's
                // own `Markdown`, at the size and spacing of the prose.
                writeln!(
                    out,
                    "text_style_fixed: mod.theme.font_code{{font_size: {} line_spacing: {spacing}}}",
                    size * 0.75
                )
                .unwrap();
                // `TextFlow` paints each run with its `font_color`, so that is
                // where the ink goes, not `draw_text.color`.
                let ink = super::hex_rgba(a.color.unwrap_or(0xff111927));
                writeln!(out, "font_color: {ink}").unwrap();
                // The node's fill colours the inline-code chips. Their padding
                // and margin are the theme's.
                let chip = super::hex_rgba(a.bg.unwrap_or(0xfff4f4f5));
                writeln!(out, "draw_block +: {{code_color: {chip}}}").unwrap();
            }
            NodeKind::Text | NodeKind::Input => {
                let (size, font) = text_font(a)?;
                let weight = a.weight.unwrap_or(400);
                if let Some(rotation)=a.rotation {
                    writeln!(out,"draw_text.rotation: {}",rotation.to_radians()).unwrap();
                }
                if n.kind == NodeKind::Text && a.font_asc.is_some() {
                    writeln!(out, "clip_x: false clip_y: false").unwrap();
                }
                if n.kind == NodeKind::Input && a.font_asc.is_some() {
                    writeln!(out, "clip_y: false").unwrap();
                }
                let align_property = if n.kind == NodeKind::Input {
                    "label_align"
                } else {
                    "align"
                };
                // A measured box no taller than one line holds a single line.
                // Text with no measured height takes it from its content, so
                // it wraps unless it is marked `single_line`.
                let one_line = a.line_height.unwrap_or(size * 1.3) + 0.5;
                let single_line = a.variant.as_deref() == Some("single_line")
                    || a.h.is_some_and(|h| h <= one_line);
                writeln!(
                    out,
                    "padding: 0 text: {:?} {align_property}: Align{{x: {} y: 0.5}}",
                    a.text.as_deref().unwrap_or(""),
                    a.alignx.unwrap_or(0.)
                )
                .unwrap();
                if n.kind == NodeKind::Input {
                    if let Some(left)=a.padleft {
                        writeln!(out,"padding: Inset{{left: {left}}}").unwrap();
                    }
                    if a.variant.as_deref() == Some("multiline") {
                        writeln!(out, "is_multiline: true flow: Right {{wrap: true}}").unwrap();
                    }
                    writeln!(
                        out,
                        "empty_text: {:?} is_password: {}",
                        a.placeholder.as_deref().unwrap_or(""),
                        a.password.unwrap_or(0) != 0
                    )
                    .unwrap();
                } else if single_line {
                    // A single Sketch line must not wrap a whole word because
                    // native font advances differ by a fraction of a point.
                    // A line that fills its width cannot know how long its
                    // text will be (a reusable row binds a title of any
                    // length), so it ends in an ellipsis when it overflows. A
                    // measured line keeps its measured box and clips as before.
                    if a.fillw == Some(1) {
                        writeln!(
                            out,
                            "flow: Right max_lines: 1 text_overflow: TextOverflow.Ellipsis"
                        )
                        .unwrap();
                    } else {
                        writeln!(out, "flow: Right").unwrap();
                    }
                }
                // Use real font metrics and a measured line height.
                let line_box = natural_line_box(font);
                let spacing = if a.font_asc.is_some() {1.0} else {a.line_height.unwrap_or(size * line_box) / (size * line_box)};
                let shift = if a
                    .text
                    .as_deref()
                    .unwrap_or("")
                    .chars()
                    .any(|c| c as u32 >= 0x1f000)
                    || font.contains("Inter")
                {
                    0.04
                } else {
                    0.18
                };
                let emoji = emoji_resource();
                let asc=a.font_asc.unwrap_or(shift);
                let desc=a.font_desc.unwrap_or(shift);
                let resource = font_resource(font)?;
                writeln!(
                    out,
                    "draw_text.text_style: TextStyle{{font_family: FontFamily{{latin := FontMember{{res: {resource} asc: {asc} desc: {desc} weight: {weight}}} symbols := FontMember{{res: {SYMBOLS} asc: 0 desc: 0 weight: 400}} emoji := FontMember{{res: {emoji} asc: 0 desc: 0}}}} font_size: {} line_spacing: {spacing} letter_spacing: {}}}",
                    size * 0.75,
                    a.tracking.unwrap_or(0.)
                )
                .unwrap();
                writeln!(
                    out,
                    "draw_text.color: {}",
                    super::hex_rgba(a.color.unwrap_or(0xff111927))
                )
                .unwrap();
            }
            NodeKind::Image => {
                let src = a.src.as_ref().ok_or("design image resource required")?;
                if !design_asset_allowed(src) {
                    return Err("design assets must use the lab's local asset server".into());
                }
                writeln!(out, "src: http_resource({src:?}) fit: ImageFit.Stretch").unwrap();
                // Sketch graphic canvases are exported at exactly 2x. Sample
                // premultiplied texels before interpolation to preserve alpha
                // edges without introducing dark fringes at fractional frames.
                // A filled or fitted image without a measured frame must state
                // its raster size, since the shader samples by it.
                let dim_w = a.image_width.or(a.w.map(|w| w * 2.));
                let dim_h = a.image_height.or(a.h.map(|h| h * 2.));
                let (Some(dim_w), Some(dim_h)) = (dim_w, dim_h) else {
                    return Err("design image size required".into());
                };
                writeln!(
                    out,
                    "draw_bg.image_dim_w: {dim_w:.1} draw_bg.image_dim_h: {dim_h:.1}"
                )
                .unwrap();
            }
            NodeKind::Svg => {
                if a.variant.as_deref()==Some("glass_svg") {
                    writeln!(out,"blur: {} draw_svg.backdrop_tint: {}",a.blur.unwrap_or(1.),super::hex_rgba(a.color.unwrap_or(0))).unwrap();
                    writeln!(out,"draw_svg.overlay_blend: {}",a.value.unwrap_or(0.)).unwrap();
                }
                let src=a.src.as_ref().ok_or("design SVG resource required")?;
                if !design_asset_allowed(src) || !src.ends_with(".svg") {
                    return Err("design vectors require a local SVG asset".into());
                }
                writeln!(out,"animating: false draw_svg.svg: http_resource({src:?}) draw_svg.preserve_viewbox: true draw_svg.preserve_aspect: false").unwrap();
            }
            _ => unreachable!(),
        }
        writeln!(out, "}}").unwrap();
        if wrapped { writeln!(out, "}}").unwrap(); }
        Ok(())
    }
    let mount = Frame {
        x: 0.,
        y: 0.,
        w: tree.attrs.w.unwrap_or(0.).into(),
    };
    let mut out = String::new();
    emit(tree, &mut out, None, false, mount)?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scroll_content_flows_relative_to_parents_and_chrome_stays_absolute() {
        let tree=prepare(r#"{t:"stack" x:0 y:0 w:406 h:776 c:[
            {t:"stack" id:"reader" variant:"scroll_y" x:0 y:88 w:406 h:616 c:[
                {t:"stack" id:"content" x:0 y:88 w:406 h:1800 c:[
                    {t:"stack" id:"last" x:23 y:1800 w:360 h:24}
                ]}
            ]}
            {t:"stack" id:"toolbar" x:0 y:704 w:406 h:49}
        ]}"#).unwrap();
        let ui=to_makepad_ui(&tree).unwrap();
        assert!(ui.contains("reader := ScrollYView {\nwidth: 406 height: 616\nabs_pos: vec2(0, 88)"));
        assert!(ui.contains("clip_x: true clip_y: true"));
        assert!(ui.contains("content := View {\nwidth: 406 height: 1800\nmargin: Inset{left: 0 top: 0 right: 0 bottom: 0}"));
        assert!(ui.contains("last := View {\nwidth: 360 height: 24\nmargin: Inset{left: 23 top: 1712 right: 0 bottom: 0}"));
        assert!(ui.contains("toolbar := View {\nwidth: 406 height: 49\nabs_pos: vec2(0, 704)"));
    }

    #[test]
    fn semantic_index_keeps_no_selection_and_nonzero_values() {
        for index in [-1,0,4] {
            let node=prepare(&format!("{{t:\"stack\",w:100,h:40,kit_index:{index}}}")).unwrap();
            assert_eq!(node.attrs.kit_index,Some(index));
        }
    }

    #[test]
    fn semantic_controls_bind_to_real_native_descendants_after_instrumentation() {
        let mut node=prepare(r#"{t:"stack",id:"button",w:100,h:40,c:[{t:"button",id:"hit",w:100,h:40,enabled:1}]}"#).unwrap();
        node.attrs.kit=Some(serde_json::json!({"widget":"KitButton","bindings":{"control":[0]}}).to_string());
        crate::l0::inspectable(&mut node);
        let ui=to_makepad_ui(&node).unwrap();
        assert!(ui.contains("beauty_0 := KitButton"));
        assert!(ui.contains("beauty_0_0 := DesignNativeButton"));
        assert_eq!(kit_contract(&node).unwrap().unwrap()["bindings"]["control"],"beauty_0_0");
        node.children[0].kind=NodeKind::Stack;
        assert!(to_makepad_ui(&node).unwrap_err().contains("native Button"));
        node.attrs.kit=Some(serde_json::json!({"widget":"KitFormField","bindings":{}}).to_string());
        assert!(to_makepad_ui(&node).unwrap_err().contains("required native part"));
        node.attrs.kit=Some(serde_json::json!({"widget":"KitButton","bindings":{"control":[]}}).to_string());
        assert!(to_makepad_ui(&node).unwrap_err().contains("target itself"));
    }

    #[test]
    fn camo_selectors_use_native_widget_types_and_source_colors() {
        let tree=prepare(r#"{t:"stack" w:375 h:812 c:[
            {t:"radio" variant:"camo" w:18 h:18 on:1 color:4278220799}
            {t:"checkbox" variant:"camo" w:18 h:18 on:0}
            {t:"toggle" variant:"camo" w:44 h:24 on:1}
        ]}"#).unwrap();
        let ui=to_makepad_ui(&tree).unwrap();
        assert!(ui.contains("DesignCamoRadio {"));
        assert!(ui.contains("DesignCamoCheckbox {"));
        assert!(ui.contains("DesignCamoToggle {"));
        assert!(ui.contains("draw_bg.ink: #0077ffff"));
    }

    #[test]
    fn glass_component_keeps_artboard_background_in_scene_pass() {
        let tree=prepare(r#"{t:"stack" id:"page" w:375 h:812 bg:4294967295 c:[
            {t:"stack" id:"tooltip" variant:"glass_group" x:100 y:200 w:110 h:47 c:[
                {t:"stack" variant:"glass_surface" x:100 y:200 w:110 h:47 bg:2147483648}
            ]}
        ]}"#).unwrap();
        let ui=to_makepad_ui(&tree).unwrap();
        assert!(ui.contains("page := DesignSurface"));
        assert!(ui.contains("tooltip := DesignOverlay"));
        assert_eq!(ui.matches("DesignOverlay {").count(),1);
    }

    #[test]
    fn overlay_glass_preserves_backdrop_tint_in_native_shader() {
        let tree=prepare(r#"{t:"stack" variant:"glass_overlay" w:100 h:40
            bg:2164260863 color:2148798246 blur:12}"#).unwrap();
        let ui=to_makepad_ui(&tree).unwrap();
        assert!(ui.contains("draw_bg.overlay_blend: 1"));
        assert!(ui.contains("draw_bg.backdrop_tint: #140f2680"),"{ui}");
    }

    #[test]
    fn native_design_preserves_source_text_and_control_state() {
        let mut tree = prepare(r#"{t:"stack" id:"source_root" w:393 h:852 c:[
            {t:"stack" variant:"button" id:"source_button" enabled:0 x:24 y:546 w:345 h:52 c:[
                {t:"text" id:"source_text" text:"Login" x:177.5 y:561 w:38 h:22
                 size:14 weight:700 line_height:22 font_src:"self:resources/taskplan/PlusJakartaSans.ttf"}
            ]}
            {t:"toggle" id:"source_toggle" x:325 y:642 w:44 h:24 on:1}
        ]}"#).unwrap();
        let manifest = crate::l0::inspectable(&mut tree);
        assert_eq!(manifest[1]["original_id"], "source_button");
        assert_eq!(manifest[1]["enabled"], 0);
        assert_eq!(manifest[2]["text"], "Login");
        assert_eq!(manifest[3]["on"], 1);
        let ui = to_makepad_ui(&tree).unwrap();
        assert!(ui.contains("DesignButton {"));
        assert!(ui.contains("enabled: false"));
        assert!(ui.contains("font_size: 10.5"));
        assert!(ui.contains("DesignToggle {"));
        assert!(ui.contains("active: true"));
    }

    #[test]
    fn unsupported_nodes_and_nonlocal_graphics_fail_closed() {
        let unsupported = prepare(r#"{t:"web" w:100 h:20}"#).unwrap();
        assert!(to_makepad_ui(&unsupported).is_err());
        let image =
            prepare(r#"{t:"image" w:40 h:40 src:"https://example.com/image.png"}"#).unwrap();
        assert!(to_makepad_ui(&image).is_err());
    }

    #[test]
    fn html_document_uses_platform_browser_and_rejects_external_sources() {
        let tree=prepare(r#"{t:"web" w:406 h:616 src:"data:text/html;charset=utf-8;base64,PGI+SGVsbG88L2I+"}"#).unwrap();
        let ui=to_makepad_ui(&tree).unwrap();
        assert!(ui.contains("Browser {"));
        assert!(ui.contains("backend: BrowserBackend.Native"));
        let remote=prepare(r#"{t:"web" w:406 h:616 src:"https://example.com"}"#).unwrap();
        assert!(to_makepad_ui(&remote).is_err());
    }

    #[test]
    fn native_input_retains_editable_value_placeholder_and_focus_contract() {
        let mut tree = prepare(
            r#"{t:"input" w:313 h:22 text:"example@gmai" placeholder:""
            focused:1 password:0 size:14 weight:400 line_height:22
            font_src:"self:resources/taskplan/PlusJakartaSans.ttf"}"#,
        )
        .unwrap();
        let manifest = crate::l0::inspectable(&mut tree);
        assert_eq!(manifest[0]["kind"], "Input");
        assert_eq!(manifest[0]["focused"], 1);
        assert_eq!(manifest[0]["text"], "example@gmai");
        let ui = to_makepad_ui(&tree).unwrap();
        assert!(ui.contains("DesignInput {"));
        assert!(ui.contains("label_align: Align"));
        assert!(ui.contains("is_password: false"));
    }

    #[test]
    fn graphic_only_design_retains_its_inspectable_image() {
        let tree = prepare(
            r#"{t:"stack" w:393 h:852 c:[
            {t:"image" id:"logo" x:100 y:200 w:50 h:20 src:"http://127.0.0.1:8794/logo.png"}
        ]}"#,
        )
        .unwrap();
        assert_eq!(tree.count(), 2);
        let ui = to_makepad_ui(&tree).unwrap();
        assert!(ui.contains("logo := DesignImage"));
        assert!(ui.contains("draw_bg.image_dim_w: 100.0 draw_bg.image_dim_h: 40.0"));
    }

    #[test]
    fn row_and_col_stacks_place_their_children_in_a_flow() {
        let tree = prepare(
            r#"{t:"stack" id:"page" w:300 h:200 c:[
            {t:"stack" id:"toolbar" variant:"row" x:10 y:20 w:280 h:40 padx:12 pady:6
             spacing:8 bg:4294967295 radius:10 c:[
                {t:"stack" id:"icon" x:22 y:26 w:28 h:28}
                {t:"text" id:"title" text:"Inbox" x:58 y:30 w:60 h:20 size:14 line_height:20
                 font_src:"self:resources/taskplan/PlusJakartaSans.ttf"}
            ]}
            {t:"stack" id:"list" variant:"col" x:10 y:70 w:280 h:120 pad:4 c:[
                {t:"stack" id:"item" x:14 y:74 w:272 h:20}
            ]}
        ]}"#,
        )
        .unwrap();
        let ui = to_makepad_ui(&tree).unwrap();
        // The container keeps its own frame and lays its children out.
        let toolbar = "toolbar := DesignSurface {\nwidth: 280 height: 40\nabs_pos: vec2(10, 20)\n\
            flow: Right padding: Inset{left: 12 top: 6 right: 12 bottom: 6}\nspacing: 8\n\
            show_bg: true draw_bg.color: #ffffffff draw_bg.radius: 10\n";
        assert!(ui.contains(toolbar), "{ui}");
        let list = "list := View {\nwidth: 280 height: 120\nabs_pos: vec2(10, 70)\n\
            flow: Down padding: Inset{left: 4 top: 4 right: 4 bottom: 4}\n";
        assert!(ui.contains(list), "{ui}");
        // Its children are placed by the flow: no `abs_pos`, margin or wrapper.
        for child in [
            "icon := View {\nwidth: 28 height: 28\nflow: Overlay",
            "title := Label {\nwidth: 60 height: 20\npadding: 0",
            "item := View {\nwidth: 272 height: 20\nflow: Overlay",
        ] {
            assert!(ui.contains(child), "{ui}");
        }
    }

    #[test]
    fn fill_and_fit_flags_replace_the_measured_extent() {
        let tree = prepare(
            r#"{t:"stack" id:"card" w:360 h:120 fillw:1 fith:1 c:[
            {t:"stack" id:"chip" x:10 y:10 w:80 h:24 fitw:1}
            {t:"stack" id:"control" w:360 h:120 fillw:1 fillh:1}
        ]}"#,
        )
        .unwrap();
        let ui = to_makepad_ui(&tree).unwrap();
        for node in [
            "card := View {\nwidth: Fill height: Fit\n",
            "chip := View {\nwidth: Fit height: 24\n",
            "control := View {\nwidth: Fill height: Fill\n",
        ] {
            assert!(ui.contains(node), "{ui}");
        }
        // A flag stands in for a missing measurement; an axis with neither fails.
        let unmeasured = prepare(r#"{t:"stack" fillw:1 fith:1}"#).unwrap();
        assert!(to_makepad_ui(&unmeasured).is_ok());
        let no_height = prepare(r#"{t:"stack" fillw:1}"#).unwrap();
        let error = to_makepad_ui(&no_height).unwrap_err();
        assert_eq!(error, "design height required");
        // Text sized by its content wraps; a measured single line does not.
        let font = r#"size:14 line_height:20 font_src:"self:resources/Inter.ttf""#;
        let lower = |source: String| to_makepad_ui(&prepare(&source).unwrap()).unwrap();
        let paragraph = lower(format!("{{t:\"text\" text:\"A\" fillw:1 fith:1 {font}}}"));
        assert!(!paragraph.contains("flow: Right"), "{paragraph}");
        let line = lower(format!("{{t:\"text\" text:\"A\" w:40 h:20 {font}}}"));
        assert!(line.contains("flow: Right"), "{line}");
    }

    #[test]
    fn a_filled_single_line_ends_in_an_ellipsis() {
        let font = r#"size:14 line_height:20 font_src:"self:resources/Inter.ttf""#;
        let lower = |source: String| to_makepad_ui(&prepare(&source).unwrap()).unwrap();
        let filled = lower(format!("{{t:\"text\" text:\"A\" w:4 h:20 fillw:1 {font}}}"));
        let ellipsis = "flow: Right max_lines: 1 text_overflow: TextOverflow.Ellipsis\n";
        assert!(filled.contains(ellipsis), "{filled}");
        let measured = lower(format!("{{t:\"text\" text:\"A\" w:40 h:20 {font}}}"));
        assert!(measured.contains("flow: Right\n"), "{measured}");
        assert!(!measured.contains("max_lines"), "{measured}");
    }

    #[test]
    fn markdown_text_sets_inline_code_in_the_theme_code_style() {
        let tree = prepare(
            r#"{t:"text" id:"answer" variant:"markdown" text:"Run `cargo test` first."
            x:20 y:40 w:356 h:120 size:17.5 line_height:38 color:4280098079 bg:4294243573
            font_src:"self:resources/Inter-400.ttf"}"#,
        )
        .unwrap();
        let ui = to_makepad_ui(&tree).unwrap();
        // The source pitch separates paragraphs; wrapped lines are capped.
        let spacing = 17.5 * WRAPPED_LINE_PITCH / (17.5 * TEXT_FLOW_LINE_ADVANCE);
        let prose = "text_style_normal: TextStyle{font_family: FontFamily{latin := FontMember{\
            res: crate_resource(\"self:resources/Inter-400.ttf\") asc: 0.04 desc: 0.04 weight: 400}";
        for line in [
            "answer := Markdown {\nwidth: 356 height: 120\n".to_string(),
            "body: \"Run `cargo test` first.\"\nfont_size: 13.125\n".to_string(),
            "paragraph_spacing: 38\n".to_string(),
            prose.to_string(),
            format!("font_size: 13.125 line_spacing: {spacing}}}\n"),
            format!("text_style_fixed: mod.theme.font_code{{font_size: 13.125 line_spacing: {spacing}}}\n"),
            "font_color: #1d1d1fff\n".to_string(),
            "draw_block +: {code_color: #f4f4f5ff}\n".to_string(),
        ] {
            assert!(ui.contains(&line), "{line:?} in {ui}");
        }
    }

    #[test]
    fn right_anchored_nodes_keep_their_gap_to_the_parent_edge() {
        let tree = prepare(
            r#"{t:"stack" id:"composer" w:374 h:120 c:[
            {t:"stack" id:"dock" x:0 y:40 w:374 h:80 c:[
                {t:"stack" id:"send" alignx:1 x:328 y:60 w:36 h:36 bg:4278190080 c:[
                    {t:"svg" id:"arrow" x:338 y:70 w:16 h:16 src:"http://127.0.0.1:8794/a.svg"}
                ]}
            ]}
        ]}"#,
        )
        .unwrap();
        let ui = to_makepad_ui(&tree).unwrap();
        // A full-width wrapper aligns the node right, and its margin keeps the
        // measured 10px gap. Offsets are relative to the parent.
        let send = "View {width: Fill height: 36 margin: Inset{top: 20} align: Align{x: 1.0}\n\
            send := DesignSurface {\nwidth: 36 height: 36\nmargin: Inset{right: 10}\n";
        assert!(ui.contains(send), "{ui}");
        // Its children move with it.
        let arrow = "margin: Inset{left: 10 top: 10 right: 0 bottom: 0}";
        assert!(ui.contains(arrow), "{ui}");
        // Text right-aligns its own run instead.
        let label = prepare(
            r#"{t:"text" text:"9:41" alignx:1 x:300 w:60 h:20 size:14 line_height:20
            font_src:"self:resources/Inter.ttf"}"#,
        )
        .unwrap();
        let ui = to_makepad_ui(&label).unwrap();
        assert!(ui.starts_with("Label {\n"), "{ui}");
        assert!(ui.contains("align: Align{x: 1 y: 0.5}"), "{ui}");
    }

    #[test]
    fn unsized_images_and_empty_font_resources_fail_closed() {
        let src = r#"src:"http://127.0.0.1:8794/photo.png""#;
        let image = prepare(&format!("{{t:\"image\" fillw:1 fillh:1 {src}}}")).unwrap();
        let error = to_makepad_ui(&image).unwrap_err();
        assert_eq!(error, "design image size required");
        let sized = prepare(&format!(
            "{{t:\"image\" fillw:1 fillh:1 image_width:200 image_height:100 {src}}}"
        ))
        .unwrap();
        let ui = to_makepad_ui(&sized).unwrap();
        let dims = "draw_bg.image_dim_w: 200.0 draw_bg.image_dim_h: 100.0";
        assert!(ui.contains(dims), "{ui}");
        let text = prepare(r#"{t:"text" text:"Hi" fillw:1 fith:1 size:14 font_src:""}"#).unwrap();
        let error = to_makepad_ui(&text).unwrap_err();
        assert_eq!(error, "design font resource required");
    }
}
