// Makepad Plot Zoo - Matplotlib-style plotting gallery

use makepad_widgets::*;
use makepad_plot::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use makepad_plot::plot::LinePlot;
    use makepad_plot::plot::BarPlot;
    use makepad_plot::plot::ScatterPlot;
    use makepad_plot::plot::PieChart;
    use makepad_plot::plot::HistogramChart;
    use makepad_plot::plot::BoxPlotChart;
    use makepad_plot::plot::HeatmapChart;
    use makepad_plot::plot::StemPlot;
    use makepad_plot::plot::ViolinPlot;
    use makepad_plot::plot::PolarPlot;
    use makepad_plot::plot::ContourPlot;
    use makepad_plot::plot::QuiverPlot;
    use makepad_plot::plot::Surface3D;
    use makepad_plot::plot::Scatter3D;
    use makepad_plot::plot::Line3D;
    use makepad_plot::plot::SubplotGrid;
    use makepad_plot::plot::LinePlotDual;
    use makepad_plot::plot::CandlestickChart;
    use makepad_plot::plot::RadarChart;
    use makepad_plot::plot::WaterfallChart;
    use makepad_plot::plot::GaugeChart;
    use makepad_plot::plot::FunnelChart;
    use makepad_plot::plot::Heatmap;
    use makepad_plot::plot::Treemap;
    use makepad_plot::plot::BubbleChart;
    use makepad_plot::plot::DonutChart;
    use makepad_plot::plot::AreaChart;
    use makepad_plot::plot::StepPlot;
    use makepad_plot::plot::Stackplot;
    use makepad_plot::plot::HexbinChart;
    use makepad_plot::plot::Streamgraph;
    use makepad_plot::plot::SankeyDiagram;

    // Override theme fonts with Manrope
    THEME_FONT_REGULAR = {
        font_family: {
            latin = font("crate://self/resources/Manrope-Regular.ttf", 0.0, 0.0),
        }
        line_spacing: 1.2
    }

    THEME_FONT_BOLD = {
        font_family: {
            latin = font("crate://self/resources/Manrope-Regular.ttf", 0.0, 0.0),
        }
        line_spacing: 1.2
    }

    // Clickable chart card component
    ChartCard = <RoundedView> {
        width: Fill,
        height: 280,
        margin: 8,
        padding: 12,
        cursor: Hand,
        show_bg: true,
        draw_bg: {
            color: #fafafa,
            border_radius: 8.0
        }
    }

    // Detail page chart card (larger)
    DetailChartCard = <RoundedView> {
        width: Fill,
        height: 350,
        margin: 8,
        padding: 12,
        show_bg: true,
        draw_bg: {
            color: #fafafa,
            border_radius: 8.0
        }
    }

    // Back button style
    BackButton = <Button> {
        text: "← Back to Gallery"
        draw_text: {
            text_style: <THEME_FONT_BOLD> { font_size: 12.0 }
            color: #fff
        }
        draw_bg: {
            color: #4A90D9
        }
        width: 150,
        height: 36,
        margin: { bottom: 16 }
    }

    // Section title
    SectionTitle = <Label> {
        draw_text: {
            text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
            color: #333
        }
        margin: { top: 16, bottom: 8 }
    }

    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                window: { title: "Makepad Plot Zoo" },
                pass: { clear_color: #f0f0f0 }

                body = <View> {
                    flow: Overlay,  // Stack pages, show one at a time

                    // ==================== MAIN GALLERY PAGE ====================
                    main_page = <ScrollXYView> {
                        visible: true,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        <Label> {
                            text: "Makepad Plot Zoo"
                            draw_text: {
                                text_style: <THEME_FONT_BOLD> { font_size: 24.0 }
                                color: #333
                            }
                        }
                        <Label> {
                            text: "Click any chart to see variations and examples"
                            draw_text: {
                                text_style: <THEME_FONT_BOLD> { font_size: 12.0 }
                                color: #666
                            }
                            margin: { bottom: 16 }
                        }

                        // Row 1: Line, Math, Scatter
                        <View> {
                            flow: Right,
                            spacing: 16,
                            height: 300,

                            line_card = <ChartCard> {
                                <Label> {
                                    text: "Line Chart"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_line_plot = <LinePlot> { height: Fill, width: Fill }
                            }

                            math_card = <ChartCard> {
                                <Label> {
                                    text: "Mathematical Functions"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_math_plot = <LinePlot> { height: Fill, width: Fill }
                            }

                            scatter_card = <ChartCard> {
                                <Label> {
                                    text: "Scatter Plot"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_scatter_plot = <ScatterPlot> { height: Fill, width: Fill }
                            }
                        }

                        // Row 2: Pie, Histogram, Box Plot
                        <View> {
                            flow: Right,
                            spacing: 16,
                            height: 300,

                            pie_card = <ChartCard> {
                                <Label> {
                                    text: "Pie Chart"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_pie_chart = <PieChart> { height: Fill, width: Fill }
                            }

                            histogram_card = <ChartCard> {
                                <Label> {
                                    text: "Histogram"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_histogram = <HistogramChart> { height: Fill, width: Fill }
                            }

                            boxplot_card = <ChartCard> {
                                <Label> {
                                    text: "Box Plot"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_boxplot = <BoxPlotChart> { height: Fill, width: Fill }
                            }
                        }

                        // Row 3: Heatmap, Bar Chart, Stock Time Series
                        <View> {
                            flow: Right,
                            spacing: 16,
                            height: 300,

                            heatmap_card = <ChartCard> {
                                <Label> {
                                    text: "Heatmap"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_heatmap = <HeatmapChart> { height: Fill, width: Fill }
                            }

                            bar_card = <ChartCard> {
                                <Label> {
                                    text: "Bar Chart"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_bar_plot = <BarPlot> { height: Fill, width: Fill }
                            }

                            timeseries_card = <ChartCard> {
                                <Label> {
                                    text: "Time Series / Stock"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_time_plot = <LinePlot> { height: Fill, width: Fill }
                            }
                        }

                        // Row 4: Phase 2 - Violin, Polar, Contour, Quiver
                        <View> {
                            flow: Right,
                            spacing: 16,
                            height: 300,

                            violin_card = <ChartCard> {
                                <Label> {
                                    text: "Violin Plot (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_violin = <ViolinPlot> { height: Fill, width: Fill }
                            }

                            polar_card = <ChartCard> {
                                <Label> {
                                    text: "Polar Plot (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_polar = <PolarPlot> { height: Fill, width: Fill }
                            }

                            contour_card = <ChartCard> {
                                <Label> {
                                    text: "Contour Plot (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_contour = <ContourPlot> { height: Fill, width: Fill }
                            }

                            quiver_card = <ChartCard> {
                                <Label> {
                                    text: "Quiver Plot (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_quiver = <QuiverPlot> { height: Fill, width: Fill }
                            }
                        }

                        // Row 5: Phase 3 - 3D Charts
                        <View> {
                            flow: Right,
                            spacing: 16,
                            height: 300,

                            surface3d_card = <ChartCard> {
                                <Label> {
                                    text: "3D Surface (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_surface3d = <Surface3D> { height: Fill, width: Fill }
                            }

                            scatter3d_card = <ChartCard> {
                                <Label> {
                                    text: "3D Scatter (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_scatter3d = <Scatter3D> { height: Fill, width: Fill }
                            }

                            line3d_card = <ChartCard> {
                                <Label> {
                                    text: "3D Line (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_line3d = <Line3D> { height: Fill, width: Fill }
                            }
                        }

                        // Row 6: Phase 4 - Layout & Composition
                        <View> {
                            flow: Right,
                            spacing: 16,
                            height: 300,

                            subplot_card = <ChartCard> {
                                <Label> {
                                    text: "Subplot Grid (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_subplot = <SubplotGrid> {
                                    height: Fill, width: Fill,
                                    flow: Down,
                                    spacing: 8,
                                    <View> {
                                        flow: Right, spacing: 8, width: Fill, height: 115,
                                        subplot_line1 = <LinePlot> { width: 150, height: Fill }
                                        subplot_bar1 = <BarPlot> { width: 150, height: Fill }
                                    }
                                    <View> {
                                        flow: Right, spacing: 8, width: Fill, height: 115,
                                        subplot_scatter1 = <ScatterPlot> { width: 150, height: Fill }
                                        subplot_pie1 = <PieChart> { width: 150, height: Fill }
                                    }
                                }
                            }

                            twinx_card = <ChartCard> {
                                <Label> {
                                    text: "Dual Y-Axis (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_twinx = <LinePlotDual> { height: Fill, width: Fill }
                            }
                        }

                        // Row 7: Phase 5 - Advanced Scales
                        <View> {
                            flow: Right,
                            spacing: 16,
                            height: 300,

                            scales_card = <ChartCard> {
                                <Label> {
                                    text: "Log & SymLog Scales (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_log_scale = <LinePlot> { height: Fill, width: Fill }
                            }

                            time_scale_card = <ChartCard> {
                                <Label> {
                                    text: "Time Scale (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_time_scale = <LinePlot> { height: Fill, width: Fill }
                            }
                        }

                        // Row 8: Phase 7 - Colormaps
                        <View> {
                            flow: Right,
                            spacing: 16,
                            height: 300,

                            colormap_card = <ChartCard> {
                                <Label> {
                                    text: "Colormaps & Normalization (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_colormap_heatmap = <HeatmapChart> { height: Fill, width: Fill }
                            }

                            normalizer_card = <ChartCard> {
                                <Label> {
                                    text: "Log Normalization (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_lognorm_heatmap = <HeatmapChart> { height: Fill, width: Fill }
                            }
                        }

                        // Row 9: Phase 8 - Annotations
                        <View> {
                            flow: Right,
                            spacing: 16,
                            height: 300,

                            annotations_card = <ChartCard> {
                                <Label> {
                                    text: "Annotations & References (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_annotations = <LinePlot> { height: Fill, width: Fill }
                            }
                        }

                        // Row 10: Phase 9 - Error Bars & Fill Between
                        <View> {
                            flow: Right,
                            spacing: 16,
                            height: 300,

                            errorbars_card = <ChartCard> {
                                <Label> {
                                    text: "Error Bars (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_errorbars = <LinePlot> { height: Fill, width: Fill }
                            }

                            fillbetween_card = <ChartCard> {
                                <Label> {
                                    text: "Fill Between / Area (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_fillbetween = <LinePlot> { height: Fill, width: Fill }
                            }
                        }

                        // Row 11: Phase 10 - Financial & Comparison Charts
                        <View> {
                            flow: Right,
                            spacing: 16,
                            height: 300,

                            stacked_card = <ChartCard> {
                                <Label> {
                                    text: "Stacked/Grouped Bars (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_stacked_bars = <BarPlot> { height: Fill, width: Fill }
                            }

                            candlestick_card = <ChartCard> {
                                <Label> {
                                    text: "Candlestick Chart (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_candlestick = <CandlestickChart> { height: Fill, width: Fill }
                            }

                            radar_card = <ChartCard> {
                                <Label> {
                                    text: "Radar/Spider Chart (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_radar = <RadarChart> { height: Fill, width: Fill }
                            }
                        }

                        // Row 12: Phase 11 - Business Charts
                        <View> {
                            flow: Right,
                            spacing: 16,
                            height: 300,

                            waterfall_card = <ChartCard> {
                                <Label> {
                                    text: "Waterfall Chart (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_waterfall = <WaterfallChart> { height: Fill, width: Fill }
                            }

                            gauge_card = <ChartCard> {
                                <Label> {
                                    text: "Gauge Chart (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_gauge = <GaugeChart> { height: Fill, width: Fill }
                            }

                            funnel_card = <ChartCard> {
                                <Label> {
                                    text: "Funnel Chart (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_funnel = <FunnelChart> { height: Fill, width: Fill }
                            }
                        }

                        // Row 13: Heatmap, Treemap, Bubble Chart
                        <View> {
                            flow: Right, spacing: 16, height: 280, padding: { left: 24.0, right: 24.0 },
                            heatmap_card = <ChartCard> {
                                <Label> {
                                    text: "Heatmap (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_heatmap_new = <Heatmap> { height: Fill, width: Fill }
                            }

                            treemap_card = <ChartCard> {
                                <Label> {
                                    text: "Treemap (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_treemap = <Treemap> { height: Fill, width: Fill }
                            }

                            bubble_card = <ChartCard> {
                                <Label> {
                                    text: "Bubble Chart (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_bubble = <BubbleChart> { height: Fill, width: Fill }
                            }
                        }

                        // Row 14: Donut, Area, Step
                        <View> {
                            flow: Right, spacing: 16, height: 280, padding: { left: 24.0, right: 24.0 },
                            donut_card = <ChartCard> {
                                <Label> {
                                    text: "Donut Chart (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_donut = <DonutChart> { height: Fill, width: Fill }
                            }

                            area_card = <ChartCard> {
                                <Label> {
                                    text: "Area Chart (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_area = <AreaChart> { height: Fill, width: Fill }
                            }

                            step_card = <ChartCard> {
                                <Label> {
                                    text: "Step Plot (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_step = <StepPlot> { height: Fill, width: Fill }
                            }
                        }

                        // Row 15: NEW from makepad-d3 - Stackplot, Hexbin, Streamgraph, Sankey
                        <View> {
                            flow: Right, spacing: 16, height: 280, padding: { left: 24.0, right: 24.0 },
                            stackplot_card = <ChartCard> {
                                <Label> {
                                    text: "Stackplot (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_stackplot = <Stackplot> { height: Fill, width: Fill }
                            }

                            hexbin_card = <ChartCard> {
                                <Label> {
                                    text: "Hexbin Chart (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_hexbin = <HexbinChart> { height: Fill, width: Fill }
                            }

                            streamgraph_card = <ChartCard> {
                                <Label> {
                                    text: "Streamgraph (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_streamgraph = <Streamgraph> { height: Fill, width: Fill }
                            }

                            sankey_card = <ChartCard> {
                                <Label> {
                                    text: "Sankey Diagram (NEW!)"
                                    draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 12.0 }, color: #333 }
                                }
                                main_sankey = <SankeyDiagram> { height: Fill, width: Fill }
                            }
                        }
                    }

                    // ==================== LINE CHART DETAIL PAGE ====================
                    line_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_line = <BackButton> {}

                        <Label> {
                            text: "Line Chart Variations"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <SectionTitle> { text: "Line Styles (NEW!)" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Dashed Lines", draw_text: { color: #333 } }
                                detail_line_dashed = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Dotted Lines", draw_text: { color: #333 } }
                                detail_line_dotted = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Mixed Styles", draw_text: { color: #333 } }
                                detail_line_mixed = <LinePlot> {}
                            }
                        }

                        <SectionTitle> { text: "Marker Styles (NEW!)" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Marker Gallery", draw_text: { color: #333 } }
                                detail_line_markers = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Line + Markers", draw_text: { color: #333 } }
                                detail_line_with_markers = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Scatter with Markers", draw_text: { color: #333 } }
                                detail_line_scatter_markers = <LinePlot> {}
                            }
                        }

                        <SectionTitle> { text: "Step Plots (NEW!)" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Step Pre", draw_text: { color: #333 } }
                                detail_line_step_pre = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Step Post", draw_text: { color: #333 } }
                                detail_line_step_post = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Step Mid", draw_text: { color: #333 } }
                                detail_line_step_mid = <LinePlot> {}
                            }
                        }

                        <SectionTitle> { text: "Error Bars (NEW!)" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Y Error Bars", draw_text: { color: #333 } }
                                detail_line_yerr = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "X and Y Error", draw_text: { color: #333 } }
                                detail_line_xyerr = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Asymmetric Error", draw_text: { color: #333 } }
                                detail_line_asymerr = <LinePlot> {}
                            }
                        }

                        <SectionTitle> { text: "Reference Lines & Spans (NEW!)" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "axvline/axhline", draw_text: { color: #333 } }
                                detail_line_reflines = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "axvspan/axhspan", draw_text: { color: #333 } }
                                detail_line_spans = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Combined", draw_text: { color: #333 } }
                                detail_line_combined = <LinePlot> {}
                            }
                        }

                        <SectionTitle> { text: "Area Charts (fill_between)" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Filled Area", draw_text: { color: #333 } }
                                detail_line_area = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Stacked Area", draw_text: { color: #333 } }
                                detail_line_stacked = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Confidence Interval", draw_text: { color: #333 } }
                                detail_line_confidence = <LinePlot> {}
                            }
                        }
                    }

                    // ==================== MATH DETAIL PAGE ====================
                    math_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_math = <BackButton> {}

                        <Label> {
                            text: "Mathematical Functions & Calculus"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <SectionTitle> { text: "Integral Visualization" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Definite Integral", draw_text: { color: #333 } }
                                detail_math_integral = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Area Under Curve", draw_text: { color: #333 } }
                                detail_math_area = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Riemann Sum", draw_text: { color: #333 } }
                                detail_math_riemann = <LinePlot> {}
                            }
                        }

                        <SectionTitle> { text: "Trigonometric Functions" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "sin(x), cos(x)", draw_text: { color: #333 } }
                                detail_math_trig = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "tan(x)", draw_text: { color: #333 } }
                                detail_math_tan = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Parametric: sin/cos", draw_text: { color: #333 } }
                                detail_math_parametric = <LinePlot> {}
                            }
                        }

                        <SectionTitle> { text: "Derivatives & Slopes" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "f(x) and f'(x)", draw_text: { color: #333 } }
                                detail_math_derivative = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Tangent Line", draw_text: { color: #333 } }
                                detail_math_tangent = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Second Derivative", draw_text: { color: #333 } }
                                detail_math_second = <LinePlot> {}
                            }
                        }

                        <SectionTitle> { text: "Special Functions" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Exponential e^x", draw_text: { color: #333 } }
                                detail_math_exp = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Logarithm ln(x)", draw_text: { color: #333 } }
                                detail_math_log = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Gaussian/Normal", draw_text: { color: #333 } }
                                detail_math_gaussian = <LinePlot> {}
                            }
                        }
                    }

                    // ==================== BAR CHART DETAIL PAGE ====================
                    bar_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_bar = <BackButton> {}

                        <Label> {
                            text: "Bar Chart Variations"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <SectionTitle> { text: "Basic Bar Charts" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Vertical Bars", draw_text: { color: #333 } }
                                detail_bar_vertical = <BarPlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Horizontal Bars (NEW!)", draw_text: { color: #333 } }
                                detail_bar_horizontal = <BarPlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Colored Bars", draw_text: { color: #333 } }
                                detail_bar_colored = <BarPlot> {}
                            }
                        }

                        <SectionTitle> { text: "Grouped & Stacked (NEW!)" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Grouped Bars", draw_text: { color: #333 } }
                                detail_bar_grouped = <BarPlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Stacked Bars", draw_text: { color: #333 } }
                                detail_bar_stacked = <BarPlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Horizontal Stacked", draw_text: { color: #333 } }
                                detail_bar_hstacked = <BarPlot> {}
                            }
                        }

                        <SectionTitle> { text: "Stem Plot (Lollipop)" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Basic Stem (NEW!)", draw_text: { color: #333 } }
                                detail_stem_basic = <StemPlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Stem with Markers", draw_text: { color: #333 } }
                                detail_stem_markers = <StemPlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Multi-Series Stem", draw_text: { color: #333 } }
                                detail_stem_multi = <StemPlot> {}
                            }
                        }
                    }

                    // ==================== SCATTER DETAIL PAGE ====================
                    scatter_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_scatter = <BackButton> {}

                        <Label> {
                            text: "Scatter Plot Variations"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Basic Scatter", draw_text: { color: #333 } }
                                detail_scatter_basic = <ScatterPlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Correlation", draw_text: { color: #333 } }
                                detail_scatter_correlation = <ScatterPlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Multi-Series", draw_text: { color: #333 } }
                                detail_scatter_multi = <ScatterPlot> {}
                            }
                        }
                    }

                    // ==================== PIE DETAIL PAGE ====================
                    pie_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_pie = <BackButton> {}

                        <Label> {
                            text: "Pie Chart Variations"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Basic Pie", draw_text: { color: #333 } }
                                detail_pie_basic = <PieChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Doughnut", draw_text: { color: #333 } }
                                detail_pie_doughnut = <PieChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "With Percentages", draw_text: { color: #333 } }
                                detail_pie_percentages = <PieChart> {}
                            }
                        }
                    }

                    // ==================== STATISTICS DETAIL PAGE ====================
                    stats_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_stats = <BackButton> {}

                        <Label> {
                            text: "Statistical Plots"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Histogram", draw_text: { color: #333 } }
                                detail_stats_histogram = <HistogramChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Box Plot", draw_text: { color: #333 } }
                                detail_stats_boxplot = <BoxPlotChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Correlation Heatmap", draw_text: { color: #333 } }
                                detail_stats_heatmap = <HeatmapChart> {}
                            }
                        }
                    }

                    // ==================== TIME SERIES DETAIL PAGE ====================
                    timeseries_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_time = <BackButton> {}

                        <Label> {
                            text: "Time Series & Financial"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Stock Prices", draw_text: { color: #333 } }
                                detail_time_stock = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Log Scale", draw_text: { color: #333 } }
                                detail_time_log = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Multiple Assets", draw_text: { color: #333 } }
                                detail_time_multi = <LinePlot> {}
                            }
                        }
                    }

                    // ==================== VIOLIN PLOT DETAIL PAGE ====================
                    violin_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_violin = <BackButton> {}

                        <Label> {
                            text: "Violin Plot Variations"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Basic Violin", draw_text: { color: #333 } }
                                detail_violin_basic = <ViolinPlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "With Box Plot", draw_text: { color: #333 } }
                                detail_violin_box = <ViolinPlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Multi-Group", draw_text: { color: #333 } }
                                detail_violin_multi = <ViolinPlot> {}
                            }
                        }
                    }

                    // ==================== POLAR PLOT DETAIL PAGE ====================
                    polar_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_polar = <BackButton> {}

                        <Label> {
                            text: "Polar Plot Variations"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Radar Chart", draw_text: { color: #333 } }
                                detail_polar_radar = <PolarPlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Rose Diagram", draw_text: { color: #333 } }
                                detail_polar_rose = <PolarPlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Spiral", draw_text: { color: #333 } }
                                detail_polar_spiral = <PolarPlot> {}
                            }
                        }
                    }

                    // ==================== CONTOUR PLOT DETAIL PAGE ====================
                    contour_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_contour = <BackButton> {}

                        <Label> {
                            text: "Contour Plot Variations"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Contour Lines", draw_text: { color: #333 } }
                                detail_contour_lines = <ContourPlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Filled Contour", draw_text: { color: #333 } }
                                detail_contour_filled = <ContourPlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Saddle Point", draw_text: { color: #333 } }
                                detail_contour_saddle = <ContourPlot> {}
                            }
                        }
                    }

                    // ==================== QUIVER PLOT DETAIL PAGE ====================
                    quiver_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_quiver = <BackButton> {}

                        <Label> {
                            text: "Quiver Plot Variations"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Rotation Field", draw_text: { color: #333 } }
                                detail_quiver_rotation = <QuiverPlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Gradient Field", draw_text: { color: #333 } }
                                detail_quiver_gradient = <QuiverPlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Sink/Source", draw_text: { color: #333 } }
                                detail_quiver_sink = <QuiverPlot> {}
                            }
                        }
                    }

                    // ==================== 3D CHARTS DETAIL PAGE ====================
                    // ==================== SURFACE3D DETAIL PAGE ====================
                    surface3d_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_surface3d = <BackButton> {}

                        <Label> {
                            text: "3D Surface Charts"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Peaks Function", draw_text: { color: #333 } }
                                detail_surface_peaks = <Surface3D> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Saddle Surface", draw_text: { color: #333 } }
                                detail_surface_saddle = <Surface3D> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Ripple Surface", draw_text: { color: #333 } }
                                detail_surface_ripple = <Surface3D> {}
                            }
                        }

                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Gaussian Hills", draw_text: { color: #333 } }
                                detail_surface_gaussian = <Surface3D> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Sinc Function", draw_text: { color: #333 } }
                                detail_surface_sinc = <Surface3D> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Wireframe View", draw_text: { color: #333 } }
                                detail_surface_wireframe = <Surface3D> {}
                            }
                        }
                    }

                    // ==================== SCATTER3D DETAIL PAGE ====================
                    scatter3d_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_scatter3d = <BackButton> {}

                        <Label> {
                            text: "3D Scatter Charts"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Random Clusters", draw_text: { color: #333 } }
                                detail_scatter3d_random = <Scatter3D> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Helix Pattern", draw_text: { color: #333 } }
                                detail_scatter3d_helix = <Scatter3D> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Spherical Shell", draw_text: { color: #333 } }
                                detail_scatter3d_sphere = <Scatter3D> {}
                            }
                        }

                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Gaussian Cloud", draw_text: { color: #333 } }
                                detail_scatter3d_gaussian = <Scatter3D> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Spiral Galaxy", draw_text: { color: #333 } }
                                detail_scatter3d_spiral = <Scatter3D> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Toroidal Distribution", draw_text: { color: #333 } }
                                detail_scatter3d_torus = <Scatter3D> {}
                            }
                        }
                    }

                    // ==================== LINE3D DETAIL PAGE ====================
                    line3d_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_line3d = <BackButton> {}

                        <Label> {
                            text: "3D Line Charts"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Lorenz Attractor", draw_text: { color: #333 } }
                                detail_line3d_lorenz = <Line3D> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Trefoil Knot", draw_text: { color: #333 } }
                                detail_line3d_trefoil = <Line3D> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Lissajous Curve", draw_text: { color: #333 } }
                                detail_line3d_lissajous = <Line3D> {}
                            }
                        }

                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Spring Helix", draw_text: { color: #333 } }
                                detail_line3d_spring = <Line3D> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Figure-8 Knot", draw_text: { color: #333 } }
                                detail_line3d_figure8 = <Line3D> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Torus Knot", draw_text: { color: #333 } }
                                detail_line3d_torus = <Line3D> {}
                            }
                        }
                    }

                    // ==================== SUBPLOT DETAIL PAGE ====================
                    subplot_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_subplot = <BackButton> {}

                        <Label> {
                            text: "Subplot Grid Layouts"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        // 2x2 Subplot Grid
                        <View> {
                            flow: Right, spacing: 16, height: 620,
                            <RoundedView> {
                                width: Fill, height: Fill,
                                padding: 12,
                                show_bg: true,
                                draw_bg: { color: #fafafa, border_radius: 8.0 }
                                flow: Down,
                                <Label> { text: "2x2 Grid: Line, Scatter, Bar, Area", draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 14.0 }, color: #333 } }
                                <SubplotGrid> {
                                    height: Fill, width: Fill,
                                    flow: Down,
                                    spacing: 20,
                                    <View> {
                                        flow: Right, spacing: 20, width: Fill, height: 280,
                                        detail_subplot_line = <LinePlot> { width: 400, height: Fill }
                                        detail_subplot_scatter = <ScatterPlot> { width: 400, height: Fill }
                                    }
                                    <View> {
                                        flow: Right, spacing: 20, width: Fill, height: 280,
                                        detail_subplot_bar = <BarPlot> { width: 400, height: Fill }
                                        detail_subplot_area = <LinePlot> { width: 400, height: Fill }
                                    }
                                }
                            }
                        }

                        // 1x3 Subplot Strip
                        <View> {
                            flow: Right, spacing: 16, height: 320,
                            <RoundedView> {
                                width: Fill, height: Fill,
                                padding: 12,
                                show_bg: true,
                                draw_bg: { color: #fafafa, border_radius: 8.0 }
                                flow: Down,
                                <Label> { text: "1x3 Strip: Time Series Comparison", draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 14.0 }, color: #333 } }
                                <SubplotGrid> {
                                    height: Fill, width: Fill,
                                    flow: Right,
                                    spacing: 15,
                                    detail_subplot_ts1 = <LinePlot> { width: Fill, height: Fill }
                                    detail_subplot_ts2 = <LinePlot> { width: Fill, height: Fill }
                                    detail_subplot_ts3 = <LinePlot> { width: Fill, height: Fill }
                                }
                            }
                        }
                    }

                    // ==================== DUAL AXIS DETAIL PAGE ====================
                    dualaxis_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_dualaxis = <BackButton> {}

                        <Label> {
                            text: "Dual Y-Axis Charts"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Temperature & Humidity", draw_text: { color: #333 } }
                                detail_twinx_temp = <LinePlotDual> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Sales & Growth Rate", draw_text: { color: #333 } }
                                detail_twinx_sales = <LinePlotDual> {}
                            }
                        }

                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Stock Price & Volume", draw_text: { color: #333 } }
                                detail_twinx_stock = <LinePlotDual> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Pressure & Altitude", draw_text: { color: #333 } }
                                detail_twinx_pressure = <LinePlotDual> {}
                            }
                        }
                    }

                    // ==================== LOG SCALES DETAIL PAGE ====================
                    logscales_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_logscales = <BackButton> {}

                        <Label> {
                            text: "Logarithmic Scales"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Log Scale - Exponential Growth", draw_text: { color: #333 } }
                                detail_log_exp = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Log Scale - Power Law", draw_text: { color: #333 } }
                                detail_log_power = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Log-Log Scale", draw_text: { color: #333 } }
                                detail_log_log = <LinePlot> {}
                            }
                        }

                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "SymLog - Pos/Neg Data", draw_text: { color: #333 } }
                                detail_symlog = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Semi-Log X Scale", draw_text: { color: #333 } }
                                detail_semilog_x = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Decibel Scale", draw_text: { color: #333 } }
                                detail_decibel = <LinePlot> {}
                            }
                        }
                    }

                    // ==================== TIME SCALE DETAIL PAGE ====================
                    timescale_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_timescale = <BackButton> {}

                        <Label> {
                            text: "Time Scale Charts"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Daily Time Series", draw_text: { color: #333 } }
                                detail_time_daily = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Weekly Time Series", draw_text: { color: #333 } }
                                detail_time_weekly = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Monthly Time Series", draw_text: { color: #333 } }
                                detail_time_monthly = <LinePlot> {}
                            }
                        }

                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Hourly Data", draw_text: { color: #333 } }
                                detail_time_hourly = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Multi-Year Trend", draw_text: { color: #333 } }
                                detail_time_yearly = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Minute-Level Data", draw_text: { color: #333 } }
                                detail_time_minute = <LinePlot> {}
                            }
                        }
                    }

                    // ==================== COLORMAP DETAIL PAGE ====================
                    colormaps_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_colormaps = <BackButton> {}

                        <Label> {
                            text: "Colormap Gallery"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <SectionTitle> { text: "Perceptually Uniform Colormaps" }
                        <View> {
                            flow: Right, spacing: 16, height: 270,
                            <DetailChartCard> {
                                <Label> { text: "Viridis", draw_text: { color: #333 } }
                                detail_cmap_viridis = <HeatmapChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Plasma", draw_text: { color: #333 } }
                                detail_cmap_plasma = <HeatmapChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Inferno", draw_text: { color: #333 } }
                                detail_cmap_inferno = <HeatmapChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Magma", draw_text: { color: #333 } }
                                detail_cmap_magma = <HeatmapChart> {}
                            }
                        }

                        <SectionTitle> { text: "Diverging & Colorblind-Friendly" }
                        <View> {
                            flow: Right, spacing: 16, height: 270,
                            <DetailChartCard> {
                                <Label> { text: "Cividis (Colorblind)", draw_text: { color: #333 } }
                                detail_cmap_cividis = <HeatmapChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Coolwarm (Diverging)", draw_text: { color: #333 } }
                                detail_cmap_coolwarm = <HeatmapChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "RdBu (Red-Blue)", draw_text: { color: #333 } }
                                detail_cmap_rdbu = <HeatmapChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Spectral", draw_text: { color: #333 } }
                                detail_cmap_spectral = <HeatmapChart> {}
                            }
                        }

                        <SectionTitle> { text: "Sequential Colormaps" }
                        <View> {
                            flow: Right, spacing: 16, height: 270,
                            <DetailChartCard> {
                                <Label> { text: "Blues", draw_text: { color: #333 } }
                                detail_cmap_blues = <HeatmapChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Greens", draw_text: { color: #333 } }
                                detail_cmap_greens = <HeatmapChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Oranges", draw_text: { color: #333 } }
                                detail_cmap_oranges = <HeatmapChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Reds", draw_text: { color: #333 } }
                                detail_cmap_reds = <HeatmapChart> {}
                            }
                        }

                        <SectionTitle> { text: "Classic & Special Colormaps" }
                        <View> {
                            flow: Right, spacing: 16, height: 270,
                            <DetailChartCard> {
                                <Label> { text: "Jet (Rainbow)", draw_text: { color: #333 } }
                                detail_cmap_jet = <HeatmapChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Hot", draw_text: { color: #333 } }
                                detail_cmap_hot = <HeatmapChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Turbo", draw_text: { color: #333 } }
                                detail_cmap_turbo = <HeatmapChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Greys", draw_text: { color: #333 } }
                                detail_cmap_greys = <HeatmapChart> {}
                            }
                        }
                    }

                    // ==================== LOG NORM DETAIL PAGE ====================
                    lognorm_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_lognorm = <BackButton> {}

                        <Label> {
                            text: "Normalization Examples"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <SectionTitle> { text: "Linear vs Log Normalization" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Linear Norm (Default)", draw_text: { color: #333 } }
                                detail_norm_linear = <HeatmapChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Log Norm (Power Law)", draw_text: { color: #333 } }
                                detail_norm_log = <HeatmapChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Comparison", draw_text: { color: #333 } }
                                detail_norm_compare = <HeatmapChart> {}
                            }
                        }

                        <SectionTitle> { text: "Surface Plots with Colormaps" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Surface - Viridis", draw_text: { color: #333 } }
                                detail_surface_viridis = <Surface3D> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Surface - Plasma", draw_text: { color: #333 } }
                                detail_surface_plasma = <Surface3D> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Surface - Coolwarm", draw_text: { color: #333 } }
                                detail_surface_coolwarm = <Surface3D> {}
                            }
                        }
                    }

                    // ==================== ANNOTATIONS DETAIL PAGE ====================
                    annotations_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_annotations = <BackButton> {}

                        <Label> {
                            text: "Annotations & Reference Lines"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <SectionTitle> { text: "Text Annotations" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Text Labels", draw_text: { color: #333 } }
                                detail_text_annotations = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Arrow Annotations", draw_text: { color: #333 } }
                                detail_arrow_annotations = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Math Annotations", draw_text: { color: #333 } }
                                detail_math_annotations = <LinePlot> {}
                            }
                        }

                        <SectionTitle> { text: "Reference Lines & Spans" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Vertical Lines (axvline)", draw_text: { color: #333 } }
                                detail_vlines = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Horizontal Lines (axhline)", draw_text: { color: #333 } }
                                detail_hlines = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Shaded Regions", draw_text: { color: #333 } }
                                detail_spans = <LinePlot> {}
                            }
                        }
                    }

                    // ==================== ERROR BARS DETAIL PAGE ====================
                    errorbars_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_errorbars = <BackButton> {}

                        <Label> {
                            text: "Error Bars & Uncertainty Visualization"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <SectionTitle> { text: "Error Bars" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Symmetric Y Error", draw_text: { color: #333 } }
                                detail_yerr_symmetric = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Asymmetric Y Error", draw_text: { color: #333 } }
                                detail_yerr_asymmetric = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "X and Y Error Bars", draw_text: { color: #333 } }
                                detail_xy_error = <LinePlot> {}
                            }
                        }

                        <SectionTitle> { text: "Fill Between / Area Charts" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Confidence Band", draw_text: { color: #333 } }
                                detail_confidence_band = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Area Chart (Baseline)", draw_text: { color: #333 } }
                                detail_area_chart = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Stacked Areas", draw_text: { color: #333 } }
                                detail_stacked_area = <LinePlot> {}
                            }
                        }
                    }

                    // ==================== FILL BETWEEN DETAIL PAGE ====================
                    fillbetween_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_fillbetween = <BackButton> {}

                        <Label> {
                            text: "Fill Between & Area Charts"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <SectionTitle> { text: "Fill Between Curves" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Between Two Lines", draw_text: { color: #333 } }
                                detail_fill_two_lines = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Confidence Interval", draw_text: { color: #333 } }
                                detail_fill_confidence = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Multiple Fills", draw_text: { color: #333 } }
                                detail_fill_multiple = <LinePlot> {}
                            }
                        }

                        <SectionTitle> { text: "Area Charts" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Simple Area", draw_text: { color: #333 } }
                                detail_area_simple = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Stacked Area", draw_text: { color: #333 } }
                                detail_area_stacked = <LinePlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Gradient Fill", draw_text: { color: #333 } }
                                detail_area_gradient = <LinePlot> {}
                            }
                        }
                    }

                    // ==================== STACKED BARS DETAIL PAGE ====================
                    stackedbars_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_stackedbars = <BackButton> {}

                        <Label> {
                            text: "Stacked & Grouped Bar Charts"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <SectionTitle> { text: "Bar Variations" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Grouped Bars", draw_text: { color: #333 } }
                                detail_grouped_bars = <BarPlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Stacked Bars", draw_text: { color: #333 } }
                                detail_stacked_bars = <BarPlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Horizontal Stacked", draw_text: { color: #333 } }
                                detail_hstacked_bars = <BarPlot> {}
                            }
                        }
                    }

                    // ==================== CANDLESTICK DETAIL PAGE ====================
                    candlestick_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_candlestick = <BackButton> {}

                        <Label> {
                            text: "Candlestick / Financial Charts"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <SectionTitle> { text: "OHLC Visualizations" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Basic Candlestick", draw_text: { color: #333 } }
                                detail_candlestick_basic = <CandlestickChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Uptrend Pattern", draw_text: { color: #333 } }
                                detail_candlestick_uptrend = <CandlestickChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Volatile Market", draw_text: { color: #333 } }
                                detail_candlestick_volatile = <CandlestickChart> {}
                            }
                        }
                    }

                    // ==================== RADAR CHART DETAIL PAGE ====================
                    radar_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_radar = <BackButton> {}

                        <Label> {
                            text: "Radar / Spider Charts"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <SectionTitle> { text: "Multi-Axis Comparison" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Skills Comparison", draw_text: { color: #333 } }
                                detail_radar_skills = <RadarChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Performance Metrics", draw_text: { color: #333 } }
                                detail_radar_metrics = <RadarChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Multi-Series", draw_text: { color: #333 } }
                                detail_radar_multi = <RadarChart> {}
                            }
                        }
                    }

                    // ==================== WATERFALL CHART DETAIL PAGE ====================
                    waterfall_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_waterfall = <BackButton> {}

                        <Label> {
                            text: "Waterfall Charts"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <SectionTitle> { text: "Cumulative Effect Visualization" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Revenue Bridge", draw_text: { color: #333 } }
                                detail_waterfall_revenue = <WaterfallChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Cost Breakdown", draw_text: { color: #333 } }
                                detail_waterfall_cost = <WaterfallChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Profit Analysis", draw_text: { color: #333 } }
                                detail_waterfall_profit = <WaterfallChart> {}
                            }
                        }
                    }

                    // ==================== GAUGE CHART DETAIL PAGE ====================
                    gauge_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_gauge = <BackButton> {}

                        <Label> {
                            text: "Gauge Charts"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <SectionTitle> { text: "Single Value Indicators" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Performance Score", draw_text: { color: #333 } }
                                detail_gauge_performance = <GaugeChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Battery Level", draw_text: { color: #333 } }
                                detail_gauge_battery = <GaugeChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Speed Indicator", draw_text: { color: #333 } }
                                detail_gauge_speed = <GaugeChart> {}
                            }
                        }
                    }

                    // ==================== FUNNEL CHART DETAIL PAGE ====================
                    funnel_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_funnel = <BackButton> {}

                        <Label> {
                            text: "Funnel Charts"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <SectionTitle> { text: "Progressive Stage Reduction" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Sales Pipeline", draw_text: { color: #333 } }
                                detail_funnel_sales = <FunnelChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "User Journey", draw_text: { color: #333 } }
                                detail_funnel_journey = <FunnelChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Hiring Process", draw_text: { color: #333 } }
                                detail_funnel_hiring = <FunnelChart> {}
                            }
                        }
                    }

                    // ==================== HEATMAP DETAIL PAGE ====================
                    heatmap_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_heatmap = <BackButton> {}

                        <Label> {
                            text: "Heatmap Charts"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <SectionTitle> { text: "Grid-Based Data Visualization" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Correlation Matrix", draw_text: { color: #333 } }
                                detail_heatmap_correlation = <Heatmap> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Monthly Sales", draw_text: { color: #333 } }
                                detail_heatmap_sales = <Heatmap> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Activity Levels", draw_text: { color: #333 } }
                                detail_heatmap_activity = <Heatmap> {}
                            }
                        }
                    }

                    // ==================== TREEMAP DETAIL PAGE ====================
                    treemap_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_treemap = <BackButton> {}

                        <Label> {
                            text: "Treemap Charts"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <SectionTitle> { text: "Hierarchical Data Visualization" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Market Share", draw_text: { color: #333 } }
                                detail_treemap_market = <Treemap> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Budget Allocation", draw_text: { color: #333 } }
                                detail_treemap_budget = <Treemap> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Disk Usage", draw_text: { color: #333 } }
                                detail_treemap_disk = <Treemap> {}
                            }
                        }
                    }

                    // ==================== BUBBLE CHART DETAIL PAGE ====================
                    bubble_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_bubble = <BackButton> {}

                        <Label> {
                            text: "Bubble Charts"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <SectionTitle> { text: "Scatter Plot with Size Dimension" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Population vs GDP", draw_text: { color: #333 } }
                                detail_bubble_gdp = <BubbleChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Product Comparison", draw_text: { color: #333 } }
                                detail_bubble_products = <BubbleChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Investment Portfolio", draw_text: { color: #333 } }
                                detail_bubble_investment = <BubbleChart> {}
                            }
                        }
                    }

                    // ==================== DONUT CHART DETAIL PAGE ====================
                    donut_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_donut = <BackButton> {}

                        <Label> {
                            text: "Donut Charts"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <SectionTitle> { text: "Pie Chart with Center Hole" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Browser Usage", draw_text: { color: #333 } }
                                detail_donut_browser = <DonutChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Revenue by Region", draw_text: { color: #333 } }
                                detail_donut_revenue = <DonutChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Task Status", draw_text: { color: #333 } }
                                detail_donut_tasks = <DonutChart> {}
                            }
                        }
                    }

                    // ==================== AREA CHART DETAIL PAGE ====================
                    area_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_area = <BackButton> {}

                        <Label> {
                            text: "Area Charts"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <SectionTitle> { text: "Stacked Area Visualization" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Traffic Sources", draw_text: { color: #333 } }
                                detail_area_traffic = <AreaChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Energy Mix", draw_text: { color: #333 } }
                                detail_area_energy = <AreaChart> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Revenue Trend", draw_text: { color: #333 } }
                                detail_area_revenue = <AreaChart> {}
                            }
                        }
                    }

                    // ==================== STEP PLOT DETAIL PAGE ====================
                    step_detail_page = <ScrollXYView> {
                        visible: false,
                        flow: Down,
                        padding: 20,
                        spacing: 10,

                        back_button_step = <BackButton> {}

                        <Label> {
                            text: "Step Plots"
                            draw_text: { text_style: <THEME_FONT_BOLD> { font_size: 20.0 }, color: #333 }
                        }

                        <SectionTitle> { text: "Discrete Step-wise Lines" }
                        <View> {
                            flow: Right, spacing: 16, height: 370,
                            <DetailChartCard> {
                                <Label> { text: "Price Changes (Pre)", draw_text: { color: #333 } }
                                detail_step_price = <StepPlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "CPU Usage (Post)", draw_text: { color: #333 } }
                                detail_step_cpu = <StepPlot> {}
                            }
                            <DetailChartCard> {
                                <Label> { text: "Queue Length (Mid)", draw_text: { color: #333 } }
                                detail_step_queue = <StepPlot> {}
                            }
                        }
                    }
                }
            }
        }
    }
}

// Page navigation state
#[derive(Default, Clone, Copy, PartialEq)]
pub enum CurrentPage {
    #[default]
    Main,
    LineDetail,
    BarDetail,
    ScatterDetail,
    PieDetail,
    MathDetail,
    StatsDetail,
    TimeSeriesDetail,
    ViolinDetail,
    PolarDetail,
    ContourDetail,
    QuiverDetail,
    Surface3DDetail,
    Scatter3DDetail,
    Line3DDetail,
    SubplotDetail,
    DualAxisDetail,
    LogScalesDetail,
    TimeScaleDetail,
    ColormapDetail,
    LogNormDetail,
    AnnotationsDetail,
    ErrorBarsDetail,
    FillBetweenDetail,
    StackedBarsDetail,
    CandlestickDetail,
    RadarDetail,
    WaterfallDetail,
    GaugeDetail,
    FunnelDetail,
    HeatmapDetail,
    TreemapDetail,
    BubbleDetail,
    DonutDetail,
    AreaDetail,
    StepDetail,
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    current_page: CurrentPage,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        makepad_plot::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());

        match event {
            Event::Startup => {
                self.setup_main_gallery(cx);
            }
            Event::Actions(actions) => {
                self.handle_actions(cx, actions);
            }
            _ => {}
        }
    }
}

