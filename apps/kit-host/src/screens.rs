//! The reference catalog's screens, baked in.
//!
//! These are the **same `.octoscript` files** the Octoscript-Android catalog renders
//! with real `com.google.android.material.*` views. Baking the identical source
//! here is what makes the two backends comparable: same input, so any difference
//! on screen is this renderer's.
//!
//! `include_str!` rather than reading the directory, because cargo-makepad
//! builds Android inside a generated wrapper crate that never runs a build
//! script (the same reason the flutter kit is baked).

/// Shared helpers every screen composes from (`section`, `caption`, `group`, …).
pub const KIT: &str = include_str!("../../../components/material/screens/kit.octoscript");

/// Every route, in the order the reference lists them.
pub const SCREENS: &[(&str, &str)] = &[
    // The semantic-vocabulary experiment: a NEW app authored only from
    // WIDGETS.md by a generator, to test the vocabulary as a contract.
    ("brew", include_str!("../../../components/material/screens/brew.octoscript")),
    // The three-screen ordering app (tabs, per-drink customization, rewards).
    ("siren", include_str!("../../../components/material/screens/siren.octoscript")),
    // The promotion harness: every control writes a slot, a caption renders it.
    ("wired", include_str!("../../../components/material/screens/wired.octoscript")),
    ("adaptive", include_str!("../../../components/material/screens/adaptive.octoscript")),
    ("allcomponents", include_str!("../../../components/material/screens/allcomponents.octoscript")),
    ("badge", include_str!("../../../components/material/screens/badge.octoscript")),
    ("bottomappbar", include_str!("../../../components/material/screens/bottomappbar.octoscript")),
    ("bottomnav", include_str!("../../../components/material/screens/bottomnav.octoscript")),
    ("bottomsheet", include_str!("../../../components/material/screens/bottomsheet.octoscript")),
    ("button", include_str!("../../../components/material/screens/button.octoscript")),
    ("card", include_str!("../../../components/material/screens/card.octoscript")),
    ("carousel", include_str!("../../../components/material/screens/carousel.octoscript")),
    ("checkbox", include_str!("../../../components/material/screens/checkbox.octoscript")),
    ("chip", include_str!("../../../components/material/screens/chip.octoscript")),
    ("color", include_str!("../../../components/material/screens/color.octoscript")),
    ("datepicker", include_str!("../../../components/material/screens/datepicker.octoscript")),
    ("dialog", include_str!("../../../components/material/screens/dialog.octoscript")),
    ("divider", include_str!("../../../components/material/screens/divider.octoscript")),
    ("dockedtoolbar", include_str!("../../../components/material/screens/dockedtoolbar.octoscript")),
    ("elevation", include_str!("../../../components/material/screens/elevation.octoscript")),
    ("fab", include_str!("../../../components/material/screens/fab.octoscript")),
    ("floatingtoolbar", include_str!("../../../components/material/screens/floatingtoolbar.octoscript")),
    ("font", include_str!("../../../components/material/screens/font.octoscript")),
    ("imageview", include_str!("../../../components/material/screens/imageview.octoscript")),
    ("listitem", include_str!("../../../components/material/screens/listitem.octoscript")),
    ("loadingindicator", include_str!("../../../components/material/screens/loadingindicator.octoscript")),
    ("materialswitch", include_str!("../../../components/material/screens/materialswitch.octoscript")),
    ("menu", include_str!("../../../components/material/screens/menu.octoscript")),
    ("musicplayer", include_str!("../../../components/material/screens/musicplayer.octoscript")),
    ("navigationdrawer", include_str!("../../../components/material/screens/navigationdrawer.octoscript")),
    ("navigationrail", include_str!("../../../components/material/screens/navigationrail.octoscript")),
    ("octoswidgets", include_str!("../../../components/material/screens/octoswidgets.octoscript")),
    ("preferences", include_str!("../../../components/material/screens/preferences.octoscript")),
    ("progressindicator", include_str!("../../../components/material/screens/progressindicator.octoscript")),
    ("radiobutton", include_str!("../../../components/material/screens/radiobutton.octoscript")),
    ("search", include_str!("../../../components/material/screens/search.octoscript")),
    ("shapetheming", include_str!("../../../components/material/screens/shapetheming.octoscript")),
    ("sidesheet", include_str!("../../../components/material/screens/sidesheet.octoscript")),
    ("slider", include_str!("../../../components/material/screens/slider.octoscript")),
    ("snackbar", include_str!("../../../components/material/screens/snackbar.octoscript")),
    ("tabs", include_str!("../../../components/material/screens/tabs.octoscript")),
    ("textfield", include_str!("../../../components/material/screens/textfield.octoscript")),
    ("timepicker", include_str!("../../../components/material/screens/timepicker.octoscript")),
    ("topappbar", include_str!("../../../components/material/screens/topappbar.octoscript")),
    ("transition", include_str!("../../../components/material/screens/transition.octoscript")),
];

