//! Native resource/font-engine regression, without taking over a display.
//! HTTP completion is supplied deterministically; a live asset-server/GUI run
//! remains a separate host acceptance check.
use makepad_widgets::makepad_draw::text::fonts::Fonts;
use makepad_widgets::*;
use std::{cell::RefCell, rc::Rc};

#[test]
fn a_served_bundle_font_loads_after_http_and_chinese_uses_the_declared_fallback() {
    // This test has its own process. Do not let the developer machine's fonts
    // hide a missing declared CJK face in a package.
    std::env::set_var("MAKEPAD_SYSTEM_FONTS", "0");
    let mut cx = Cx::new(Box::new(|_, _| {}));
    cx.set_font_set(makepad_platform::FontSet::International);
    let url = "http://127.0.0.1:12345/assets/Body.ttf";
    let source = format!(
        r#"{{t:"text" id:"sample" text:"Hello 中文" w:240 h:80 size:18 line_height:28 font_src:{url:?}}}"#
    );
    let tree = octoscript_makepad::design::prepare(&source).unwrap();
    let native = octoscript_makepad::design::to_makepad_ui(&tree).unwrap();
    let view = cx.with_vm(|vm| {
        makepad_widgets::script_mod(vm);
        let value = vm
            .eval_checked(
                ScriptMod {
                    cargo_manifest_path: env!("CARGO_MANIFEST_DIR").into(),
                    module_path: module_path!().into(),
                    file: file!().into(),
                    line: 1,
                    column: 0,
                    code: format!(
                        "use mod.prelude.widgets.*\nreturn View{{width:Fill height:Fill {native}}}"
                    ),
                    values: vec![],
                },
                2_000_000,
            )
            .expect("generated font source must evaluate");
        View::script_from_value(vm, value)
    });
    let label = view.label(&cx, ids!(sample));
    let label = label.borrow().expect("native Label exists");
    let text = &label.draw_text;
    assert!(text
        .text_style
        .font_family
        .member_ids()
        .any(|id| id == "cjk"));
    let request = cx
        .script_data
        .resources
        .http_resources
        .iter()
        .find(|request| request.abs_path == url)
        .expect("bundle font uses HTTP resource loader")
        .request_id;
    text.text_style.ensure_fonts_loaded(&mut cx);
    let fonts = cx.get_global::<Rc<RefCell<Fonts>>>().clone();
    let before = fonts
        .borrow_mut()
        .get_or_load_font_family(text.text_style.font_family_id());
    let previous_primary = before.fonts().first().map(|font| font.id());
    assert!(cx.get_resource_font_bytes_by_path(url).is_none());
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../makepad/widgets/resources/Roboto-Regular.ttf");
    let bytes = std::fs::read(path).expect("prepared pinned Makepad font asset");
    assert!(cx
        .script_data
        .resources
        .handle_http_response(request, bytes));
    assert!(cx.get_resource_font_bytes_by_path(url).is_some());
    text.text_style.ensure_fonts_loaded(&mut cx);
    let after = fonts
        .borrow_mut()
        .get_or_load_font_family(text.text_style.font_family_id());
    assert_ne!(
        after.fonts().first().map(|font| font.id()),
        previous_primary,
        "the asynchronously loaded bundled face becomes the primary font"
    );
    // First layout requests the lazy CJK face; the second consumes it.
    text.layout(
        &mut cx,
        0.,
        0.,
        Some(240.),
        true,
        Align::default(),
        "Hello 中文",
    );
    let laidout = text.layout(
        &mut cx,
        0.,
        0.,
        Some(240.),
        true,
        Align::default(),
        "Hello 中文",
    );
    let glyphs: Vec<_> = laidout
        .rows
        .iter()
        .flat_map(|row| row.glyphs.iter())
        .collect();
    assert!(!glyphs.is_empty());
    assert!(
        glyphs.iter().all(|glyph| glyph.id != 0),
        "no tofu after CJK fallback loads"
    );
}
