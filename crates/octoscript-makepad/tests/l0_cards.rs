//! The whole L0 path, end to end, in the repository where its consumer lives.
//!
//! ```text
//!   L0 ledger  →  realize  →  kit::lower  →  _kit.octoscript  →  octoscript_render  →  UiNode
//! ```
//!
//! `octoscript-ui-l0` cannot check this itself: it has no dependency on
//! `octoscript-render` and must not acquire one, since that would pull a second
//! makepad lineage into its lockfile. So the profile's repository can only
//! assert the contract, and twice it asserted one that was wrong — five of
//! twenty-three roles mapped to tags that did not exist, and the attempt was
//! reverted.
//!
//! Here the contract is executed. `octoscript-ui-l0` is a dev-dependency, which is
//! possible only because it was extracted from `octoscript-core` and depends on
//! `serde_json` alone.

use octoscript_render::makepad_script::*;
use octoscript_ui_l0::{kit, realize, RealizeLimits};

/// The kit body plus the default palette. A theme is a palette prefix (see
/// `_palette_dark.octoscript`); these tests assert structure, not colour, so they
/// assemble the default and `l0_kit.rs` covers the moods.
const KIT_BODY: &str = include_str!("../../../components/l0/_kit.octoscript");
const BASE: &str = include_str!("../../../components/l0/_palette_dark.octoscript");
const DERIVE: &str = include_str!("../../../components/l0/_derive.octoscript");

#[test]
fn inspection_ids_and_selected_state_survive_native_translation() {
    let mut tree = build("theme dark\nview root Surface { Chip(text: \"Selected\", active: .on) }", serde_json::json!({}));
    let mut again = tree.clone();
    let first = octoscript_makepad::l0::inspectable(&mut tree);
    assert_eq!(first, octoscript_makepad::l0::inspectable(&mut again));
    assert_eq!(first.len(), tree.count());
    let ui = octoscript_makepad::to_makepad_l0_ui(&tree);
    assert!(ui.contains("selected: true"), "{ui}");
    for node in first {
        assert!(ui.contains(&format!("{} :=", node["id"].as_str().unwrap())));
    }
}

/// Base, then the derivation, then the body — the default mood's assembly. Omit
/// `_derive.octoscript` and every SIZE is an undefined name, which is 0: the tree
/// still builds and every padding, radius and font size is gone.
fn kit() -> String {
    format!("{BASE}\n{DERIVE}\n{KIT_BODY}")
}

const WEATHER: &str =
    include_str!("../../../../octoscript/crates/octoscript-ui-l0/tests/fixtures/weather.card");
const NEWS: &str = include_str!("../../../../octoscript/crates/octoscript-ui-l0/tests/fixtures/news.card");
const STOCK: &str =
    include_str!("../../../../octoscript/crates/octoscript-ui-l0/tests/fixtures/stock.card");

fn register_missing_capabilities(vm: &mut ScriptVm) {
    let sys = vm.new_module(id!(sys));
    // Deterministic missing-data adapters. Live calls remain in kit lowering,
    // so structural tests must register their capabilities even with seed data.
    for name in [
        "news",
        "stock",
        "movers",
        "geocode",
        "geocodenum",
        "weather",
        "weathercond",
        "photo",
        "daylight",
        "dayname",
        "moonphase",
        "weekmin",
        "weekmax",
        "gps",
        "route",
        "nav",
        "cities",
        "airquality",
        "aqinum",
        "satellite",
    ] {
        vm.add_method(
            sys,
            LiveId::from_str(name),
            script_args_def!(a = NIL, b = NIL, c = NIL, d = NIL, e = NIL),
            |vm, _| vm.bx.heap.new_string_from_str("—"),
        );
    }
    vm.set_injected_global(id!(sys), sys.into());
}

fn build(card: &str, data: serde_json::Value) -> octoscript_render::UiNode {
    let report = realize(card, &data, RealizeLimits::default());
    assert!(
        report.diagnostics.is_empty(),
        "card did not realize cleanly: {:#?}",
        report.diagnostics
    );
    let root = report.complete_root().expect("a complete realized tree");
    // The tail is a bare VARIABLE, not a call — `fn f() {…}` then `f()` is nil.
    let src = format!("{}\n{}", kit(), kit::lower(&root));
    octoscript_render::build(&src, register_missing_capabilities)
        .unwrap_or_else(|| panic!("the lowered card evaluated to nil:\n{}", kit::lower(&root)))
}

