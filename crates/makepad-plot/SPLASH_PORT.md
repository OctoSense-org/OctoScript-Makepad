# Makepad 2.0 "Splash" Update — Design & Port Plan

## Goal

Update makepad-plot from Makepad 1.0 (`live_design!` DSL, wyeworks rev `53b2e5c84`)
to **Makepad 2.0** and make **every plot widget usable in Splash fashion** — i.e.
declarable and scriptable from the Splash DSL (`script_mod!` / ScriptVm), exactly like
the built-in Makepad 2.0 widgets (`Button`, `Label`, `CandlestickChart`, …).

## What "Splash fashion" means (from makepad dev branch)

Makepad 2.0 replaces `live_design!` with the **Splash script system**:

- Widgets are Rust structs deriving `#[derive(Script, ScriptHook, Widget)]` with
  `#[source] source: ScriptObjectRef`, `#[walk]`, `#[layout]`, `#[live]` props,
  `#[rust]` state and `#[deref]` composition.
- Each widget module registers itself with the VM in a `script_mod!` block:

  ```
  script_mod! {
      use mod.prelude.widgets_internal.*
      use mod.widgets.*
      mod.plot.LinePlotBase = #(LinePlot::register_widget(vm))
      mod.plot.LinePlot = set_type_default() do mod.plot.LinePlotBase{ width: Fill height: Fill }
  }
  ```

- Splash scripts then declare widgets declaratively (`line := LinePlot{ title: "..." }`)
  and call methods at runtime (`ui.line.set_data([...])`) which are dispatched to the
  widget's `Widget::script_call(vm, method, args)` implementation.
- Reference implementation: `~/home/makepad/widgets/src/chart.rs` (ChartView +
  CandlestickChart/LineChart/... registered in `mod.widgets`), app skeleton:
  `~/home/makepad/examples/charts/src/main.rs`.

## Architecture

### Dependency

```toml
makepad-widgets = { path = "../makepad/widgets" }   # Makepad 2.0 (dev branch)
```

The old `math_widget` dependency is dropped (Makepad 1.0 only); LaTeX annotations
render as plain text in this port.

### Rendering layer

Old 1.0 `elements.rs` custom shaders (`DrawPlotLine`, `DrawPieSlice`, …) are replaced
by Makepad 2.0's **`DrawVector`** immediate-mode vector canvas
(`move_to/line_to/bezier_to/close/rect/circle/ellipse`, `stroke(w)/fill()`,
`set_color`, gradients) plus `DrawColor` for rects and `DrawText.draw_abs` for text.

### Module layout

```
src/
├── lib.rs            # modules, re-exports, pub fn script_mod(vm) → creates mod.plot
├── types.rs          # Series, ScaleType, LineStyle, MarkerStyle, StepStyle,
│                     # LegendPosition, Colormap, annotations… (ported ~verbatim)
├── plot_view.rs      # PlotView: core cartesian plot widget
└── charts/
    ├── line.rs       # LinePlot, StepPlot, AreaChart, StemPlot, LinePlotDual,
    │                 # Stackplot, Streamgraph
    ├── bar.rs        # BarPlot, HistogramChart, WaterfallChart, CandlestickChart
    ├── scatter.rs    # ScatterPlot, BubbleChart, HexbinChart
    ├── circular.rs   # PieChart, DonutChart, GaugeChart, FunnelChart, PolarPlot, RadarChart
    ├── stats.rs      # BoxPlotChart, ViolinPlot
    ├── field.rs      # HeatmapChart, ContourPlot, QuiverPlot
    ├── three_d.rs    # Surface3D, Scatter3D, Line3D
    ├── hierarchy.rs  # Treemap, SankeyDiagram
    └── grid.rs       # SubplotGrid
```

### PlotView core (`plot_view.rs`)

One shared base widget (used via `#[deref]` by all cartesian charts), owning:

- `DrawColor draw_bg`, `DrawColor draw_grid`, `DrawVector draw_vector`, `DrawText draw_text`
- plot margins, `title`, `xlabel`, `ylabel` (`#[live]` strings)
- viewport (x/y min/max) + `ScaleType` per axis; `data_to_px` / `px_to_data`
- nice-tick generation (via `ScaleType::generate_ticks` / `format_tick`), grid + axis
  labels + border
- legend drawing (position enum, swatch + label)
- pan (drag) & zoom (scroll) interaction, matching `ChartView`
- draw helpers over `DrawVector`: styled lines (solid/dashed/dotted/dash-dot via dash
  marching), 8 marker styles, polygon fill, bars, polyline arcs / pie slices
  (line segments every ~4°), arrows

### Per-chart pattern

