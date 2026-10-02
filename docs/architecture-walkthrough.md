# Code walkthrough: from app source to native widgets

OctoSense is a shell that hosts apps. Octos runs their agents. This repository
turns UI descriptions into native Makepad widgets. Follow a small text label
through that process first, then use the later sections for L0 cards, previews
and application state.

## What this repository runs

A native Rust application is compiled into a binary with Makepad widgets and
Rust event handlers. An OctoScript app supplies source and data to such a host;
the host evaluates it and mounts native widgets. Both use Makepad's UI event
loop. Agents, network access and storage come from the host's adapters and grants.

A **DSL** is a language designed for a particular task; here it describes a UI.
The plain-data DSL represents each element as an object with a tag (`t`),
attributes such as `text`, and optional children (`c`). A **VM**, or virtual
machine, evaluates that source. The renderer converts its result into ordinary
Rust data before a backend creates widget source.

<a id="read-the-code-in-this-order"></a>

## Follow one label from source to screen

This illustrative Rust fragment uses the public renderer APIs. It was checked
against the source and an existing test, but has not been executed here. It
needs no host capabilities:

```rust
let source = r#"{t:"text", text:"Hello", h:28}"#;
let tree = octoscript_render::build(source, |_| {}).expect("valid UI source");
assert_eq!(tree.kind, octoscript_node::NodeKind::Text);
assert_eq!(tree.attrs.text.as_deref(), Some("Hello"));
let widget_source = octoscript_makepad::to_makepad_ui(&tree);
```

1. **Evaluate the source.** In [eval.rs](../crates/octoscript-render/src/eval.rs),
   `build` creates a fresh `ScriptVm` and evaluates the object. Its `walk_inner`
   function reads `t` as the node kind, `text` as an attribute, and `h` as height.
   The result is a `UiNode` with kind `Text`, text `Hello`, height `28` and no
   children. The [node model](../crates/octoscript-node/src/node.rs) is portable
   Rust data with no renderer dependency.
2. **Translate the tree.** In [lib.rs](../crates/octoscript-makepad/src/lib.rs),
   `to_makepad_ui` clones the tree, expands marks for AI-authored text, resolves
   text contrast, then emits widget source. `widget_name` maps `Text` to `Label`;
   `emit_attrs` writes the text property. The `Label` portion is shown below;
   surrounding layout and generated font properties are omitted:

   ```text
   Label {
       text: "Hello"
   }
   ```

3. **Mount the widgets.** The string still needs a native host. In the
   [catalog host](../apps/kit-host/src/main.rs), `App::mount` evaluates generated
   widget source on the app's **main VM** through `cx.with_vm`, then assigns the
   resulting `View` to `Splash.view`. Splash is the container that displays it.
   The earlier evaluation built a data tree; this evaluation creates the live UI.
4. **Handle the next action.** `handle_actions` and `handle_event` receive UI
   actions, update Rust-held state and remount the UI when needed. The host
   supplies the current values again because each `build` uses a fresh VM.
   Rendering once does not subscribe the label to a database.

For a larger version of this path, read `column_of_text_becomes_view_with_label`
in the [translator tests](../crates/octoscript-makepad/src/lib.rs). It checks
that a styled column emits a `RoundedView` containing a `Label` and the
expected text.

## Where L0 cards and measured designs enter

An **L0 card** is a checked UI description that can refer to app data and
instance state. **Realization** checks those references against the supplied
values and produces a complete card tree. **Lowering** converts that tree into
the more detailed UI source needed by a rendering backend. A **kit** supplies
the component definitions and styling used during that conversion.

| Input | Entry point | Output |
| --- | --- | --- |
| Plain-data UI DSL, as in the label above | `octoscript_render::build` | Portable `UiNode` tree |
| L0 card, data and optional instance state | `octoscript_makepad::l0::prepare_with_state` | Realized and lowered source plus `UiNode` |
| Design source with explicit positions and sizes | `octoscript_makepad::design::prepare` | Checked `UiNode`, preserving that geometry |
| `UiNode` | `to_makepad_ui`, `to_makepad_l0_ui`, or `design::to_makepad_ui` | Makepad widget source for a host to mount |

In [l0.rs](../crates/octoscript-makepad/src/l0.rs), `prepare_with_state` calls
`realize_with_state`, then `complete_root` refuses incomplete output. Preparation
then takes one of two paths:

