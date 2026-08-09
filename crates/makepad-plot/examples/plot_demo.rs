// makepad-plot demo — all plot widgets declared in the Splash DSL

pub use makepad_plot;
pub use makepad_plot::makepad_widgets;

use makepad_widgets::*;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    use mod.plot.*

    let Row = SubplotRow{
        width: Fill
        height: 330
    }

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.inner_size: vec2(1500, 980)
                body +: {
                    View{
                        width: Fill
                        height: Fill
                        flow: Down
                        header := View{
                            width: Fill
                            height: Fit
                            padding: Inset{top: 8, bottom: 8, left: 12, right: 12}
                            spacing: 12
                            flow: Right
                            align: Align{y: 0.5}
                            title_label := Label{
                                text: "makepad-plot 2.0 — Splash widget gallery"
                                draw_text.text_style.font_size: 16
                            }
                            feed := Button{
                                text: "Feed data via script_call"
                                on_click: ||{
                                    ui.line.set_title("LinePlot (scripted)")
                                    ui.line.add_series("script", [0 1 2 3 4 5 6 7 8 9 10], [0 2 1 3 2.5 4 3.5 5 4 6 5.5])
                                    ui.bar.set_data(["Q1" "Q2" "Q3" "Q4"], [12 19 8 15])
                                    ui.bar.set_title("BarPlot (scripted)")
                                    ui.gauge.set_value(42)
                                    ui.pie.set_data(["Rust" "C++" "Python" "Go"], [45 25 20 10])
                                    ui.heatmap.set_colormap("Plasma")
                                }
                            }
                        }
                        scroller := ScrollYView{
                            width: Fill
                            height: Fill
                            flow: Down
                            spacing: 8
                            padding: Inset{top: 8, bottom: 8, left: 8, right: 8}

                            Row{
                                line := LinePlot{ title: "LinePlot" interactive: true legend: LegendPosition.TopRight }
                                step := StepPlot{ title: "StepPlot" }
                                area := AreaChart{ title: "AreaChart" }
                            }
                            Row{
                                stem := StemPlot{ title: "StemPlot" }
                                dual := LinePlotDual{ title: "LinePlotDual" }
                                stackplot := Stackplot{ title: "Stackplot" }
                            }
                            Row{
                                stream := Streamgraph{ title: "Streamgraph" }
                                bar := BarPlot{ title: "BarPlot" }
                                hist := HistogramChart{ title: "HistogramChart" }
                            }
                            Row{
                                waterfall := WaterfallChart{ title: "WaterfallChart" }
                                candle := CandlestickChart{ title: "CandlestickChart" }
                                scatter := ScatterPlot{ title: "ScatterPlot" interactive: true }
                            }
                            Row{
                                bubble := BubbleChart{ title: "BubbleChart" }
                                hexbin := HexbinChart{ title: "HexbinChart" }
                                pie := PieChart{ title: "PieChart" }
                            }
                            Row{
                                donut := DonutChart{ title: "DonutChart" }
                                gauge := GaugeChart{ title: "GaugeChart" }
                                funnel := FunnelChart{ title: "FunnelChart" }
                            }
                            Row{
                                polar := PolarPlot{ title: "PolarPlot" }
                                radar := RadarChart{ title: "RadarChart" }
                                boxplot := BoxPlotChart{ title: "BoxPlotChart" }
                            }
                            Row{
                                violin := ViolinPlot{ title: "ViolinPlot" }
                                heatmap := HeatmapChart{ title: "HeatmapChart" }
                                contour := ContourPlot{ title: "ContourPlot" }
                            }
                            Row{
                                quiver := QuiverPlot{ title: "QuiverPlot" }
                                surface3d := Surface3D{ title: "Surface3D" }
                                scatter3d := Scatter3D{ title: "Scatter3D" }
                            }
                            Row{
                                line3d := Line3D{ title: "Line3D" }
                                treemap := Treemap{ title: "Treemap" }
                                sankey := SankeyDiagram{ title: "SankeyDiagram" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl MatchEvent for App {
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions) {}
}

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        makepad_widgets::script_mod(vm);
        makepad_plot::script_mod(vm);
        self::script_mod(vm)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