```rust
#[derive(Script, ScriptHook, Widget)]
pub struct LinePlot {
    #[source] source: ScriptObjectRef,
    #[deref]  plot_view: PlotView,
    #[rust]   series: Vec<Series>,
    ...
}
impl Widget for LinePlot {
    fn handle_event(...) { self.plot_view.handle_event(...) }
    fn draw_walk(...)    { /* fit-viewport, grid, draw series, legend */ }
    fn script_call(...)  { /* set_data, add_series, set_title, … */ }
}
```

- **Default demo data**: when a chart has no data it self-populates with generated
  demo data, so a bare `LinePlot{}` in a Splash script renders immediately
  (same convention as makepad's `chart.rs`).
- **Splash-callable methods** via `script_call` (dispatched from
  `ui.<id>.method(...)`): every chart gets at least `set_title(t)`,
  `set_xlabel(t)` / `set_ylabel(t)` where meaningful, plus data setters, e.g.
  - `LinePlot`: `set_data(xs, ys)`, `add_series(label, xs, ys)`, `clear()`
  - `BarPlot`: `set_data(labels, values)`
  - `PieChart`/`DonutChart`: `set_data(labels, values)`
  - `HeatmapChart`: `set_data(rows)` (array of arrays)
  - `Scatter3D`/`Surface3D`: numeric array setters
  - array args parsed with `vm.bx.heap.vec_len` / `vec_value(...).as_f64()`
- **Registration**: each `charts/*.rs` has a `script_mod!` block registering
  `mod.plot.<Name>Base` + prototype `mod.plot.<Name>` with defaults.

### VM wiring

`lib.rs`:

```rust
pub fn script_mod(vm: &mut ScriptVm) {
    vm.bx.heap.new_module(id!(plot));
    crate::plot_view::script_mod(vm);
    crate::charts::line::script_mod(vm);
    // … all chart modules
}
```

Apps opt in with:

```rust
impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        makepad_widgets::script_mod(vm);
        makepad_plot::script_mod(vm);
        self::script_mod(vm)
    }
    ...
}
```

and in Splash: `use mod.plot.*`.

### Demo (`examples/plot_demo.rs`)

Splash-declared demo app (the `plot_demo` bin): a window with a scrollable grid
showing **all ported chart widgets**, each rendering its built-in demo data, with
pan/zoom on cartesian charts — the whole UI written in the Splash DSL.

## Old → New mapping cheat-sheet

| Makepad 1.0 (old code)            | Makepad 2.0 (this port)                     |
|-----------------------------------|---------------------------------------------|
| `#[derive(Live, LiveHook, Widget)]` | `#[derive(Script, ScriptHook, Widget)]` + `#[source]` |
| `live_design!{ pub X = {{X}} {…} }` | `script_mod!{ mod.plot.XBase = #(X::register_widget(vm)) … }` |
| `Vec4` / `vec4(…)`                | `Vec4f` / `vec4f(…)` (script-side colors)    |
| `DrawPlotLine::draw_line_styled`  | `PlotView::draw_line_styled` (DrawVector dash marching) |
| `DrawPlotPoint::draw_marker`      | `PlotView::draw_marker` (DrawVector shapes) |
| `DrawPieSlice/DrawArc` shaders    | polyline arc fans on DrawVector             |
| `PlotLabel` / `Math` widgets      | `DrawText::draw_abs` (+ font_size via `text_style`) |
| widget `Ref` rust setter APIs     | kept, **plus** Splash `script_call` methods |

## Engine findings from the port (verified empirically)

- **`#[deref]` flattens** the inner widget's script fields onto the outer type:
  prototypes must set `plot_margin` / `draw_bg` / etc. at top level, never nested
  under the deref field name.
- **`draw_text` prototypes need `text_style: theme.font_regular{}`** or nothing renders
  ("empty font family").
- **Text glyph quads write depth over their whole quad**, so any DrawVector geometry
  flushed after a text call is depth-rejected underneath glyphs (background-colored
  holes in fills). PlotView therefore queues all text and draws it after the vector
  flush in `end()`.
- **`Flow::Right{wrap:true}` breaks DrawVector widgets**: the deferred row-wrap align
  pass shifts instance rects but not vector vertex geometry, so the first widget of
  each wrapped row loses its vector layer. `SubplotGrid` uses explicit
  `SubplotRow`s instead of a wrapping flow for this reason.
- **DrawVector geometry is not clipped** by the enclosing turtle, so charts must keep
  their geometry inside their own rect (3D charts clamp zoom to 0.5–1.6).
- **Splash array literals are typed arrays, not objects**: parse method args with
  `value.as_array()` + `heap.array_len/array_index_unchecked` (with an object-vec
  fallback), and read numbers with `as_number()` — `as_f64()` returns `None` for
  integer-encoded values.

## Non-goals (this pass)

- LaTeX/math rendering of annotations (plain text instead)
- GPU-gradient parity for every 1.0 shader effect (solid/per-vertex color instead;
  DrawVector gradients can be layered in later)
- The old 5.6k-line Rust demo (`plot_demo.rs`) is replaced by the Splash demo.