- Native kit components load `native/<mood>/kit.json`. `kit_pack::lower` produces
  design source, and `design::prepare` builds its tree.
- Other components load the palette, theme choices such as radius and density
  (the code calls these **theme axes**), helpers that derive style values, and
  `_kit.octoscript`. The assembled source is then evaluated into a tree.

`PreparedCard.native_components` tells the host which translator to use.
`realize_with_state`, `InstanceStore` and `kit_pack` belong to the external
`octoscript-ui-l0` crate in the
[OctoScript repository](https://github.com/OctoSense-org/Octoscript/tree/main/crates/octoscript-ui-l0).
This workspace's [runtime lock](../runtime.json) selects that crate's revision.

The [design translator](../crates/octoscript-makepad/src/design.rs) preserves
explicit geometry. It also checks **semantic kit bindings**: mappings from a
component's role, such as an input field, to the actual child widget that fills
that role. For example, `kit_contract` requires an `input` binding to name a
native `TextInput`. This lets a styled component retain real input behavior.

The regular and L0 translators both expand AI text marks before resolving
contrast and emitting source; expanding the marks again does not duplicate
them. The measured-design translator has its own emission path. For L0 events,
`to_makepad_l0_ui_with_events` embeds a host-selected event channel. The host
handles the events, updates data and state, and decides when to render again.

## Runtime limits and native host details

The two evaluation stages have separate limits:

| Stage | Behavior |
| --- | --- |
| Source → `UiNode` through `build` | `eval_checked` allows 2,000,000 instructions. Tree walking allows depth 128 and 65,536 nodes. Evaluation errors, unknown tags, malformed children or exhausted limits fail the whole build with `None`. |
| Widget source → live UI in catalog `kit-host` | Uses `eval_with_append_source` on the main VM, without an explicit instruction budget. |
| Widget source → live UI in preview `beauty-host` | Uses `eval_checked(sm, 2_000_000)` on the main VM. |

The [preview host](../apps/kit-host/src/beauty.rs) adds inspection and replacement
handling around the normal rendering steps. `mount_request` reads the
`BEAUTY_REQUEST` JSON file, prepares a design or L0 card, assigns inspectable IDs,
and replaces `Splash.view`. It retains the old view through replacement drawing,
retires overlays and WebViews, and restores focus, selection and scroll state.

[Native themes](../crates/octoscript-widgets/src/lib.rs) register compiled widgets
and shaders through `script_mod!` modules.
[makepad-d3](../crates/makepad-d3/README.md) and
[makepad-plot](../crates/makepad-plot/README.md) add chart widgets under `mod.d3.*`
and `mod.plot.*`. The node crate also supplies
[AI text marks](../crates/octoscript-node/src/ai.rs),
[numeric widget state](../crates/octoscript-node/src/state.rs) and
[unit conversions](../crates/octoscript-node/src/units.rs).

Keep the source set consistent when building a host.
[runtime.json](../runtime.json) selects Makepad and OctoScript revisions;
[Cargo.toml](../Cargo.toml) declares the crates and sibling overrides. Two
Makepad copies can create incompatible Rust types and separate VM heaps.
Check a consuming app's runtime lock before changing its framework checkout.

## Run the framework examples

The commands below are source-checked recipes. Their build, GUI and device
execution is **unverified** in this documentation pass.

Run from this repository after installing Rust, Cargo and Makepad's native
build prerequisites. First prepare and check the dependencies; preparation uses
Git and network access.

```sh
python3 tools/runtime.py prepare
python3 tools/runtime.py verify --cargo-manifest Cargo.toml
cargo test --release -p octoscript-node -p octoscript-render -p octoscript-makepad
```

To inspect generated widget source without opening a window, run the
[translate example](../crates/octoscript-makepad/examples/translate.rs). It uses
a built-in sample; append `-- /absolute/path/example.octoscript` to read your
own source instead.

```sh
cargo run -p octoscript-makepad --example translate
```

Choose a native viewer below. Each command runs until you close that viewer:

```sh
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
[OctoSense walkthrough](https://github.com/OctoSense-org/OctoSense/blob/61c668279a7c38a0f8056134d8d29e42ed715806/docs/architecture-walkthrough.md).

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
