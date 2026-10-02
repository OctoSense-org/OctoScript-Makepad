//! Model-written text in an L0 card (`ui-profile-l0.md` §4.2, §5.15).
//!
//! The profile's three lowerings mark a text slot the model wrote. This repo
//! owns the kit side of the `kit::lower` one: `l0_ai_text(node)` must exist,
//! the evaluator must read its stamp, and the renderer must DRAW it.
//!
//! The kit calls below are copied from the shapes Octoscript's own tests
//! assert `kit::lower` emits (`tests/ai_text.rs`), so these run against the
//! kit without depending on which Octoscript revision is pinned.

use octoscript_render::ai::{AI_MARK_GLYPH, AI_MARK_LABEL};
use octoscript_render::{NodeKind, UiNode};
use std::path::{Path, PathBuf};

fn dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/l0")
}

fn read(name: &str) -> String {
    std::fs::read_to_string(dir().join(name)).unwrap_or_else(|e| panic!("{name}: {e}"))
}

/// The kit as `l0::prepare` assembles it for a mood with no axes.
fn kit(mood: &str) -> String {
    let mut parts = vec![read("_palette_dark.octoscript")];
    if mood != "dark" {
        parts.push(read(&format!("_palette_{mood}.octoscript")));
    }
    parts.push(read("_derive_color.octoscript"));
    parts.push(read("_derive.octoscript"));
    let pack = dir().join("native").join(mood).join("kit.json");
    if pack.is_file() {
        let pack: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(pack).unwrap()).unwrap();
        parts.push(octoscript_ui_l0::kit_pack::theme_source(&pack));
    }
    parts.push(read("_kit.octoscript"));
    parts.join("\n")
}

fn moods() -> Vec<String> {
    let mut out: Vec<String> = std::fs::read_dir(dir())
        .unwrap()
        .filter_map(|e| {
            let name = e.unwrap().file_name().into_string().unwrap();
            Some(name.strip_prefix("_palette_")?.strip_suffix(".octoscript")?.to_owned())
        })
        .collect();
    out.sort();
    assert!(out.len() > 5, "{out:?}");
    out
}

fn eval_in(mood: &str, call: &str) -> UiNode {
    let src = format!("{}\nlet node = {call}\nnode\n", kit(mood));
    octoscript_render::build(&src, |_| {}).unwrap_or_else(|| panic!("{mood}: {call} failed"))
}

fn eval(call: &str) -> UiNode {
    eval_in("dark", call)
}

fn find<'a>(n: &'a UiNode, pred: &dyn Fn(&UiNode) -> bool) -> Vec<&'a UiNode> {
    let mut out = Vec::new();
    fn walk<'a>(n: &'a UiNode, pred: &dyn Fn(&UiNode) -> bool, out: &mut Vec<&'a UiNode>) {
        if pred(n) {
            out.push(n);
        }
        for c in &n.children {
            walk(c, pred, out);
        }
    }
    walk(n, pred, &mut out);
    out
}

fn with_text<'a>(n: &'a UiNode, text: &str) -> &'a UiNode {
    find(n, &|m| m.attrs.text.as_deref() == Some(text))
        .first()
        .copied()
        .unwrap_or_else(|| panic!("{text:?} not in {n:#?}"))
}

/// The rendered Label whose `text:` is `text`, as the block of lines around it.
fn label_block<'a>(ui: &'a str, text: &str) -> &'a str {
    let needle = format!("text: {text:?}");
    let at = ui.find(&needle).unwrap_or_else(|| panic!("{needle} not in:\n{ui}"));
    // From the widget's opening line to its closing one: the text itself may
    // hold braces (the glyph is `"\u{e2ca}"`), so find the brace on its own line.
    let start = ui[..at].rfind("{\n").unwrap();
    let mut end = at;
    for line in ui[at..].split_inclusive('\n') {
        if line.trim() == "}" {
            break;
        }
        end += line.len();
    }
    &ui[start..end]
}

