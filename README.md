# Octoscript-Makepad

English | [简体中文](README.zh-CN.md)

Themed, cross-platform **component kits** for apps built on the **Octoscript DSL → makepad native-widget** renderer.

Author a UI once as plain-data Octoscript DSL; it is evaluated in the makepad-script VM, translated to makepad's own widget dialect, and mounted as **real native makepad widgets** at runtime (with on-device hot reload). This repo is the home for the render pipeline **and** the themed component sets that ride on it — Material 3 today; **iOS** and **liquid-glass** planned.

## The pipeline

```
Octoscript DSL  ──►  octoscript-render  ──►  UiNode tree  ──►  octoscript-makepad  ──►  makepad dialect string
{t:"column",       (makepad-script VM,   (backend-       (pure translation)     View{…}/Label{…}/…
 c:[ … ]}           renderer-free)        agnostic)                              │
                                                                                 ▼
                                                          makepad `Splash` widget .set_text() → live native widgets
```

- **`crates/octoscript-render`** — backend-agnostic core: evaluates the Octoscript DSL in the makepad-script VM and walks it into a `UiNode` tree. Depends only on `makepad-script`. Unit-tested.
- **`crates/octoscript-makepad`** — the makepad backend: `to_makepad_ui(&UiNode) -> String` turns the tree into makepad's `View{}/Label{}/…` dialect. Pure, unit-tested — no makepad-platform/draw needed to build or test.
- **`crates/octoscript-widgets`** — the **themed native-widget kits** (Material 3 now; iOS / liquid-glass later), as **external `script_mod!` variants of makepad's widgets** (see *Fork-free theming* below).
- **`crates/makepad-d3`** — the **d3 grammar as native widgets** (scales, shapes, layouts, hierarchies, geo, 3D), registered into the VM under `mod.d3.*`. Grafted in with its history 2026-08-09 (was `mofa-org/makepad-d3`).
- **`crates/makepad-plot`** — the **matplotlib chart set** (32 widgets: line, bar, scatter, pie, box, violin, heatmap, contour, quiver, 3D surface/scatter/line, gauge, treemap, …) under `mod.plot.*`. Grafted in with its history 2026-08-09 (was `mofa-org/makepad-matplot`).
  Both build against the SAME makepad checkout as `octoscript-widgets` — one makepad per workspace, or two copies of `makepad-widgets` meet in one binary and each registers into its own heap. Their namespaces are disjoint, so a card may use either or both.
- **`components/<theme>/`** — each theme's **component library**, authored as `.octoscript` (e.g. `components/material/catalog.octoscript`, ~35 Material components + demo screens). Pure data — hot-reloadable, no rebuild.
- **`components/flutter/`** — the **flutter/samples port**: one `.octoscript` per sample directory, 108 routes (see below).

## The flutter/samples port

> **These are static illustrations, not widget ports.** 86% of the kit's nodes
> are layout containers and none are buttons; the DSL has no `onPressed`, no
> state model and no animation, so a Flutter widget cannot be reproduced —
> only pictured. See the [kit README](components/flutter/README.md) for the
> independent review that established this and the two defects it found.


