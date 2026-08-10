# makepad-plot

Matplotlib-style plotting library for **Makepad 2.0 (Splash)** — 32 chart widgets,
all declarable and scriptable from the Splash DSL.

![gallery](resources/gallery.png)

## Widgets

`LinePlot` `StepPlot` `AreaChart` `StemPlot` `LinePlotDual` `Stackplot` `Streamgraph`
`BarPlot` `HistogramChart` `WaterfallChart` `CandlestickChart` `ScatterPlot`
`BubbleChart` `HexbinChart` `PieChart` `DonutChart` `GaugeChart` `FunnelChart`
`PolarPlot` `RadarChart` `BoxPlotChart` `ViolinPlot` `HeatmapChart` (alias `Heatmap`)
`ContourPlot` `QuiverPlot` `Surface3D` `Scatter3D` `Line3D` `Treemap` `SankeyDiagram`
`SubplotGrid`/`SubplotRow`

Every chart renders built-in demo data out of the box, so a bare `LinePlot{}` in a
Splash script shows something immediately. Cartesian charts support pan/zoom
(`interactive: true`); 3D charts support drag-rotate and scroll-zoom.

## Usage

```rust
// app main.rs
pub use makepad_plot;
pub use makepad_plot::makepad_widgets;
use makepad_widgets::*;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    use mod.plot.*

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                body +: {
                    line := LinePlot{ title: "hello" interactive: true legend: LegendPosition.TopRight }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live] ui: WidgetRef,
}

impl MatchEvent for App {}

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        makepad_widgets::script_mod(vm);
        makepad_plot::script_mod(vm);   // registers mod.plot
        self::script_mod(vm)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
```

Feed data at runtime from Splash (e.g. in an `on_click`):

```splash
ui.line.set_title("LinePlot (scripted)")
ui.line.add_series("script", [0 1 2 3 4 5], [0 2 1 3 2.5 4])
ui.bar.set_data(["Q1" "Q2" "Q3" "Q4"], [12 19 8 15])
ui.gauge.set_value(42)
ui.heatmap.set_colormap("Plasma")
```

Each widget's Splash methods (`set_data`, `add_series`, `set_title`, `set_xlim`, …)
are implemented via `Widget::script_call` — see the chart modules under `src/charts/`.

## Demo

```
cargo run    # opens the full widget gallery (examples/plot_demo.rs)
```

## Design & porting notes

See [SPLASH_PORT.md](SPLASH_PORT.md) for the architecture and the Makepad 2.0 engine
findings discovered during the port (draw-order/depth rules for text over vector
geometry, wrap-flow limitations, Splash array parsing, etc.).
