//! Source-measured Splash designs. Geometry is in logical pixels, text sizes
//! in CSS/Sketch pixels (Makepad's text API uses points, hence 72/96).
//! This path preserves explicit design styles; it does not apply a theme.
use octoscript_render::{NodeKind, UiNode};
use std::fmt::Write;

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

pub fn to_makepad_ui(tree: &UiNode) -> Result<String, String> {
    fn emit(n: &UiNode, out: &mut String, flow_origin: Option<(f64, f64)>) -> Result<(), String> {
        let a = &n.attrs;
        let scroll_y = n.kind == NodeKind::Stack && a.variant.as_deref() == Some("scroll_y");
        // Leaf widgets may reuse their Walk for internal text layout. Keep the
        // positioning margin on a wrapper so it cannot be applied twice.
        let wrapped = flow_origin.is_some() && n.kind != NodeKind::Stack;
        if wrapped {
            let (x, y) = flow_origin.unwrap();
            writeln!(out, "View {{width: {} height: {} margin: Inset{{left: {} top: {} right: 0 bottom: 0}} flow: Overlay padding: 0 clip_x: false clip_y: false",
                a.w.ok_or("design width required")?, a.h.ok_or("design height required")?,
                a.x.unwrap_or(0.) - x, a.y.unwrap_or(0.) - y).unwrap();
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
        writeln!(
            out,
            "width: {} height: {}",
            a.w.ok_or("design width required")?,
            a.h.ok_or("design height required")?
        )
        .unwrap();
        // Source frames stay window-local. Inside a scroll viewport, convert
        // them to parent-relative overlay margins so native layout applies the
        // scroll offset and measures the complete content extent.
        if wrapped {
            writeln!(out, "margin: 0").unwrap();
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
                let clip=scroll_y || a.variant.as_deref()==Some("clip");
                writeln!(out, "flow: Overlay padding: 0 clip_x: {clip} clip_y: {clip}").unwrap();
                if let Some(bg) = a.bg {
                    writeln!(out, "show_bg: true draw_bg.color: {}", super::hex_rgba(bg)).unwrap();
                }
                if let Some(selected) = a.selected {
                    writeln!(out, "selected: {}", selected != 0).unwrap();
                }
                for c in &n.children {
                    let origin = if scroll_y || flow_origin.is_some() {
                        Some((a.x.unwrap_or(0.), a.y.unwrap_or(0.)))
                    } else { None };
                    emit(c, out, origin)?;
                }
            }
            NodeKind::Text | NodeKind::Input => {
                let size = a.size.ok_or("design font size required")?;
                let font = a.font_src.as_ref().ok_or("design font resource required")?;
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
                } else if a.variant.as_deref()==Some("single_line") || a.h.unwrap_or(0.) <= a.line_height.unwrap_or(size * 1.3) + 0.5 {
                    // A single Sketch line must not wrap a whole word because
                    // native font advances differ by a fraction of a point.
                    writeln!(out, "flow: Right").unwrap();
                }
                // Use real font metrics and a measured line height. The
                // bundled families' natural line box is supplied by importer.
                let line_box = if font.contains("PlusJakarta") {
                    1.26
                } else if font.contains("Poppins") {
                    1.5
                } else {
                    2478. / 2048.
                };
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
                let emoji = if cfg!(target_os = "macos") {
                    "file_resource(\"/System/Library/Fonts/Apple Color Emoji.ttc\")"
                } else {
                    "crate_resource(\"makepad_widgets:resources/NotoColorEmoji.ttf\")"
                };
                let asc=a.font_asc.unwrap_or(shift);
                let desc=a.font_desc.unwrap_or(shift);
                let resource=if let Some(path)=font.strip_prefix("file:") {
                    if !std::path::Path::new(path).is_absolute() {return Err("platform font path must be absolute".into());}
                    format!("file_resource({path:?})")
                } else {format!("crate_resource({font:?})")};
                writeln!(out, "draw_text.text_style: TextStyle{{font_family: FontFamily{{latin := FontMember{{res: {resource} asc: {asc} desc: {desc} weight: {weight}}} symbols := FontMember{{res: crate_resource(\"makepad_widgets:resources/jetbrains_mono_variable.ttf\") asc: 0 desc: 0 weight: 400}} emoji := FontMember{{res: {emoji} asc: 0 desc: 0}}}} font_size: {} line_spacing: {spacing} letter_spacing: {}}}", size * 0.75,a.tracking.unwrap_or(0.)).unwrap();
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
                writeln!(
                    out,
                    "draw_bg.image_dim_w: {:.1} draw_bg.image_dim_h: {:.1}",
                    a.image_width.unwrap_or(a.w.unwrap() * 2.),
                    a.image_height.unwrap_or(a.h.unwrap() * 2.)
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
    let mut out = String::new();
    emit(tree, &mut out, None)?;
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
}
