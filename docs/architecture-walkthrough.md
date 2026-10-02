# Code walkthrough: from app source to native widgets

This guide is for a Rust developer new to OctoSense. It describes framework
revision `8f103d0c650196d455cbcaa55b13911475121224`, also selected by the
OctoSense workspace inspected for this walkthrough. Read the consuming app's
lock before substituting a newer checkout.

## What this repository runs

OctoSense is a shell that hosts apps. Octos is the agent kernel. This repository
is the shared **UI renderer**. A rendered card does not acquire an AI agent,
network access, or an app database just because it renders here. Those services
come from the consuming host's adapters and grants.

A native Rust application is compiled into a binary with Makepad widgets and
Rust event handlers. An OctoScript app supplies source/data to such a host;
the host evaluates it and mounts native widgets. Both end up on Makepad's UI
event loop. Neither requires one Tokio task per widget.

There are distinct source paths:

| Input | Entry point | Output |
| --- | --- | --- |
| Plain-data UI DSL, such as `{t:"text", text:"Hello"}` | `octoscript_render::build` | Portable `UiNode` tree |
| L0 card, data and optional instance state | `octoscript_makepad::l0::prepare_with_state` | Realized and lowered source plus `UiNode` |
| Source-measured design | `octoscript_makepad::design::prepare` | Checked `UiNode`, preserving explicit geometry |
| `UiNode` | `to_makepad_ui`, `to_makepad_l0_ui`, or `design::to_makepad_ui` | Makepad widget source for a host to mount |

These entry points are not interchangeable. `design::to_makepad_ui` preserves
measured geometry and validates semantic kit bindings; the themed translation
applies its own widget mapping. Native L0 kits choose the design path internally.

## Read the code in this order

1. [Workspace manifest](../Cargo.toml) and [runtime lock](../runtime.json): one
   Makepad source set and one OctoScript L0 revision. The sibling patches are
   intentional. Two Makepad copies can produce incompatible Rust types and
   separate VM heaps; matching package names alone does not make them one crate.
2. [Node model](../crates/octoscript-node/src/node.rs): `UiNode` contains a
   `NodeKind`, optional `Attrs`, and child nodes. This crate has no dependencies.
   It is the portable data boundary; it is not a window or a running agent.
3. [Evaluator](../crates/octoscript-render/src/eval.rs): `build` creates a fresh
   `ScriptVm`, lets the host register capabilities, installs pure L0 helpers,
   calls `eval_checked` with an instruction budget, and walks the returned
   object into the tree. Unknown tags, bad children and exhausted depth/node
   budgets fail the whole build (`None`). It uses Makepad's script VM, not an
   implicit Octos agent turn. State must be supplied again by the host on a
   later evaluation; arbitrary VM globals do not survive a fresh `build`.
4. [L0 preparation](../crates/octoscript-makepad/src/l0.rs):
   `realize_with_state` checks the card against data/state, then `complete_root`
   refuses incomplete output. Native kit components load `native/<mood>/kit.json`
   and use `kit_pack::lower` plus `design::prepare`. The other path loads the
   palette, ordered theme axes, derivation helpers and `_kit.octoscript`, then
   evaluates the lowered source. Missing capabilities do not become fabricated
   fixture data. `PreparedCard.native_components` tells the host which renderer
   to use.
5. [Translation](../crates/octoscript-makepad/src/lib.rs) and
   [design translation](../crates/octoscript-makepad/src/design.rs): the output
   is source text describing native Makepad widgets, not HTML. Translation
   alone does not create a window. `to_makepad_l0_ui_with_events` provides a
   host-selected event channel; the host is responsible for handling events,
   updating state/data, and deciding when to render again.
6. [Native themes](../crates/octoscript-widgets/src/lib.rs): compiled
   `script_mod!` modules register themed widgets and their shaders. This is why
   replacing a runtime string is not equivalent to compiling a shader variant.
7. [Catalog host](../apps/kit-host/src/main.rs): `App::mount` builds source from
   the route and Rust-held state, evaluates it, translates the tree, evaluates
   the widget source on the app's **main VM** with `cx.with_vm`, and assigns the
   resulting `View` to `Splash.view`. `handle_actions`/`handle_event` turn UI
   actions into updates and remounts. Follow one button through those methods.
