//! The L0 theme kit is a contract with `splash-render`, and this is where that
//! consumer lives — so the contract is CHECKED here rather than asserted
//! elsewhere.
//!
//! Two earlier attempts to lower L0 at this renderer were reverted for exactly
//! that: they mapped roles to tags nobody had run, and five of twenty-three did
//! not exist. The repository that owns the profile has no dependency on this
//! one, so nothing there can catch a drift. These tests can.

/// The kit body, which defines no colours — a palette is concatenated before it.
const KIT_BODY: &str = include_str!("../../../components/l0/_kit.splash");

/// The base: every knob and every colour.
const BASE: &str = include_str!("../../../components/l0/_palette_dark.splash");
/// Every size, computed from the knobs — AFTER the mood's delta.
const DERIVE: &str = include_str!("../../../components/l0/_derive.splash");

/// Each mood's delta. `dark` is the base itself, so its delta is empty.
const DELTAS: &[(&str, &str)] = &[
    ("dark", ""),
    ("light", include_str!("../../../components/l0/_palette_light.splash")),
    ("glass", include_str!("../../../components/l0/_palette_glass.splash")),
    ("photo", include_str!("../../../components/l0/_palette_photo.splash")),
];

/// The kit as a host assembles it: base, delta, derive, body — in that order.
fn kit_with(theme: &str) -> String {
    let (_, delta) = DELTAS
        .iter()
        .find(|(n, _)| *n == theme)
        .unwrap_or_else(|| panic!("no palette for theme {theme:?}"));
    format!("{BASE}\n{delta}\n{DERIVE}\n{KIT_BODY}")
}

/// The default assembly, for the role tests below.
fn kit() -> String {
    kit_with("dark")
}

