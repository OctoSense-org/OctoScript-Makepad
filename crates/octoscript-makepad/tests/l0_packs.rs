use std::path::Path;

fn directory() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/l0")
}

#[test]
fn standard_l0_roles_receive_the_imported_source_fonts_and_size() {
    for (theme, family) in [
        ("camo", "DMSans-Regular"),
        ("camo_light", "DMSans-Regular"),
        ("atro", "Montserrat-Regular"),
        ("atro_light", "Montserrat-Regular"),
        ("taskplan_light", "PlusJakartaSans"),
    ] {
        let source =
            format!("theme {theme}\nview root Surface {{ TextBody(text: \"Source type\") }}");
        let prepared =
            octoscript_makepad::l0::prepare(&source, &serde_json::json!({}), &directory()).unwrap();
        fn text(n: &octoscript_render::UiNode) -> Option<&octoscript_render::UiNode> {
            if n.attrs.text.as_deref() == Some("Source type") {
                return Some(n);
            }
            n.children.iter().find_map(text)
        }
        let node = text(&prepared.tree).unwrap();
        assert!(
            node.attrs
                .font_src
                .as_deref()
                .unwrap_or("")
                .contains(family),
            "{theme}: {:?}",
            node.attrs.font_src
        );
        assert!(
            node.attrs.size.unwrap() > 8.0,
            "source type scale was not applied"
        );
        assert!(octoscript_makepad::to_makepad_l0_ui(&prepared.tree).contains(family));
    }
}

#[test]
fn theme_modes_answer_the_same_registered_component_names() {
    for family in ["atro", "camo"] {
        let read = |theme: &str| -> serde_json::Value {
            serde_json::from_str(
                &std::fs::read_to_string(directory().join("native").join(theme).join("kit.json"))
                    .unwrap(),
            )
            .unwrap()
        };
        let dark = read(family);
        let light = read(&format!("{family}_light"));
        let a = dark["components"].as_object().unwrap();
        let b = light["components"].as_object().unwrap();
        assert_eq!(a.keys().collect::<Vec<_>>(), b.keys().collect::<Vec<_>>());
        assert_ne!(
            dark["tokens"]["color.content.primary"]["value"],
            light["tokens"]["color.content.primary"]["value"]
        );
        assert!(dark["compounds"].as_object().unwrap().len() > 50);
    }
}

#[test]
fn l0_event_target_routes_to_a_host_owned_channel() {
    let source = "state selected { shape: enum[off, on], initial: .off }\n\
                  event flip { selected: cycle(.off, .on) }\n\
                  view root Row(on_tap: flip) { TextBody(text: selected) }";
    let prepared = octoscript_makepad::l0::prepare(source, &serde_json::json!({}), &directory()).unwrap();
    let ui = octoscript_makepad::to_makepad_l0_ui_with_events(&prepared.tree, "card-runtime-42");
    assert!(ui.contains("OctoscriptTap {"));
    assert!(ui.contains("agent.notify(\"card-runtime-42\""));
    assert!(ui.contains("l0:{"));
    assert!(!ui.contains("NAV("));
}