8. [Preview host](../apps/kit-host/src/beauty.rs): `mount_request` reads the
   `BEAUTY_REQUEST` JSON file, prepares a design or L0 card, assigns inspectable
   IDs, builds a `View` using checked evaluation on the main VM, and replaces
   `Splash.view`. It retains the old view through replacement drawing, retires
   overlays/WebViews, and restores requested focus/selection/scroll state.

The last two paths do **not** call `Splash::set_text()` for their current
mount. That API's isolate behavior explains old bring-up notes, but not these
hosts' current VM ownership. A Splash widget is a mounting container here.
Other consumers must be checked independently.

## Run the framework examples

Commands below are source-derived recipes, not a record of a GUI/device run.
Run them from this repository. Install Rust/Cargo and platform-native Makepad
build prerequisites first. Dependency preparation needs Git/network access.

```sh
python3 tools/runtime.py prepare
python3 tools/runtime.py verify --cargo-manifest Cargo.toml
cargo test --release -p octoscript-node -p octoscript-render -p octoscript-makepad
cargo run -p kit-host --bin kit-host
SPLASH_ROUTE=l0/weather cargo run -p kit-host --bin kit-host
cargo run -p flutter-samples
```

`kit-host` has two binaries; name `--bin kit-host` explicitly. A device-pushed
route/source file takes precedence over the desktop environment fallback; see
`current_route` and `current_source` if a route seems ignored. Flutter samples
are a visual catalog with documented limitations, not evidence that every
Flutter platform integration has been implemented.

For a measured design/L0 acceptance run:

```sh
cargo build --release -p kit-host --bin beauty-host
BEAUTY_REQUEST=/absolute/path/request.json target/release/beauty-host --remote
```

`request.json` must supply `card` (source-file path), `data` (JSON-file path),
`width` and `height`. L0 input also needs `kit_dir`; measured design input uses
`"format": "design"`. Optional `layout`/`actions` paths collect measurements
and actions. Even a design request currently reads the data file, so provide
an existing JSON file (for example `{}`). Paths are read by the host process;
use absolute paths for reproducibility. Obtain real card/kit inputs from the
[design flow](https://github.com/OctoSense-org/OctoScript-App-Design-Flow).
The native HTTP instrument is enabled by `--remote`; follow its emitted
connection information for `/snap`, `/g`, input routes and `/gq`.
`MAKEPAD_HIDE_WINDOWS=1` is for automated native GPU checks, not proof of a
portable headless renderer. This documentation pass did not run a GPU preview.

Android requires Makepad's Android toolchain and a device/emulator; the sample
command is `cargo makepad android run -p flutter-samples --release`.
The actual OctoSense desktop/home/ROM packages are built in the OctoSense and
ROM repositories, not by `kit-host`. Launching this catalog will not install an
app into App Hub or create an app peer.

## State, app data and agents are different things

`octoscript-node::state` is a small widget-state helper, not the application's
persistent database. L0 `InstanceStore` supplies per-instance UI state. The
host owns app records, network services and agent integration, and passes
approved values/capabilities into evaluation. For production app stores and
peer routing, continue with the
[OctoSense walkthrough](https://github.com/OctoSense-org/OctoSense/blob/main/docs/architecture-walkthrough.md).

Do not do blocking network requests or model inference in a Makepad event
handler. A host should schedule that work outside the UI thread and send a
completion back to update state and redraw. This renderer itself supplies no
Tokio agent scheduler. Its synchronous checked evaluation and host callbacks
must remain bounded regardless of what asynchronous runtime the app uses.

## Verification and debugging

Portable tests exercise tree validity, evaluation budgets, lowering and emitted
widget contracts. They cannot prove font metrics, keyboard focus, WebView
lifecycle or touch behavior. For those, use the native preview and inspect its
measured layout/actions/screenshots. Common starting points:

- `build` returns `None`: inspect the source, registered capabilities, unknown
  tags and instruction/tree limits before changing the widget translator.
- A themed control is blank: check registration of compiled widgets and the VM
  on which the final view was evaluated.
- A card shows old values: follow host event handling, state update and remount;
  evaluating a source once does not subscribe it to a database.
- Cargo reports duplicate Makepad crates: repair the pinned sibling source set
  and verify the resolved graph before editing type definitions.
