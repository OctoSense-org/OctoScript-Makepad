# Code walkthrough: from app source to native widgets

[runtime.json](../runtime.json) selects Makepad and OctoScript revisions;
[Cargo.toml](../Cargo.toml) declares the crates and sibling overrides. Check the
consuming app's runtime lock before changing its framework checkout.

## What this repository runs

OctoSense is a shell that hosts apps. Octos is the agent kernel. This repository
is the shared UI renderer. Agents, network access and storage come from the
host's adapters and grants.

A native Rust application is compiled into a binary with Makepad widgets and
Rust event handlers. An OctoScript app supplies source and data to such a host;
the host evaluates it and mounts native widgets. Both end up on Makepad's UI
event loop.

There are distinct source paths:

| Input | Entry point | Output |
| --- | --- | --- |
| Plain-data UI DSL, such as `{t:"text", text:"Hello"}` | `octoscript_render::build` | Portable `UiNode` tree |
| L0 card, data and optional instance state | `octoscript_makepad::l0::prepare_with_state` | Realized and lowered source plus `UiNode` |
| Source-measured design | `octoscript_makepad::design::prepare` | Checked `UiNode`, preserving explicit geometry |
| `UiNode` | `to_makepad_ui`, `to_makepad_l0_ui`, or `design::to_makepad_ui` | Makepad widget source for a host to mount |

`design::to_makepad_ui` preserves
measured geometry and validates semantic kit bindings; the themed translation
applies its own widget mapping. Native L0 kits choose the design path internally.

## Read the code in this order

1. [Workspace manifest](../Cargo.toml) and [runtime lock](../runtime.json): one
   Makepad source set and one OctoScript L0 revision. The sibling patches are
   intentional. Two Makepad copies can produce incompatible Rust types and
   separate VM heaps; matching package names alone does not make them one crate.
2. [Node model](../crates/octoscript-node/src/node.rs): `UiNode` contains a
   `NodeKind`, optional `Attrs`, and child nodes. This crate has no dependencies.
   It is the portable data boundary between backends. The same crate contains
   [ai.rs](../crates/octoscript-node/src/ai.rs), which expands AI-authored text
   marks; [state.rs](../crates/octoscript-node/src/state.rs), which provides
   numeric widget state; and [units.rs](../crates/octoscript-node/src/units.rs),
   which implements shared unit conversions.
3. [Evaluator](../crates/octoscript-render/src/eval.rs): `build` creates a fresh
   `ScriptVm`, lets the host register capabilities, installs pure L0 helpers,
   calls `eval_checked` with an instruction budget, and walks the returned
   object into the tree. Unknown tags, bad children and exhausted depth/node
   budgets fail the whole build (`None`). The limits come from `octoscript_node`:
   2,000,000 evaluation instructions, depth 128 and 65,536 nodes. The host
   supplies state on each evaluation because `build` creates a fresh VM.