impl App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Main page card clicks
        if self.ui.view(&[id!(line_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::LineDetail);
        }
        if self.ui.view(&[id!(bar_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::BarDetail);
        }
        if self.ui.view(&[id!(scatter_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::ScatterDetail);
        }
        if self.ui.view(&[id!(pie_card)]).finger_up(actions).is_some() ||
           self.ui.view(&[id!(histogram_card)]).finger_up(actions).is_some() ||
           self.ui.view(&[id!(boxplot_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::StatsDetail);
        }
        if self.ui.view(&[id!(heatmap_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::StatsDetail);
        }
        if self.ui.view(&[id!(math_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::MathDetail);
        }
        if self.ui.view(&[id!(timeseries_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::TimeSeriesDetail);
        }
        if self.ui.view(&[id!(violin_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::ViolinDetail);
        }
        if self.ui.view(&[id!(polar_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::PolarDetail);
        }
        if self.ui.view(&[id!(contour_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::ContourDetail);
        }
        if self.ui.view(&[id!(quiver_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::QuiverDetail);
        }
        if self.ui.view(&[id!(surface3d_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::Surface3DDetail);
        }
        if self.ui.view(&[id!(scatter3d_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::Scatter3DDetail);
        }
        if self.ui.view(&[id!(line3d_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::Line3DDetail);
        }
        if self.ui.view(&[id!(subplot_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::SubplotDetail);
        }
        if self.ui.view(&[id!(twinx_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::DualAxisDetail);
        }
        if self.ui.view(&[id!(scales_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::LogScalesDetail);
        }
        if self.ui.view(&[id!(time_scale_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::TimeScaleDetail);
        }
        if self.ui.view(&[id!(colormap_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::ColormapDetail);
        }
        if self.ui.view(&[id!(normalizer_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::LogNormDetail);
        }
        if self.ui.view(&[id!(annotations_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::AnnotationsDetail);
        }
        if self.ui.view(&[id!(errorbars_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::ErrorBarsDetail);
        }
        if self.ui.view(&[id!(fillbetween_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::FillBetweenDetail);
        }
        if self.ui.view(&[id!(stacked_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::StackedBarsDetail);
        }
        if self.ui.view(&[id!(candlestick_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::CandlestickDetail);
        }
        if self.ui.view(&[id!(radar_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::RadarDetail);
        }
        if self.ui.view(&[id!(waterfall_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::WaterfallDetail);
        }
        if self.ui.view(&[id!(gauge_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::GaugeDetail);
        }
        if self.ui.view(&[id!(funnel_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::FunnelDetail);
        }
        if self.ui.view(&[id!(heatmap_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::HeatmapDetail);
        }
        if self.ui.view(&[id!(treemap_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::TreemapDetail);
        }
        if self.ui.view(&[id!(bubble_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::BubbleDetail);
        }
        if self.ui.view(&[id!(donut_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::DonutDetail);
        }
        if self.ui.view(&[id!(area_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::AreaDetail);
        }
        if self.ui.view(&[id!(step_card)]).finger_up(actions).is_some() {
            self.navigate_to(cx, CurrentPage::StepDetail);
        }
        // New charts from makepad-d3 (no detail pages yet - stay on gallery)
        if self.ui.view(&[id!(stackplot_card)]).finger_up(actions).is_some() ||
           self.ui.view(&[id!(hexbin_card)]).finger_up(actions).is_some() ||
           self.ui.view(&[id!(streamgraph_card)]).finger_up(actions).is_some() ||
           self.ui.view(&[id!(sankey_card)]).finger_up(actions).is_some() {
            // TODO: Add detail pages for these charts
        }

        // Back buttons
        if self.ui.button(&[id!(back_button_line)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_bar)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_scatter)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_pie)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_math)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_stats)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_time)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_violin)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_polar)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_contour)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_quiver)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_surface3d)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_scatter3d)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_line3d)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_subplot)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_dualaxis)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_logscales)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_timescale)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_colormaps)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_lognorm)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_annotations)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_errorbars)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_fillbetween)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_stackedbars)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_candlestick)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_radar)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_waterfall)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_gauge)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_funnel)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_heatmap)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_treemap)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_bubble)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_donut)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_area)]).clicked(actions) ||
           self.ui.button(&[id!(back_button_step)]).clicked(actions) {
            self.navigate_to(cx, CurrentPage::Main);
        }
    }

    fn navigate_to(&mut self, cx: &mut Cx, page: CurrentPage) {
        // Hide all pages
        self.ui.view(&[id!(main_page)]).set_visible(cx, false);
        self.ui.view(&[id!(line_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(bar_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(scatter_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(pie_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(math_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(stats_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(timeseries_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(violin_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(polar_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(contour_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(quiver_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(surface3d_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(scatter3d_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(line3d_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(subplot_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(dualaxis_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(logscales_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(timescale_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(colormaps_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(lognorm_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(annotations_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(errorbars_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(fillbetween_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(stackedbars_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(candlestick_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(radar_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(waterfall_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(gauge_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(funnel_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(heatmap_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(treemap_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(bubble_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(donut_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(area_detail_page)]).set_visible(cx, false);
        self.ui.view(&[id!(step_detail_page)]).set_visible(cx, false);

        // Show target page and setup
        match page {
            CurrentPage::Main => {
                self.ui.view(&[id!(main_page)]).set_visible(cx, true);
            }
            CurrentPage::LineDetail => {
                self.ui.view(&[id!(line_detail_page)]).set_visible(cx, true);
                self.setup_line_detail(cx);
            }
            CurrentPage::BarDetail => {
                self.ui.view(&[id!(bar_detail_page)]).set_visible(cx, true);
                self.setup_bar_detail(cx);
            }
            CurrentPage::ScatterDetail => {
                self.ui.view(&[id!(scatter_detail_page)]).set_visible(cx, true);
                self.setup_scatter_detail(cx);
            }
            CurrentPage::PieDetail => {
                self.ui.view(&[id!(pie_detail_page)]).set_visible(cx, true);
                self.setup_pie_detail(cx);
            }
            CurrentPage::MathDetail => {
                self.ui.view(&[id!(math_detail_page)]).set_visible(cx, true);
                self.setup_math_detail(cx);
            }
            CurrentPage::StatsDetail => {
                self.ui.view(&[id!(stats_detail_page)]).set_visible(cx, true);
                self.setup_stats_detail(cx);
            }
            CurrentPage::TimeSeriesDetail => {
                self.ui.view(&[id!(timeseries_detail_page)]).set_visible(cx, true);
                self.setup_timeseries_detail(cx);
            }
            CurrentPage::ViolinDetail => {
                self.ui.view(&[id!(violin_detail_page)]).set_visible(cx, true);
                self.setup_violin_detail(cx);
            }
            CurrentPage::PolarDetail => {
                self.ui.view(&[id!(polar_detail_page)]).set_visible(cx, true);
                self.setup_polar_detail(cx);
            }
            CurrentPage::ContourDetail => {
                self.ui.view(&[id!(contour_detail_page)]).set_visible(cx, true);
                self.setup_contour_detail(cx);
            }
            CurrentPage::QuiverDetail => {
                self.ui.view(&[id!(quiver_detail_page)]).set_visible(cx, true);
                self.setup_quiver_detail(cx);
            }
            CurrentPage::Surface3DDetail => {
                self.ui.view(&[id!(surface3d_detail_page)]).set_visible(cx, true);
                self.setup_surface3d_detail(cx);
            }
            CurrentPage::Scatter3DDetail => {
                self.ui.view(&[id!(scatter3d_detail_page)]).set_visible(cx, true);
                self.setup_scatter3d_detail(cx);
            }
            CurrentPage::Line3DDetail => {
                self.ui.view(&[id!(line3d_detail_page)]).set_visible(cx, true);
                self.setup_line3d_detail(cx);
            }
            CurrentPage::SubplotDetail => {
                self.ui.view(&[id!(subplot_detail_page)]).set_visible(cx, true);
                self.setup_subplot_detail(cx);
            }
            CurrentPage::DualAxisDetail => {
                self.ui.view(&[id!(dualaxis_detail_page)]).set_visible(cx, true);
                self.setup_dualaxis_detail(cx);
            }
            CurrentPage::LogScalesDetail => {
                self.ui.view(&[id!(logscales_detail_page)]).set_visible(cx, true);
                self.setup_logscales_detail(cx);
            }
            CurrentPage::TimeScaleDetail => {
                self.ui.view(&[id!(timescale_detail_page)]).set_visible(cx, true);
                self.setup_timescale_detail(cx);
            }
            CurrentPage::ColormapDetail => {
                self.ui.view(&[id!(colormaps_detail_page)]).set_visible(cx, true);
                self.setup_colormaps_detail(cx);
            }
            CurrentPage::LogNormDetail => {
                self.ui.view(&[id!(lognorm_detail_page)]).set_visible(cx, true);
                self.setup_lognorm_detail(cx);
            }
            CurrentPage::AnnotationsDetail => {
                self.ui.view(&[id!(annotations_detail_page)]).set_visible(cx, true);
                self.setup_annotations_detail(cx);
            }
            CurrentPage::ErrorBarsDetail => {
                self.ui.view(&[id!(errorbars_detail_page)]).set_visible(cx, true);
                self.setup_errorbars_detail(cx);
            }
            CurrentPage::FillBetweenDetail => {
                self.ui.view(&[id!(fillbetween_detail_page)]).set_visible(cx, true);
                self.setup_fillbetween_detail(cx);
            }
            CurrentPage::StackedBarsDetail => {
                self.ui.view(&[id!(stackedbars_detail_page)]).set_visible(cx, true);
                self.setup_stackedbars_detail(cx);
            }
            CurrentPage::CandlestickDetail => {
                self.ui.view(&[id!(candlestick_detail_page)]).set_visible(cx, true);
                self.setup_candlestick_detail(cx);
            }
            CurrentPage::RadarDetail => {
                self.ui.view(&[id!(radar_detail_page)]).set_visible(cx, true);
                self.setup_radar_detail(cx);
            }
            CurrentPage::WaterfallDetail => {
                self.ui.view(&[id!(waterfall_detail_page)]).set_visible(cx, true);
                self.setup_waterfall_detail(cx);
            }
            CurrentPage::GaugeDetail => {
                self.ui.view(&[id!(gauge_detail_page)]).set_visible(cx, true);
                self.setup_gauge_detail(cx);
            }
            CurrentPage::FunnelDetail => {
                self.ui.view(&[id!(funnel_detail_page)]).set_visible(cx, true);
                self.setup_funnel_detail(cx);
            }
            CurrentPage::HeatmapDetail => {
                self.ui.view(&[id!(heatmap_detail_page)]).set_visible(cx, true);
                self.setup_heatmap_detail(cx);
            }
            CurrentPage::TreemapDetail => {
                self.ui.view(&[id!(treemap_detail_page)]).set_visible(cx, true);
                self.setup_treemap_detail(cx);
            }
            CurrentPage::BubbleDetail => {
                self.ui.view(&[id!(bubble_detail_page)]).set_visible(cx, true);
                self.setup_bubble_detail(cx);
            }
            CurrentPage::DonutDetail => {
                self.ui.view(&[id!(donut_detail_page)]).set_visible(cx, true);
                self.setup_donut_detail(cx);
            }
            CurrentPage::AreaDetail => {
                self.ui.view(&[id!(area_detail_page)]).set_visible(cx, true);
                self.setup_area_detail(cx);
            }
            CurrentPage::StepDetail => {
                self.ui.view(&[id!(step_detail_page)]).set_visible(cx, true);
                self.setup_step_detail(cx);
            }
        }

        self.current_page = page;
        self.ui.redraw(cx);
    }

    fn setup_main_gallery(&mut self, cx: &mut Cx) {
        // Line Plot preview
        let line_plot = self.ui.line_plot(&[id!(main_line_plot)]);
        line_plot.set_title("Temperature");
        line_plot.add_series(Series::new("Temp").with_data(
            vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
            vec![20.0, 22.0, 25.0, 23.0, 28.0, 26.0],
        ));
        line_plot.redraw(cx);

        // Bar Plot preview
        let bar_plot = self.ui.bar_plot(&[id!(main_bar_plot)]);
        bar_plot.set_title("Sales");
        bar_plot.set_data(
            vec!["Jan".into(), "Feb".into(), "Mar".into(), "Apr".into()],
            vec![120.0, 150.0, 180.0, 90.0],
        );
        bar_plot.redraw(cx);

        // Scatter Plot preview
        let scatter_plot = self.ui.scatter_plot(&[id!(main_scatter_plot)]);
        scatter_plot.set_title("Correlation");
        scatter_plot.add_series(Series::new("Data").with_data(
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0],
            vec![2.3, 3.1, 4.5, 5.2, 6.8, 7.1, 8.9],
        ));
        scatter_plot.redraw(cx);

        // Pie Chart preview
        let pie_chart = self.ui.pie_chart(&[id!(main_pie_chart)]);
        pie_chart.set_title("Market Share");
        pie_chart.set_data(
            vec!["A".into(), "B".into(), "C".into(), "D".into()],
            vec![40.0, 30.0, 20.0, 10.0],
        );
        pie_chart.redraw(cx);

        // Histogram preview
        let histogram = self.ui.histogram_chart(&[id!(main_histogram)]);
        histogram.set_title("Distribution");
        let data: Vec<f64> = (0..100).map(|i| 50.0 + (i as f64 * 0.37).sin() * 20.0).collect();
        histogram.set_values(data);
        histogram.set_num_bins(10);
        histogram.redraw(cx);

        // Box Plot preview
        let boxplot = self.ui.box_plot_chart(&[id!(main_boxplot)]);
        boxplot.set_title("Groups");
        boxplot.add_from_values("A", &vec![10.0, 20.0, 30.0, 25.0, 15.0, 35.0, 22.0]);
        boxplot.add_from_values("B", &vec![15.0, 25.0, 35.0, 30.0, 20.0, 40.0, 28.0]);
        boxplot.redraw(cx);

        // Heatmap preview
        let heatmap = self.ui.heatmap_chart(&[id!(main_heatmap)]);
        heatmap.set_title("Correlation");
        heatmap.set_data(vec![
            vec![1.0, 0.8, 0.3],
            vec![0.8, 1.0, 0.5],
            vec![0.3, 0.5, 1.0],
        ]);
        heatmap.redraw(cx);

        // Math plot preview (integral)
        let math_plot = self.ui.line_plot(&[id!(main_math_plot)]);
        math_plot.set_title("Integral ∫f(x)dx");
        let func = |x: f64| (x - 3.0) * (x - 5.0) * (x - 7.0) + 85.0;
        let x: Vec<f64> = (0..=50).map(|i| i as f64 * 0.2).collect();
        let y: Vec<f64> = x.iter().map(|&xi| func(xi)).collect();
        math_plot.fill_between_baseline(x.clone(), y.clone(), 0.0, vec4(0.85, 0.85, 0.85, 0.8));
        math_plot.add_series(Series::new("f(x)").with_data(x, y).with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        math_plot.set_show_points(false);
        math_plot.set_ylim(0.0, 150.0);
        math_plot.redraw(cx);

        // Time series preview
        let time_plot = self.ui.line_plot(&[id!(main_time_plot)]);
        time_plot.set_title("Stock Price");
        let prices: Vec<f64> = vec![100.0, 102.0, 98.0, 105.0, 110.0, 108.0, 115.0, 120.0, 118.0, 125.0];
        let times: Vec<f64> = (0..10).map(|i| i as f64).collect();
        time_plot.add_series(Series::new("MSFT").with_data(times, prices).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        time_plot.set_show_points(false);
        time_plot.redraw(cx);

        // Violin plot preview
        let violin = self.ui.violin_plot(&[id!(main_violin)]);
        violin.set_title("Distribution");
        violin.add_from_values("A", &(0..30).map(|i| 50.0 + (i as f64 * 0.5).sin() * 15.0).collect::<Vec<_>>());
        violin.add_from_values("B", &(0..30).map(|i| 60.0 + (i as f64 * 0.7).cos() * 12.0).collect::<Vec<_>>());
        violin.set_show_box(true);
        violin.redraw(cx);

        // Polar plot preview
        let polar = self.ui.polar_plot(&[id!(main_polar)]);
        polar.set_title("Radar");
        let theta: Vec<f64> = (0..8).map(|i| i as f64 * std::f64::consts::PI / 4.0).collect();
        let r: Vec<f64> = vec![3.0, 4.0, 2.0, 5.0, 3.5, 4.5, 2.5, 3.0];
        polar.add_series(PolarSeries::new("Data").with_data(theta, r).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        polar.redraw(cx);

        // Contour plot preview
        let contour = self.ui.contour_plot(&[id!(main_contour)]);
        contour.set_title("Surface");
        let n = 20;
        let mut data = Vec::new();
        for i in 0..n {
            let mut row = Vec::new();
            for j in 0..n {
                let x = (i as f64 - n as f64 / 2.0) / 5.0;
                let y = (j as f64 - n as f64 / 2.0) / 5.0;
                row.push((-x * x - y * y).exp());
            }
            data.push(row);
        }
        contour.set_data(data);
        contour.set_x_range(-2.0, 2.0);
        contour.set_y_range(-2.0, 2.0);
        contour.set_filled(true);
        contour.redraw(cx);

        // Quiver plot preview
        let quiver = self.ui.quiver_plot(&[id!(main_quiver)]);
        quiver.set_title("Vector Field");
        let mut x = Vec::new();
        let mut y = Vec::new();
        let mut u = Vec::new();
        let mut v = Vec::new();
        for i in 0..5 {
            for j in 0..5 {
                let xi = i as f64 - 2.0;
                let yi = j as f64 - 2.0;
                x.push(xi);
                y.push(yi);
                u.push(-yi);
                v.push(xi);
            }
        }
        quiver.set_data(x, y, u, v);
        quiver.redraw(cx);

        // Surface3D preview
        let surface = self.ui.surface3_d(&[id!(main_surface3d)]);
        surface.set_title("3D Surface");
        let size = 60;
        let mut z: Vec<Vec<f64>> = vec![vec![0.0; size]; size];
        for i in 0..size {
            for j in 0..size {
                let x = (i as f64 - 30.0) / 10.0;
                let y = (j as f64 - 30.0) / 10.0;
                z[i][j] = (x*x + y*y).sqrt().sin();
            }
        }
        surface.set_data(z);
        surface.set_wireframe(false);
        surface.set_surface(true);
        surface.set_colormap(Colormap::Viridis);
        surface.redraw(cx);

        // Scatter3D preview
        let scatter3d = self.ui.scatter3_d(&[id!(main_scatter3d)]);
        scatter3d.set_title("3D Scatter");
        let x: Vec<f64> = (0..50).map(|i| (i as f64 * 0.2).sin() * 2.0).collect();
        let y: Vec<f64> = (0..50).map(|i| (i as f64 * 0.2).cos() * 2.0).collect();
        let z: Vec<f64> = (0..50).map(|i| i as f64 * 0.1).collect();
        scatter3d.set_data(x, y, z);
        scatter3d.redraw(cx);

        // Line3D preview
        let line3d = self.ui.line3_d(&[id!(main_line3d)]);
        line3d.set_title("3D Helix");
        let t: Vec<f64> = (0..100).map(|i| i as f64 * 0.1).collect();
        let x: Vec<f64> = t.iter().map(|&ti| ti.cos()).collect();
        let y: Vec<f64> = t.iter().map(|&ti| ti.sin()).collect();
        let z: Vec<f64> = t.iter().map(|&ti| ti * 0.1).collect();
        line3d.add_series(Line3DSeries::new("Helix").with_data(x, y, z).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        line3d.redraw(cx);

        // Subplot Grid preview - setup individual plots
        let subplot_line = self.ui.line_plot(&[id!(subplot_line1)]);
        subplot_line.add_series(Series::new("Sin").with_data(
            (0..20).map(|i| i as f64 * 0.3).collect(),
            (0..20).map(|i| (i as f64 * 0.3).sin()).collect(),
        ));
        subplot_line.redraw(cx);

        let subplot_bar = self.ui.bar_plot(&[id!(subplot_bar1)]);
        subplot_bar.set_data(vec!["A".into(), "B".into(), "C".into()], vec![3.0, 5.0, 2.0]);
        subplot_bar.redraw(cx);

        let subplot_scatter = self.ui.scatter_plot(&[id!(subplot_scatter1)]);
        subplot_scatter.add_series(Series::new("Pts").with_data(
            vec![1.0, 2.0, 3.0, 4.0], vec![1.5, 3.0, 2.5, 4.0]
        ));
        subplot_scatter.redraw(cx);

        let subplot_pie = self.ui.pie_chart(&[id!(subplot_pie1)]);
        subplot_pie.set_data(vec!["X".into(), "Y".into(), "Z".into()], vec![50.0, 30.0, 20.0]);
        subplot_pie.redraw(cx);

        // Dual Y-Axis preview
        let twinx = self.ui.line_plot_dual(&[id!(main_twinx)]);
        twinx.set_title("Temperature & Humidity");
        twinx.set_ylabel("Temp (°C)");
        twinx.set_y2label("Humidity (%)");
        twinx.add_series_left(Series::new("Temp").with_data(
            vec![0.0, 1.0, 2.0, 3.0, 4.0],
            vec![20.0, 22.0, 25.0, 23.0, 21.0],
        ).with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        twinx.add_series_right(Series::new("Humidity").with_data(
            vec![0.0, 1.0, 2.0, 3.0, 4.0],
            vec![65.0, 70.0, 55.0, 60.0, 75.0],
        ).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        twinx.set_grid(true);
        twinx.set_legend(LegendPosition::TopRight);
        twinx.redraw(cx);

        // Log Scale preview
        let log_scale = self.ui.line_plot(&[id!(main_log_scale)]);
        log_scale.set_title("Exponential (Log Y)");
        let x: Vec<f64> = (0..30).map(|i| i as f64 * 0.3).collect();
        let y: Vec<f64> = x.iter().map(|&xi| (xi * 0.5).exp()).collect();
        log_scale.add_series(Series::new("e^(0.5x)").with_data(x, y).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        log_scale.set_y_scale(ScaleType::Log);
        log_scale.redraw(cx);

        // Time Scale preview
        let time_scale = self.ui.line_plot(&[id!(main_time_scale)]);
        time_scale.set_title("Time Series");
        let base_time = 1704067200.0; // Jan 1, 2024
        let x: Vec<f64> = (0..7).map(|i| base_time + i as f64 * 86400.0).collect();
        let y: Vec<f64> = vec![100.0, 105.0, 103.0, 108.0, 112.0, 110.0, 115.0];
        time_scale.add_series(Series::new("Daily").with_data(x, y).with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        time_scale.set_x_scale(ScaleType::Time);
        time_scale.redraw(cx);

        // =======================
        // COLORMAP PREVIEW (Row 8)
        // =======================
        let colormap_heatmap = self.ui.heatmap_chart(&[id!(main_colormap_heatmap)]);
        colormap_heatmap.set_title("Colormap Gallery");
        let data: Vec<Vec<f64>> = (0..15).map(|i| {
            (0..15).map(|j| {
                let x = (i as f64 - 7.0) / 3.0;
                let y = (j as f64 - 7.0) / 3.0;
                (-x * x - y * y).exp()
            }).collect()
        }).collect();
        colormap_heatmap.set_data(data);
        colormap_heatmap.set_colormap(Colormap::Turbo);
        colormap_heatmap.set_show_values(false);
        colormap_heatmap.redraw(cx);

        let lognorm_heatmap = self.ui.heatmap_chart(&[id!(main_lognorm_heatmap)]);
        lognorm_heatmap.set_title("Normalization Demo");
        let data: Vec<Vec<f64>> = (0..15).map(|i| {
            (0..15).map(|j| {
                let x = i as f64 / 15.0 * 3.0;
                let y = j as f64 / 15.0 * 3.0;
                (x + y).exp() / 50.0
            }).collect()
        }).collect();
        lognorm_heatmap.set_data(data);
        lognorm_heatmap.set_colormap(Colormap::Plasma);
        lognorm_heatmap.set_show_values(false);
        lognorm_heatmap.redraw(cx);

        // =======================
        // ANNOTATIONS PREVIEW (Row 9)
        // =======================
        let annotations = self.ui.line_plot(&[id!(main_annotations)]);
        annotations.set_title("Annotations Demo");
        let x: Vec<f64> = (0..30).map(|i| i as f64 * 0.3).collect();
        annotations.add_series(Series::new("Data")
            .with_data(x.clone(), x.iter().map(|&v| (v * 0.5).sin() * 3.0 + 5.0).collect())
            .with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        // Add sample annotations
        annotations.axvline(3.0, vec4(0.9, 0.2, 0.2, 1.0), 1.5, LineStyle::Dashed);
        annotations.axhline(5.0, vec4(0.2, 0.7, 0.2, 1.0), 1.5, LineStyle::Dotted);
        annotations.annotate("Maximum", 4.5, 7.5, vec4(0.1, 0.1, 0.1, 1.0), 11.0);
        annotations.set_legend(LegendPosition::TopRight);
        annotations.redraw(cx);

        // =======================
        // ERROR BARS PREVIEW (Row 10)
        // =======================
        let errorbars = self.ui.line_plot(&[id!(main_errorbars)]);
        errorbars.set_title("Error Bars Demo");
        let x: Vec<f64> = (0..8).map(|i| i as f64).collect();
        let y: Vec<f64> = vec![2.0, 3.5, 4.0, 3.8, 5.2, 6.0, 5.5, 7.0];
        let yerr: Vec<f64> = vec![0.3, 0.5, 0.4, 0.6, 0.4, 0.5, 0.7, 0.4];
        errorbars.add_series(Series::new("Measurements")
            .with_data(x, y)
            .with_yerr(yerr)
            .with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        errorbars.set_show_points(true);
        errorbars.set_legend(LegendPosition::TopRight);
        errorbars.redraw(cx);

        let fillbetween = self.ui.line_plot(&[id!(main_fillbetween)]);
        fillbetween.set_title("Fill Between Demo");
        let x: Vec<f64> = (0..30).map(|i| i as f64 * 0.3).collect();
        let y: Vec<f64> = x.iter().map(|&v| (v * 0.5).sin() * 2.0 + 4.0).collect();
        let y_upper: Vec<f64> = y.iter().map(|&v| v + 0.8).collect();
        let y_lower: Vec<f64> = y.iter().map(|&v| v - 0.8).collect();
        fillbetween.fill_between(x.clone(), y_lower, y_upper, vec4(0.12, 0.47, 0.71, 0.3));
        fillbetween.add_series(Series::new("Mean")
            .with_data(x, y)
            .with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        fillbetween.set_legend(LegendPosition::TopRight);
        fillbetween.redraw(cx);

        // =======================
        // STACKED BARS PREVIEW (Row 11)
        // =======================
        let stacked_bars = self.ui.bar_plot(&[id!(main_stacked_bars)]);
        stacked_bars.set_title("Grouped Bars");
        stacked_bars.set_groups(
            vec!["Q1".into(), "Q2".into(), "Q3".into(), "Q4".into()],
            vec![
                BarGroup { label: "Sales".into(), values: vec![120.0, 150.0, 180.0, 200.0], color: Some(vec4(0.12, 0.47, 0.71, 1.0)) },
                BarGroup { label: "Revenue".into(), values: vec![100.0, 130.0, 160.0, 190.0], color: Some(vec4(0.84, 0.15, 0.16, 1.0)) },
            ]
        );
        stacked_bars.redraw(cx);

        // Candlestick preview
        let candlestick = self.ui.candlestick_chart(&[id!(main_candlestick)]);
        candlestick.set_title("Stock OHLC");
        candlestick.set_data(vec![
            Candle::new(0.0, 100.0, 105.0, 98.0, 103.0),
            Candle::new(1.0, 103.0, 108.0, 102.0, 106.0),
            Candle::new(2.0, 106.0, 110.0, 104.0, 105.0),
            Candle::new(3.0, 105.0, 107.0, 100.0, 101.0),
            Candle::new(4.0, 101.0, 106.0, 99.0, 104.0),
            Candle::new(5.0, 104.0, 112.0, 103.0, 110.0),
            Candle::new(6.0, 110.0, 115.0, 108.0, 113.0),
        ]);
        candlestick.redraw(cx);

        // Radar preview
        let radar = self.ui.radar_chart(&[id!(main_radar)]);
        radar.set_title("Skills Radar");
        radar.set_axes(vec!["Speed".into(), "Power".into(), "Defense".into(), "Stamina".into(), "Agility".into()]);
        radar.add_series(RadarSeries::new("Player A", vec![85.0, 70.0, 65.0, 80.0, 90.0])
            .with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        radar.set_max_value(100.0);
        radar.redraw(cx);

        // =======================
        // PHASE 12 PREVIEWS (Row 12-13)
        // =======================

        // Waterfall preview
        let waterfall = self.ui.waterfall_chart(&[id!(main_waterfall)]);
        waterfall.set_title("Cash Flow");
        waterfall.set_data(vec![
            WaterfallEntry::new("Start", 100.0),
            WaterfallEntry::new("Q1", 30.0),
            WaterfallEntry::new("Q2", -20.0),
            WaterfallEntry::new("Q3", 50.0),
            WaterfallEntry::total("End", 160.0),
        ]);
        waterfall.redraw(cx);

        // Gauge preview
        let gauge = self.ui.gauge_chart(&[id!(main_gauge)]);
        gauge.set_title("Performance");
        gauge.set_value(72.0);
        gauge.set_range(0.0, 100.0);
        gauge.redraw(cx);

        // Funnel preview
        let funnel = self.ui.funnel_chart(&[id!(main_funnel)]);
        funnel.set_title("Sales Funnel");
        funnel.set_data(vec![
            FunnelStage::new("Leads", 1000.0),
            FunnelStage::new("Qualified", 600.0),
            FunnelStage::new("Proposal", 300.0),
            FunnelStage::new("Closed", 100.0),
        ]);
        funnel.redraw(cx);

        // Heatmap grid preview (Phase 12)
        let heatmap_new = self.ui.heatmap(&[id!(main_heatmap_new)]);
        heatmap_new.set_title("Activity");
        heatmap_new.set_data(vec![
            vec![3.0, 5.0, 8.0, 4.0],
            vec![6.0, 9.0, 2.0, 7.0],
            vec![1.0, 4.0, 6.0, 3.0],
        ]);
        heatmap_new.set_colormap(Colormap::Viridis);
        heatmap_new.redraw(cx);

        // Treemap preview
        let treemap = self.ui.treemap(&[id!(main_treemap)]);
        treemap.set_title("Market Share");
        treemap.set_data(vec![
            TreemapNode::new("A", 40.0).with_color(vec4(0.12, 0.47, 0.71, 1.0)),
            TreemapNode::new("B", 30.0).with_color(vec4(0.17, 0.63, 0.17, 1.0)),
            TreemapNode::new("C", 20.0).with_color(vec4(0.94, 0.78, 0.0, 1.0)),
            TreemapNode::new("D", 10.0).with_color(vec4(0.84, 0.15, 0.16, 1.0)),
        ]);
        treemap.redraw(cx);

        // Bubble chart preview
        let bubble = self.ui.bubble_chart(&[id!(main_bubble)]);
        bubble.set_title("GDP vs Pop");
        bubble.add_series(BubbleSeries::new("Countries").with_points(vec![
            BubblePoint::new(10.0, 50.0, 100.0).with_color(vec4(0.12, 0.47, 0.71, 1.0)),
            BubblePoint::new(30.0, 30.0, 80.0).with_color(vec4(0.17, 0.63, 0.17, 1.0)),
            BubblePoint::new(50.0, 70.0, 60.0).with_color(vec4(0.94, 0.78, 0.0, 1.0)),
            BubblePoint::new(70.0, 40.0, 120.0).with_color(vec4(0.84, 0.15, 0.16, 1.0)),
        ]).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        bubble.redraw(cx);

        // =======================
        // PHASE 13 PREVIEWS (Row 14)
        // =======================

        // Donut chart preview
        let donut = self.ui.donut_chart(&[id!(main_donut)]);
        donut.set_title("Usage");
        donut.set_data(vec![
            DonutSlice::new("Chrome", 60.0).with_color(vec4(0.17, 0.63, 0.17, 1.0)),
            DonutSlice::new("Safari", 25.0).with_color(vec4(0.12, 0.47, 0.71, 1.0)),
            DonutSlice::new("Other", 15.0).with_color(vec4(0.94, 0.78, 0.0, 1.0)),
        ]);
        donut.set_inner_radius_ratio(0.5);
        donut.redraw(cx);

        // Area chart preview
        let area = self.ui.area_chart(&[id!(main_area)]);
        area.set_title("Traffic");
        area.add_series(AreaSeries::new("Visitors")
            .with_data(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
                       vec![100.0, 150.0, 120.0, 180.0, 200.0, 170.0])
            .with_color(vec4(0.12, 0.47, 0.71, 0.7)));
        area.set_show_grid(true);
        area.redraw(cx);

        // Step plot preview
        let step = self.ui.step_plot(&[id!(main_step)]);
        step.set_title("Price");
        step.add_series(StepSeries::new("Stock")
            .with_data(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
                       vec![100.0, 105.0, 103.0, 110.0, 108.0, 115.0])
            .with_style(StepStyle::Pre)
            .with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        step.set_show_grid(true);
        step.redraw(cx);

        // =======================
        // NEW FROM MAKEPAD-D3 (Row 15)
        // =======================

        // Stackplot preview
        let stackplot = self.ui.stackplot(&[id!(main_stackplot)]);
        stackplot.set_title("Stacked Area");
        stackplot.set_data(
            vec![
                StackSeries::new("Series A", vec![10.0, 15.0, 12.0, 18.0, 14.0, 20.0])
                    .with_color(vec4(0.12, 0.47, 0.71, 0.8)),
                StackSeries::new("Series B", vec![8.0, 12.0, 10.0, 14.0, 16.0, 12.0])
                    .with_color(vec4(0.17, 0.63, 0.17, 0.8)),
                StackSeries::new("Series C", vec![5.0, 8.0, 6.0, 10.0, 8.0, 9.0])
                    .with_color(vec4(0.94, 0.78, 0.0, 0.8)),
            ],
            vec!["Jan".into(), "Feb".into(), "Mar".into(), "Apr".into(), "May".into(), "Jun".into()],
        );
        stackplot.redraw(cx);

        // Hexbin preview
        let hexbin = self.ui.hexbin_chart(&[id!(main_hexbin)]);
        hexbin.set_title("Hexagonal Bins");
        // Generate random-ish clustered points
        let mut points = Vec::new();
        for i in 0..200 {
            let t = i as f64 * 0.1;
            let x = (t * 0.3).sin() * 3.0 + (i as f64 % 5.0) * 0.5 + 5.0;
            let y = (t * 0.4).cos() * 3.0 + (i as f64 % 7.0) * 0.4 + 5.0;
            points.push(HexbinPoint { x, y });
        }
        hexbin.set_data(points);
        hexbin.set_hex_radius(0.5);
        hexbin.set_colors(vec4(0.9, 0.9, 0.95, 1.0), vec4(0.12, 0.47, 0.71, 1.0));  // Light to blue
        hexbin.redraw(cx);

        // Streamgraph preview
        let streamgraph = self.ui.streamgraph(&[id!(main_streamgraph)]);
        streamgraph.set_title("Streamgraph");
        streamgraph.set_data(
            vec![
                StreamSeries::new("Layer 1", vec![10.0, 15.0, 12.0, 18.0, 14.0, 20.0, 16.0])
                    .with_color(vec4(0.12, 0.47, 0.71, 0.8)),
                StreamSeries::new("Layer 2", vec![8.0, 12.0, 10.0, 14.0, 16.0, 12.0, 10.0])
                    .with_color(vec4(0.17, 0.63, 0.17, 0.8)),
                StreamSeries::new("Layer 3", vec![5.0, 8.0, 6.0, 10.0, 8.0, 9.0, 7.0])
                    .with_color(vec4(0.94, 0.78, 0.0, 0.8)),
                StreamSeries::new("Layer 4", vec![6.0, 4.0, 8.0, 5.0, 9.0, 6.0, 8.0])
                    .with_color(vec4(0.84, 0.15, 0.16, 0.8)),
            ],
            vec!["Jan".into(), "Feb".into(), "Mar".into(), "Apr".into(), "May".into(), "Jun".into(), "Jul".into()],
        );
        streamgraph.redraw(cx);

        // Sankey diagram preview
        let sankey = self.ui.sankey_diagram(&[id!(main_sankey)]);
        sankey.set_title("Flow Diagram");
        sankey.set_data(
            vec![
                SankeyNode::new("Source A", 0, 30.0, vec4(0.12, 0.47, 0.71, 1.0)),
                SankeyNode::new("Source B", 0, 20.0, vec4(0.17, 0.63, 0.17, 1.0)),
                SankeyNode::new("Process", 1, 50.0, vec4(0.94, 0.78, 0.0, 1.0)),
                SankeyNode::new("Output X", 2, 25.0, vec4(0.84, 0.15, 0.16, 1.0)),
                SankeyNode::new("Output Y", 2, 25.0, vec4(0.58, 0.40, 0.74, 1.0)),
            ],
            vec![
                SankeyLink::new(0, 2, 30.0),  // Source A -> Process
                SankeyLink::new(1, 2, 20.0),  // Source B -> Process
                SankeyLink::new(2, 3, 25.0),  // Process -> Output X
                SankeyLink::new(2, 4, 25.0),  // Process -> Output Y
            ],
        );
        sankey.redraw(cx);
    }

    fn setup_line_detail(&mut self, cx: &mut Cx) {
        // ===================
        // LINE STYLES SECTION
        // ===================

        // Dashed lines
        let dashed = self.ui.line_plot(&[id!(detail_line_dashed)]);
        dashed.set_title("Dashed Lines (Interactive - drag to pan, scroll to zoom)");
        let x: Vec<f64> = (0..30).map(|i| i as f64 * 0.3).collect();
        dashed.add_series(Series::new("Dashed")
            .with_data(x.clone(), x.iter().map(|&v| (v * 0.5).sin() * 5.0 + 10.0).collect())
            .with_color(vec4(0.12, 0.47, 0.71, 1.0))
            .with_line_style(LineStyle::Dashed));
        dashed.add_series(Series::new("Also Dashed")
            .with_data(x.clone(), x.iter().map(|&v| (v * 0.3).cos() * 4.0 + 5.0).collect())
            .with_color(vec4(0.84, 0.15, 0.16, 1.0))
            .with_line_style(LineStyle::Dashed));
        dashed.set_legend(LegendPosition::TopRight);
        dashed.set_show_points(false);
        dashed.set_interactive(true);  // Enable pan/zoom
        dashed.redraw(cx);

        // Dotted lines
        let dotted = self.ui.line_plot(&[id!(detail_line_dotted)]);
        dotted.set_title("Dotted Lines");
        let x: Vec<f64> = (0..30).map(|i| i as f64 * 0.3).collect();
        dotted.add_series(Series::new("Dotted")
            .with_data(x.clone(), x.iter().map(|&v| (v * 0.4).sin() * 6.0 + 12.0).collect())
            .with_color(vec4(0.17, 0.63, 0.17, 1.0))
            .with_line_style(LineStyle::Dotted));
        dotted.add_series(Series::new("Also Dotted")
            .with_data(x.clone(), x.iter().map(|&v| (v * 0.6).cos() * 5.0 + 6.0).collect())
            .with_color(vec4(0.58, 0.40, 0.74, 1.0))
            .with_line_style(LineStyle::Dotted));
        dotted.set_legend(LegendPosition::TopRight);
        dotted.set_show_points(false);
        dotted.redraw(cx);

        // Mixed line styles
        let mixed = self.ui.line_plot(&[id!(detail_line_mixed)]);
        mixed.set_title("All Line Styles");
        let x: Vec<f64> = (0..30).map(|i| i as f64 * 0.3).collect();
        mixed.add_series(Series::new("Solid")
            .with_data(x.clone(), x.iter().map(|&v| v * 0.8 + 2.0).collect())
            .with_color(vec4(0.12, 0.47, 0.71, 1.0))
            .with_line_style(LineStyle::Solid));
        mixed.add_series(Series::new("Dashed")
            .with_data(x.clone(), x.iter().map(|&v| v * 0.8 + 6.0).collect())
            .with_color(vec4(0.84, 0.15, 0.16, 1.0))
            .with_line_style(LineStyle::Dashed));
        mixed.add_series(Series::new("Dotted")
            .with_data(x.clone(), x.iter().map(|&v| v * 0.8 + 10.0).collect())
            .with_color(vec4(0.17, 0.63, 0.17, 1.0))
            .with_line_style(LineStyle::Dotted));
        mixed.add_series(Series::new("DashDot")
            .with_data(x.clone(), x.iter().map(|&v| v * 0.8 + 14.0).collect())
            .with_color(vec4(0.58, 0.40, 0.74, 1.0))
            .with_line_style(LineStyle::DashDot));
        mixed.set_legend(LegendPosition::TopRight);
        mixed.set_show_points(false);
        mixed.redraw(cx);

        // ====================
        // MARKER STYLES SECTION
        // ====================

        // Marker gallery - all marker types
        let markers = self.ui.line_plot(&[id!(detail_line_markers)]);
        markers.set_title("All Marker Types");
        let marker_types = [
            (MarkerStyle::Circle, "Circle"),
            (MarkerStyle::Square, "Square"),
            (MarkerStyle::TriangleUp, "Triangle Up"),
            (MarkerStyle::TriangleDown, "Triangle Down"),
            (MarkerStyle::Diamond, "Diamond"),
            (MarkerStyle::Cross, "Cross"),
            (MarkerStyle::Plus, "Plus"),
            (MarkerStyle::Star, "Star"),
        ];
        for (i, (style, name)) in marker_types.iter().enumerate() {
            let x_val = i as f64 + 1.0;
            markers.add_series(Series::new(*name)
                .with_data(vec![x_val], vec![5.0])
                .with_color(get_color(i))
                .with_marker(*style)
                .with_marker_size(12.0));
        }
        markers.set_xlim(0.0, 9.0);
        markers.set_ylim(0.0, 10.0);
        markers.set_show_points(true);
        markers.redraw(cx);

        // Lines with markers
        let with_markers = self.ui.line_plot(&[id!(detail_line_with_markers)]);
        with_markers.set_title("Lines + Markers");
        let x: Vec<f64> = (0..8).map(|i| i as f64).collect();
        with_markers.add_series(Series::new("Circle")
            .with_data(x.clone(), x.iter().map(|&v| v * 0.8 + 2.0).collect())
            .with_color(vec4(0.12, 0.47, 0.71, 1.0))
            .with_marker(MarkerStyle::Circle)
            .with_marker_size(8.0));
        with_markers.add_series(Series::new("Square")
            .with_data(x.clone(), x.iter().map(|&v| v * 0.9 + 5.0).collect())
            .with_color(vec4(0.84, 0.15, 0.16, 1.0))
            .with_marker(MarkerStyle::Square)
            .with_marker_size(8.0));
        with_markers.add_series(Series::new("Diamond")
            .with_data(x.clone(), x.iter().map(|&v| v * 0.7 + 8.0).collect())
            .with_color(vec4(0.17, 0.63, 0.17, 1.0))
            .with_marker(MarkerStyle::Diamond)
            .with_marker_size(8.0));
        with_markers.set_legend(LegendPosition::TopLeft);
        with_markers.set_show_points(true);
        with_markers.redraw(cx);

        // Scatter with markers (no lines)
        let scatter_markers = self.ui.line_plot(&[id!(detail_line_scatter_markers)]);
        scatter_markers.set_title("Scatter Markers");
        // Generate random-looking data
        let x1: Vec<f64> = vec![1.0, 2.5, 3.2, 4.8, 5.5, 6.2, 7.0];
        let y1: Vec<f64> = vec![2.0, 3.5, 2.8, 4.2, 5.0, 4.5, 6.0];
        let x2: Vec<f64> = vec![1.5, 2.8, 3.5, 4.2, 5.8, 6.5, 7.2];
        let y2: Vec<f64> = vec![4.0, 5.2, 6.0, 5.5, 7.0, 6.8, 8.0];
        scatter_markers.add_series(Series::new("Group A")
            .with_data(x1, y1)
            .with_color(vec4(0.12, 0.47, 0.71, 1.0))
            .with_marker(MarkerStyle::Circle)
            .with_marker_size(10.0));
        scatter_markers.add_series(Series::new("Group B")
            .with_data(x2, y2)
            .with_color(vec4(0.84, 0.15, 0.16, 1.0))
            .with_marker(MarkerStyle::Star)
            .with_marker_size(10.0));
        scatter_markers.set_legend(LegendPosition::TopLeft);
        scatter_markers.set_show_points(true);
        scatter_markers.redraw(cx);

        // ===================
        // STEP PLOTS SECTION
        // ===================

        // Step Pre
        let step_pre = self.ui.line_plot(&[id!(detail_line_step_pre)]);
        step_pre.set_title("Step Pre (stairs)");
        let x: Vec<f64> = (0..10).map(|i| i as f64).collect();
        let y: Vec<f64> = vec![2.0, 3.0, 2.5, 5.0, 4.0, 6.0, 5.5, 7.0, 6.0, 8.0];
        step_pre.add_series(Series::new("Pre")
            .with_data(x, y)
            .with_color(vec4(0.12, 0.47, 0.71, 1.0))
            .with_step(StepStyle::Pre)
            .with_marker(MarkerStyle::Circle)
            .with_marker_size(6.0));
        step_pre.set_show_points(true);
        step_pre.redraw(cx);

        // Step Post
        let step_post = self.ui.line_plot(&[id!(detail_line_step_post)]);
        step_post.set_title("Step Post");
        let x: Vec<f64> = (0..10).map(|i| i as f64).collect();
        let y: Vec<f64> = vec![2.0, 3.0, 2.5, 5.0, 4.0, 6.0, 5.5, 7.0, 6.0, 8.0];
        step_post.add_series(Series::new("Post")
            .with_data(x, y)
            .with_color(vec4(0.84, 0.15, 0.16, 1.0))
            .with_step(StepStyle::Post)
            .with_marker(MarkerStyle::Circle)
            .with_marker_size(6.0));
        step_post.set_show_points(true);
        step_post.redraw(cx);

        // Step Mid
        let step_mid = self.ui.line_plot(&[id!(detail_line_step_mid)]);
        step_mid.set_title("Step Mid");
        let x: Vec<f64> = (0..10).map(|i| i as f64).collect();
        let y: Vec<f64> = vec![2.0, 3.0, 2.5, 5.0, 4.0, 6.0, 5.5, 7.0, 6.0, 8.0];
        step_mid.add_series(Series::new("Mid")
            .with_data(x, y)
            .with_color(vec4(0.17, 0.63, 0.17, 1.0))
            .with_step(StepStyle::Mid)
            .with_marker(MarkerStyle::Circle)
            .with_marker_size(6.0));
        step_mid.set_show_points(true);
        step_mid.redraw(cx);

        // ====================
        // ERROR BARS SECTION
        // ====================

        // Y Error bars
        let yerr = self.ui.line_plot(&[id!(detail_line_yerr)]);
        yerr.set_title("Y Error Bars");
        let x: Vec<f64> = (1..=8).map(|i| i as f64).collect();
        let y: Vec<f64> = vec![3.0, 4.5, 3.8, 5.2, 4.8, 6.0, 5.5, 7.0];
        let err: Vec<f64> = vec![0.5, 0.6, 0.4, 0.7, 0.5, 0.6, 0.4, 0.5];
        yerr.add_series(Series::new("Measurements")
            .with_data(x, y)
            .with_color(vec4(0.12, 0.47, 0.71, 1.0))
            .with_yerr(err)
            .with_marker(MarkerStyle::Circle)
            .with_marker_size(6.0));
        yerr.set_show_points(true);
        yerr.redraw(cx);

        // X and Y Error bars
        let xyerr = self.ui.line_plot(&[id!(detail_line_xyerr)]);
        xyerr.set_title("X and Y Error");
        let x: Vec<f64> = vec![2.0, 4.0, 6.0, 8.0];
        let y: Vec<f64> = vec![3.0, 5.0, 4.0, 6.0];
        let xerr: Vec<f64> = vec![0.3, 0.4, 0.35, 0.3];
        let yerr_vals: Vec<f64> = vec![0.5, 0.6, 0.5, 0.7];
        xyerr.add_series(Series::new("Data")
            .with_data(x, y)
            .with_color(vec4(0.84, 0.15, 0.16, 1.0))
            .with_xerr(xerr)
            .with_yerr(yerr_vals)
            .with_marker(MarkerStyle::Square)
            .with_marker_size(8.0));
        xyerr.set_show_points(true);
        xyerr.redraw(cx);

        // Asymmetric error bars
        let asymerr = self.ui.line_plot(&[id!(detail_line_asymerr)]);
        asymerr.set_title("Asymmetric Errors");
        let x: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let y: Vec<f64> = vec![2.0, 4.0, 3.5, 5.0, 4.5, 6.0];
        let yerr_minus: Vec<f64> = vec![0.3, 0.4, 0.3, 0.5, 0.4, 0.3];
        let yerr_plus: Vec<f64> = vec![0.8, 1.0, 0.7, 0.9, 0.8, 1.0];
        asymerr.add_series(Series::new("Asymmetric")
            .with_data(x, y)
            .with_color(vec4(0.17, 0.63, 0.17, 1.0))
            .with_yerr_asymmetric(yerr_minus, yerr_plus)
            .with_marker(MarkerStyle::Diamond)
            .with_marker_size(8.0));
        asymerr.set_show_points(true);
        asymerr.redraw(cx);

        // =============================
        // REFERENCE LINES & SPANS SECTION
        // =============================

        // axvline and axhline
        let reflines = self.ui.line_plot(&[id!(detail_line_reflines)]);
        reflines.set_title("Reference Lines");
        let x: Vec<f64> = (0..30).map(|i| i as f64 * 0.3).collect();
        reflines.add_series(Series::new("Data")
            .with_data(x.clone(), x.iter().map(|&v| (v * 0.5).sin() * 3.0 + 5.0).collect())
            .with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        reflines.axvline(3.0, vec4(0.84, 0.15, 0.16, 1.0), 2.0, LineStyle::Dashed);
        reflines.axvline(6.0, vec4(0.84, 0.15, 0.16, 1.0), 2.0, LineStyle::Dashed);
        reflines.axhline(5.0, vec4(0.17, 0.63, 0.17, 1.0), 2.0, LineStyle::Dotted);
        reflines.axhline(7.0, vec4(0.58, 0.40, 0.74, 1.0), 2.0, LineStyle::Solid);
        reflines.set_show_points(false);
        reflines.redraw(cx);

        // axvspan and axhspan
        let spans = self.ui.line_plot(&[id!(detail_line_spans)]);
        spans.set_title("Shaded Regions");
        let x: Vec<f64> = (0..40).map(|i| i as f64 * 0.25).collect();
        spans.add_series(Series::new("Signal")
            .with_data(x.clone(), x.iter().map(|&v| (v * 0.4).sin() * 4.0 + 5.0).collect())
            .with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        spans.axvspan(2.0, 4.0, vec4(1.0, 0.5, 0.05, 0.3));
        spans.axvspan(7.0, 8.0, vec4(0.17, 0.63, 0.17, 0.3));
        spans.axhspan(2.0, 3.0, vec4(0.58, 0.40, 0.74, 0.2));
        spans.set_show_points(false);
        spans.redraw(cx);

        // Combined: data with reference lines and spans
        let combined = self.ui.line_plot(&[id!(detail_line_combined)]);
        combined.set_title("Combined View");
        let x: Vec<f64> = (0..50).map(|i| i as f64 * 0.2).collect();
        let y: Vec<f64> = x.iter().map(|&v| (v * 0.3).sin() * 3.0 + v * 0.3 + 2.0).collect();
        // Add threshold region
        combined.axhspan(4.0, 6.0, vec4(0.17, 0.63, 0.17, 0.15));
        // Add event markers
        combined.axvline(3.0, vec4(0.84, 0.15, 0.16, 0.8), 1.5, LineStyle::Dashed);
        combined.axvline(7.0, vec4(0.84, 0.15, 0.16, 0.8), 1.5, LineStyle::Dashed);
        combined.add_series(Series::new("Trend")
            .with_data(x, y)
            .with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        combined.axhline(5.0, vec4(0.17, 0.63, 0.17, 0.8), 1.5, LineStyle::Dotted);
        combined.set_show_points(false);
        combined.redraw(cx);

        // ====================
        // AREA CHARTS SECTION
        // ====================

        // Area chart
        let area = self.ui.line_plot(&[id!(detail_line_area)]);
        area.set_title("Filled Area");
        let x: Vec<f64> = (0..30).map(|i| i as f64 * 0.3).collect();
        let y: Vec<f64> = x.iter().map(|&v| (v * 0.5).sin().abs() * 10.0 + 5.0).collect();
        area.fill_between_baseline(x.clone(), y.clone(), 0.0, vec4(0.12, 0.47, 0.71, 0.5));
        area.add_series(Series::new("Area").with_data(x, y).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        area.set_show_points(false);
        area.redraw(cx);

        // Stacked area
        let stacked = self.ui.line_plot(&[id!(detail_line_stacked)]);
        stacked.set_title("Stacked Areas");
        let x: Vec<f64> = (0..25).map(|i| i as f64 * 0.4).collect();
        let y1: Vec<f64> = x.iter().map(|&v| (v * 0.3).sin().abs() * 8.0 + 5.0).collect();
        let y2: Vec<f64> = x.iter().map(|&v| (v * 0.5).sin().abs() * 6.0 + 15.0).collect();
        let y3: Vec<f64> = x.iter().map(|&v| (v * 0.7).sin().abs() * 4.0 + 25.0).collect();
        stacked.fill_between_baseline(x.clone(), y1.clone(), 0.0, vec4(0.12, 0.47, 0.71, 0.6));
        stacked.fill_between(x.clone(), y2.clone(), y1.clone(), vec4(0.17, 0.63, 0.17, 0.6));
        stacked.fill_between(x.clone(), y3.clone(), y2.clone(), vec4(1.0, 0.5, 0.05, 0.6));
        stacked.add_series(Series::new("A").with_data(x.clone(), y1));
        stacked.add_series(Series::new("B").with_data(x.clone(), y2));
        stacked.add_series(Series::new("C").with_data(x, y3));
        stacked.set_show_points(false);
        stacked.set_legend(LegendPosition::TopLeft);
        stacked.redraw(cx);

        // Confidence interval
        let conf = self.ui.line_plot(&[id!(detail_line_confidence)]);
        conf.set_title("Confidence Band");
        let x: Vec<f64> = (0..30).map(|i| i as f64 * 0.3).collect();
        let y: Vec<f64> = x.iter().map(|&v| v * 0.8 + (v * 0.5).sin() * 2.0).collect();
        let y_upper: Vec<f64> = y.iter().map(|&v| v + 2.0).collect();
        let y_lower: Vec<f64> = y.iter().map(|&v| v - 2.0).collect();
        conf.fill_between(x.clone(), y_upper, y_lower, vec4(0.12, 0.47, 0.71, 0.3));
        conf.add_series(Series::new("Mean").with_data(x, y).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        conf.set_show_points(false);
        conf.redraw(cx);
    }

    fn setup_math_detail(&mut self, cx: &mut Cx) {
        // Definite Integral with LaTeX
        let integral = self.ui.line_plot(&[id!(detail_math_integral)]);
        integral.set_title("Definite Integral");
        let func = |x: f64| (x - 3.0) * (x - 5.0) * (x - 7.0) + 85.0;
        let x: Vec<f64> = (0..=100).map(|i| i as f64 * 0.1).collect();
        let y: Vec<f64> = x.iter().map(|&xi| func(xi)).collect();
        let a = 2.0;
        let b = 9.0;
        let x_fill: Vec<f64> = (0..=70).map(|i| a + i as f64 * (b - a) / 70.0).collect();
        let y_fill: Vec<f64> = x_fill.iter().map(|&xi| func(xi)).collect();
        integral.fill_between_baseline(x_fill, y_fill, 0.0, vec4(0.85, 0.85, 0.85, 0.8));
        integral.add_series(Series::new("f(x)").with_data(x, y).with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        integral.annotate_math(r"\int_a^b f(x)\mathrm{d}x", 5.5, 40.0, vec4(0.0, 0.0, 0.0, 1.0), 16.0);
        integral.set_ylim(0.0, 150.0);
        integral.set_xlim(0.0, 10.0);
        integral.set_show_points(false);
        integral.redraw(cx);

        // Area under parabola
        let area = self.ui.line_plot(&[id!(detail_math_area)]);
        area.set_title("Area Under Parabola");
        let x: Vec<f64> = (0..=100).map(|i| -5.0 + i as f64 * 0.1).collect();
        let y: Vec<f64> = x.iter().map(|&xi| 25.0 - xi * xi).collect();
        let x_pos: Vec<f64> = x.iter().filter(|&&xi| xi >= -5.0 && xi <= 5.0 && 25.0 - xi * xi >= 0.0).cloned().collect();
        let y_pos: Vec<f64> = x_pos.iter().map(|&xi| 25.0 - xi * xi).collect();
        area.fill_between_baseline(x_pos, y_pos, 0.0, vec4(0.17, 0.63, 0.17, 0.5));
        area.add_series(Series::new("y = 25 - x²").with_data(x, y).with_color(vec4(0.17, 0.63, 0.17, 1.0)));
        area.annotate_math(r"y = 25 - x^2", 0.0, 15.0, vec4(0.0, 0.0, 0.0, 1.0), 14.0);
        area.set_show_points(false);
        area.redraw(cx);

        // Riemann sum visualization
        let riemann = self.ui.line_plot(&[id!(detail_math_riemann)]);
        riemann.set_title("Riemann Sum");
        let func = |x: f64| x * x;
        let x: Vec<f64> = (0..=50).map(|i| i as f64 * 0.1).collect();
        let y: Vec<f64> = x.iter().map(|&xi| func(xi)).collect();
        // Add rectangles as fill regions (approximation)
        for i in 0..10 {
            let x_left = i as f64 * 0.5;
            let x_right = (i + 1) as f64 * 0.5;
            let height = func(x_left);
            riemann.fill_between(
                vec![x_left, x_right],
                vec![height, height],
                vec![0.0, 0.0],
                vec4(0.12, 0.47, 0.71, 0.4)
            );
        }
        riemann.add_series(Series::new("x²").with_data(x, y).with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        riemann.annotate_math(r"f(x) = x^2", 3.0, 15.0, vec4(0.0, 0.0, 0.0, 1.0), 14.0);
        riemann.set_show_points(false);
        riemann.redraw(cx);

        // Trig functions
        let trig = self.ui.line_plot(&[id!(detail_math_trig)]);
        trig.set_title("Trigonometric Functions");
        let x: Vec<f64> = (0..=100).map(|i| i as f64 * 0.1).collect();
        let sin_y: Vec<f64> = x.iter().map(|&xi| xi.sin()).collect();
        let cos_y: Vec<f64> = x.iter().map(|&xi| xi.cos()).collect();
        trig.add_series(Series::new("sin(x)").with_data(x.clone(), sin_y).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        trig.add_series(Series::new("cos(x)").with_data(x, cos_y).with_color(vec4(1.0, 0.5, 0.05, 1.0)));
        trig.set_legend(LegendPosition::TopRight);
        trig.set_show_points(false);
        trig.annotate_math(r"\sin(x), \cos(x)", 5.0, 0.5, vec4(0.0, 0.0, 0.0, 1.0), 12.0);
        trig.redraw(cx);

        // Tangent function
        let tan = self.ui.line_plot(&[id!(detail_math_tan)]);
        tan.set_title("Tangent Function");
        let x: Vec<f64> = (0..=100).map(|i| -1.5 + i as f64 * 0.03).collect();
        let y: Vec<f64> = x.iter().map(|&xi| xi.tan().max(-10.0).min(10.0)).collect();
        tan.add_series(Series::new("tan(x)").with_data(x, y).with_color(vec4(0.58, 0.40, 0.74, 1.0)));
        tan.set_ylim(-10.0, 10.0);
        tan.set_show_points(false);
        tan.annotate_math(r"\tan(x)", 0.5, 5.0, vec4(0.0, 0.0, 0.0, 1.0), 14.0);
        tan.redraw(cx);

        // Parametric (Lissajous-like)
        let param = self.ui.line_plot(&[id!(detail_math_parametric)]);
        param.set_title("Parametric Curve");
        let t: Vec<f64> = (0..=200).map(|i| i as f64 * 0.05).collect();
        let x: Vec<f64> = t.iter().map(|&ti| (ti * 3.0).sin()).collect();
        let y: Vec<f64> = t.iter().map(|&ti| (ti * 2.0).cos()).collect();
        param.add_series(Series::new("Lissajous").with_data(x, y).with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        param.set_show_points(false);
        param.annotate_math(r"x=\sin(3t), y=\cos(2t)", 0.0, 0.0, vec4(0.0, 0.0, 0.0, 1.0), 11.0);
        param.redraw(cx);

        // Derivative
        let deriv = self.ui.line_plot(&[id!(detail_math_derivative)]);
        deriv.set_title("Function & Derivative");
        let x: Vec<f64> = (0..=100).map(|i| -2.0 + i as f64 * 0.06).collect();
        let y: Vec<f64> = x.iter().map(|&xi| xi * xi * xi - 3.0 * xi).collect();
        let dy: Vec<f64> = x.iter().map(|&xi| 3.0 * xi * xi - 3.0).collect();
        deriv.add_series(Series::new("f(x)").with_data(x.clone(), y).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        deriv.add_series(Series::new("f'(x)").with_data(x, dy).with_color(vec4(1.0, 0.5, 0.05, 1.0)));
        deriv.set_legend(LegendPosition::TopRight);
        deriv.set_show_points(false);
        deriv.annotate_math(r"f(x)=x^3-3x", 2.0, 5.0, vec4(0.0, 0.0, 0.0, 1.0), 12.0);
        deriv.redraw(cx);

        // Tangent line at a point
        let tangent = self.ui.line_plot(&[id!(detail_math_tangent)]);
        tangent.set_title("Tangent Line");
        let x: Vec<f64> = (0..=100).map(|i| -3.0 + i as f64 * 0.06).collect();
        let y: Vec<f64> = x.iter().map(|&xi| xi * xi).collect();
        // Tangent at x=1: slope = 2, point (1,1), line: y = 2x - 1
        let tang_x: Vec<f64> = vec![-1.0, 3.0];
        let tang_y: Vec<f64> = vec![-3.0, 5.0];
        tangent.add_series(Series::new("x²").with_data(x, y).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        tangent.add_series(Series::new("tangent").with_data(tang_x, tang_y).with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        tangent.set_show_points(false);
        tangent.annotate_math(r"y=2x-1", 2.0, 3.5, vec4(0.0, 0.0, 0.0, 1.0), 12.0);
        tangent.redraw(cx);

        // Second derivative
        let second = self.ui.line_plot(&[id!(detail_math_second)]);
        second.set_title("Second Derivative");
        let x: Vec<f64> = (0..=100).map(|i| -2.0 + i as f64 * 0.04).collect();
        let y: Vec<f64> = x.iter().map(|&xi| xi.sin()).collect();
        let dy: Vec<f64> = x.iter().map(|&xi| xi.cos()).collect();
        let ddy: Vec<f64> = x.iter().map(|&xi| -xi.sin()).collect();
        second.add_series(Series::new("sin(x)").with_data(x.clone(), y).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        second.add_series(Series::new("cos(x)").with_data(x.clone(), dy).with_color(vec4(0.17, 0.63, 0.17, 1.0)));
        second.add_series(Series::new("-sin(x)").with_data(x, ddy).with_color(vec4(1.0, 0.5, 0.05, 1.0)));
        second.set_legend(LegendPosition::TopRight);
        second.set_show_points(false);
        second.redraw(cx);

        // Exponential
        let exp = self.ui.line_plot(&[id!(detail_math_exp)]);
        exp.set_title("Exponential Function");
        let x: Vec<f64> = (0..=60).map(|i| -2.0 + i as f64 * 0.1).collect();
        let y: Vec<f64> = x.iter().map(|&xi| xi.exp()).collect();
        exp.add_series(Series::new("e^x").with_data(x, y).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        exp.set_show_points(false);
        exp.annotate_math(r"e^x", 2.0, 5.0, vec4(0.0, 0.0, 0.0, 1.0), 16.0);
        exp.redraw(cx);

        // Logarithm
        let log = self.ui.line_plot(&[id!(detail_math_log)]);
        log.set_title("Natural Logarithm");
        let x: Vec<f64> = (1..=100).map(|i| i as f64 * 0.1).collect();
        let y: Vec<f64> = x.iter().map(|&xi| xi.ln()).collect();
        log.add_series(Series::new("ln(x)").with_data(x, y).with_color(vec4(0.17, 0.63, 0.17, 1.0)));
        log.set_show_points(false);
        log.annotate_math(r"\ln(x)", 5.0, 1.0, vec4(0.0, 0.0, 0.0, 1.0), 16.0);
        log.redraw(cx);

        // Gaussian
        let gauss = self.ui.line_plot(&[id!(detail_math_gaussian)]);
        gauss.set_title("Gaussian / Normal Distribution");
        let x: Vec<f64> = (0..=100).map(|i| -4.0 + i as f64 * 0.08).collect();
        let y: Vec<f64> = x.iter().map(|&xi| (-xi * xi / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt()).collect();
        gauss.fill_between_baseline(x.clone(), y.clone(), 0.0, vec4(0.58, 0.40, 0.74, 0.4));
        gauss.add_series(Series::new("N(0,1)").with_data(x, y).with_color(vec4(0.58, 0.40, 0.74, 1.0)));
        gauss.set_show_points(false);
        gauss.annotate_math(r"\frac{1}{\sqrt{2\pi}}e^{-x^2/2}", 0.0, 0.25, vec4(0.0, 0.0, 0.0, 1.0), 11.0);
        gauss.redraw(cx);
    }

    fn setup_bar_detail(&mut self, cx: &mut Cx) {
        // ====================
        // BASIC BAR CHARTS
        // ====================

        // Vertical bars
        let vertical = self.ui.bar_plot(&[id!(detail_bar_vertical)]);
        vertical.set_title("Vertical Bars");
        vertical.set_data(
            vec!["A".into(), "B".into(), "C".into(), "D".into(), "E".into()],
            vec![65.0, 59.0, 80.0, 81.0, 56.0],
        );
        vertical.redraw(cx);

        // Horizontal bars
        let horizontal = self.ui.bar_plot(&[id!(detail_bar_horizontal)]);
        horizontal.set_title("Horizontal Bars");
        horizontal.set_data(
            vec!["Python".into(), "Rust".into(), "Go".into(), "JavaScript".into()],
            vec![85.0, 72.0, 65.0, 90.0],
        );
        horizontal.set_horizontal(true);
        horizontal.redraw(cx);

        // Colored bars
        let colored = self.ui.bar_plot(&[id!(detail_bar_colored)]);
        colored.set_title("Colored Bars");
        colored.set_data(
            vec!["Jan".into(), "Feb".into(), "Mar".into(), "Apr".into(), "May".into()],
            vec![120.0, 150.0, 180.0, 90.0, 200.0],
        );
        colored.redraw(cx);

        // ====================
        // GROUPED & STACKED
        // ====================

        // Grouped bars
        let grouped = self.ui.bar_plot(&[id!(detail_bar_grouped)]);
        grouped.set_title("Grouped Bars");
        grouped.set_groups(
            vec!["Q1".into(), "Q2".into(), "Q3".into(), "Q4".into()],
            vec![
                BarGroup::new("2022", vec![65.0, 70.0, 80.0, 90.0]).with_color(vec4(0.12, 0.47, 0.71, 1.0)),
                BarGroup::new("2023", vec![75.0, 85.0, 95.0, 110.0]).with_color(vec4(0.17, 0.63, 0.17, 1.0)),
                BarGroup::new("2024", vec![90.0, 100.0, 115.0, 130.0]).with_color(vec4(0.84, 0.15, 0.16, 1.0)),
            ]
        );
        grouped.redraw(cx);

        // Stacked bars
        let stacked = self.ui.bar_plot(&[id!(detail_bar_stacked)]);
        stacked.set_title("Stacked Bars");
        stacked.set_groups(
            vec!["2021".into(), "2022".into(), "2023".into()],
            vec![
                BarGroup::new("Product A", vec![40.0, 50.0, 60.0]).with_color(vec4(0.12, 0.47, 0.71, 1.0)),
                BarGroup::new("Product B", vec![30.0, 40.0, 50.0]).with_color(vec4(0.17, 0.63, 0.17, 1.0)),
                BarGroup::new("Product C", vec![20.0, 30.0, 40.0]).with_color(vec4(0.84, 0.15, 0.16, 1.0)),
            ]
        );
        stacked.set_stacked(true);
        stacked.redraw(cx);

        // Horizontal stacked
        let hstacked = self.ui.bar_plot(&[id!(detail_bar_hstacked)]);
        hstacked.set_title("Horizontal Stacked");
        hstacked.set_groups(
            vec!["Team A".into(), "Team B".into(), "Team C".into()],
            vec![
                BarGroup::new("Tasks", vec![25.0, 30.0, 20.0]).with_color(vec4(0.12, 0.47, 0.71, 1.0)),
                BarGroup::new("Reviews", vec![15.0, 20.0, 25.0]).with_color(vec4(0.17, 0.63, 0.17, 1.0)),
                BarGroup::new("Meetings", vec![10.0, 15.0, 12.0]).with_color(vec4(0.58, 0.40, 0.74, 1.0)),
            ]
        );
        hstacked.set_horizontal(true);
        hstacked.set_stacked(true);
        hstacked.redraw(cx);

        // ====================
        // STEM PLOTS
        // ====================

        // Basic stem plot
        let stem_basic = self.ui.stem_plot(&[id!(detail_stem_basic)]);
        stem_basic.set_title("Basic Stem Plot");
        let x: Vec<f64> = (0..15).map(|i| i as f64).collect();
        let y: Vec<f64> = x.iter().map(|&v| (v * 0.5).sin() * 5.0 + 6.0).collect();
        stem_basic.add_series(Series::new("Signal").with_data(x, y).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        stem_basic.redraw(cx);

        // Stem with custom markers
        let stem_markers = self.ui.stem_plot(&[id!(detail_stem_markers)]);
        stem_markers.set_title("Stem with Markers");
        let x: Vec<f64> = (0..12).map(|i| i as f64).collect();
        let y: Vec<f64> = vec![3.0, 5.0, 2.0, 7.0, 4.0, 8.0, 3.0, 6.0, 5.0, 9.0, 4.0, 7.0];
        stem_markers.add_series(Series::new("Data")
            .with_data(x, y)
            .with_color(vec4(0.84, 0.15, 0.16, 1.0))
            .with_marker(MarkerStyle::Diamond)
            .with_marker_size(10.0));
        stem_markers.redraw(cx);

        // Multi-series stem
        let stem_multi = self.ui.stem_plot(&[id!(detail_stem_multi)]);
        stem_multi.set_title("Multi-Series Stem");
        let x: Vec<f64> = (0..10).map(|i| i as f64).collect();
        stem_multi.add_series(Series::new("Series A")
            .with_data(x.clone(), x.iter().map(|&v| (v * 0.4).sin() * 4.0 + 5.0).collect())
            .with_color(vec4(0.12, 0.47, 0.71, 1.0))
            .with_marker(MarkerStyle::Circle));
        stem_multi.add_series(Series::new("Series B")
            .with_data(x.clone(), x.iter().map(|&v| (v * 0.6).cos() * 3.0 + 6.0).collect())
            .with_color(vec4(0.17, 0.63, 0.17, 1.0))
            .with_marker(MarkerStyle::Square));
        stem_multi.set_legend(LegendPosition::TopRight);
        stem_multi.redraw(cx);
    }

    fn setup_scatter_detail(&mut self, cx: &mut Cx) {
        let basic = self.ui.scatter_plot(&[id!(detail_scatter_basic)]);
        basic.set_title("Basic Scatter (Interactive - drag to pan, scroll to zoom)");
        basic.add_series(Series::new("Points").with_data(
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
            vec![2.1, 3.5, 4.2, 5.8, 6.1, 7.3, 8.0, 9.2],
        ));
        basic.set_interactive(true);  // Enable pan/zoom
        basic.set_use_gradient(true);  // Enable gradient points
        basic.redraw(cx);

        let corr = self.ui.scatter_plot(&[id!(detail_scatter_correlation)]);
        corr.set_title("Strong Correlation (Interactive)");
        let x: Vec<f64> = (0..30).map(|i| i as f64 * 0.5 + (i as f64 * 0.3).sin() * 0.5).collect();
        let y: Vec<f64> = x.iter().map(|&xi| xi * 0.9 + (xi * 0.5).cos() * 0.3 + 2.0).collect();
        corr.add_series(Series::new("Data").with_data(x, y));
        corr.set_interactive(true);  // Enable pan/zoom
        corr.set_use_gradient(true);  // Enable gradient points
        corr.redraw(cx);

        let multi = self.ui.scatter_plot(&[id!(detail_scatter_multi)]);
        multi.set_title("Multi-Series");
        multi.add_series(Series::new("Group A").with_data(
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![2.0, 3.0, 5.0, 4.0, 6.0],
        ).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        multi.add_series(Series::new("Group B").with_data(
            vec![1.5, 2.5, 3.5, 4.5, 5.5],
            vec![3.0, 4.0, 3.5, 5.5, 7.0],
        ).with_color(vec4(1.0, 0.5, 0.05, 1.0)));
        multi.set_legend(LegendPosition::TopLeft);
        multi.set_use_gradient(true);  // Enable gradient points
        multi.redraw(cx);
    }

    fn setup_pie_detail(&mut self, cx: &mut Cx) {
        let basic = self.ui.pie_chart(&[id!(detail_pie_basic)]);
        basic.set_title("Basic Pie");
        basic.set_data(
            vec!["Red".into(), "Blue".into(), "Green".into(), "Yellow".into()],
            vec![30.0, 25.0, 25.0, 20.0],
        );
        basic.redraw(cx);

        let doughnut = self.ui.pie_chart(&[id!(detail_pie_doughnut)]);
        doughnut.set_title("Doughnut");
        doughnut.set_data(
            vec!["A".into(), "B".into(), "C".into()],
            vec![40.0, 35.0, 25.0],
        );
        doughnut.redraw(cx);

        let pct = self.ui.pie_chart(&[id!(detail_pie_percentages)]);
        pct.set_title("With Percentages");
        pct.set_data(
            vec!["Chrome".into(), "Firefox".into(), "Safari".into(), "Edge".into()],
            vec![65.0, 15.0, 12.0, 8.0],
        );
        pct.set_show_percentages(true);
        pct.redraw(cx);
    }

    fn setup_stats_detail(&mut self, cx: &mut Cx) {
        let hist = self.ui.histogram_chart(&[id!(detail_stats_histogram)]);
        hist.set_title("Normal Distribution");
        let data: Vec<f64> = (0..500).map(|i| {
            50.0 + (i as f64 * 0.37).sin() * 15.0 + (i as f64 * 0.73).cos() * 10.0
        }).collect();
        hist.set_values(data);
        hist.set_num_bins(20);
        hist.redraw(cx);

        let boxplot = self.ui.box_plot_chart(&[id!(detail_stats_boxplot)]);
        boxplot.set_title("Group Comparison");
        boxplot.add_from_values("Group A", &(0..50).map(|i| 50.0 + (i as f64 * 0.5).sin() * 15.0).collect::<Vec<_>>());
        boxplot.add_from_values("Group B", &(0..50).map(|i| 60.0 + (i as f64 * 0.7).cos() * 12.0).collect::<Vec<_>>());
        boxplot.add_from_values("Group C", &(0..50).map(|i| 45.0 + (i as f64 * 0.3).sin() * 20.0).collect::<Vec<_>>());
        boxplot.redraw(cx);

        let heat = self.ui.heatmap_chart(&[id!(detail_stats_heatmap)]);
        heat.set_title("Correlation Matrix");
        heat.set_data(vec![
            vec![1.0, 0.8, 0.3, -0.2, 0.5],
            vec![0.8, 1.0, 0.5, 0.1, 0.3],
            vec![0.3, 0.5, 1.0, 0.7, -0.1],
            vec![-0.2, 0.1, 0.7, 1.0, 0.4],
            vec![0.5, 0.3, -0.1, 0.4, 1.0],
        ]);
        heat.set_x_labels(vec!["A".into(), "B".into(), "C".into(), "D".into(), "E".into()]);
        heat.set_y_labels(vec!["A".into(), "B".into(), "C".into(), "D".into(), "E".into()]);
        heat.set_show_values(true);
        heat.redraw(cx);
    }

    fn setup_timeseries_detail(&mut self, cx: &mut Cx) {
        // Stock prices
        let stock = self.ui.line_plot(&[id!(detail_time_stock)]);
        stock.set_title("Stock Prices (MSFT)");
        let prices: Vec<f64> = vec![
            39.81, 36.35, 43.22, 28.37, 25.45, 32.54, 28.4, 28.4, 24.53, 28.02,
            23.34, 17.65, 24.84, 24.0, 22.25, 27.56, 28.14, 29.7, 26.93, 23.21,
            20.82, 23.65, 26.12, 26.95, 25.92, 23.73, 24.53, 21.26, 20.71, 22.25,
        ];
        let times: Vec<f64> = (0..30).map(|i| i as f64).collect();
        stock.add_series(Series::new("MSFT").with_data(times, prices).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        stock.set_show_points(false);
        stock.set_line_width(2.0);
        stock.redraw(cx);

        // Log scale
        let log = self.ui.line_plot(&[id!(detail_time_log)]);
        log.set_title("Log Scale Growth");
        let x: Vec<f64> = (1..=20).map(|i| i as f64).collect();
        let y: Vec<f64> = x.iter().map(|&xi| 2.0_f64.powf(xi)).collect();
        log.add_series(Series::new("2^x").with_data(x, y).with_color(vec4(0.17, 0.63, 0.17, 1.0)));
        log.set_y_scale(ScaleType::Log);
        log.set_show_points(false);
        log.redraw(cx);

        // Multiple stocks
        let multi = self.ui.line_plot(&[id!(detail_time_multi)]);
        multi.set_title("Multiple Assets");
        let base: Vec<f64> = (0..50).map(|i| i as f64).collect();
        let msft: Vec<f64> = base.iter().map(|&i| 100.0 + (i * 0.3).sin() * 10.0 + i * 0.5).collect();
        let aapl: Vec<f64> = base.iter().map(|&i| 120.0 + (i * 0.5).cos() * 15.0 + i * 0.8).collect();
        let goog: Vec<f64> = base.iter().map(|&i| 80.0 + (i * 0.2).sin() * 8.0 + i * 0.3).collect();
        multi.add_series(Series::new("MSFT").with_data(base.clone(), msft).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        multi.add_series(Series::new("AAPL").with_data(base.clone(), aapl).with_color(vec4(0.58, 0.40, 0.74, 1.0)));
        multi.add_series(Series::new("GOOG").with_data(base, goog).with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        multi.set_legend(LegendPosition::TopLeft);
        multi.set_show_points(false);
        multi.redraw(cx);
    }

    fn setup_violin_detail(&mut self, cx: &mut Cx) {
        // Basic violin plot
        let violin1 = self.ui.violin_plot(&[id!(detail_violin_basic)]);
        violin1.set_title("Basic Violin Plot");
        // Generate normal-ish distribution data
        let data1: Vec<f64> = (0..200).map(|i| {
            let t = i as f64 / 200.0;
            50.0 + (t * 10.0).sin() * 15.0 + (t * 37.0).cos() * 10.0
        }).collect();
        let data2: Vec<f64> = (0..200).map(|i| {
            let t = i as f64 / 200.0;
            70.0 + (t * 15.0).sin() * 20.0 + (t * 23.0).cos() * 8.0
        }).collect();
        violin1.add_from_values("Group A", &data1);
        violin1.add_from_values("Group B", &data2);
        violin1.redraw(cx);

        // Violin with box plot inside
        let violin2 = self.ui.violin_plot(&[id!(detail_violin_box)]);
        violin2.set_title("Violin with Box Plot");
        let data3: Vec<f64> = (0..300).map(|i| {
            let t = i as f64 / 300.0;
            30.0 + (t * 20.0).sin() * 25.0 + (t * 50.0).cos() * 5.0
        }).collect();
        violin2.add_from_values("Distribution", &data3);
        violin2.set_show_box(true);
        violin2.redraw(cx);

        // Multiple violins comparison
        let violin3 = self.ui.violin_plot(&[id!(detail_violin_multi)]);
        violin3.set_title("Multi-Group Comparison");
        let data_a: Vec<f64> = (0..150).map(|i| 40.0 + (i as f64 * 0.2).sin() * 20.0).collect();
        let data_b: Vec<f64> = (0..150).map(|i| 60.0 + (i as f64 * 0.3).cos() * 15.0).collect();
        let data_c: Vec<f64> = (0..150).map(|i| 80.0 + (i as f64 * 0.4).sin() * 25.0).collect();
        violin3.add_from_values("Control", &data_a);
        violin3.add_from_values("Treatment A", &data_b);
        violin3.add_from_values("Treatment B", &data_c);
        violin3.set_show_box(true);
        violin3.redraw(cx);
    }

    fn setup_polar_detail(&mut self, cx: &mut Cx) {
        // Radar chart style
        let polar1 = self.ui.polar_plot(&[id!(detail_polar_radar)]);
        polar1.set_title("Radar Chart");
        let angles: Vec<f64> = (0..6).map(|i| i as f64 * std::f64::consts::PI / 3.0).collect();
        let values: Vec<f64> = vec![0.8, 0.6, 0.9, 0.7, 0.5, 0.8];
        polar1.add_series(PolarSeries::new("Skills").with_data(angles.clone(), values).with_color(vec4(0.12, 0.47, 0.71, 0.7)));
        let values2: Vec<f64> = vec![0.6, 0.8, 0.5, 0.9, 0.7, 0.6];
        polar1.add_series(PolarSeries::new("Target").with_data(angles, values2).with_color(vec4(0.84, 0.15, 0.16, 0.7)));
        polar1.redraw(cx);

        // Rose diagram
        let polar2 = self.ui.polar_plot(&[id!(detail_polar_rose)]);
        polar2.set_title("Rose Curve");
        let theta: Vec<f64> = (0..360).map(|i| i as f64 * std::f64::consts::PI / 180.0).collect();
        let r: Vec<f64> = theta.iter().map(|&t| (4.0 * t).cos().abs()).collect();
        polar2.add_series(PolarSeries::new("r=cos(4θ)").with_data(theta, r).with_color(vec4(0.58, 0.40, 0.74, 1.0)));
        polar2.redraw(cx);

        // Spiral pattern
        let polar3 = self.ui.polar_plot(&[id!(detail_polar_spiral)]);
        polar3.set_title("Spiral Pattern");
        let theta: Vec<f64> = (0..200).map(|i| i as f64 * 0.1).collect();
        let r: Vec<f64> = theta.iter().map(|&t| t * 0.05).collect();
        polar3.add_series(PolarSeries::new("Spiral").with_data(theta, r).with_color(vec4(0.17, 0.63, 0.17, 1.0)));
        polar3.redraw(cx);
    }

    fn setup_contour_detail(&mut self, cx: &mut Cx) {
        // Basic contour - Gaussian
        let contour1 = self.ui.contour_plot(&[id!(detail_contour_lines)]);
        contour1.set_title("Gaussian Contour");
        let size = 50;
        let mut z1: Vec<Vec<f64>> = vec![vec![0.0; size]; size];
        for i in 0..size {
            for j in 0..size {
                let x = (i as f64 - 25.0) / 10.0;
                let y = (j as f64 - 25.0) / 10.0;
                z1[i][j] = (-x*x - y*y).exp();
            }
        }
        contour1.set_data(z1);
        contour1.set_x_range(-2.5, 2.5);
        contour1.set_y_range(-2.5, 2.5);
        contour1.redraw(cx);

        // Filled contour
        let contour2 = self.ui.contour_plot(&[id!(detail_contour_filled)]);
        contour2.set_title("Filled Contour");
        let size = 40;
        let mut z2: Vec<Vec<f64>> = vec![vec![0.0; size]; size];
        for i in 0..size {
            for j in 0..size {
                let x = (i as f64 - 20.0) / 8.0;
                let y = (j as f64 - 20.0) / 8.0;
                z2[i][j] = (x * y).sin();
            }
        }
        contour2.set_data(z2);
        contour2.set_x_range(-2.5, 2.5);
        contour2.set_y_range(-2.5, 2.5);
        contour2.set_filled(true);
        contour2.redraw(cx);

        // Saddle surface
        let contour3 = self.ui.contour_plot(&[id!(detail_contour_saddle)]);
        contour3.set_title("Saddle Surface");
        let size = 45;
        let mut z3: Vec<Vec<f64>> = vec![vec![0.0; size]; size];
        for i in 0..size {
            for j in 0..size {
                let x = (i as f64 - 22.5) / 10.0;
                let y = (j as f64 - 22.5) / 10.0;
                z3[i][j] = x*x - y*y;
            }
        }
        contour3.set_data(z3);
        contour3.set_x_range(-2.25, 2.25);
        contour3.set_y_range(-2.25, 2.25);
        contour3.redraw(cx);
    }

    fn setup_quiver_detail(&mut self, cx: &mut Cx) {
        // Rotational field
        let quiver1 = self.ui.quiver_plot(&[id!(detail_quiver_rotation)]);
        quiver1.set_title("Rotational Field");
        let mut x1 = Vec::new();
        let mut y1 = Vec::new();
        let mut u1 = Vec::new();
        let mut v1 = Vec::new();
        for i in 0..10 {
            for j in 0..10 {
                let px = (i as f64 - 4.5) * 0.4;
                let py = (j as f64 - 4.5) * 0.4;
                x1.push(px);
                y1.push(py);
                u1.push(-py * 0.3);
                v1.push(px * 0.3);
            }
        }
        quiver1.set_data(x1, y1, u1, v1);
        quiver1.redraw(cx);

        // Source/sink field
        let quiver2 = self.ui.quiver_plot(&[id!(detail_quiver_sink)]);
        quiver2.set_title("Point Source Field");
        let mut x2 = Vec::new();
        let mut y2 = Vec::new();
        let mut u2 = Vec::new();
        let mut v2 = Vec::new();
        for i in 0..12 {
            for j in 0..12 {
                let px = (i as f64 - 5.5) * 0.35;
                let py = (j as f64 - 5.5) * 0.35;
                let r = (px*px + py*py).sqrt().max(0.3);
                u2.push(px / r * 0.25);
                v2.push(py / r * 0.25);
                x2.push(px);
                y2.push(py);
            }
        }
        quiver2.set_data(x2, y2, u2, v2);
        quiver2.set_color(vec4(0.84, 0.15, 0.16, 1.0));
        quiver2.redraw(cx);

        // Gradient field
        let quiver3 = self.ui.quiver_plot(&[id!(detail_quiver_gradient)]);
        quiver3.set_title("Gradient Field");
        let mut x3 = Vec::new();
        let mut y3 = Vec::new();
        let mut u3 = Vec::new();
        let mut v3 = Vec::new();
        for i in 0..11 {
            for j in 0..11 {
                let px = (i as f64 - 5.0) * 0.4;
                let py = (j as f64 - 5.0) * 0.4;
                // Gradient of f(x,y) = sin(x)*cos(y)
                u3.push(px.cos() * py.cos() * 0.3);
                v3.push(-px.sin() * py.sin() * 0.3);
                x3.push(px);
                y3.push(py);
            }
        }
        quiver3.set_data(x3, y3, u3, v3);
        quiver3.set_color(vec4(0.17, 0.63, 0.17, 1.0));
        quiver3.redraw(cx);
    }

    fn setup_surface3d_detail(&mut self, cx: &mut Cx) {
        // Surface3D - Peaks function
        let surface1 = self.ui.surface3_d(&[id!(detail_surface_peaks)]);
        surface1.set_title("Peaks Function");
        let n = 80;
        let mut z1: Vec<Vec<f64>> = Vec::new();
        for j in 0..n {
            let mut row = Vec::new();
            for i in 0..n {
                let x = (i as f64 / (n - 1) as f64) * 6.0 - 3.0;
                let y = (j as f64 / (n - 1) as f64) * 6.0 - 3.0;
                let z = 3.0 * (1.0 - x).powi(2) * (-x.powi(2) - (y + 1.0).powi(2)).exp()
                    - 10.0 * (x / 5.0 - x.powi(3) - y.powi(5)) * (-x.powi(2) - y.powi(2)).exp()
                    - (1.0 / 3.0) * (-(x + 1.0).powi(2) - y.powi(2)).exp();
                row.push(z);
            }
            z1.push(row);
        }
        surface1.set_data(z1);
        surface1.set_x_range(-3.0, 3.0);
        surface1.set_y_range(-3.0, 3.0);
        surface1.set_wireframe(false);
        surface1.set_surface(true);
        surface1.set_colormap(Colormap::Viridis);
        surface1.redraw(cx);

        // Surface3D - Saddle
        let surface2 = self.ui.surface3_d(&[id!(detail_surface_saddle)]);
        surface2.set_title("Saddle Surface");
        let n2 = 80;
        let mut z2: Vec<Vec<f64>> = Vec::new();
        for j in 0..n2 {
            let mut row = Vec::new();
            for i in 0..n2 {
                let x = (i as f64 / (n2 - 1) as f64) * 4.0 - 2.0;
                let y = (j as f64 / (n2 - 1) as f64) * 4.0 - 2.0;
                row.push(x * x - y * y);
            }
            z2.push(row);
        }
        surface2.set_data(z2);
        surface2.set_x_range(-2.0, 2.0);
        surface2.set_y_range(-2.0, 2.0);
        surface2.set_wireframe(false);
        surface2.set_surface(true);
        surface2.set_colormap(Colormap::Plasma);
        surface2.redraw(cx);

        // Surface3D - Ripple
        let surface3 = self.ui.surface3_d(&[id!(detail_surface_ripple)]);
        surface3.set_title("Ripple Surface");
        let n3 = 100;
        let mut z3: Vec<Vec<f64>> = Vec::new();
        for j in 0..n3 {
            let mut row = Vec::new();
            for i in 0..n3 {
                let x = (i as f64 / (n3 - 1) as f64) * 6.0 - 3.0;
                let y = (j as f64 / (n3 - 1) as f64) * 6.0 - 3.0;
                let r = (x * x + y * y).sqrt();
                row.push((r * 2.0).sin() / (r + 0.5));
            }
            z3.push(row);
        }
        surface3.set_data(z3);
        surface3.set_x_range(-3.0, 3.0);
        surface3.set_y_range(-3.0, 3.0);
        surface3.set_wireframe(false);
        surface3.set_surface(true);
        surface3.set_colormap(Colormap::Coolwarm);
        surface3.redraw(cx);

        // Surface3D - Gaussian Hills
        let surface4 = self.ui.surface3_d(&[id!(detail_surface_gaussian)]);
        surface4.set_title("Gaussian Hills");
        let n4 = 80;
        let mut z4: Vec<Vec<f64>> = Vec::new();
        for j in 0..n4 {
            let mut row = Vec::new();
            for i in 0..n4 {
                let x = (i as f64 / (n4 - 1) as f64) * 6.0 - 3.0;
                let y = (j as f64 / (n4 - 1) as f64) * 6.0 - 3.0;
                let z = (-(x - 1.0).powi(2) - (y - 1.0).powi(2)).exp()
                    + 0.8 * (-(x + 1.5).powi(2) - (y + 1.0).powi(2)).exp()
                    + 0.6 * (-(x - 0.5).powi(2) - (y + 1.5).powi(2)).exp();
                row.push(z);
            }
            z4.push(row);
        }
        surface4.set_data(z4);
        surface4.set_x_range(-3.0, 3.0);
        surface4.set_y_range(-3.0, 3.0);
        surface4.set_wireframe(false);
        surface4.set_surface(true);
        surface4.set_colormap(Colormap::Blues);
        surface4.redraw(cx);

        // Surface3D - Sinc Function
        let surface5 = self.ui.surface3_d(&[id!(detail_surface_sinc)]);
        surface5.set_title("Sinc Function");
        let n5 = 80;
        let mut z5: Vec<Vec<f64>> = Vec::new();
        for j in 0..n5 {
            let mut row = Vec::new();
            for i in 0..n5 {
                let x = (i as f64 / (n5 - 1) as f64) * 10.0 - 5.0;
                let y = (j as f64 / (n5 - 1) as f64) * 10.0 - 5.0;
                let r = (x * x + y * y).sqrt().max(0.001);
                row.push(r.sin() / r);
            }
            z5.push(row);
        }
        surface5.set_data(z5);
        surface5.set_x_range(-5.0, 5.0);
        surface5.set_y_range(-5.0, 5.0);
        surface5.set_wireframe(false);
        surface5.set_surface(true);
        surface5.set_colormap(Colormap::Plasma);
        surface5.redraw(cx);

        // Surface3D - Wireframe View
        let surface6 = self.ui.surface3_d(&[id!(detail_surface_wireframe)]);
        surface6.set_title("Wireframe Mode");
        let n6 = 40;
        let mut z6: Vec<Vec<f64>> = Vec::new();
        for j in 0..n6 {
            let mut row = Vec::new();
            for i in 0..n6 {
                let x = (i as f64 / (n6 - 1) as f64) * 4.0 - 2.0;
                let y = (j as f64 / (n6 - 1) as f64) * 4.0 - 2.0;
                row.push((x * 2.0).sin() * (y * 2.0).cos());
            }
            z6.push(row);
        }
        surface6.set_data(z6);
        surface6.set_x_range(-2.0, 2.0);
        surface6.set_y_range(-2.0, 2.0);
        surface6.set_wireframe(true);
        surface6.set_surface(false);
        surface6.set_colormap(Colormap::Viridis);
        surface6.redraw(cx);
    }

    fn setup_scatter3d_detail(&mut self, cx: &mut Cx) {
        // Scatter3D - Random clusters
        let scatter1 = self.ui.scatter3_d(&[id!(detail_scatter3d_random)]);
        scatter1.set_title("3D Clusters");
        let mut x_cluster = Vec::new();
        let mut y_cluster = Vec::new();
        let mut z_cluster = Vec::new();
        let centers = [(0.0, 0.0, 0.0), (2.0, 2.0, 2.0), (-1.5, 1.5, -1.0)];
        for (cx_c, cy_c, cz_c) in centers.iter() {
            for k in 0..30 {
                let angle = k as f64 * 0.2;
                let r = (k as f64 * 0.1).sin().abs() * 0.8;
                x_cluster.push(cx_c + r * angle.cos());
                y_cluster.push(cy_c + r * angle.sin());
                z_cluster.push(cz_c + (k as f64 * 0.15).sin() * 0.5);
            }
        }
        scatter1.set_data(x_cluster, y_cluster, z_cluster);
        scatter1.set_color(vec4(0.12, 0.47, 0.71, 1.0));
        scatter1.redraw(cx);

        // Scatter3D - Double helix
        let scatter2 = self.ui.scatter3_d(&[id!(detail_scatter3d_helix)]);
        scatter2.set_title("Double Helix");
        let mut x_helix = Vec::new();
        let mut y_helix = Vec::new();
        let mut z_helix = Vec::new();
        for k in 0..80 {
            let t = k as f64 * 0.1;
            x_helix.push(t.cos());
            y_helix.push(t.sin());
            z_helix.push(t * 0.2);
            x_helix.push((t + std::f64::consts::PI).cos());
            y_helix.push((t + std::f64::consts::PI).sin());
            z_helix.push(t * 0.2);
        }
        scatter2.set_data(x_helix, y_helix, z_helix);
        scatter2.set_color(vec4(0.84, 0.15, 0.16, 1.0));
        scatter2.redraw(cx);

        // Scatter3D - Spherical Shell
        let scatter3 = self.ui.scatter3_d(&[id!(detail_scatter3d_sphere)]);
        scatter3.set_title("Spherical Shell");
        let mut x_sphere = Vec::new();
        let mut y_sphere = Vec::new();
        let mut z_sphere = Vec::new();
        for i in 0..200 {
            let phi = i as f64 * 0.1;
            let theta = i as f64 * 0.31;
            let r = 2.0;
            x_sphere.push(r * phi.sin() * theta.cos());
            y_sphere.push(r * phi.sin() * theta.sin());
            z_sphere.push(r * phi.cos());
        }
        scatter3.set_data(x_sphere, y_sphere, z_sphere);
        scatter3.set_color(vec4(0.17, 0.63, 0.17, 1.0));
        scatter3.redraw(cx);

        // Scatter3D - Gaussian Cloud
        let scatter4 = self.ui.scatter3_d(&[id!(detail_scatter3d_gaussian)]);
        scatter4.set_title("Gaussian Cloud");
        let mut x_gauss = Vec::new();
        let mut y_gauss = Vec::new();
        let mut z_gauss = Vec::new();
        for i in 0..150 {
            let t = i as f64 * 0.1;
            let r = (t * 0.3).sin().abs() * 2.0;
            x_gauss.push(r * (t * 2.0).cos());
            y_gauss.push(r * (t * 2.0).sin());
            z_gauss.push((t * 0.5).sin() * 2.0);
        }
        scatter4.set_data(x_gauss, y_gauss, z_gauss);
        scatter4.set_color(vec4(0.58, 0.40, 0.74, 1.0));
        scatter4.redraw(cx);

        // Scatter3D - Spiral Galaxy
        let scatter5 = self.ui.scatter3_d(&[id!(detail_scatter3d_spiral)]);
        scatter5.set_title("Spiral Galaxy");
        let mut x_spiral = Vec::new();
        let mut y_spiral = Vec::new();
        let mut z_spiral = Vec::new();
        for arm in 0..3 {
            let offset = arm as f64 * 2.094; // 2*PI/3
            for i in 0..60 {
                let t = i as f64 * 0.15;
                let r = 0.3 + t * 0.4;
                x_spiral.push(r * (t + offset).cos());
                y_spiral.push(r * (t + offset).sin());
                z_spiral.push((i as f64 * 0.05).sin() * 0.3);
            }
        }
        scatter5.set_data(x_spiral, y_spiral, z_spiral);
        scatter5.set_color(vec4(1.0, 0.5, 0.0, 1.0));
        scatter5.redraw(cx);

        // Scatter3D - Torus Distribution
        let scatter6 = self.ui.scatter3_d(&[id!(detail_scatter3d_torus)]);
        scatter6.set_title("Torus Distribution");
        let mut x_torus = Vec::new();
        let mut y_torus = Vec::new();
        let mut z_torus = Vec::new();
        let major_r = 2.0;
        let minor_r = 0.8;
        for i in 0..150 {
            let u = i as f64 * 0.15;
            let v = i as f64 * 0.47;
            x_torus.push((major_r + minor_r * v.cos()) * u.cos());
            y_torus.push((major_r + minor_r * v.cos()) * u.sin());
            z_torus.push(minor_r * v.sin());
        }
        scatter6.set_data(x_torus, y_torus, z_torus);
        scatter6.set_color(vec4(0.89, 0.47, 0.76, 1.0));
        scatter6.redraw(cx);
    }

    fn setup_line3d_detail(&mut self, cx: &mut Cx) {
        // Line3D - Lorenz attractor
        let line1 = self.ui.line3_d(&[id!(detail_line3d_lorenz)]);
        line1.set_title("Lorenz Attractor");
        let mut x_l = Vec::new();
        let mut y_l = Vec::new();
        let mut z_l = Vec::new();
        let sigma = 10.0;
        let rho = 28.0;
        let beta = 8.0 / 3.0;
        let dt = 0.01;
        let mut x = 1.0;
        let mut y = 1.0;
        let mut z = 1.0;
        for _ in 0..3000 {
            let dx = sigma * (y - x) * dt;
            let dy = (x * (rho - z) - y) * dt;
            let dz = (x * y - beta * z) * dt;
            x += dx;
            y += dy;
            z += dz;
            x_l.push(x);
            y_l.push(y);
            z_l.push(z);
        }
        line1.add_series(
            Line3DSeries::new("Lorenz")
                .with_data(x_l, y_l, z_l)
                .with_color(vec4(0.58, 0.40, 0.74, 1.0))
        );
        line1.redraw(cx);

        // Line3D - Trefoil Knot
        let line2 = self.ui.line3_d(&[id!(detail_line3d_trefoil)]);
        line2.set_title("Trefoil Knot");
        let mut x_t = Vec::new();
        let mut y_t = Vec::new();
        let mut z_t = Vec::new();
        for i in 0..500 {
            let t = i as f64 * 0.02 * std::f64::consts::PI;
            x_t.push((2.0 + (2.0 * t).cos()) * t.cos());
            y_t.push((2.0 + (2.0 * t).cos()) * t.sin());
            z_t.push((2.0 * t).sin());
        }
        line2.add_series(
            Line3DSeries::new("Trefoil")
                .with_data(x_t, y_t, z_t)
                .with_color(vec4(0.12, 0.47, 0.71, 1.0))
        );
        line2.redraw(cx);

        // Line3D - Lissajous Curve
        let line3 = self.ui.line3_d(&[id!(detail_line3d_lissajous)]);
        line3.set_title("Lissajous Curve");
        let mut x_li = Vec::new();
        let mut y_li = Vec::new();
        let mut z_li = Vec::new();
        for i in 0..500 {
            let t = i as f64 * 0.02 * std::f64::consts::PI;
            x_li.push((3.0 * t).sin() * 2.0);
            y_li.push((4.0 * t).sin() * 2.0);
            z_li.push((5.0 * t).sin() * 2.0);
        }
        line3.add_series(
            Line3DSeries::new("Lissajous")
                .with_data(x_li, y_li, z_li)
                .with_color(vec4(0.84, 0.15, 0.16, 1.0))
        );
        line3.redraw(cx);

        // Line3D - Spring Helix
        let line4 = self.ui.line3_d(&[id!(detail_line3d_spring)]);
        line4.set_title("Spring Helix");
        let mut x_s = Vec::new();
        let mut y_s = Vec::new();
        let mut z_s = Vec::new();
        for i in 0..400 {
            let t = i as f64 * 0.05;
            x_s.push(t.cos() * 2.0);
            y_s.push(t.sin() * 2.0);
            z_s.push(t * 0.3);
        }
        line4.add_series(
            Line3DSeries::new("Spring")
                .with_data(x_s, y_s, z_s)
                .with_color(vec4(0.17, 0.63, 0.17, 1.0))
        );
        line4.redraw(cx);

        // Line3D - Figure-8 Knot
        let line5 = self.ui.line3_d(&[id!(detail_line3d_figure8)]);
        line5.set_title("Figure-8 Knot");
        let mut x_f = Vec::new();
        let mut y_f = Vec::new();
        let mut z_f = Vec::new();
        for i in 0..500 {
            let t = i as f64 * 0.02 * std::f64::consts::PI;
            x_f.push((2.0 + (2.0 * t).cos()) * (3.0 * t).cos());
            y_f.push((2.0 + (2.0 * t).cos()) * (3.0 * t).sin());
            z_f.push((4.0 * t).sin());
        }
        line5.add_series(
            Line3DSeries::new("Figure8")
                .with_data(x_f, y_f, z_f)
                .with_color(vec4(1.0, 0.5, 0.0, 1.0))
        );
        line5.redraw(cx);

        // Line3D - Torus Knot
        let line6 = self.ui.line3_d(&[id!(detail_line3d_torus)]);
        line6.set_title("Torus Knot (3,7)");
        let mut x_tk = Vec::new();
        let mut y_tk = Vec::new();
        let mut z_tk = Vec::new();
        let p = 3.0;
        let q = 7.0;
        for i in 0..500 {
            let t = i as f64 * 0.02 * std::f64::consts::PI;
            let r = 2.0 + (q * t).cos();
            x_tk.push(r * (p * t).cos());
            y_tk.push(r * (p * t).sin());
            z_tk.push((q * t).sin());
        }
        line6.add_series(
            Line3DSeries::new("TorusKnot")
                .with_data(x_tk, y_tk, z_tk)
                .with_color(vec4(0.89, 0.47, 0.76, 1.0))
        );
        line6.redraw(cx);
    }

    fn setup_subplot_detail(&mut self, cx: &mut Cx) {
        // 2x2 Grid: Line Plot
        let subplot_line = self.ui.line_plot(&[id!(detail_subplot_line)]);
        let x: Vec<f64> = (0..50).map(|i| i as f64 * 0.15).collect();
        let y: Vec<f64> = x.iter().map(|&xi| xi.sin()).collect();
        subplot_line.add_series(Series::new("Sine").with_data(x, y).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        subplot_line.redraw(cx);

        // 2x2 Grid: Scatter Plot
        let subplot_scatter = self.ui.scatter_plot(&[id!(detail_subplot_scatter)]);
        let x_s: Vec<f64> = (0..30).map(|i| (i as f64 * 0.37).sin() * 3.0 + 5.0).collect();
        let y_s: Vec<f64> = (0..30).map(|i| (i as f64 * 0.23).cos() * 2.0 + 3.0).collect();
        subplot_scatter.add_series(Series::new("Scatter").with_data(x_s, y_s).with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        subplot_scatter.redraw(cx);

        // 2x2 Grid: Bar Plot
        let subplot_bar = self.ui.bar_plot(&[id!(detail_subplot_bar)]);
        subplot_bar.set_data(
            vec!["Mon".into(), "Tue".into(), "Wed".into(), "Thu".into(), "Fri".into()],
            vec![5.0, 8.0, 6.0, 9.0, 7.0]
        );
        subplot_bar.redraw(cx);

        // 2x2 Grid: Area Plot (line plot)
        let subplot_area = self.ui.line_plot(&[id!(detail_subplot_area)]);
        let x_a: Vec<f64> = (0..40).map(|i| i as f64 * 0.2).collect();
        let y_a: Vec<f64> = x_a.iter().map(|&xi| (xi * 0.5).sin().abs() * 3.0 + 1.0).collect();
        subplot_area.add_series(Series::new("Area").with_data(x_a, y_a).with_color(vec4(0.17, 0.63, 0.17, 1.0)));
        subplot_area.redraw(cx);

        // 1x3 Strip: Time Series 1
        let ts1 = self.ui.line_plot(&[id!(detail_subplot_ts1)]);
        let x_ts1: Vec<f64> = (0..50).map(|i| i as f64).collect();
        let y_ts1: Vec<f64> = x_ts1.iter().map(|&xi| (xi * 0.2).sin() * 10.0 + 50.0).collect();
        ts1.add_series(Series::new("Series A").with_data(x_ts1, y_ts1).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        ts1.set_title("Sensor A");
        ts1.redraw(cx);

        // 1x3 Strip: Time Series 2
        let ts2 = self.ui.line_plot(&[id!(detail_subplot_ts2)]);
        let x_ts2: Vec<f64> = (0..50).map(|i| i as f64).collect();
        let y_ts2: Vec<f64> = x_ts2.iter().map(|&xi| (xi * 0.15).cos() * 15.0 + 45.0).collect();
        ts2.add_series(Series::new("Series B").with_data(x_ts2, y_ts2).with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        ts2.set_title("Sensor B");
        ts2.redraw(cx);

        // 1x3 Strip: Time Series 3
        let ts3 = self.ui.line_plot(&[id!(detail_subplot_ts3)]);
        let x_ts3: Vec<f64> = (0..50).map(|i| i as f64).collect();
        let y_ts3: Vec<f64> = x_ts3.iter().map(|&xi| (xi * 0.25).sin() * 8.0 + (xi * 0.1).cos() * 5.0 + 55.0).collect();
        ts3.add_series(Series::new("Series C").with_data(x_ts3, y_ts3).with_color(vec4(0.17, 0.63, 0.17, 1.0)));
        ts3.set_title("Sensor C");
        ts3.redraw(cx);
    }

    fn setup_dualaxis_detail(&mut self, cx: &mut Cx) {
        // Dual Y-Axis - Temperature & Humidity
        let twinx1 = self.ui.line_plot_dual(&[id!(detail_twinx_temp)]);
        twinx1.set_title("Weather Data");
        twinx1.set_xlabel("Hour");
        twinx1.set_ylabel("Temperature (C)");
        twinx1.set_y2label("Humidity (%)");
        let hours: Vec<f64> = (0..24).map(|i| i as f64).collect();
        let temp: Vec<f64> = hours.iter().map(|&h| 15.0 + 10.0 * ((h - 14.0) * 0.26).cos()).collect();
        let humidity: Vec<f64> = hours.iter().map(|&h| 60.0 - 20.0 * ((h - 6.0) * 0.26).cos()).collect();
        twinx1.add_series_left(Series::new("Temperature").with_data(hours.clone(), temp).with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        twinx1.add_series_right(Series::new("Humidity").with_data(hours, humidity).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        twinx1.set_grid(true);
        twinx1.set_legend(LegendPosition::TopRight);
        twinx1.redraw(cx);

        // Dual Y-Axis - Sales & Growth
        let twinx2 = self.ui.line_plot_dual(&[id!(detail_twinx_sales)]);
        twinx2.set_title("Business Metrics");
        twinx2.set_xlabel("Month");
        twinx2.set_ylabel("Sales ($K)");
        twinx2.set_y2label("Growth (%)");
        let months: Vec<f64> = (1..=12).map(|i| i as f64).collect();
        let sales: Vec<f64> = vec![100.0, 120.0, 115.0, 140.0, 155.0, 170.0, 180.0, 175.0, 195.0, 210.0, 225.0, 250.0];
        let growth: Vec<f64> = vec![0.0, 20.0, -4.2, 21.7, 10.7, 9.7, 5.9, -2.8, 11.4, 7.7, 7.1, 11.1];
        twinx2.add_series_left(Series::new("Sales").with_data(months.clone(), sales).with_color(vec4(0.17, 0.63, 0.17, 1.0)));
        twinx2.add_series_right(Series::new("Growth").with_data(months, growth).with_color(vec4(1.0, 0.5, 0.0, 1.0)));
        twinx2.set_grid(true);
        twinx2.set_legend(LegendPosition::TopLeft);
        twinx2.redraw(cx);

        // Dual Y-Axis - Stock Price & Volume
        let twinx3 = self.ui.line_plot_dual(&[id!(detail_twinx_stock)]);
        twinx3.set_title("Stock Analysis");
        twinx3.set_xlabel("Day");
        twinx3.set_ylabel("Price ($)");
        twinx3.set_y2label("Volume (M)");
        let days: Vec<f64> = (1..=30).map(|i| i as f64).collect();
        let price: Vec<f64> = days.iter().map(|&d| 100.0 + (d * 0.3).sin() * 15.0 + d * 0.5).collect();
        let volume: Vec<f64> = days.iter().map(|&d| 5.0 + (d * 0.5).cos().abs() * 8.0).collect();
        twinx3.add_series_left(Series::new("Price").with_data(days.clone(), price).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        twinx3.add_series_right(Series::new("Volume").with_data(days, volume).with_color(vec4(0.58, 0.40, 0.74, 1.0)));
        twinx3.set_grid(true);
        twinx3.set_legend(LegendPosition::TopRight);
        twinx3.redraw(cx);

        // Dual Y-Axis - Pressure & Altitude
        let twinx4 = self.ui.line_plot_dual(&[id!(detail_twinx_pressure)]);
        twinx4.set_title("Atmospheric Data");
        twinx4.set_xlabel("Distance (km)");
        twinx4.set_ylabel("Pressure (hPa)");
        twinx4.set_y2label("Altitude (m)");
        let distance: Vec<f64> = (0..50).map(|i| i as f64 * 2.0).collect();
        let pressure: Vec<f64> = distance.iter().map(|&d| 1013.0 - d * 0.5 + (d * 0.1).sin() * 5.0).collect();
        let altitude: Vec<f64> = distance.iter().map(|&d| 100.0 + d * 8.0 + (d * 0.15).cos() * 50.0).collect();
        twinx4.add_series_left(Series::new("Pressure").with_data(distance.clone(), pressure).with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        twinx4.add_series_right(Series::new("Altitude").with_data(distance, altitude).with_color(vec4(0.17, 0.63, 0.17, 1.0)));
        twinx4.set_grid(true);
        twinx4.set_legend(LegendPosition::TopLeft);
        twinx4.redraw(cx);
    }

    fn setup_logscales_detail(&mut self, cx: &mut Cx) {
        // Log Scale - Exponential Growth (y = e^x)
        let log_exp = self.ui.line_plot(&[id!(detail_log_exp)]);
        log_exp.set_title("Exponential Growth (Log Y)");
        let x: Vec<f64> = (0..50).map(|i| i as f64 * 0.2).collect();
        let y: Vec<f64> = x.iter().map(|&xi| (xi * 0.5).exp()).collect();
        log_exp.add_series(Series::new("e^(0.5x)").with_data(x, y).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        log_exp.set_y_scale(ScaleType::Log);
        log_exp.set_interactive(true);
        log_exp.redraw(cx);

        // Log Scale - Power Law (y = x^3)
        let log_power = self.ui.line_plot(&[id!(detail_log_power)]);
        log_power.set_title("Power Law (Log Y)");
        let x: Vec<f64> = (1..100).map(|i| i as f64 * 0.5).collect();
        let y: Vec<f64> = x.iter().map(|&xi| xi.powi(3)).collect();
        log_power.add_series(Series::new("x³").with_data(x, y).with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        log_power.set_y_scale(ScaleType::Log);
        log_power.set_interactive(true);
        log_power.redraw(cx);

        // Log-Log Scale (both axes logarithmic)
        let log_log = self.ui.line_plot(&[id!(detail_log_log)]);
        log_log.set_title("Log-Log Scale");
        let x: Vec<f64> = (1..100).map(|i| i as f64).collect();
        let y: Vec<f64> = x.iter().map(|&xi| xi.powf(2.5)).collect();
        log_log.add_series(Series::new("x^2.5").with_data(x, y).with_color(vec4(0.17, 0.63, 0.17, 1.0)));
        log_log.set_x_scale(ScaleType::Log);
        log_log.set_y_scale(ScaleType::Log);
        log_log.set_interactive(true);
        log_log.redraw(cx);

        // SymLog Scale - Data crossing zero
        let symlog = self.ui.line_plot(&[id!(detail_symlog)]);
        symlog.set_title("SymLog Scale (Handles Negative)");
        let x: Vec<f64> = (-100..=100).map(|i| i as f64 * 0.1).collect();
        let y: Vec<f64> = x.iter().map(|&xi| xi.powi(3)).collect();
        symlog.add_series(Series::new("x³").with_data(x.clone(), y).with_color(vec4(0.58, 0.40, 0.74, 1.0)));
        // Also add a linear reference line
        symlog.add_series(Series::new("x").with_data(x.clone(), x).with_color(vec4(0.5, 0.5, 0.5, 0.5)));
        symlog.set_y_scale(ScaleType::SymLog);
        symlog.set_interactive(true);
        symlog.redraw(cx);

        // Semi-Log X Scale
        let semilog_x = self.ui.line_plot(&[id!(detail_semilog_x)]);
        semilog_x.set_title("Semi-Log X Scale");
        let x: Vec<f64> = (1..100).map(|i| i as f64).collect();
        let y: Vec<f64> = x.iter().map(|&xi| xi.ln() * 10.0).collect();
        semilog_x.add_series(Series::new("10·ln(x)").with_data(x, y).with_color(vec4(1.0, 0.5, 0.0, 1.0)));
        semilog_x.set_x_scale(ScaleType::Log);
        semilog_x.set_interactive(true);
        semilog_x.redraw(cx);

        // Decibel Scale (common in audio/signal processing)
        let decibel = self.ui.line_plot(&[id!(detail_decibel)]);
        decibel.set_title("Decibel Scale");
        let x: Vec<f64> = (1..100).map(|i| i as f64 * 0.01).collect();
        let y: Vec<f64> = x.iter().map(|&xi| 20.0 * xi.log10()).collect();
        decibel.add_series(Series::new("20·log₁₀(x)").with_data(x, y).with_color(vec4(0.89, 0.47, 0.76, 1.0)));
        decibel.set_interactive(true);
        decibel.redraw(cx);
    }

    fn setup_timescale_detail(&mut self, cx: &mut Cx) {
        let base_time = 1704067200.0; // Jan 1, 2024 00:00:00 UTC

        // Daily Time Series
        let time_daily = self.ui.line_plot(&[id!(detail_time_daily)]);
        time_daily.set_title("Daily Time Series");
        let x: Vec<f64> = (0..7).map(|i| base_time + i as f64 * 86400.0).collect();
        let y: Vec<f64> = vec![100.0, 105.0, 103.0, 108.0, 112.0, 110.0, 115.0];
        time_daily.add_series(Series::new("Daily").with_data(x, y).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        time_daily.set_x_scale(ScaleType::Time);
        time_daily.set_interactive(true);
        time_daily.redraw(cx);

        // Weekly Time Series
        let time_weekly = self.ui.line_plot(&[id!(detail_time_weekly)]);
        time_weekly.set_title("Weekly Time Series");
        let x: Vec<f64> = (0..8).map(|i| base_time + i as f64 * 7.0 * 86400.0).collect();
        let y: Vec<f64> = vec![200.0, 215.0, 210.0, 225.0, 240.0, 235.0, 250.0, 260.0];
        time_weekly.add_series(Series::new("Weekly").with_data(x, y).with_color(vec4(0.17, 0.63, 0.17, 1.0)));
        time_weekly.set_x_scale(ScaleType::Time);
        time_weekly.set_interactive(true);
        time_weekly.redraw(cx);

        // Monthly Time Series
        let time_monthly = self.ui.line_plot(&[id!(detail_time_monthly)]);
        time_monthly.set_title("Monthly Time Series");
        let x: Vec<f64> = (0..12).map(|i| base_time + i as f64 * 30.0 * 86400.0).collect();
        let y: Vec<f64> = vec![50.0, 55.0, 60.0, 70.0, 85.0, 100.0, 95.0, 90.0, 75.0, 65.0, 55.0, 52.0];
        time_monthly.add_series(Series::new("Monthly").with_data(x, y).with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        time_monthly.set_x_scale(ScaleType::Time);
        time_monthly.set_interactive(true);
        time_monthly.redraw(cx);

        // Hourly Data
        let time_hourly = self.ui.line_plot(&[id!(detail_time_hourly)]);
        time_hourly.set_title("Hourly Data");
        let x: Vec<f64> = (0..24).map(|i| base_time + i as f64 * 3600.0).collect();
        let y: Vec<f64> = (0..24).map(|i| 20.0 + 10.0 * ((i as f64 - 14.0) * 0.26).cos()).collect();
        time_hourly.add_series(Series::new("Hourly").with_data(x, y).with_color(vec4(0.58, 0.40, 0.74, 1.0)));
        time_hourly.set_x_scale(ScaleType::Time);
        time_hourly.set_interactive(true);
        time_hourly.redraw(cx);

        // Multi-Year Trend
        let time_yearly = self.ui.line_plot(&[id!(detail_time_yearly)]);
        time_yearly.set_title("Multi-Year Trend");
        let x: Vec<f64> = (0..5).map(|i| base_time + i as f64 * 365.0 * 86400.0).collect();
        let y: Vec<f64> = vec![1000.0, 1200.0, 1500.0, 1900.0, 2500.0];
        time_yearly.add_series(Series::new("Yearly").with_data(x, y).with_color(vec4(1.0, 0.5, 0.0, 1.0)));
        time_yearly.set_x_scale(ScaleType::Time);
        time_yearly.set_interactive(true);
        time_yearly.redraw(cx);

        // Minute-Level Data
        let time_minute = self.ui.line_plot(&[id!(detail_time_minute)]);
        time_minute.set_title("Minute-Level Data");
        let x: Vec<f64> = (0..60).map(|i| base_time + i as f64 * 60.0).collect();
        let y: Vec<f64> = (0..60).map(|i| 50.0 + (i as f64 * 0.2).sin() * 20.0 + (i as f64 * 0.05).cos() * 10.0).collect();
        time_minute.add_series(Series::new("Per Minute").with_data(x, y).with_color(vec4(0.89, 0.47, 0.76, 1.0)));
        time_minute.set_x_scale(ScaleType::Time);
        time_minute.set_interactive(true);
        time_minute.redraw(cx);
    }

    fn setup_colormaps_detail(&mut self, cx: &mut Cx) {
        // Generate sample data - 2D Gaussian
        let size = 20;
        let generate_data = || -> Vec<Vec<f64>> {
            (0..size).map(|i| {
                (0..size).map(|j| {
                    let x = (i as f64 - size as f64 / 2.0) / (size as f64 / 4.0);
                    let y = (j as f64 - size as f64 / 2.0) / (size as f64 / 4.0);
                    (-x * x - y * y).exp()
                }).collect()
            }).collect()
        };
        let data = generate_data();

        // Perceptually Uniform Colormaps
        let viridis = self.ui.heatmap_chart(&[id!(detail_cmap_viridis)]);
        viridis.set_title("Viridis");
        viridis.set_data(data.clone());
        viridis.set_colormap(Colormap::Viridis);
        viridis.set_show_values(false);
        viridis.redraw(cx);

        let plasma = self.ui.heatmap_chart(&[id!(detail_cmap_plasma)]);
        plasma.set_title("Plasma");
        plasma.set_data(data.clone());
        plasma.set_colormap(Colormap::Plasma);
        plasma.set_show_values(false);
        plasma.redraw(cx);

        let inferno = self.ui.heatmap_chart(&[id!(detail_cmap_inferno)]);
        inferno.set_title("Inferno");
        inferno.set_data(data.clone());
        inferno.set_colormap(Colormap::Inferno);
        inferno.set_show_values(false);
        inferno.redraw(cx);

        let magma = self.ui.heatmap_chart(&[id!(detail_cmap_magma)]);
        magma.set_title("Magma");
        magma.set_data(data.clone());
        magma.set_colormap(Colormap::Magma);
        magma.set_show_values(false);
        magma.redraw(cx);

        // Diverging & Colorblind-Friendly
        let cividis = self.ui.heatmap_chart(&[id!(detail_cmap_cividis)]);
        cividis.set_title("Cividis (Colorblind)");
        cividis.set_data(data.clone());
        cividis.set_colormap(Colormap::Cividis);
        cividis.set_show_values(false);
        cividis.redraw(cx);

        let coolwarm = self.ui.heatmap_chart(&[id!(detail_cmap_coolwarm)]);
        coolwarm.set_title("Coolwarm");
        coolwarm.set_data(data.clone());
        coolwarm.set_colormap(Colormap::Coolwarm);
        coolwarm.set_show_values(false);
        coolwarm.redraw(cx);

        let rdbu = self.ui.heatmap_chart(&[id!(detail_cmap_rdbu)]);
        rdbu.set_title("RdBu (Red-Blue)");
        rdbu.set_data(data.clone());
        rdbu.set_colormap(Colormap::RdBu);
        rdbu.set_show_values(false);
        rdbu.redraw(cx);

        let spectral = self.ui.heatmap_chart(&[id!(detail_cmap_spectral)]);
        spectral.set_title("Spectral");
        spectral.set_data(data.clone());
        spectral.set_colormap(Colormap::Spectral);
        spectral.set_show_values(false);
        spectral.redraw(cx);

        // Sequential Colormaps
        let blues = self.ui.heatmap_chart(&[id!(detail_cmap_blues)]);
        blues.set_title("Blues");
        blues.set_data(data.clone());
        blues.set_colormap(Colormap::Blues);
        blues.set_show_values(false);
        blues.redraw(cx);

        let greens = self.ui.heatmap_chart(&[id!(detail_cmap_greens)]);
        greens.set_title("Greens");
        greens.set_data(data.clone());
        greens.set_colormap(Colormap::Greens);
        greens.set_show_values(false);
        greens.redraw(cx);

        let oranges = self.ui.heatmap_chart(&[id!(detail_cmap_oranges)]);
        oranges.set_title("Oranges");
        oranges.set_data(data.clone());
        oranges.set_colormap(Colormap::Oranges);
        oranges.set_show_values(false);
        oranges.redraw(cx);

        let reds = self.ui.heatmap_chart(&[id!(detail_cmap_reds)]);
        reds.set_title("Reds");
        reds.set_data(data.clone());
        reds.set_colormap(Colormap::Reds);
        reds.set_show_values(false);
        reds.redraw(cx);

        // Classic & Special
        let jet = self.ui.heatmap_chart(&[id!(detail_cmap_jet)]);
        jet.set_title("Jet (Rainbow)");
        jet.set_data(data.clone());
        jet.set_colormap(Colormap::Jet);
        jet.set_show_values(false);
        jet.redraw(cx);

        let hot = self.ui.heatmap_chart(&[id!(detail_cmap_hot)]);
        hot.set_title("Hot");
        hot.set_data(data.clone());
        hot.set_colormap(Colormap::Hot);
        hot.set_show_values(false);
        hot.redraw(cx);

        let turbo = self.ui.heatmap_chart(&[id!(detail_cmap_turbo)]);
        turbo.set_title("Turbo");
        turbo.set_data(data.clone());
        turbo.set_colormap(Colormap::Turbo);
        turbo.set_show_values(false);
        turbo.redraw(cx);

        let greys = self.ui.heatmap_chart(&[id!(detail_cmap_greys)]);
        greys.set_title("Greys");
        greys.set_data(data);
        greys.set_colormap(Colormap::Greys);
        greys.set_show_values(false);
        greys.redraw(cx);
    }

    fn setup_lognorm_detail(&mut self, cx: &mut Cx) {
        // Generate exponential data for log normalization demo
        let size = 20;
        let exp_data: Vec<Vec<f64>> = (0..size).map(|i| {
            (0..size).map(|j| {
                let x = i as f64 / size as f64 * 4.0;
                let y = j as f64 / size as f64 * 4.0;
                (x + y).exp() / 10.0  // Values range from ~0.1 to ~300
            }).collect()
        }).collect();

        // Linear Norm (default) - small values get compressed
        let linear_norm = self.ui.heatmap_chart(&[id!(detail_norm_linear)]);
        linear_norm.set_title("Linear Normalization");
        linear_norm.set_data(exp_data.clone());
        linear_norm.set_colormap(Colormap::Viridis);
        linear_norm.set_show_values(false);
        linear_norm.redraw(cx);

        // Log Norm - reveals structure in small values
        let log_norm = self.ui.heatmap_chart(&[id!(detail_norm_log)]);
        log_norm.set_title("Log Normalization");
        // For log norm, we transform the data
        let log_data: Vec<Vec<f64>> = exp_data.iter().map(|row| {
            row.iter().map(|&v| v.max(0.001).log10()).collect()
        }).collect();
        log_norm.set_data(log_data);
        log_norm.set_colormap(Colormap::Viridis);
        log_norm.set_show_values(false);
        log_norm.redraw(cx);

        // Comparison with different colormap
        let compare = self.ui.heatmap_chart(&[id!(detail_norm_compare)]);
        compare.set_title("Original Data - Plasma");
        compare.set_data(exp_data.clone());
        compare.set_colormap(Colormap::Plasma);
        compare.set_show_values(false);
        compare.redraw(cx);

        // Surface plots with different colormaps
        let surface_viridis = self.ui.surface3_d(&[id!(detail_surface_viridis)]);
        surface_viridis.set_title("Viridis Surface");
        let surface_data: Vec<Vec<f64>> = (0..30).map(|i| {
            (0..30).map(|j| {
                let x = (i as f64 - 15.0) / 5.0;
                let y = (j as f64 - 15.0) / 5.0;
                (x * x + y * y).sqrt().sin() * 2.0
            }).collect()
        }).collect();
        surface_viridis.set_data(surface_data.clone());
        surface_viridis.set_colormap(Colormap::Viridis);
        surface_viridis.set_surface(true);  // Enable colored surface rendering
        surface_viridis.redraw(cx);

        let surface_plasma = self.ui.surface3_d(&[id!(detail_surface_plasma)]);
        surface_plasma.set_title("Plasma Surface");
        surface_plasma.set_data(surface_data.clone());
        surface_plasma.set_colormap(Colormap::Plasma);
        surface_plasma.set_surface(true);  // Enable colored surface rendering
        surface_plasma.redraw(cx);

        let surface_coolwarm = self.ui.surface3_d(&[id!(detail_surface_coolwarm)]);
        surface_coolwarm.set_title("Coolwarm Surface");
        surface_coolwarm.set_data(surface_data);
        surface_coolwarm.set_colormap(Colormap::Coolwarm);
        surface_coolwarm.set_surface(true);  // Enable colored surface rendering
        surface_coolwarm.redraw(cx);
    }

    fn setup_annotations_detail(&mut self, cx: &mut Cx) {
        // Sample data for all annotation demos
        let x: Vec<f64> = (0..50).map(|i| i as f64 * 0.2).collect();
        let y: Vec<f64> = x.iter().map(|&v| (v * 0.5).sin() * 3.0 + 5.0 + (v * 0.1).cos()).collect();

        // ===================
        // TEXT ANNOTATIONS
        // ===================

        // Text Labels
        let text_plot = self.ui.line_plot(&[id!(detail_text_annotations)]);
        text_plot.set_title("Text Annotations");
        text_plot.add_series(Series::new("Signal")
            .with_data(x.clone(), y.clone())
            .with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        text_plot.annotate("Start Region", 2.0, 6.5, vec4(0.1, 0.1, 0.1, 1.0), 10.0);
        text_plot.annotate("Peak", 5.0, 7.8, vec4(0.8, 0.2, 0.2, 1.0), 10.0);
        text_plot.annotate("Trough", 7.5, 3.5, vec4(0.2, 0.6, 0.2, 1.0), 10.0);
        text_plot.set_legend(LegendPosition::TopRight);
        text_plot.redraw(cx);

        // Arrow Annotations
        let arrow_plot = self.ui.line_plot(&[id!(detail_arrow_annotations)]);
        arrow_plot.set_title("Arrow Annotations");
        arrow_plot.add_series(Series::new("Data")
            .with_data(x.clone(), y.clone())
            .with_color(vec4(0.17, 0.63, 0.17, 1.0)));
        arrow_plot.add_arrow(ArrowAnnotation::new(3.0, 8.0, 4.5, 7.5)
            .with_color(vec4(0.9, 0.2, 0.2, 1.0))
            .with_text("Max here"));
        arrow_plot.add_arrow(ArrowAnnotation::new(8.0, 3.0, 6.5, 4.0)
            .with_color(vec4(0.2, 0.2, 0.9, 1.0))
            .with_text("Local min"));
        arrow_plot.set_legend(LegendPosition::TopRight);
        arrow_plot.redraw(cx);

        // Math/Formula Annotations
        let math_plot = self.ui.line_plot(&[id!(detail_math_annotations)]);
        math_plot.set_title("Math Annotations");
        let x_math: Vec<f64> = (0..50).map(|i| i as f64 * 0.2).collect();
        let y_math: Vec<f64> = x_math.iter().map(|&v| v.sin() * 2.0 + 3.0).collect();
        math_plot.add_series(Series::new("y = sin(x)")
            .with_data(x_math.clone(), y_math)
            .with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        math_plot.annotate("π/2", 1.57, 5.2, vec4(0.1, 0.1, 0.1, 1.0), 10.0);
        math_plot.annotate("π", 3.14, 3.2, vec4(0.1, 0.1, 0.1, 1.0), 10.0);
        math_plot.annotate("3π/2", 4.71, 1.2, vec4(0.1, 0.1, 0.1, 1.0), 10.0);
        math_plot.annotate("2π", 6.28, 3.2, vec4(0.1, 0.1, 0.1, 1.0), 10.0);
        math_plot.set_legend(LegendPosition::TopRight);
        math_plot.redraw(cx);

        // ===================
        // REFERENCE LINES & SPANS
        // ===================

        // Vertical Lines (axvline)
        let vlines_plot = self.ui.line_plot(&[id!(detail_vlines)]);
        vlines_plot.set_title("Vertical Lines (axvline)");
        vlines_plot.add_series(Series::new("Process")
            .with_data(x.clone(), y.clone())
            .with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        vlines_plot.axvline(2.5, vec4(0.9, 0.2, 0.2, 1.0), 2.0, LineStyle::Solid);
        vlines_plot.axvline(5.0, vec4(0.2, 0.7, 0.2, 1.0), 2.0, LineStyle::Dashed);
        vlines_plot.axvline(7.5, vec4(0.9, 0.6, 0.1, 1.0), 2.0, LineStyle::Dotted);
        vlines_plot.annotate("Start", 2.5, 8.0, vec4(0.9, 0.2, 0.2, 1.0), 9.0);
        vlines_plot.annotate("Checkpoint", 5.0, 8.0, vec4(0.2, 0.7, 0.2, 1.0), 9.0);
        vlines_plot.annotate("End", 7.5, 8.0, vec4(0.9, 0.6, 0.1, 1.0), 9.0);
        vlines_plot.set_legend(LegendPosition::TopRight);
        vlines_plot.redraw(cx);

        // Horizontal Lines (axhline)
        let hlines_plot = self.ui.line_plot(&[id!(detail_hlines)]);
        hlines_plot.set_title("Horizontal Lines (axhline)");
        hlines_plot.add_series(Series::new("Temperature")
            .with_data(x.clone(), y.clone())
            .with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        hlines_plot.axhline(7.0, vec4(0.9, 0.2, 0.2, 1.0), 2.0, LineStyle::Solid);
        hlines_plot.axhline(5.0, vec4(0.2, 0.2, 0.9, 1.0), 2.0, LineStyle::Dashed);
        hlines_plot.axhline(3.0, vec4(0.2, 0.7, 0.2, 1.0), 2.0, LineStyle::Dotted);
        hlines_plot.annotate("Upper Limit", 8.5, 7.0, vec4(0.9, 0.2, 0.2, 1.0), 9.0);
        hlines_plot.annotate("Target", 8.5, 5.0, vec4(0.2, 0.2, 0.9, 1.0), 9.0);
        hlines_plot.annotate("Lower Limit", 8.5, 3.0, vec4(0.2, 0.7, 0.2, 1.0), 9.0);
        hlines_plot.set_legend(LegendPosition::TopRight);
        hlines_plot.redraw(cx);

        // Shaded Regions (axvspan, axhspan)
        let spans_plot = self.ui.line_plot(&[id!(detail_spans)]);
        spans_plot.set_title("Shaded Regions (axvspan/axhspan)");
        spans_plot.add_series(Series::new("Signal")
            .with_data(x.clone(), y.clone())
            .with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        spans_plot.axvspan(2.0, 4.0, vec4(0.9, 0.2, 0.2, 0.2));
        spans_plot.axvspan(6.0, 8.0, vec4(0.2, 0.7, 0.2, 0.2));
        spans_plot.axhspan(4.0, 6.0, vec4(0.2, 0.2, 0.9, 0.15));
        spans_plot.annotate("Danger Zone", 3.0, 8.0, vec4(0.9, 0.2, 0.2, 1.0), 9.0);
        spans_plot.annotate("Safe Zone", 7.0, 8.0, vec4(0.2, 0.7, 0.2, 1.0), 9.0);
        spans_plot.annotate("Normal Range", 0.5, 5.0, vec4(0.2, 0.2, 0.9, 1.0), 9.0);
        spans_plot.set_legend(LegendPosition::TopRight);
        spans_plot.redraw(cx);
    }

    fn setup_errorbars_detail(&mut self, cx: &mut Cx) {
        // ===================
        // ERROR BARS
        // ===================

        // Symmetric Y Error Bars
        let yerr_sym = self.ui.line_plot(&[id!(detail_yerr_symmetric)]);
        yerr_sym.set_title("Symmetric Y Error Bars");
        let x: Vec<f64> = (0..10).map(|i| i as f64).collect();
        let y: Vec<f64> = vec![2.0, 3.5, 4.2, 3.8, 5.5, 6.2, 5.8, 7.0, 6.5, 8.0];
        let yerr: Vec<f64> = vec![0.3, 0.5, 0.4, 0.6, 0.4, 0.5, 0.7, 0.4, 0.5, 0.3];
        yerr_sym.add_series(Series::new("Experimental Data")
            .with_data(x, y)
            .with_yerr(yerr)
            .with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        yerr_sym.set_show_points(true);
        yerr_sym.set_legend(LegendPosition::TopRight);
        yerr_sym.redraw(cx);

        // Asymmetric Y Error Bars
        let yerr_asym = self.ui.line_plot(&[id!(detail_yerr_asymmetric)]);
        yerr_asym.set_title("Asymmetric Y Error Bars");
        let x: Vec<f64> = (0..8).map(|i| i as f64).collect();
        let y: Vec<f64> = vec![1.0, 2.5, 4.0, 3.5, 5.0, 6.5, 6.0, 7.5];
        let yerr_minus: Vec<f64> = vec![0.2, 0.3, 0.5, 0.4, 0.3, 0.6, 0.5, 0.4];
        let yerr_plus: Vec<f64> = vec![0.5, 0.7, 0.4, 0.8, 0.6, 0.4, 0.9, 0.6];
        yerr_asym.add_series(Series::new("Measurements")
            .with_data(x, y)
            .with_yerr_asymmetric(yerr_minus, yerr_plus)
            .with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        yerr_asym.set_show_points(true);
        yerr_asym.set_legend(LegendPosition::TopRight);
        yerr_asym.redraw(cx);

        // X and Y Error Bars
        let xy_err = self.ui.line_plot(&[id!(detail_xy_error)]);
        xy_err.set_title("X and Y Error Bars");
        let x: Vec<f64> = vec![1.0, 2.5, 4.0, 5.5, 7.0, 8.5];
        let y: Vec<f64> = vec![2.0, 3.5, 3.0, 5.0, 4.5, 6.0];
        let xerr: Vec<f64> = vec![0.3, 0.4, 0.5, 0.3, 0.4, 0.3];
        let yerr: Vec<f64> = vec![0.4, 0.5, 0.6, 0.4, 0.5, 0.4];
        xy_err.add_series(Series::new("2D Uncertainty")
            .with_data(x, y)
            .with_xerr(xerr)
            .with_yerr(yerr)
            .with_color(vec4(0.17, 0.63, 0.17, 1.0)));
        xy_err.set_show_points(true);
        xy_err.set_legend(LegendPosition::TopRight);
        xy_err.redraw(cx);

        // ===================
        // FILL BETWEEN / AREA CHARTS
        // ===================

        // Confidence Band
        let conf_band = self.ui.line_plot(&[id!(detail_confidence_band)]);
        conf_band.set_title("Confidence Band (±1σ)");
        let x: Vec<f64> = (0..50).map(|i| i as f64 * 0.2).collect();
        let y: Vec<f64> = x.iter().map(|&v| (v * 0.5).sin() * 2.0 + 4.0).collect();
        let y_upper: Vec<f64> = y.iter().map(|&v| v + 0.6).collect();
        let y_lower: Vec<f64> = y.iter().map(|&v| v - 0.6).collect();
        conf_band.fill_between(x.clone(), y_lower, y_upper, vec4(0.12, 0.47, 0.71, 0.25));
        conf_band.add_series(Series::new("Mean")
            .with_data(x, y)
            .with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        conf_band.set_legend(LegendPosition::TopRight);
        conf_band.redraw(cx);

        // Area Chart (Fill to baseline)
        let area_chart = self.ui.line_plot(&[id!(detail_area_chart)]);
        area_chart.set_title("Area Chart (Baseline)");
        let x: Vec<f64> = (0..30).map(|i| i as f64 * 0.3).collect();
        let y: Vec<f64> = x.iter().map(|&v| (v * 0.4).sin().abs() * 3.0 + 1.0).collect();
        area_chart.fill_between_baseline(x.clone(), y.clone(), 0.0, vec4(0.84, 0.15, 0.16, 0.3));
        area_chart.add_series(Series::new("Revenue")
            .with_data(x, y)
            .with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        area_chart.set_legend(LegendPosition::TopRight);
        area_chart.redraw(cx);

        // Stacked Areas
        let stacked = self.ui.line_plot(&[id!(detail_stacked_area)]);
        stacked.set_title("Stacked Areas");
        let x: Vec<f64> = (0..20).map(|i| i as f64 * 0.5).collect();
        let y1: Vec<f64> = x.iter().map(|&v| (v * 0.3).sin().abs() * 2.0 + 1.0).collect();
        let y2: Vec<f64> = y1.iter().zip(x.iter()).map(|(&v1, &xv)| v1 + (xv * 0.2).cos().abs() * 1.5 + 0.5).collect();
        let y3: Vec<f64> = y2.iter().zip(x.iter()).map(|(&v2, &xv)| v2 + (xv * 0.25).sin().abs() * 1.0 + 0.3).collect();
        // Fill from bottom to top
        stacked.fill_between_baseline(x.clone(), y1.clone(), 0.0, vec4(0.12, 0.47, 0.71, 0.5));
        stacked.fill_between(x.clone(), y1.clone(), y2.clone(), vec4(0.17, 0.63, 0.17, 0.5));
        stacked.fill_between(x.clone(), y2.clone(), y3.clone(), vec4(0.84, 0.15, 0.16, 0.5));
        stacked.add_series(Series::new("Layer 1").with_data(x.clone(), y1).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        stacked.add_series(Series::new("Layer 2").with_data(x.clone(), y2).with_color(vec4(0.17, 0.63, 0.17, 1.0)));
        stacked.add_series(Series::new("Layer 3").with_data(x, y3).with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        stacked.set_legend(LegendPosition::TopRight);
        stacked.redraw(cx);
    }

    fn setup_fillbetween_detail(&mut self, cx: &mut Cx) {
        // ===================
        // FILL BETWEEN CURVES
        // ===================

        // Between Two Lines
        let two_lines = self.ui.line_plot(&[id!(detail_fill_two_lines)]);
        two_lines.set_title("Fill Between Two Lines");
        let x: Vec<f64> = (0..40).map(|i| i as f64 * 0.25).collect();
        let y1: Vec<f64> = x.iter().map(|&v| (v * 0.5).sin() * 2.0 + 3.0).collect();
        let y2: Vec<f64> = x.iter().map(|&v| (v * 0.5).cos() * 2.0 + 5.0).collect();
        two_lines.fill_between(x.clone(), y1.clone(), y2.clone(), vec4(0.5, 0.2, 0.8, 0.3));
        two_lines.add_series(Series::new("sin(x)").with_data(x.clone(), y1).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        two_lines.add_series(Series::new("cos(x)").with_data(x, y2).with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        two_lines.set_legend(LegendPosition::TopRight);
        two_lines.redraw(cx);

        // Confidence Interval
        let confidence = self.ui.line_plot(&[id!(detail_fill_confidence)]);
        confidence.set_title("95% Confidence Interval");
        let x: Vec<f64> = (0..50).map(|i| i as f64 * 0.2).collect();
        let y: Vec<f64> = x.iter().map(|&v| v * 0.8 + (v * 0.3).sin() * 0.5).collect();
        let ci_upper: Vec<f64> = y.iter().enumerate().map(|(i, &v)| v + 0.3 + (i as f64) * 0.02).collect();
        let ci_lower: Vec<f64> = y.iter().enumerate().map(|(i, &v)| v - 0.3 - (i as f64) * 0.02).collect();
        confidence.fill_between(x.clone(), ci_lower, ci_upper, vec4(0.17, 0.63, 0.17, 0.25));
        confidence.add_series(Series::new("Trend")
            .with_data(x, y)
            .with_color(vec4(0.17, 0.63, 0.17, 1.0))
            .with_line_width(2.0));
        confidence.set_legend(LegendPosition::TopRight);
        confidence.redraw(cx);

        // Multiple Fills
        let multi_fill = self.ui.line_plot(&[id!(detail_fill_multiple)]);
        multi_fill.set_title("Multiple Fill Regions");
        let x: Vec<f64> = (0..40).map(|i| i as f64 * 0.25).collect();
        let y_base: Vec<f64> = x.iter().map(|&v| 2.0 + (v * 0.2).sin()).collect();
        let y_mid: Vec<f64> = x.iter().map(|&v| 4.0 + (v * 0.25).cos()).collect();
        let y_top: Vec<f64> = x.iter().map(|&v| 6.0 + (v * 0.15).sin() * 0.5).collect();
        multi_fill.fill_between(x.clone(), y_base.clone(), y_mid.clone(), vec4(0.12, 0.47, 0.71, 0.3));
        multi_fill.fill_between(x.clone(), y_mid.clone(), y_top.clone(), vec4(0.84, 0.15, 0.16, 0.3));
        multi_fill.add_series(Series::new("Low").with_data(x.clone(), y_base).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        multi_fill.add_series(Series::new("Mid").with_data(x.clone(), y_mid).with_color(vec4(0.5, 0.3, 0.7, 1.0)));
        multi_fill.add_series(Series::new("High").with_data(x, y_top).with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        multi_fill.set_legend(LegendPosition::TopRight);
        multi_fill.redraw(cx);

        // ===================
        // AREA CHARTS
        // ===================

        // Simple Area
        let simple_area = self.ui.line_plot(&[id!(detail_area_simple)]);
        simple_area.set_title("Simple Area Chart");
        let x: Vec<f64> = (0..30).map(|i| i as f64 * 0.3).collect();
        let y: Vec<f64> = x.iter().map(|&v| (v * 0.4).sin() * 2.0 + 3.0).collect();
        simple_area.fill_between_baseline(x.clone(), y.clone(), 0.0, vec4(0.12, 0.47, 0.71, 0.4));
        simple_area.add_series(Series::new("Values")
            .with_data(x, y)
            .with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        simple_area.set_legend(LegendPosition::TopRight);
        simple_area.redraw(cx);

        // Stacked Area
        let stacked_area = self.ui.line_plot(&[id!(detail_area_stacked)]);
        stacked_area.set_title("Stacked Area Chart");
        let x: Vec<f64> = (0..25).map(|i| i as f64 * 0.4).collect();
        let y1: Vec<f64> = x.iter().map(|&v| (v * 0.3).sin().abs() * 1.5 + 1.0).collect();
        let y2: Vec<f64> = y1.iter().zip(x.iter()).map(|(&y, &xv)| y + (xv * 0.2).cos().abs() * 1.2 + 0.8).collect();
        let y3: Vec<f64> = y2.iter().zip(x.iter()).map(|(&y, &xv)| y + (xv * 0.25).sin().abs() * 1.0 + 0.5).collect();
        stacked_area.fill_between_baseline(x.clone(), y1.clone(), 0.0, vec4(0.12, 0.47, 0.71, 0.6));
        stacked_area.fill_between(x.clone(), y1.clone(), y2.clone(), vec4(0.17, 0.63, 0.17, 0.6));
        stacked_area.fill_between(x.clone(), y2.clone(), y3.clone(), vec4(0.84, 0.15, 0.16, 0.6));
        stacked_area.add_series(Series::new("Product A").with_data(x.clone(), y1).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        stacked_area.add_series(Series::new("Product B").with_data(x.clone(), y2).with_color(vec4(0.17, 0.63, 0.17, 1.0)));
        stacked_area.add_series(Series::new("Product C").with_data(x, y3).with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        stacked_area.set_legend(LegendPosition::TopRight);
        stacked_area.redraw(cx);

        // Gradient Fill (using multiple overlapping fills)
        let gradient = self.ui.line_plot(&[id!(detail_area_gradient)]);
        gradient.set_title("Layered Fill Effect");
        let x: Vec<f64> = (0..40).map(|i| i as f64 * 0.25).collect();
        let y: Vec<f64> = x.iter().map(|&v| (v * 0.3).sin() * 3.0 + 4.0).collect();
        // Create layered effect with decreasing alpha
        let y_90: Vec<f64> = y.iter().map(|&v| v * 0.9).collect();
        let y_70: Vec<f64> = y.iter().map(|&v| v * 0.7).collect();
        let y_50: Vec<f64> = y.iter().map(|&v| v * 0.5).collect();
        gradient.fill_between_baseline(x.clone(), y.clone(), 0.0, vec4(0.12, 0.47, 0.71, 0.15));
        gradient.fill_between_baseline(x.clone(), y_90, 0.0, vec4(0.12, 0.47, 0.71, 0.2));
        gradient.fill_between_baseline(x.clone(), y_70, 0.0, vec4(0.12, 0.47, 0.71, 0.25));
        gradient.fill_between_baseline(x.clone(), y_50, 0.0, vec4(0.12, 0.47, 0.71, 0.3));
        gradient.add_series(Series::new("Data")
            .with_data(x, y)
            .with_color(vec4(0.12, 0.47, 0.71, 1.0))
            .with_line_width(2.0));
        gradient.set_legend(LegendPosition::TopRight);
        gradient.redraw(cx);
    }

    fn setup_stackedbars_detail(&mut self, cx: &mut Cx) {
        // Grouped Bars
        let grouped = self.ui.bar_plot(&[id!(detail_grouped_bars)]);
        grouped.set_title("Grouped Bar Chart");
        grouped.set_groups(
            vec!["Q1".into(), "Q2".into(), "Q3".into(), "Q4".into()],
            vec![
                BarGroup { label: "Product A".into(), values: vec![120.0, 150.0, 180.0, 200.0], color: Some(vec4(0.12, 0.47, 0.71, 1.0)) },
                BarGroup { label: "Product B".into(), values: vec![100.0, 130.0, 160.0, 190.0], color: Some(vec4(0.84, 0.15, 0.16, 1.0)) },
                BarGroup { label: "Product C".into(), values: vec![80.0, 110.0, 140.0, 170.0], color: Some(vec4(0.17, 0.63, 0.17, 1.0)) },
            ]
        );
        grouped.redraw(cx);

        // Stacked Bars
        let stacked = self.ui.bar_plot(&[id!(detail_stacked_bars)]);
        stacked.set_title("Stacked Bar Chart");
        stacked.set_stacked(true);
        stacked.set_groups(
            vec!["Jan".into(), "Feb".into(), "Mar".into(), "Apr".into(), "May".into()],
            vec![
                BarGroup { label: "Online".into(), values: vec![50.0, 60.0, 70.0, 65.0, 80.0], color: Some(vec4(0.12, 0.47, 0.71, 1.0)) },
                BarGroup { label: "In-Store".into(), values: vec![40.0, 45.0, 50.0, 55.0, 60.0], color: Some(vec4(0.84, 0.15, 0.16, 1.0)) },
                BarGroup { label: "Phone".into(), values: vec![20.0, 25.0, 30.0, 28.0, 35.0], color: Some(vec4(0.17, 0.63, 0.17, 1.0)) },
            ]
        );
        stacked.redraw(cx);

        // Horizontal Stacked Bars
        let hstacked = self.ui.bar_plot(&[id!(detail_hstacked_bars)]);
        hstacked.set_title("Horizontal Stacked Bars");
        hstacked.set_horizontal(true);
        hstacked.set_stacked(true);
        hstacked.set_groups(
            vec!["Team A".into(), "Team B".into(), "Team C".into()],
            vec![
                BarGroup { label: "Completed".into(), values: vec![45.0, 55.0, 40.0], color: Some(vec4(0.17, 0.63, 0.17, 1.0)) },
                BarGroup { label: "In Progress".into(), values: vec![25.0, 20.0, 30.0], color: Some(vec4(1.0, 0.65, 0.0, 1.0)) },
                BarGroup { label: "Pending".into(), values: vec![15.0, 10.0, 20.0], color: Some(vec4(0.84, 0.15, 0.16, 1.0)) },
            ]
        );
        hstacked.redraw(cx);
    }

    fn setup_candlestick_detail(&mut self, cx: &mut Cx) {
        // Basic Candlestick
        let basic = self.ui.candlestick_chart(&[id!(detail_candlestick_basic)]);
        basic.set_title("Basic OHLC Chart");
        basic.set_data(vec![
            Candle::new(0.0, 100.0, 105.0, 98.0, 103.0),
            Candle::new(1.0, 103.0, 108.0, 101.0, 106.0),
            Candle::new(2.0, 106.0, 110.0, 104.0, 105.0),
            Candle::new(3.0, 105.0, 107.0, 100.0, 101.0),
            Candle::new(4.0, 101.0, 106.0, 99.0, 104.0),
            Candle::new(5.0, 104.0, 109.0, 103.0, 107.0),
            Candle::new(6.0, 107.0, 112.0, 105.0, 110.0),
            Candle::new(7.0, 110.0, 115.0, 108.0, 113.0),
            Candle::new(8.0, 113.0, 118.0, 111.0, 116.0),
            Candle::new(9.0, 116.0, 120.0, 114.0, 118.0),
        ]);
        basic.redraw(cx);

        // Uptrend Pattern
        let uptrend = self.ui.candlestick_chart(&[id!(detail_candlestick_uptrend)]);
        uptrend.set_title("Uptrend Pattern");
        uptrend.set_colors(vec4(0.0, 0.7, 0.3, 1.0), vec4(0.9, 0.1, 0.1, 1.0));
        let mut candles = Vec::new();
        let mut base = 50.0;
        for i in 0..15 {
            let open = base + (i as f64 * 0.5).sin() * 2.0;
            let trend = i as f64 * 1.5;
            let close = open + trend * 0.3 + 2.0;
            let high = close.max(open) + 1.5;
            let low = close.min(open) - 1.0;
            candles.push(Candle::new(i as f64, open + trend, high + trend, low + trend, close + trend));
            base = close;
        }
        uptrend.set_data(candles);
        uptrend.redraw(cx);

        // Volatile Market
        let volatile = self.ui.candlestick_chart(&[id!(detail_candlestick_volatile)]);
        volatile.set_title("Volatile Market");
        let mut candles = Vec::new();
        let mut price = 100.0;
        for i in 0..12 {
            let volatility = 8.0 + (i as f64 * 0.5).sin() * 4.0;
            let direction: f64 = if i % 3 == 0 { -1.0 } else { 1.0 };
            let open = price;
            let close = price + direction * volatility * 0.5;
            let high = open.max(close) + volatility * 0.4;
            let low = open.min(close) - volatility * 0.4;
            candles.push(Candle::new(i as f64, open, high, low, close));
            price = close;
        }
        volatile.set_data(candles);
        volatile.redraw(cx);
    }

    fn setup_radar_detail(&mut self, cx: &mut Cx) {
        // Skills Comparison (with gradient fills)
        let skills = self.ui.radar_chart(&[id!(detail_radar_skills)]);
        skills.set_title("Skills Comparison");
        skills.set_axes(vec![
            "Programming".into(), "Design".into(), "Communication".into(),
            "Leadership".into(), "Problem Solving".into(), "Creativity".into()
        ]);
        skills.add_series(RadarSeries::new("Developer", vec![95.0, 60.0, 75.0, 65.0, 90.0, 70.0])
            .with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        skills.add_series(RadarSeries::new("Designer", vec![50.0, 95.0, 80.0, 55.0, 70.0, 95.0])
            .with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        skills.set_max_value(100.0);
        skills.set_use_gradient(true);
        skills.redraw(cx);

        // Performance Metrics (with gradient fills)
        let metrics = self.ui.radar_chart(&[id!(detail_radar_metrics)]);
        metrics.set_title("Performance Metrics");
        metrics.set_axes(vec![
            "Speed".into(), "Accuracy".into(), "Efficiency".into(),
            "Quality".into(), "Reliability".into()
        ]);
        metrics.add_series(RadarSeries::new("Current Quarter", vec![85.0, 90.0, 75.0, 88.0, 92.0])
            .with_color(vec4(0.17, 0.63, 0.17, 1.0)));
        metrics.set_max_value(100.0);
        metrics.set_use_gradient(true);
        metrics.redraw(cx);

        // Multi-Series Comparison (with gradient fills)
        let multi = self.ui.radar_chart(&[id!(detail_radar_multi)]);
        multi.set_title("Product Comparison");
        multi.set_axes(vec![
            "Price".into(), "Quality".into(), "Features".into(),
            "Support".into(), "Performance".into()
        ]);
        multi.add_series(RadarSeries::new("Product A", vec![70.0, 85.0, 90.0, 75.0, 80.0])
            .with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        multi.add_series(RadarSeries::new("Product B", vec![90.0, 70.0, 75.0, 85.0, 70.0])
            .with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        multi.add_series(RadarSeries::new("Product C", vec![80.0, 80.0, 80.0, 80.0, 85.0])
            .with_color(vec4(0.17, 0.63, 0.17, 1.0)));
        multi.set_max_value(100.0);
        multi.set_use_gradient(true);
        multi.redraw(cx);
    }

    fn setup_waterfall_detail(&mut self, cx: &mut Cx) {
        // Revenue Bridge
        let revenue = self.ui.waterfall_chart(&[id!(detail_waterfall_revenue)]);
        revenue.set_title("Revenue Bridge");
        revenue.set_data(vec![
            WaterfallEntry::new("Starting", 1000.0),
            WaterfallEntry::new("New Sales", 350.0),
            WaterfallEntry::new("Upsells", 150.0),
            WaterfallEntry::new("Churn", -120.0),
            WaterfallEntry::new("Discounts", -80.0),
            WaterfallEntry::total("Ending", 1300.0),
        ]);
        revenue.redraw(cx);

        // Cost Breakdown
        let cost = self.ui.waterfall_chart(&[id!(detail_waterfall_cost)]);
        cost.set_title("Cost Breakdown");
        cost.set_data(vec![
            WaterfallEntry::new("Gross Profit", 500.0),
            WaterfallEntry::new("Salaries", -150.0),
            WaterfallEntry::new("Marketing", -80.0),
            WaterfallEntry::new("Operations", -60.0),
            WaterfallEntry::new("R&D", -70.0),
            WaterfallEntry::total("Net Profit", 140.0),
        ]);
        cost.redraw(cx);

        // Profit Analysis
        let profit = self.ui.waterfall_chart(&[id!(detail_waterfall_profit)]);
        profit.set_title("Profit Analysis");
        profit.set_data(vec![
            WaterfallEntry::new("Q1", 200.0),
            WaterfallEntry::new("Q2 Growth", 50.0),
            WaterfallEntry::new("Q3 Loss", -30.0),
            WaterfallEntry::new("Q4 Growth", 80.0),
            WaterfallEntry::total("Year End", 300.0),
        ]);
        profit.redraw(cx);
    }

    fn setup_gauge_detail(&mut self, cx: &mut Cx) {
        // Performance Score
        let performance = self.ui.gauge_chart(&[id!(detail_gauge_performance)]);
        performance.set_title("Performance");
        performance.set_value(78.0);
        performance.set_range(0.0, 100.0);
        performance.set_unit("%");
        performance.set_thresholds(vec![
            (40.0, vec4(0.84, 0.15, 0.16, 1.0)),  // Red below 40
            (70.0, vec4(0.94, 0.78, 0.0, 1.0)),   // Yellow 40-70
            (100.0, vec4(0.17, 0.63, 0.17, 1.0)), // Green above 70
        ]);
        performance.redraw(cx);

        // Battery Level
        let battery = self.ui.gauge_chart(&[id!(detail_gauge_battery)]);
        battery.set_title("Battery");
        battery.set_value(45.0);
        battery.set_range(0.0, 100.0);
        battery.set_unit("%");
        battery.set_thresholds(vec![
            (20.0, vec4(0.84, 0.15, 0.16, 1.0)),  // Red below 20
            (50.0, vec4(0.94, 0.78, 0.0, 1.0)),   // Yellow 20-50
            (100.0, vec4(0.17, 0.63, 0.17, 1.0)), // Green above 50
        ]);
        battery.redraw(cx);

        // Speed Indicator
        let speed = self.ui.gauge_chart(&[id!(detail_gauge_speed)]);
        speed.set_title("Speed");
        speed.set_value(120.0);
        speed.set_range(0.0, 200.0);
        speed.set_unit("km/h");
        speed.set_thresholds(vec![
            (80.0, vec4(0.17, 0.63, 0.17, 1.0)),   // Green below 80
            (140.0, vec4(0.94, 0.78, 0.0, 1.0)),   // Yellow 80-140
            (200.0, vec4(0.84, 0.15, 0.16, 1.0)),  // Red above 140
        ]);
        speed.redraw(cx);
    }

    fn setup_funnel_detail(&mut self, cx: &mut Cx) {
        // Sales Pipeline
        let sales = self.ui.funnel_chart(&[id!(detail_funnel_sales)]);
        sales.set_title("Sales Pipeline");
        sales.set_data(vec![
            FunnelStage::new("Leads", 1000.0).with_color(vec4(0.12, 0.47, 0.71, 1.0)),
            FunnelStage::new("Qualified", 650.0).with_color(vec4(0.17, 0.63, 0.17, 1.0)),
            FunnelStage::new("Proposals", 300.0).with_color(vec4(0.94, 0.78, 0.0, 1.0)),
            FunnelStage::new("Negotiations", 150.0).with_color(vec4(0.84, 0.45, 0.1, 1.0)),
            FunnelStage::new("Closed", 80.0).with_color(vec4(0.84, 0.15, 0.16, 1.0)),
        ]);
        sales.set_show_percentages(true);
        sales.redraw(cx);

        // User Journey
        let journey = self.ui.funnel_chart(&[id!(detail_funnel_journey)]);
        journey.set_title("User Journey");
        journey.set_data(vec![
            FunnelStage::new("Visitors", 10000.0).with_color(vec4(0.56, 0.27, 0.68, 1.0)),
            FunnelStage::new("Sign Ups", 3500.0).with_color(vec4(0.12, 0.47, 0.71, 1.0)),
            FunnelStage::new("Active", 2000.0).with_color(vec4(0.17, 0.63, 0.17, 1.0)),
            FunnelStage::new("Paying", 500.0).with_color(vec4(0.94, 0.78, 0.0, 1.0)),
        ]);
        journey.set_show_percentages(true);
        journey.redraw(cx);

        // Hiring Process
        let hiring = self.ui.funnel_chart(&[id!(detail_funnel_hiring)]);
        hiring.set_title("Hiring Funnel");
        hiring.set_data(vec![
            FunnelStage::new("Applications", 500.0).with_color(vec4(0.12, 0.47, 0.71, 1.0)),
            FunnelStage::new("Screened", 200.0).with_color(vec4(0.17, 0.63, 0.17, 1.0)),
            FunnelStage::new("Interviews", 50.0).with_color(vec4(0.94, 0.78, 0.0, 1.0)),
            FunnelStage::new("Offers", 15.0).with_color(vec4(0.84, 0.45, 0.1, 1.0)),
            FunnelStage::new("Hired", 10.0).with_color(vec4(0.84, 0.15, 0.16, 1.0)),
        ]);
        hiring.set_show_percentages(true);
        hiring.redraw(cx);
    }

    fn setup_heatmap_detail(&mut self, cx: &mut Cx) {
        // Correlation Matrix
        let correlation = self.ui.heatmap(&[id!(detail_heatmap_correlation)]);
        correlation.set_title("Correlation Matrix");
        correlation.set_data(vec![
            vec![1.0, 0.8, 0.3, -0.2],
            vec![0.8, 1.0, 0.5, -0.1],
            vec![0.3, 0.5, 1.0, 0.4],
            vec![-0.2, -0.1, 0.4, 1.0],
        ]);
        correlation.set_labels(
            vec!["A".into(), "B".into(), "C".into(), "D".into()],
            vec!["A".into(), "B".into(), "C".into(), "D".into()],
        );
        correlation.set_colormap(Colormap::RdBu);
        correlation.set_range(-1.0, 1.0);
        correlation.set_show_values(true);
        correlation.redraw(cx);

        // Monthly Sales
        let sales = self.ui.heatmap(&[id!(detail_heatmap_sales)]);
        sales.set_title("Monthly Sales");
        sales.set_data(vec![
            vec![120.0, 150.0, 180.0, 200.0],
            vec![100.0, 130.0, 160.0, 190.0],
            vec![80.0, 110.0, 140.0, 170.0],
        ]);
        sales.set_labels(
            vec!["Q1".into(), "Q2".into(), "Q3".into(), "Q4".into()],
            vec!["2021".into(), "2022".into(), "2023".into()],
        );
        sales.set_colormap(Colormap::Viridis);
        sales.set_show_values(true);
        sales.redraw(cx);

        // Activity Levels
        let activity = self.ui.heatmap(&[id!(detail_heatmap_activity)]);
        activity.set_title("Weekly Activity");
        activity.set_data(vec![
            vec![5.0, 8.0, 12.0, 15.0, 10.0, 3.0, 2.0],
            vec![6.0, 10.0, 14.0, 16.0, 12.0, 4.0, 3.0],
            vec![4.0, 7.0, 11.0, 13.0, 9.0, 2.0, 1.0],
        ]);
        activity.set_labels(
            vec!["Mon".into(), "Tue".into(), "Wed".into(), "Thu".into(), "Fri".into(), "Sat".into(), "Sun".into()],
            vec!["Morning".into(), "Afternoon".into(), "Evening".into()],
        );
        activity.set_colormap(Colormap::Hot);
        activity.set_show_values(true);
        activity.redraw(cx);
    }

    fn setup_treemap_detail(&mut self, cx: &mut Cx) {
        // Market Share
        let market = self.ui.treemap(&[id!(detail_treemap_market)]);
        market.set_title("Market Share");
        market.set_data(vec![
            TreemapNode::new("Company A", 35.0).with_color(vec4(0.12, 0.47, 0.71, 1.0)),
            TreemapNode::new("Company B", 25.0).with_color(vec4(0.17, 0.63, 0.17, 1.0)),
            TreemapNode::new("Company C", 20.0).with_color(vec4(0.94, 0.78, 0.0, 1.0)),
            TreemapNode::new("Company D", 12.0).with_color(vec4(0.84, 0.45, 0.1, 1.0)),
            TreemapNode::new("Others", 8.0).with_color(vec4(0.5, 0.5, 0.5, 1.0)),
        ]);
        market.set_show_labels(true);
        market.redraw(cx);

        // Budget Allocation
        let budget = self.ui.treemap(&[id!(detail_treemap_budget)]);
        budget.set_title("Budget Allocation");
        budget.set_data(vec![
            TreemapNode::new("Salaries", 45.0).with_color(vec4(0.12, 0.47, 0.71, 1.0)),
            TreemapNode::new("Marketing", 20.0).with_color(vec4(0.84, 0.15, 0.16, 1.0)),
            TreemapNode::new("R&D", 18.0).with_color(vec4(0.17, 0.63, 0.17, 1.0)),
            TreemapNode::new("Operations", 12.0).with_color(vec4(0.94, 0.78, 0.0, 1.0)),
            TreemapNode::new("Admin", 5.0).with_color(vec4(0.56, 0.27, 0.68, 1.0)),
        ]);
        budget.set_show_labels(true);
        budget.redraw(cx);

        // Disk Usage
        let disk = self.ui.treemap(&[id!(detail_treemap_disk)]);
        disk.set_title("Disk Usage");
        disk.set_data(vec![
            TreemapNode::new("Videos", 150.0).with_color(vec4(0.84, 0.15, 0.16, 1.0)),
            TreemapNode::new("Photos", 80.0).with_color(vec4(0.17, 0.63, 0.17, 1.0)),
            TreemapNode::new("Documents", 40.0).with_color(vec4(0.12, 0.47, 0.71, 1.0)),
            TreemapNode::new("Music", 30.0).with_color(vec4(0.94, 0.78, 0.0, 1.0)),
            TreemapNode::new("Apps", 25.0).with_color(vec4(0.56, 0.27, 0.68, 1.0)),
            TreemapNode::new("Other", 15.0).with_color(vec4(0.5, 0.5, 0.5, 1.0)),
        ]);
        disk.set_show_labels(true);
        disk.redraw(cx);
    }

    fn setup_bubble_detail(&mut self, cx: &mut Cx) {
        // Population vs GDP (with gradient bubbles)
        let gdp = self.ui.bubble_chart(&[id!(detail_bubble_gdp)]);
        gdp.set_title("Population vs GDP");
        gdp.add_series(BubbleSeries::new("Countries").with_points(vec![
            BubblePoint::new(10.0, 50000.0, 330.0).with_label("USA"),
            BubblePoint::new(65.0, 42000.0, 67.0).with_label("Germany"),
            BubblePoint::new(1400.0, 12000.0, 1400.0).with_label("China"),
            BubblePoint::new(125.0, 40000.0, 125.0).with_label("Japan"),
            BubblePoint::new(1380.0, 2500.0, 1380.0).with_label("India"),
        ]).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        gdp.set_x_label("Population (M)");
        gdp.set_show_grid(true);
        gdp.set_use_gradient(true);
        gdp.redraw(cx);

        // Product Comparison (with gradient bubbles)
        let products = self.ui.bubble_chart(&[id!(detail_bubble_products)]);
        products.set_title("Product Comparison");
        products.add_series(BubbleSeries::new("Products").with_points(vec![
            BubblePoint::new(8.0, 4.5, 1200.0).with_color(vec4(0.17, 0.63, 0.17, 1.0)),
            BubblePoint::new(6.0, 4.0, 800.0).with_color(vec4(0.12, 0.47, 0.71, 1.0)),
            BubblePoint::new(9.0, 3.5, 500.0).with_color(vec4(0.94, 0.78, 0.0, 1.0)),
            BubblePoint::new(4.0, 4.8, 300.0).with_color(vec4(0.84, 0.15, 0.16, 1.0)),
        ]).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        products.set_x_label("Price");
        products.set_show_grid(true);
        products.set_use_gradient(true);
        products.redraw(cx);

        // Investment Portfolio (with gradient bubbles)
        let investment = self.ui.bubble_chart(&[id!(detail_bubble_investment)]);
        investment.set_title("Investment Portfolio");
        investment.add_series(BubbleSeries::new("Assets").with_points(vec![
            BubblePoint::new(5.0, 8.0, 50000.0).with_color(vec4(0.17, 0.63, 0.17, 1.0)).with_label("Stocks"),
            BubblePoint::new(2.0, 3.0, 30000.0).with_color(vec4(0.12, 0.47, 0.71, 1.0)).with_label("Bonds"),
            BubblePoint::new(8.0, 12.0, 15000.0).with_color(vec4(0.94, 0.78, 0.0, 1.0)).with_label("Crypto"),
            BubblePoint::new(1.5, 2.0, 25000.0).with_color(vec4(0.5, 0.5, 0.5, 1.0)).with_label("Real Estate"),
        ]).with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        investment.set_x_label("Risk");
        investment.set_show_grid(true);
        investment.set_use_gradient(true);
        investment.redraw(cx);
    }

    fn setup_donut_detail(&mut self, cx: &mut Cx) {
        // Browser Usage
        let browser = self.ui.donut_chart(&[id!(detail_donut_browser)]);
        browser.set_title("Browser Market Share");
        browser.set_data(vec![
            DonutSlice::new("Chrome", 65.0).with_color(vec4(0.17, 0.63, 0.17, 1.0)),
            DonutSlice::new("Safari", 18.0).with_color(vec4(0.12, 0.47, 0.71, 1.0)),
            DonutSlice::new("Firefox", 8.0).with_color(vec4(0.94, 0.78, 0.0, 1.0)),
            DonutSlice::new("Edge", 5.0).with_color(vec4(0.12, 0.56, 0.84, 1.0)),
            DonutSlice::new("Others", 4.0).with_color(vec4(0.5, 0.5, 0.5, 1.0)),
        ]);
        browser.set_inner_radius_ratio(0.5);
        browser.set_show_labels(true);
        browser.redraw(cx);

        // Revenue by Region
        let revenue = self.ui.donut_chart(&[id!(detail_donut_revenue)]);
        revenue.set_title("Revenue by Region");
        revenue.set_data(vec![
            DonutSlice::new("North America", 42.0).with_color(vec4(0.12, 0.47, 0.71, 1.0)),
            DonutSlice::new("Europe", 28.0).with_color(vec4(0.17, 0.63, 0.17, 1.0)),
            DonutSlice::new("Asia Pacific", 20.0).with_color(vec4(0.84, 0.15, 0.16, 1.0)),
            DonutSlice::new("Latin America", 7.0).with_color(vec4(0.94, 0.78, 0.0, 1.0)),
            DonutSlice::new("Other", 3.0).with_color(vec4(0.5, 0.5, 0.5, 1.0)),
        ]);
        revenue.set_inner_radius_ratio(0.6);
        revenue.set_show_labels(true);
        revenue.redraw(cx);

        // Task Status
        let tasks = self.ui.donut_chart(&[id!(detail_donut_tasks)]);
        tasks.set_title("Sprint Progress");
        tasks.set_data(vec![
            DonutSlice::new("Completed", 45.0).with_color(vec4(0.17, 0.63, 0.17, 1.0)),
            DonutSlice::new("In Progress", 30.0).with_color(vec4(0.12, 0.47, 0.71, 1.0)),
            DonutSlice::new("To Do", 15.0).with_color(vec4(0.94, 0.78, 0.0, 1.0)),
            DonutSlice::new("Blocked", 10.0).with_color(vec4(0.84, 0.15, 0.16, 1.0)),
        ]);
        tasks.set_inner_radius_ratio(0.4);
        tasks.set_show_labels(true);
        tasks.redraw(cx);
    }

    fn setup_area_detail(&mut self, cx: &mut Cx) {
        // Traffic Sources
        let traffic = self.ui.area_chart(&[id!(detail_area_traffic)]);
        traffic.set_title("Website Traffic");
        traffic.add_series(AreaSeries::new("Organic")
            .with_data(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
                       vec![1000.0, 1200.0, 1100.0, 1400.0, 1600.0, 1500.0, 1800.0])
            .with_color(vec4(0.17, 0.63, 0.17, 0.8)));
        traffic.add_series(AreaSeries::new("Direct")
            .with_data(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
                       vec![500.0, 600.0, 550.0, 700.0, 750.0, 800.0, 850.0])
            .with_color(vec4(0.12, 0.47, 0.71, 0.8)));
        traffic.add_series(AreaSeries::new("Referral")
            .with_data(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
                       vec![300.0, 350.0, 400.0, 380.0, 450.0, 420.0, 500.0])
            .with_color(vec4(0.94, 0.78, 0.0, 0.8)));
        traffic.set_stacked(true);
        traffic.set_show_grid(true);
        traffic.redraw(cx);

        // Energy Mix
        let energy = self.ui.area_chart(&[id!(detail_area_energy)]);
        energy.set_title("Energy Production");
        energy.add_series(AreaSeries::new("Solar")
            .with_data(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
                       vec![10.0, 15.0, 25.0, 35.0, 45.0, 55.0])
            .with_color(vec4(0.94, 0.78, 0.0, 0.8)));
        energy.add_series(AreaSeries::new("Wind")
            .with_data(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
                       vec![20.0, 25.0, 30.0, 35.0, 40.0, 45.0])
            .with_color(vec4(0.12, 0.47, 0.71, 0.8)));
        energy.add_series(AreaSeries::new("Hydro")
            .with_data(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
                       vec![30.0, 32.0, 31.0, 33.0, 32.0, 34.0])
            .with_color(vec4(0.17, 0.63, 0.17, 0.8)));
        energy.set_stacked(true);
        energy.set_show_grid(true);
        energy.redraw(cx);

        // Revenue Trend
        let revenue = self.ui.area_chart(&[id!(detail_area_revenue)]);
        revenue.set_title("Quarterly Revenue");
        revenue.add_series(AreaSeries::new("Q1-Q4")
            .with_data(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
                       vec![100.0, 120.0, 115.0, 140.0, 160.0, 155.0, 180.0, 200.0])
            .with_color(vec4(0.56, 0.27, 0.68, 0.7)));
        revenue.set_stacked(false);
        revenue.set_show_grid(true);
        revenue.redraw(cx);
    }

    fn setup_step_detail(&mut self, cx: &mut Cx) {
        // Price Changes (Pre-step)
        let price = self.ui.step_plot(&[id!(detail_step_price)]);
        price.set_title("Stock Price");
        price.add_series(StepSeries::new("ACME")
            .with_data(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0],
                       vec![100.0, 105.0, 103.0, 110.0, 108.0, 115.0, 112.0, 120.0])
            .with_style(StepStyle::Pre)
            .with_color(vec4(0.12, 0.47, 0.71, 1.0)));
        price.set_show_grid(true);
        price.redraw(cx);

        // CPU Usage (Post-step)
        let cpu = self.ui.step_plot(&[id!(detail_step_cpu)]);
        cpu.set_title("CPU Utilization %");
        cpu.add_series(StepSeries::new("Server 1")
            .with_data(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0],
                       vec![20.0, 45.0, 30.0, 80.0, 65.0, 40.0, 55.0, 35.0])
            .with_style(StepStyle::Post)
            .with_color(vec4(0.84, 0.15, 0.16, 1.0)));
        cpu.add_series(StepSeries::new("Server 2")
            .with_data(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0],
                       vec![35.0, 50.0, 45.0, 60.0, 55.0, 70.0, 50.0, 45.0])
            .with_style(StepStyle::Post)
            .with_color(vec4(0.17, 0.63, 0.17, 1.0)));
        cpu.set_show_grid(true);
        cpu.redraw(cx);

        // Queue Length (Mid-step)
        let queue = self.ui.step_plot(&[id!(detail_step_queue)]);
        queue.set_title("Message Queue");
        queue.add_series(StepSeries::new("Queue Depth")
            .with_data(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0],
                       vec![5.0, 12.0, 8.0, 25.0, 15.0, 10.0, 18.0, 6.0])
            .with_style(StepStyle::Mid)
            .with_color(vec4(0.56, 0.27, 0.68, 1.0)));
        queue.set_show_grid(true);
        queue.redraw(cx);
    }
}

app_main!(App);

fn main() {
    app_main()
}