fn news_data() -> serde_json::Value {
    serde_json::json!({
        "lead": [{"id":"1","title":"Rust 1.95","author":"a","points":412.0,"comments":137.0,"url":"u"}],
        "feed": [{"id":"2","title":"Another","author":"b","points":90.0,"comments":12.0,"url":"u"}],
        "article": {}, "selected": "", "env": {"locale": {"lang":"en"}}
    })
}

fn stock_data() -> serde_json::Value {
    serde_json::json!({
        "movers": [{"ticker":"NVDA","name":"Nvidia","last":184.2,"change":3.1,"pct":1.7}],
        "quote": {"name":"Nvidia","last":184.2,"change":3.1,"pct":1.7,"open":181.0,
                  "high":185.6,"low":180.2,"volume":41200000.0,"mktcap":4.52e12,"pe":58.3},
        "selected": "", "range": "m1", "env": {"locale":{}},
        "copy": {"movers":"Top Movers","open":"Open","high":"High","low":"Low",
                 "volume":"Volume","mktcap":"Mkt Cap","pe":"P/E"}
    })
}

fn weather_data() -> serde_json::Value {
    serde_json::json!({
        "place": {"name":"Kyoto","lat":35.0,"lon":135.8},
        "now": {"temp":21.0,"cond":"clear","feels":20.0,"humidity":54.0,"wind":3.2,
                "pressure":1013.0,"uv":4.0,"visibility":10.0},
        "week": {"days":[{"dayname":"Mon","hi":24.0,"lo":15.0,"cond":"clear"}],
                 "min_lo":15.0,"max_hi":24.0},
        "sun": {"rise":5.1,"set":18.9}, "moon": {"phase":0.5,"illum":50.0},
        "scene": "https://x/y.jpg", "city":"", "units":"c", "days":7.0,
        "env": {"locale":{"lang":"en","temp_unit":"c"}}
    })
}

/// The counts both evaluators must agree on.
const CONFORMANCE: &str = include_str!("../../../components/l0/conformance.txt");

fn expected(card: &str) -> usize {
    CONFORMANCE
        .lines()
        .filter(|l| !l.trim_start().starts_with('#') && !l.trim().is_empty())
        .find_map(|l| {
            let (name, n) = l.split_once(char::is_whitespace)?;
            (name == card).then(|| n.trim().parse().ok())?
        })
        .unwrap_or_else(|| panic!("no conformance entry for {card:?}"))
}

/// Every reference card builds through the kit into the tree the contract says.
///
/// An exact count, not a floor. A card that evaluated to one empty node would
/// still be `Some` and would still render as a blank screen — but more than
/// that, this number is the ONLY thing holding the two evaluators together.
/// octos-one walks the same DSL with its own VM, because taking this crate
/// there fails the lockfile, and neither repository can run the other's walk. So
/// both read this file and compare against it; a drift fails on both sides, and
/// a legitimate change to the tree is one edit that fixes both.
#[test]
fn every_reference_card_builds_through_the_kit() {
    for (name, tree) in [
        ("news", build(NEWS, news_data())),
        ("stock", build(STOCK, stock_data())),
        ("weather", build(WEATHER, weather_data())),
    ] {
        assert_eq!(
            tree.count(),
            expected(name),
            "{name} produced {} nodes, the contract says {}",
            tree.count(),
            expected(name)
        );
    }
}

/// The card's TEXT survives the trip.
///
/// A tree of the right shape carrying no words is the failure this catches: the
/// structure is what a node count checks, and the content is what a reader sees.
#[test]
fn a_card_carries_its_text_through_the_kit() {
    fn words(n: &octoscript_render::UiNode, out: &mut Vec<String>) {
        if let Some(t) = n.attrs.text.as_deref() {
            out.push(t.to_owned());
        }
        for c in &n.children {
            words(c, out);
        }
    }
    let mut out = Vec::new();
    words(&build(NEWS, news_data()), &mut out);
    let text = out.join(" | ");
    // The card's OWN words, from its `copy` declarations. A seeded headline used
    // to be checked here too and is not any more: `sys.news` is answered live, so
    // a story title lowers to the CALL rather than to the blob this test hands in,
    // and the fixture adapters return missing. What this test guards is that words
    // survive the kit at all — a right-sized tree of empty nodes would pass the
    // count check beside it — and the card's own copy proves that without
    // depending on a value the backend now fetches.
    for expected in ["HACKER NEWS", "Top Stories"] {
        assert!(text.contains(expected), "{expected:?} missing from: {text}");
    }
}