Every directory of [flutter/samples](https://github.com/flutter/samples) has a `.octoscript` file in `components/flutter/` — 27 of them, 108 routes, all swept by `cargo test`. Full write-up in [`components/flutter/README.md`](components/flutter/README.md).

Eleven directories are apps with a UI to draw, and are ported: 92 screens carrying the samples' real content — the M3 type scale at its actual sp values, the six elevation levels with their dp and surface-tint percentages, all nine `date_planner` events with their task lists, the four `libraryInstance` books, the real `destinations.json` entries.

The other sixteen exist to demonstrate Flutter's **platform integration** — `add_to_app`, `platform_channels`, `pedometer`'s FFIgen bindings, the GLSL shader samples, build tooling. There is nothing to draw, so each gets a screen naming what the sample teaches and why it does not port, rather than an invented UI.

These are **visual ports**: the pipeline evaluates the DSL to a tree once per mount, so there is no per-component state, no async, no HTTP, no navigation stack and no animation. `animations` ports its index and all 20 titles but not the animations; `compass_app` ports its five screens but not the architecture that is most of the sample. Anything a screen cannot honestly render says so on the screen.

The kit spans many files and the DSL has no `import`, so it is **concatenated** in a fixed order by `octoscript_makepad::kit` — `_kit.octoscript` first, samples sorted, `_index.octoscript` (the router) last. The test and the `assemble` example call that function; the app bakes the same files with `include_str!`, because `cargo-makepad` builds Android inside a generated wrapper crate that never runs the app's build script. A test pins the baked list to the directory so the two cannot drift.

```sh
cargo test -p octoscript-makepad     # sweep all 108 routes — no device needed
cargo run  -p flutter-samples    # run the catalog on desktop
cargo makepad android run -p flutter-samples --release    # …or on a phone
tools/visual-qa.sh               # screenshot all 108 on the device
```

Every screen has been run on a real device (OnePlus 6T) and looked at, not just
asserted on: `tools/visual-qa.sh` drives each route over adb, screenshots it and
builds contact sheets. That found nine rendering defects the route sweep
structurally cannot see — a collapsed page root, clipped descenders, unwrapped
paragraphs, three empty pickers, one-pixel chat bubbles — all fixed. The
remaining deviations from Flutter are structural and listed in the kit's README.

Tapping needed two fixes on top of that, both in the kit's README: a `View`
ignores `on_click` (only `Button`/`CheckBox`/`GlassPanel` have it), so the
translator now overlays a transparent Button on any tappable container; and
because the `Splash` isolate resolves `ui` against its own view root, the
`nav_signal` label the handler writes to has to live *inside* the mounted tree.

## Fork-free theming (the key design point)

makepad's native controls — checkbox, switch, radio, slider, text field — are drawn by their own MPSL shaders; their look is **not** reachable from the Octoscript DSL. It **is** reachable from an external crate, but only one way works:

| Mechanism | Result |
|---|---|
| Runtime `script_eval!` override of `mod.prelude.widgets.*` | shader **dropped** → widget renders blank ❌ |
| **Compiled `script_mod!`** — extend the base (`mod.widgets.CheckBox = mod.widgets.CheckBoxFlat{ draw_bg +: {…} }`), then reference into the prelude | shader **kept** ✅ |

The `script_mod!` macro compiles the MPSL at build time; a runtime string never gets compiled. So `octoscript-widgets` restyles makepad's widgets against **upstream makepad** with **no fork** — theming itself needs no upstream change. (Verified on device: a compiled variant renders; the runtime override renders blank.)

Each new theme is just more variants in `octoscript-widgets` + a `.octoscript` component library — no makepad fork per theme.

### The one upstream PR

Building + running the `kit-host` against upstream surfaced exactly **one** thing upstream doesn't have: a **`Splash` main-VM-mount option**. Upstream's `Splash` always allocates an *isolate* VM (`alloc_splash_vm_with_network(allow_net)`), but the light theme and a shared heap live on the app's **main** VM. The fix is the small `isolate: false` field this project's fork added to `widgets/src/splash.rs` — upstreaming it lets a trusted, app-generated kit mount on the main VM (correct theme, no isolate-heap animator panics). Until then the kit mounts on an isolate (dark-default theme). That is the *only* upstream change needed; everything else runs against upstream `dev` as-is.

## Shared runtime for OctoSense apps

This repository owns the shared runtime for AppCards, Mail, OctoSense desktop
and mobile, Android, OpenHarmony and browser hosts. `runtime.json` locks one underlying `OctoSense-org/makepad` revision and
one `OctoSense-org/Octoscript` revision. The same pins appear in the root Cargo
workspace; `tools/runtime.py` rejects drift. Each Cargo workspace declares
only the sibling overrides it actually uses, so locked builds stay reproducible. Mail's scrolling, text input and native HTML WebView support live
here and in the locked Makepad source, rather than in per-application patches.

Arrange the independent repositories as siblings named `octoscript-makepad`,
`octoscript` and `makepad`. Run `python3 tools/runtime.py prepare` from this
repository, or use AppCards' `tools/setup-native.py`, which also selects the
framework release. Existing local edits are preserved; `--update` only moves
clean dependency checkouts. `python3 tools/runtime.py verify --cargo-manifest
Cargo.toml` checks the source set and rejects multiple Makepad instances in the
resolved Cargo graph.

## Build

### Native application validation

Build `cargo build --release -p kit-host --bin beauty-host` from this workspace.
Launch the standalone host with `--remote`; automation sets
`MAKEPAD_HIDE_WINDOWS=1` and uses the built-in HTTP instrument on the native GPU
backend. Use `/snap`, input routes and `/g` to inspect the owned application;
finish with `/gq` and confirm process exit. Studio is not part of this workflow.

The host accepts measured AppCard designs and L0 cards, preserves text selection
and scroll state across remounts, and retires platform WebViews when navigating.
On macOS its custom-event probes expose the owned WebView's content, scroll
extent, snapshot and lifecycle for application acceptance checks.

`cargo test --release -p octoscript-node -p octoscript-render -p octoscript-makepad`
checks the portable render pipeline and component contracts.

## Status

- ✅ `octoscript-render` + `octoscript-makepad` — portable render pipeline; **compile + test against upstream `makepad-script`** (rev `e1c2164b`), no fork
- ✅ **Material 3 kit** — `components/material/catalog.octoscript`: ~35 components (buttons, FABs, cards, chips, nav bar/rail/drawer, app bars, dialog/menu/sheets as **real interactive overlays**, pickers, tabs, badges, toolbars), M3 tokens (colour, type scale + Medium weight, shape, elevation, surface tones), Font-Awesome monochrome icons, and real animation (circular spinner + shape-morph loading indicator)
- ✅ `octoscript-widgets` — Material 3 native-control variants (checkbox/switch/radio/slider/text field) + `LoadingMorph`, fork-free; **compiles against upstream `makepad-widgets`**
- ✅ **`apps/kit-host`** — generic app shell that **builds + runs against upstream makepad** (desktop, ~37 MB binary), fork-free, mounting the Material kit via `octoscript_widgets::widgets_mod`
- ⏳ **The one upstream PR:** the `Splash` main-VM-mount option (see above) — the single change needed for correct light-theme rendering
- ⏳ **Next:** that PR (or an isolate-VM theme/heap fix so the mount works isolated); Android build via `cargo-makepad`; Button **touch-ripple** as a `RippleButton` variant; **iOS** + **liquid-glass** kits

## License

Apache-2.0 (see [LICENSE](LICENSE) and [NOTICE](NOTICE)). The grafted `crates/makepad-d3`
(MIT OR Apache-2.0) and `crates/makepad-plot` (MIT) keep their original licenses, and bundled
fonts keep theirs.