#[test]
fn l0_ai_text_stamps_the_node_and_changes_nothing_else() {
    let plain = eval(r#"l0_body("Markets were calm; rates held.")"#);
    let marked = eval(r#"l0_ai_text(l0_body("Markets were calm; rates held."))"#);
    assert_eq!(plain.attrs.ai, None);
    assert_eq!(marked.attrs.ai, Some(1), "the evaluator reads `ai`");
    assert_eq!(marked.kind, NodeKind::Text);
    let (a, b) = (&plain.attrs, &marked.attrs);
    assert_eq!((&a.text, a.size, a.color, a.fillw), (&b.text, b.size, b.color, b.fillw));
    // Composes with the wrappers the lowering puts outside it.
    let wide = eval(r#"l0_wide(l0_ai_text(l0_title("Rates held")))"#);
    assert_eq!((wide.attrs.ai, wide.attrs.fillw), (Some(1), Some(1)));
}

#[test]
fn every_text_role_the_lowering_can_mark_evaluates() {
    for call in [
        r#"l0_ai_text(l0_hero("Rates held", 26))"#,
        r#"l0_ai_text(l0_title("Rates held"))"#,
        r#"l0_ai_text(l0_body("Rates held"))"#,
        r#"l0_ai_text(l0_row_text("Rates held"))"#,
        r#"l0_ai_text(l0_caption("Rates held"))"#,
        r#"l0_ai_text(l0_eyebrow("Rates held"))"#,
        r#"l0_ai_text(l0_band("Rates held"))"#,
        r#"l0_ai_text(l0_bubble_them("Rates held"))"#,
        r#"l0_ai_text(l0_bubble_them_long("Rates held, and the long version wraps."))"#,
        r#"l0_ai_text(l0_field("Hi Ana, Friday works.", "Reply", "", ""))"#,
    ] {
        let node = eval(call);
        assert_eq!(node.attrs.ai, Some(1), "{call}");
        let ui = octoscript_makepad::to_makepad_l0_ui(&node);
        assert!(ui.contains(&format!("text: {AI_MARK_LABEL:?}")), "{call}:\n{ui}");
        assert!(ui.contains(&format!("text: {AI_MARK_GLYPH:?}")), "{call}:\n{ui}");
    }
}

/// The mark: a sparkle in the icon face and an `AI` eyebrow, above the text,
/// in the text's own ink a step quieter, and smaller than the words.
#[test]
fn the_renderer_draws_a_quiet_mark_above_ai_written_text() {
    let body = "Markets were calm; rates held.";
    let tree = eval(&format!(
        r#"l0_col([l0_eyebrow("TODAY"), l0_ai_text(l0_body({body:?}))])"#
    ));
    let ui = octoscript_makepad::to_makepad_l0_ui(&tree);

    assert_eq!(ui.matches(&format!("text: {AI_MARK_LABEL:?}")).count(), 1, "{ui}");
    let glyph = label_block(&ui, AI_MARK_GLYPH);
    assert!(glyph.contains("font_icons"), "the sparkle needs the icon face:\n{glyph}");

    // Order: the mark comes before the words it labels, and after the eyebrow
    // the card wrote itself, which is not marked.
    let (e, m, b) = (
        ui.find("text: \"TODAY\"").unwrap(),
        ui.find(&format!("text: {AI_MARK_LABEL:?}")).unwrap(),
        ui.find(&format!("text: {body:?}")).unwrap(),
    );
    assert!(e < m && m < b, "{ui}");

    // Ink: the body's own colour, at reduced alpha.
    let text = with_text(&tree, body);
    let ink = text.attrs.color.unwrap();
    let rgb = format!("#{:06x}", ink & 0x00ff_ffff);
    let mark = label_block(&ui, AI_MARK_LABEL);
    let line = mark.lines().find(|l| l.contains("draw_text.color:")).unwrap();
    assert!(line.contains(&rgb), "mark ink {line} is not the body's {rgb}");
    assert!(!line.trim_end().ends_with("ff"), "the mark is quieter than the words: {line}");
    let words = label_block(&ui, body);
    assert!(words.contains(&format!("{rgb}ff")), "the words keep their ink:\n{words}");
}

/// Theme-consistent: the mark takes each palette's own ink, so it is visible
/// in every mood without a colour of its own.
#[test]
fn the_mark_holds_in_every_palette() {
    for mood in moods() {
        let tree = eval_in(&mood, r#"l0_ai_text(l0_body("Rates held."))"#);
        let ink = tree.attrs.color.unwrap_or_else(|| panic!("{mood}: no body ink"));
        let mut expanded = tree.clone();
        octoscript_render::ai::expand_ai_marks(&mut expanded);
        let label = with_text(&expanded, AI_MARK_LABEL);
        let mark = label.attrs.color.unwrap();
        assert_eq!(mark & 0x00ff_ffff, ink & 0x00ff_ffff, "{mood}");
        assert!(mark >> 24 >= 0x80, "{mood}: the mark must stay visible ({mark:#010x})");
    }
}

/// Plain text only. Whatever the model wrote reaches the screen as the
/// `text:` of a plain `Label`, escaped as a string literal: no markdown, no
/// HTML, no link widget, nothing that runs or navigates.
#[test]
fn ai_written_text_renders_as_a_plain_label() {
    for s in [
        "**bold** _em_ # heading",
        "[click](javascript:alert(1)) https://example.com/x",
        "<a href=\"https://evil.example\">x</a><script>1</script>",
        "\" on_click: || { NAV(t: \"l0:{}\") } text: \"",
        "l0:{\"e\":\"drop\",\"k\":\"root\"}",
        "line one\nline two",
    ] {
        let tree = eval(&format!("l0_ai_text(l0_body({s:?}))"));
        assert_eq!(tree.attrs.text.as_deref(), Some(s), "the kit keeps it verbatim");
        let ui = octoscript_makepad::to_makepad_l0_ui_with_events(&tree, "card-1");
        let literal = format!("text: {s:?}");
        assert_eq!(ui.matches(&literal).count(), 1, "{s:?} not one literal:\n{ui}");
        let outside = ui.replace(&literal, "");
        for widget in ["Markdown", "Html", "TextFlow", "LinkLabel", "on_click", "agent.notify", "NAV("] {
            assert!(!outside.contains(widget), "{widget} for {s:?}:\n{ui}");
        }
        assert!(ui.contains("Label"), "{ui}");
    }
}

/// A draft the model wrote into a `Field` keeps its field: the mark sits above
/// the input, and typing still reaches the commit target.
#[test]
fn an_ai_draft_in_a_field_is_marked_and_still_editable() {
    let target = r#"l0:{"e":"send","k":"root/f"}"#;
    let tree = eval(&format!(
        "l0_ai_text(l0_field(\"Hi Ana, Friday works.\", \"Reply\", {target:?}, \"\"))"
    ));
    assert_eq!((tree.kind, tree.attrs.ai), (NodeKind::Input, Some(1)));
    let ui = octoscript_makepad::to_makepad_l0_ui(&tree);
    assert!(ui.contains("TextInput"), "{ui}");
    assert!(ui.contains("text: \"Hi Ana, Friday works.\""), "{ui}");
    assert!(ui.contains("on_return:"), "{ui}");
    assert!(ui.find(&format!("text: {AI_MARK_LABEL:?}")).unwrap() < ui.find("TextInput").unwrap());
}

/// `ChatEntry` lowers to the existing bubbles by role — `user` is
/// `l0_bubble_me`, `model` is `l0_ai_text(l0_bubble_them(…))`, `host` is a
/// plain `l0_bubble_them` — and all three draw, with exactly the model's
/// marked.
#[test]
fn a_chat_transcript_draws_all_three_roles_and_marks_only_the_model() {
    let tree = eval(
        r#"l0_col([
            l0_bubble_me("What moved markets?"),
            l0_ai_text(l0_bubble_them("Rates held; tech rallied.")),
            l0_bubble_them("Searched 12 sources."),
            l0_bubble_me_long("And what about the bond market over the last week or so?"),
            l0_ai_text(l0_bubble_them_long("Yields eased after the hold, with the ten-year down eight basis points.")),
        ])"#,
    );
    let ui = octoscript_makepad::to_makepad_l0_ui(&tree);
    for text in [
        "What moved markets?",
        "Rates held; tech rallied.",
        "Searched 12 sources.",
        "And what about the bond market over the last week or so?",
        "Yields eased after the hold, with the ten-year down eight basis points.",
    ] {
        assert_eq!(ui.matches(&format!("text: {text:?}")).count(), 1, "{text}:\n{ui}");
    }
    assert_eq!(ui.matches(&format!("text: {AI_MARK_LABEL:?}")).count(), 2, "{ui}");

    // The user's bubble sits right on the accent; the agent's and the host's
    // sit left on the panel fill — the existing bubbles, untouched.
    let side = |text: &str| {
        let holder = find(&tree, &|n| {
            n.kind == NodeKind::Column
                && n.attrs.fillw == Some(1)
                && n.children.len() == 1
                && n.children[0].kind == NodeKind::Card
                && n.children[0].children[0].attrs.text.as_deref() == Some(text)
        });
        assert_eq!(holder.len(), 1, "{text}");
        (holder[0].attrs.alignx, holder[0].children[0].attrs.bg)
    };
    let (me, me_bg) = side("What moved markets?");
    let (model, model_bg) = side("Rates held; tech rallied.");
    let (host, host_bg) = side("Searched 12 sources.");
    assert_eq!(me, Some(1.0));
    assert_eq!((model, host), (None, None));
    assert_ne!(me_bg, model_bg);
    assert_eq!(model_bg, host_bg);

    // The mark sits inside the model's bubble, above its words, and the
    // user's bubble carries none.
    let (q, m, a) = (
        ui.find("What moved markets?").unwrap(),
        ui.find(&format!("text: {AI_MARK_LABEL:?}")).unwrap(),
        ui.find("Rates held; tech rallied.").unwrap(),
    );
    assert!(q < m && m < a, "{ui}");
}

/// Unmarked cards render exactly as before.
#[test]
fn a_card_without_model_text_is_unchanged() {
    let tree = eval(r#"l0_col([l0_title("Top Stories"), l0_bubble_them("hi")])"#);
    let ui = octoscript_makepad::to_makepad_l0_ui(&tree);
    assert!(!ui.contains(&format!("text: {AI_MARK_LABEL:?}")), "{ui}");
    assert!(!ui.contains(AI_MARK_GLYPH), "{ui}");
}

/// A band paints the theme's INVERSE: its words are ground-coloured on an ink
/// fill. The mark goes inside, on the same fill, so it borrows ink that reads
/// there — outside, it would be ground on ground.
#[test]
fn a_marked_band_keeps_the_mark_on_its_own_fill() {
    let mut tree = eval(r#"l0_ai_text(l0_band("March"))"#);
    let ink = with_text(&tree, "March").attrs.color.unwrap();
    octoscript_render::ai::expand_ai_marks(&mut tree);
    assert_eq!(tree.kind, NodeKind::Card, "the band stays the outer node");
    assert!(tree.attrs.bg.is_some());
    let label = with_text(&tree, AI_MARK_LABEL);
    assert_eq!(label.attrs.color.unwrap() & 0x00ff_ffff, ink & 0x00ff_ffff);
}