/// **The reason this lowering exists.** A card must carry no presentation.
///
/// `makepad::lower` emits ten hardcoded colours and a font-size ramp, so it
/// decides what a theme decides and reaches one backend of three. The kit
/// lowering must name roles only — and this asserts it of the emitted SOURCE,
/// because the built tree is full of colours by design: the kit puts them there.
#[test]
fn the_lowered_card_names_roles_and_no_presentation() {
    let report = realize(STOCK, &stock_data(), RealizeLimits::default());
    let src = kit::lower(&report.root.expect("root"));

    for forbidden in ["draw_bg", "draw_text", "SolidView", "RoundedView", "Inset{"] {
        assert!(
            !src.contains(forbidden),
            "{forbidden:?} is presentation and belongs to the theme:\n{src}"
        );
    }

    // A hex COLOUR, specifically — not any `#`.
    //
    // The first version of this forbade `#` outright and failed the moment taps
    // were added, because an instance key contains them: `for#0[NVDA]`,
    // `when#0`. A crude check that fires on correct output is worse than no
    // check, because the fix under pressure is to delete it.
    let hex: Vec<&str> = src
        .match_indices('#')
        .map(|(i, _)| &src[i + 1..])
        .filter(|rest| {
            let digits = rest.chars().take_while(|c| c.is_ascii_hexdigit()).count();
            matches!(digits, 3 | 6 | 8)
        })
        .collect();
    assert!(
        hex.is_empty(),
        "a hex colour is presentation and belongs to the theme: {:?}\n{src}",
        hex.iter().map(|h| &h[..8.min(h.len())]).collect::<Vec<_>>()
    );
    assert!(src.contains("l0_panel("), "roles must be named:\n{src}");
}

/// The marker mechanism still works, though nothing now needs it.
///
/// Every one of L0's 23 roles has a kit answer since the five data
/// visualisations stopped being markers — so no reference card exercises this,
/// and it is asserted directly instead of through one.
///
/// Kept because the failure it prevents is the worst kind: a role with no answer
/// that renders as an ABSENCE leaves a card looking complete while missing its
/// temperature bars. The next role added to the catalog will have no kit
/// function on the day it is added, and this is what stands between that and a
/// silently short card.
#[test]
fn a_role_with_no_kit_answer_is_visible_rather_than_absent() {
    let kit = kit();
    let src = format!(
        r#"{kit}
let node = l0_unsupported("Hologram")
node
"#
    );
    let tree =
        octoscript_render::build(&src, register_missing_capabilities).expect("the marker evaluates");

    fn words(n: &octoscript_render::UiNode, out: &mut String) {
        if let Some(t) = n.attrs.text.as_deref() {
            out.push_str(t);
        }
        for c in &n.children {
            words(c, out);
        }
    }
    let mut text = String::new();
    words(&tree, &mut text);
    assert!(
        text.contains("Hologram"),
        "the marker must NAME the role it stands in for, got {text:?}"
    );
}

/// And the five that used to be markers now draw.
///
/// The weather card carries four of them and the stock card the fifth. This
/// asserts they reach the tree as their own kinds — the whole point of adding
/// kinds rather than routing them through an anonymous `Shader` with a variant.
#[test]
fn the_data_visualisations_reach_the_tree_as_themselves() {
    fn kinds(n: &octoscript_render::UiNode, out: &mut Vec<String>) {
        out.push(format!("{:?}", n.kind));
        for c in &n.children {
            kinds(c, out);
        }
    }
    let mut out = Vec::new();
    kinds(&build(WEATHER, weather_data()), &mut out);
    for expected in ["TempBar", "SunArc", "MoonPhase", "AqiContour"] {
        assert!(
            out.iter().any(|k| k == expected),
            "{expected} missing: {out:?}"
        );
    }
    let mut out = Vec::new();
    let mut store = octoscript_ui_l0::InstanceStore::default();
    octoscript_ui_l0::dispatch_with(
        STOCK,
        &mut store,
        "root",
        "open_quote",
        Some(&serde_json::Value::String("NVDA".into())),
    );
    let report =
        octoscript_ui_l0::realize_with_state(STOCK, &stock_data(), &store, RealizeLimits::default());
    let src = format!("{}\n{}", kit(), kit::lower(&report.root.expect("root")));
    kinds(
        &octoscript_render::build(&src, register_missing_capabilities).expect("detail evaluates"),
        &mut out,
    );
    assert!(
        out.iter().any(|k| k == "StockPlot"),
        "StockPlot missing: {out:?}"
    );
}