/// Every role, and the call that exercises it.
///
/// Kept as data rather than as one big card so a role that evaluates to nil is
/// NAMED. Inside a larger tree an empty node is just an absence, which is the
/// failure mode `ui-profile-l0.md` §1.1 exists to prevent.
const ROLES: &[(&str, &str)] = &[
    ("l0_hero", r#"l0_hero("$184.20", 40)"#),
    ("l0_title", r#"l0_title("Top Stories")"#),
    ("l0_body", r#"l0_body("Rust 1.95 lands")"#),
    ("l0_row_text", r#"l0_row_text("NVDA")"#),
    ("l0_caption", r#"l0_caption("HACKER NEWS")"#),
    ("l0_value", r#"l0_value("41.2M")"#),
    ("l0_stat", r#"l0_stat("+1.7%", 1)"#),
    ("l0_surface", r#"l0_surface([l0_title("t")])"#),
    ("l0_surface_photo", r#"l0_surface_photo("https://x/y.jpg", [l0_title("t")])"#),
    ("l0_col", r#"l0_col([l0_title("t")])"#),
    ("l0_col_gap", r#"l0_col_gap(8, [l0_title("t")])"#),
    ("l0_row", r#"l0_row([l0_title("t")])"#),
    ("l0_row_gap", r#"l0_row_gap(6, [l0_title("t")])"#),
    ("l0_grid", r#"l0_grid([l0_title("t")])"#),
    ("l0_panel", r#"l0_panel([l0_title("t")])"#),
    ("l0_rule", r#"l0_rule()"#),
    ("l0_tile", r#"l0_tile("Open", "$181.00")"#),
    ("l0_chip", r#"l0_chip("1M", 1)"#),
    ("l0_photo", r#"l0_photo("https://x/y.jpg")"#),
    ("l0_weathericon", r#"l0_weathericon("clear")"#),
    ("l0_unsupported", r#"l0_unsupported("TempBar")"#),
];

/// A script's value is a bare VARIABLE, never a call.
///
/// `fn f() { … }` followed by `f()` evaluates to nil — which looks exactly like
/// a broken kit and is not. Every kit in this repository ends `let node = …`
/// then `node`, and getting this wrong made all 21 roles report as dead on the
/// first run.
fn eval(call: &str) -> Option<splash_render::UiNode> {
    let src = format!("{}\nlet node = {call}\nnode\n", kit());
    splash_render::build(&src, |_vm| {})
}

#[test]
fn every_role_produces_a_node() {
    let dead: Vec<&str> = ROLES
        .iter()
        .filter(|(_, call)| eval(call).is_none())
        .map(|(name, _)| *name)
        .collect();
    assert!(
        dead.is_empty(),
        "these roles evaluated to nothing: {dead:?}\n\
         A role that produces nil renders as an absence — the card looks complete \
         and is not (profile §1.1)."
    );
}

/// A role must land on the kind its meaning implies, not merely on *a* kind.
///
/// `Panel` becoming a `Text` would still produce a tree and still pass the test
/// above, and the card would be unreadable.
#[test]
fn each_role_lands_on_the_kind_its_meaning_implies() {
    for (call, expected) in [
        (r#"l0_title("t")"#, "Text"),
        (r#"l0_panel([l0_title("t")])"#, "Card"),
        (r#"l0_row([l0_title("t")])"#, "Row"),
        (r#"l0_col([l0_title("t")])"#, "Column"),
        (r#"l0_grid([l0_title("t")])"#, "Grid"),
        (r#"l0_rule()"#, "Divider"),
        (r#"l0_chip("1M", 1)"#, "Chip"),
        (r#"l0_photo("https://x/y.jpg")"#, "Image"),
        (r#"l0_weathericon("clear")"#, "WeatherIcon"),
        (r#"l0_tile("Open", "$1")"#, "Card"),
        (r#"l0_surface([l0_title("t")])"#, "Column"),
        (r#"l0_surface_photo("u", [l0_title("t")])"#, "Stack"),
    ] {
        let tree = eval(call).unwrap_or_else(|| panic!("{call} evaluated to nil"));
        assert_eq!(
            format!("{:?}", tree.kind),
            expected,
            "{call} landed on the wrong kind"
        );
    }
}

/// A chip must LOOK different when selected.
///
/// Every chip in the stock card's range picker rendered identically once, so the
/// card could not show which range was chosen — and the golden recorded that as
/// correct. Presentation is the theme's business, but "this one is selected" is
/// meaning, and it has to survive.
#[test]
fn a_selected_chip_differs_from_an_unselected_one() {
    let on = eval(r#"l0_chip("1M", 1)"#).expect("on");
    let off = eval(r#"l0_chip("1M", 0)"#).expect("off");

    // `bg`, specifically — NOT `selected`.
    //
    // The first version of this test compared `(bg, selected)`, and a mutation
    // that made the fill constant still passed: `selected` alone differed. But
    // an attribute existing in `Attrs` is not evidence that a backend draws it,
    // and this one could not be shown to reach the chip's rendering. Asserting
    // it would have repeated the original defect exactly — every chip drawn
    // identically, with a green test.
    assert_ne!(
        on.attrs.bg, off.attrs.bg,
        "a selected chip must differ in its FILL, which is drawn"
    );
}

/// A statistic's tint carries direction, not decoration.
///
/// Red-versus-green is presentation; "this value fell" is meaning. A lowering
/// that dropped the attribute lost both, which is why the profile calls `tint`
/// the instructive case.
#[test]
fn a_stat_is_tinted_by_direction() {
    let up = eval(r#"l0_stat("+1.7%", 1)"#).expect("up");
    let flat = eval(r#"l0_stat("0.0%", 0)"#).expect("flat");
    let down = eval(r#"l0_stat("-1.7%", -1)"#).expect("down");
    assert_ne!(up.attrs.color, down.attrs.color, "a rise and a fall must differ");
    assert_ne!(up.attrs.color, flat.attrs.color, "a rise and no change must differ");
    assert_ne!(down.attrs.color, flat.attrs.color, "a fall and no change must differ");
}

/// A role this renderer cannot draw renders a VISIBLE marker.
///
/// Five of the six data visualisations have no kind here. Returning nothing
/// would let a card lose its temperature bars and still look complete.
#[test]
fn an_unsupported_role_is_visible_rather_than_absent() {
    let tree = eval(r#"l0_unsupported("TempBar")"#).expect("nil");
    let mut text = String::new();
    fn walk(n: &splash_render::UiNode, out: &mut String) {
        if let Some(t) = n.attrs.text.as_deref() {
            out.push_str(t);
        }
        for c in &n.children {
            walk(c, out);
        }
    }
    walk(&tree, &mut text);
    assert!(
        text.contains("TempBar"),
        "the marker must NAME the role it stands in for, got {text:?}"
    );
}

/// Every palette defines every name, and they are the names the kit reads.
///
/// A missing entry is NOT a default. An undefined name coerces to 0 — fully
/// transparent — so the role that reads it renders as nothing and the card still
/// looks complete. That is the §1.1 failure this whole file exists to catch, and
/// with four palettes it is now four times as easy to make: adding a colour to
/// the kit means adding it to all four.
#[test]
fn l0_palettes_agree() {
    fn names(src: &str) -> std::collections::BTreeSet<String> {
        src.lines()
            .filter_map(|l| l.trim().strip_prefix("let "))
            .filter_map(|l| l.split_whitespace().next())
            .map(str::to_owned)
            .collect()
    }
    let defined: std::collections::BTreeSet<String> =
        names(BASE).union(&names(DERIVE)).cloned().collect();
    assert!(
        defined.len() >= 30,
        "base + derive looks empty: {defined:?}"
    );

    // A DELTA may only restate a token the base or the derivation defines. A
    // typo — `l0_dimm` — would otherwise bind a name nothing reads and change
    // nothing, silently, which is the whole failure mode this file exists for.
    for (theme, delta) in DELTAS {
        for name in names(delta) {
            assert!(
                defined.contains(&name),
                "delta {theme:?} defines {name:?}, which is not a token — typo?"
            );
        }
    }

    // And the kit must not read an `l0_*` colour nothing defines.
    for line in KIT_BODY.lines() {
        for tok in line.split(|c: char| !(c.is_alphanumeric() || c == '_')) {
            if let Some(rest) = tok.strip_prefix("l0_") {
                let name = format!("l0_{rest}");
                if defined.contains(&name) {
                    continue;
                }
                assert!(
                    KIT_BODY.contains(&format!("fn {name}(")),
                    "the kit reads {name:?}, which is neither a role nor a token"
                );
            }
        }
    }
}

/// Every mood assembles into a kit that still evaluates, and a knob a delta
/// moves actually reaches the roles.
///
/// The ordering this depends on is invisible: `let` evaluates at its own line, so
/// deriving sizes in the BASE would leave a delta's `radius_factor` with nothing
/// to change and every mood would silently keep the base's corners.
#[test]
fn a_delta_can_move_a_knob() {
    for (theme, _) in DELTAS {
        let src = format!(
            "{}\nlet node = l0_panel([l0_title(\"t\")])\nnode\n",
            kit_with(theme)
        );
        let tree = splash_render::build(&src, |_vm| {})
            .unwrap_or_else(|| panic!("mood {theme:?} does not evaluate"));
        assert!(tree.attrs.bg.is_some(), "mood {theme:?} lost its panel fill");
    }
    // `glass` sets radius_factor 1.5, so its panel corner must differ from dark's.
    let radius_of = |theme: &str| {
        let src = format!(
            "{}\nlet node = l0_panel([l0_title(\"t\")])\nnode\n",
            kit_with(theme)
        );
        splash_render::build(&src, |_vm| {}).expect("evaluates").attrs.radius
    };
    assert_ne!(
        radius_of("glass"),
        radius_of("dark"),
        "glass moves radius_factor and the derivation must follow it"
    );
}

/// The derived tokens equal the numbers the kit used to hardcode.
///
/// This is what "the layer moved, the look did not" means, asserted rather than
/// eyeballed — comparing screenshots cannot do it, because two runs of a card are
/// two different generations. Four device goldens assert these numbers; if a knob
/// default or a multiplier drifts, every card silently re-lays-out and only this
/// notices.
#[test]
fn the_default_tokens_are_the_numbers_the_kit_used_to_hardcode() {
    // `let node = {t: "card", radius: <token>}` is the cheapest way to read a
    // token's VALUE back out through the same evaluator the kit uses.
    fn token(name: &str) -> f32 {
        let src = format!(
            "{}\nlet node = {{t: \"card\", radius: {name}}}\nnode\n",
            kit_with("dark")
        );
        splash_render::build(&src, |_vm| {})
            .unwrap_or_else(|| panic!("{name} does not evaluate"))
            .attrs
            .radius
            .unwrap_or_else(|| panic!("{name} is undefined — which reads as 0"))
    }
    for (name, want) in [
        ("pad_page_x", 20.0),
        ("pad_page_top", 54.0),
        ("pad_page_bot", 24.0),
        ("pad_panel_x", 14.0),
        ("pad_panel_y", 12.0),
        ("pad_tile_x", 12.0),
        ("pad_tile_y", 10.0),
        ("pad_chip_x", 10.0),
        ("pad_chip_y", 5.0),
        ("gap_panel_top", 16.0),
        ("gap_row", 6.0),
        ("gap_tile", 4.0),
        ("radius_panel", 14.0),
        ("radius_tile", 12.0),
        ("radius_chip", 12.0),
        ("font_caption", 8.0),
        ("font_row", 10.0),
        ("font_body", 12.0),
        ("font_value", 14.0),
        ("font_title", 18.0),
    ] {
        assert_eq!(token(name), want, "{name} drifted from the shipped value");
    }
}