/// The reference's own name for a route -- shared so the toolbar and the index
/// cannot disagree about what a screen is called.
pub fn title_of(route: &str) -> &str {
    match route {
        INDEX => "Catalog",
        "allcomponents" => "All components",
        "brew" => "Brew",
        "siren" => "Siren Coffee",
        "wired" => "Wired controls",
            "adaptive" => "Adaptive layouts",
            "badge" => "Badge",
            "bottomappbar" => "Bottom app bar",
            "bottomnav" => "Bottom navigation",
            "bottomsheet" => "Bottom sheet",
            "button" => "Button",
            "card" => "Card",
            "carousel" => "Carousel",
            "checkbox" => "Checkbox",
            "chip" => "Chip",
            "color" => "Color palette",
            "datepicker" => "Date picker",
            "dialog" => "Dialog",
            "divider" => "Divider",
            "dockedtoolbar" => "Docked toolbar",
            "elevation" => "Elevation",
            "fab" => "Floating action button",
            "floatingtoolbar" => "Floating toolbar",
            "font" => "Typography",
            "imageview" => "Image view",
            "listitem" => "List item",
            "loadingindicator" => "Loading indicator",
            "materialswitch" => "Switch",
            "menu" => "Menu",
            "musicplayer" => "Music player",
            "navigationdrawer" => "Navigation drawer",
            "navigationrail" => "Navigation rail",
            "preferences" => "Preferences",
            "progressindicator" => "Progress indicator",
            "radiobutton" => "Radio button",
            "search" => "Search",
            "shapetheming" => "Shape theming",
            "sidesheet" => "Side sheet",
            "slider" => "Slider",
            "snackbar" => "Snackbar",
            "tabs" => "Tabs",
            "textfield" => "Text field",
            "timepicker" => "Time picker",
            "topappbar" => "Top app bar",
            "transition" => "Transition",
        other => other,
    }
}

/// Is this a route we can actually draw?
///
/// `source_for` falls back to the first screen for anything unknown, which is
/// silent: pushing a typo, or the string `home` that was never a route, drew
/// `adaptive` under the wrong title rather than reporting anything.
pub fn has(route: &str) -> bool {
    route == INDEX || SCREENS.iter().any(|(n, _)| *n == route)
}

/// The route name of the catalog index.
pub const INDEX: &str = "index";

/// The index: one tappable row per screen, in the order the reference lists
/// them. Generated rather than written as a `.octoscript` file so it cannot drift
/// out of step with `SCREENS`.
fn index_source() -> String {
    let mut rows = String::new();
    for (name, _) in SCREENS {
        // The chevron is now affordance only -- the whole row is tappable and
        // still scrolls, because the target does not capture the finger. See
        // `emit_click_overlay`.
        rows.push_str(&format!(
            "  {{t:\"row\", tapto:\"{name}\", h: 56, aligny: 0.5, fillw: 1, c:[ \
             {{t:\"text\", size: 16, text:\"{}\", fillw: 1}}, \
             {{t:\"text\", size: 16, icon: 1, text:\"\\u{{f054}}\"}} ]}},\n",
            title_of(name)
        ));
    }
    // The L0 routes, at the top: they are the reason this host exists now, and
    // burying them under fifty catalog screens makes them unfindable on a phone.
    let mut l0_rows = String::new();
    for (route, title) in crate::l0::ROUTES {
        l0_rows.push_str(&format!(
            "  {{t:\"row\", tapto:\"{route}\", h: 56, aligny: 0.5, fillw: 1, c:[ \
             {{t:\"text\", size: 16, text:\"{title}\", fillw: 1}}, \
             {{t:\"text\", size: 16, icon: 1, text:\"\\u{{f054}}\"}} ]}},\n"
        ));
    }
    format!("{{t:\"scroll\", c:[ {{t:\"col\", pad: 16, spacing: 0, c: [\n{l0_rows}{rows}]}} ]}}")
}

/// The full source for a route: shared kit first, then the screen.
pub fn source_for(route: &str) -> String {
    if route == INDEX {
        return format!("{KIT}\n{}", index_source());
    }
    let body = SCREENS
        .iter()
        .find(|(n, _)| *n == route)
        .or_else(|| SCREENS.first())
        .map(|(_, s)| *s)
        .unwrap_or("");
    format!("{KIT}\n{body}")
}
