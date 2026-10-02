# OctoScript-Makepad

English | [简体中文](README.zh-CN.md)

> **Building an OctoSense app?** You do not need to work in this repository. It is the shared UI runtime every OctoSense shell and `card-host` build against; `tools/setup-native.py` in OctoScript-App-Design-Flow checks it out for you as the sibling `octoscript-makepad/` at the pinned revision. Start from the [OctoSense organization profile](https://github.com/OctoSense-org)'s reading order: [OctoScript-App-Design-Flow `AGENTS.md`](https://github.com/OctoSense-org/OctoScript-App-Design-Flow/blob/main/AGENTS.md) → [`flows/README.md`](https://github.com/OctoSense-org/OctoScript-App-Design-Flow/blob/main/flows/README.md) → [`docs/QUICKSTART.md`](https://github.com/OctoSense-org/OctoScript-App-Design-Flow/blob/main/docs/QUICKSTART.md).

Themed, cross-platform **component kits** for apps built on the **OctoScript DSL → makepad native-widget** renderer.

Author a UI once as plain-data OctoScript DSL; it is evaluated in the makepad-script VM, translated to makepad's own widget dialect, and mounted as **real native makepad widgets** at runtime (with on-device hot reload). This repo is the home for the render pipeline **and** the themed component sets that ride on it — Material 3 today; **iOS** and **liquid-glass** planned.

Follow a text label from source to native widgets in the
[code walkthrough](docs/architecture-walkthrough.md), then explore L0 cards,
preview commands, state and agent boundaries. Contributor guidance is in
[AGENTS.md](AGENTS.md).

## The pipeline

```
OctoScript DSL  ──►  octoscript-render  ──►  UiNode tree  ──►  octoscript-makepad  ──►  makepad dialect string
{t:"column",       (makepad-script VM,   (backend-       (pure translation)     View{…}/Label{…}/…
 c:[ … ]}           renderer-free)        agnostic)                              │
                                                                                 ▼
                                                          host main VM → View → Splash.view → live native widgets
```

- **`crates/octoscript-node`** — dependency-free `UiNode`/`Attrs` model shared by render backends.
- **`crates/octoscript-render`** — backend-agnostic core: evaluates the OctoScript DSL in the makepad-script VM and walks it into a `UiNode` tree. Depends on `makepad-script`, the portable node model and `serde_json`; it has no platform/draw/widgets dependency. Unit-tested.
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

makepad's native controls — checkbox, switch, radio, slider, text field — are drawn by their own MPSL shaders; their look is **not** reachable from the OctoScript DSL. It **is** reachable from an external crate, but only one way works:

| Mechanism | Result |
|---|---|
| Runtime `script_eval!` override of `mod.prelude.widgets.*` | shader **dropped** → widget renders blank ❌ |
| **Compiled `script_mod!`** — extend the base (`mod.widgets.CheckBox = mod.widgets.CheckBoxFlat{ draw_bg +: {…} }`), then reference into the prelude | shader **kept** ✅ |

The `script_mod!` macro compiles the MPSL at build time; a runtime string never gets compiled. So `octoscript-widgets` restyles makepad's widgets against **upstream makepad** with **no fork** — theming itself needs no upstream change. (Verified on device: a compiled variant renders; the runtime override renders blank.)

Each new theme is just more variants in `octoscript-widgets` + a `.octoscript` component library — no makepad fork per theme.

### Current mount ownership

`kit-host::App::mount` and `beauty-host::App::mount_request` evaluate the generated
widget source on the host's **main VM** using `cx.with_vm`, construct a `View`,
and assign it to `Splash.view`. This keeps registered fonts and themed widgets
in the same VM. They do not currently mount through `Splash::set_text`.

Older bring-up notes described an isolate mount and a proposed `isolate: false`
field. The pinned Makepad has no such field, but these hosts no longer depend
on that proposal: the current main-VM mount is implemented in the hosts. See
[the walkthrough](docs/architecture-walkthrough.md) before copying an older
mount recipe. Other consumers can choose a different mount path.

## Shared runtime for OctoSense apps

This repository owns the shared runtime for the OctoSense shells (the desktop and
phone packagings in [OctoSense](https://github.com/OctoSense-org/OctoSense)), the App Hub's `card-host`, the App Cards and flows in
[OctoScript-App-Design-Flow](https://github.com/OctoSense-org/OctoScript-App-Design-Flow), Mail, Android, OpenHarmony and browser hosts. `runtime.json` locks one underlying `OctoSense-org/makepad` revision and
one `OctoSense-org/Octoscript` revision. The same pins appear in the root Cargo
workspace; `tools/runtime.py` rejects drift. Each Cargo workspace declares
only the sibling overrides it actually uses, so locked builds stay reproducible. Mail's scrolling, text input and native HTML WebView support live
here and in the locked Makepad source, rather than in per-application patches.

Arrange the independent repositories as siblings named `octoscript-makepad`,
`octoscript` and `makepad`. Run `python3 tools/runtime.py prepare` from this
repository, or use [OctoScript-App-Design-Flow](https://github.com/OctoSense-org/OctoScript-App-Design-Flow)'s `tools/setup-native.py`, whose
`native-runtime.lock.json` also selects the framework release. Existing local edits are preserved; `--update` only moves
clean dependency checkouts. `python3 tools/runtime.py verify --cargo-manifest
Cargo.toml` checks the source set and rejects multiple Makepad instances in the
resolved Cargo graph.

## Build

### Native application validation

Prepare and verify the pinned siblings as described above. To open the catalog,
run `cargo run -p kit-host --bin kit-host`. For a preview, build
`cargo build --release -p kit-host --bin beauty-host` from this workspace.
Set `BEAUTY_REQUEST` to a request JSON file containing card/data paths and
viewport dimensions; see the [complete recipe](docs/architecture-walkthrough.md).
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

## Status and validation scope

The source includes checked evaluation, themed and measured-design translation,
Material widgets, L0 native kits, chart widgets, and native catalog/preview
hosts. Current pins are in [runtime.json](runtime.json). Device results above
predate the current pins; rerun portable tests and native acceptance before shipping.

## License

Apache-2.0 (see [LICENSE](LICENSE) and [NOTICE](NOTICE)). The grafted `crates/makepad-d3`
(MIT OR Apache-2.0) and `crates/makepad-plot` (MIT) keep their original licenses, and bundled
fonts keep theirs.