4. [L0 preparation](../crates/octoscript-makepad/src/l0.rs):
   `realize_with_state` checks the card against data and state, then `complete_root`
   refuses incomplete output. Native kit components load `native/<mood>/kit.json`
   and use `kit_pack::lower` plus `design::prepare`. The other path loads the
   palette, ordered theme axes, derivation helpers and `_kit.octoscript`, then
   evaluates the lowered source with explicit host data. `PreparedCard.native_components` tells the host which renderer
   to use. `realize_with_state`, `InstanceStore` and `kit_pack` belong to the
   external `octoscript-ui-l0` crate in the
   [OctoScript repository](https://github.com/OctoSense-org/Octoscript/tree/main/crates/octoscript-ui-l0);
   use this workspace's runtime lock to select its source revision.
5. [Translation](../crates/octoscript-makepad/src/lib.rs) and
   [design translation](../crates/octoscript-makepad/src/design.rs):
   `to_makepad_ui` and the L0 translation path clone the tree, run
   `ai::expand_ai_marks` to place visible marks beside AI-authored text, resolve
   ink contrast, then emit Makepad widget source. The AI-mark pass is
   idempotent. The measured-design translator has its own emission path.
   `to_makepad_l0_ui_with_events` provides a
   host-selected event channel; the host is responsible for handling events,
   updating state and data, and deciding when to render again.
6. [Native themes](../crates/octoscript-widgets/src/lib.rs): compiled
   `script_mod!` modules register themed widgets and their compiled shaders.
   [makepad-d3](../crates/makepad-d3/README.md) and
   [makepad-plot](../crates/makepad-plot/README.md) provide chart widgets,
   registered under `mod.d3.*` and `mod.plot.*` on the same Makepad source set.
7. [Catalog host](../apps/kit-host/src/main.rs): `App::mount` builds source from
   the route and Rust-held state, evaluates it, translates the tree, evaluates
   the widget source on the app's **main VM** with `cx.with_vm`, and assigns the
   resulting `View` to `Splash.view`. `handle_actions` and `handle_event` turn UI
   actions into updates and remounts. Its initial tree evaluation uses the
   checked renderer, but final widget-source evaluation uses
   `eval_with_append_source`, without an explicit instruction budget.
8. [Preview host](../apps/kit-host/src/beauty.rs): `mount_request` reads the
   `BEAUTY_REQUEST` JSON file, prepares a design or L0 card, assigns inspectable
   IDs, builds a `View` using `eval_checked(sm, 2_000_000)` on the main VM,
   and replaces `Splash.view`. It retains the old view through replacement drawing, retires
   overlays and WebViews, and restores focus, selection and scroll state.

Both hosts use Splash as a container for a main-VM `View`; their mounting
paths differ in final evaluation budgets as described above.

## Run the framework examples

Run from this repository after installing Rust, Cargo and Makepad's native
build prerequisites. Dependency preparation uses Git and network access.

```sh
python3 tools/runtime.py prepare
python3 tools/runtime.py verify --cargo-manifest Cargo.toml
cargo test --release -p octoscript-node -p octoscript-render -p octoscript-makepad
cargo run -p kit-host --bin kit-host
SPLASH_ROUTE=l0/weather cargo run -p kit-host --bin kit-host
cargo run -p flutter-samples
```

`kit-host` has two binaries; name `--bin kit-host` explicitly. In
[kit-host](../apps/kit-host/src/main.rs), `current_route` reads the device route
file, then `SPLASH_ROUTE`, then defaults to `button`. Its `current_source(&self)`
first accepts a device-pushed source file, then tries the in-app screen and
finally that route. The [Flutter host](../apps/flutter-samples/src/lib.rs) has
a separate `current_source()` selecting device-pushed source or its baked
catalog. See the [Flutter catalog limitations](../components/flutter/README.md)
for the scope of that port.

For a measured design or L0 acceptance run:

```sh
cargo build --release -p kit-host --bin beauty-host
BEAUTY_REQUEST=/absolute/path/request.json target/release/beauty-host --remote
```

`request.json` must supply `card` (source-file path), `data` (JSON-file path),
`width` and `height`. L0 input also needs `kit_dir`; measured design input uses
`"format": "design"`. Optional `layout` and `actions` paths collect measurements
and actions. Even a design request currently reads the data file, so provide
an existing JSON file (for example `{}`). Paths are read by the host process;
use absolute paths for reproducibility. Obtain card and kit inputs from the
[design flow](https://github.com/OctoSense-org/OctoScript-App-Design-Flow).
The native HTTP instrument is enabled by `--remote`; follow its emitted
connection information for `/snap`, `/g`, input routes and `/gq`.
`MAKEPAD_HIDE_WINDOWS=1` hides windows during automated native GPU checks.

Android requires Makepad's Android toolchain and a device or emulator; the sample
command is `cargo makepad android run -p flutter-samples --release`.
OctoSense builds its desktop, Home and ROM packages in the OctoSense repository.

## State, app data and agents are different things

`octoscript_node::state` is a small widget-state helper, not the application's
persistent database. L0 `InstanceStore` supplies per-instance UI state. The
host owns app records, network services and agent integration, and passes
approved values and capabilities into evaluation. For production app stores and
peer routing, continue with the
[OctoSense walkthrough](https://github.com/OctoSense-org/OctoSense/blob/c3011a2057ec59738b79466f48ff2ad8d0e60130/docs/architecture-walkthrough.md).

Schedule blocking network requests and model inference outside the Makepad UI
thread, then send completions back to update state and redraw. The consuming
host owns that scheduling and the cost of its synchronous evaluation callbacks.

## Verification and debugging

Portable tests exercise tree validity, evaluation budgets, lowering and emitted
widget contracts. They cannot prove font metrics, keyboard focus, WebView
lifecycle or touch behavior. For those, use the native preview and inspect its
measured layout, actions and screenshots. Common starting points:

- `build` returns `None`: inspect the source, registered capabilities, unknown
  tags and instruction/tree limits before changing the widget translator.
- A themed control is blank: check registration of compiled widgets and the VM
  on which the final view was evaluated.
- A card shows old values: follow host event handling, state update and remount;
  evaluating a source once does not subscribe it to a database.
- Cargo reports duplicate Makepad crates: repair the pinned sibling source set
  and verify the resolved graph before editing type definitions.