/// Every widget name this backend emits must be one the kit DEFINES.
///
/// This exists because it did not. `widget_name` mapped the five data
/// visualisations to `L0TempBar` and friends, and none of them were written —
/// five dangling names, passing every test, because nothing here renders a card
/// to pixels. The mapping and the definition live in different crates and
/// nothing tied them together.
#[test]
fn every_emitted_widget_name_is_defined_in_the_kit() {
    const PRELUDE: &str = include_str!("../../octoscript-widgets/src/lib.rs");

    let mut emitted: Vec<String> = Vec::new();
    for tree in [
        build(NEWS, news_data()),
        build(STOCK, stock_data()),
        build(WEATHER, weather_data()),
    ] {
        fn names(n: &octoscript_render::UiNode, out: &mut Vec<String>) {
            out.push(octoscript_makepad::widget_name_of(n.kind).to_owned());
            for c in &n.children {
                names(c, out);
            }
        }
        names(&tree, &mut emitted);
    }
    emitted.sort();
    emitted.dedup();

    // makepad's own widgets are not the kit's to define; only the ones this
    // repository adds have to appear in the prelude.
    let missing: Vec<&String> = emitted
        .iter()
        .filter(|w| w.starts_with("L0") || w.starts_with("Flutter"))
        .filter(|w| !PRELUDE.contains(&format!("mod.widgets.{w} =")))
        .collect();
    assert!(
        missing.is_empty(),
        "these widget names are emitted and never defined: {missing:?}"
    );
    // And the check is not vacuous: the cards must reach the L0 widgets at all.
    assert!(
        emitted.iter().any(|w| w.starts_with("L0")),
        "no L0 widget was emitted, so this asserted nothing: {emitted:?}"
    );
}

/// An L1 coefficient reaches the backend at FULL precision.
///
/// `trim_num` is a display rule — one decimal, because a temperature reads as
/// 21.4 and not 21.437 — and it was also being used to emit an arithmetic
/// OPERAND. Rounding an operand silently changes the result: a converter card
/// declaring `factor 0.621371` lowered to `(42 * 0.6)` and drew 42 km as 25.2
/// miles instead of 26.1, on device, with the checker accepting the card. The
/// card was right and the number on screen was wrong, which is exactly the
/// failure §1.1 exists to prevent — and no test could see it, because both the
/// realized tree and the built widget tree were structurally perfect.
#[test]
fn an_l1_coefficient_is_not_rounded_on_its_way_to_the_backend() {
    let card = "# level: L1\n\
        state amount { shape: number, initial: 42 }\n\
        state factor { shape: number, initial: 0.621371 }\n\
        view root Surface { TextHero(value: amount * factor) }\n";
    let report = realize(card, &serde_json::json!({"amount":42}), RealizeLimits::default());
    assert!(report.diagnostics.is_empty(), "{:#?}", report.diagnostics);
    let lowered = kit::lower(&report.root.expect("root"));
    assert!(
        lowered.contains("0.621371"),
        "the coefficient was rounded on the way out:\n{lowered}"
    );
    // And a whole number must not grow a decimal point on the way through.
    assert!(
        lowered.contains(r#"sys.l0_math("*", 42, 0.621371)"#),
        "an integer operand should stay an integer:\n{lowered}"
    );
}

#[test]
fn authored_selected_chip_tokens_survive_realization_and_native_lowering() {
    let tree = build("# level: L0\nview root Surface { Chip(text: \"All\", active: .on) Chip(text: \"Today\", active: .off) }", serde_json::json!({}));
    assert_ne!(tree.children[0].attrs.bg, tree.children[1].attrs.bg);
    let ui = octoscript_makepad::to_makepad_l0_ui(&tree);
    assert!(ui.contains("text: \"All\""));
    assert!(ui.contains("text: \"Today\""));
}
