# Makepad Plot - Matplotlib Demo Development Plan

## Current Implementation Status (Updated)

### Implemented Chart Types (32 Widgets)

| Chart Type | Status | Key API Methods |
|------------|--------|-----------------|
| LinePlot | ✅ Done | add_series, fill_between, axvline, axhline, error bars, set_x/y_scale, legend |
| BarPlot | ✅ Done | set_data, set_horizontal, set_stacked, add_group, set_show_bar_labels |
| ScatterPlot | ✅ Done | add_series, set_point_radius, set_use_gradient, legend |
| PieChart | ✅ Done | add_slice, set_data, show_percentages, show_labels, legend |
| DonutChart | ✅ Done | Inner radius support, same API as PieChart |
| HistogramChart | ✅ Done | set_values, set_num_bins |
| BoxPlotChart | ✅ Done | add_item, add_from_values, show_outliers |
| ViolinPlot | ✅ Done | add_item, show_box, show_median |
| HeatmapChart | ✅ Done | set_data, 11 colormaps, show_values |
| StemPlot | ✅ Done | add_series, set_baseline, marker support |
| StepPlot | ✅ Done | Step line visualization |
| AreaChart | ✅ Done | Filled area under curves |
| PolarPlot | ✅ Done | add_series, polar coordinates, fill support |
| RadarChart | ✅ Done | Spider/radar with gradient fills |
| ContourPlot | ✅ Done | 2D contour visualization |
| QuiverPlot | ✅ Done | Vector field visualization |
| Surface3D | ✅ Done | 3D surface with colormap |
| Scatter3D | ✅ Done | 3D scatter plot |
| Line3D | ✅ Done | 3D line plot |
| SubplotGrid | ✅ Done | Grid layout for multiple plots |
| LinePlotDual | ✅ Done | Dual Y-axis support |
| CandlestickChart | ✅ Done | Financial OHLC charts |
| WaterfallChart | ✅ Done | Waterfall visualization |
| GaugeChart | ✅ Done | Gauge/meter visualization |
| FunnelChart | ✅ Done | Funnel visualization |
| Treemap | ✅ Done | Hierarchical treemap |
| BubbleChart | ✅ Done | Bubble plot with size encoding, gradient |
| Heatmap | ✅ Done | Alternative heatmap implementation |
| **Stackplot** | ✅ Done | **Stacked area with StackOrder, StackOffset (wiggle for streamgraph)** |
| **HexbinChart** | ✅ Done | **Hexagonal binning with cube coordinates, radial gradient** |
| **Streamgraph** | ✅ Done | **Centered stacked area (silhouette offset)** |
| **SankeyDiagram** | ✅ Done | **Flow diagram with curved links, auto-layout** |

### Implemented Features

| Feature | Status | Notes |
|---------|--------|-------|
| Line Styles | ✅ Done | Solid, Dashed, Dotted, DashDot |
| Marker Styles | ✅ Done | Circle, Square, TriangleUp, TriangleDown, Diamond, Cross, Plus, Star |
| Error Bars | ✅ Done | Symmetric & asymmetric, X and Y errors with caps |
| Fill Between | ✅ Done | fill_between, fill_between_baseline |
| axvline/axhline | ✅ Done | Vertical and horizontal reference lines |
| Multiple Series | ✅ Done | All chart types |
| Auto-range | ✅ Done | All charts |
| Pan/Zoom | ✅ Done | LinePlot, ScatterPlot (interactive mode) |
| LaTeX Annotations | ✅ Done | Using math_widget |
| Scale Types | ✅ Done | Linear, Log, SymLog, Time |
| Legend | ✅ Done | TopRight, TopLeft, BottomRight, BottomLeft, None |
| Grid Lines | ✅ Done | All axis-based charts |
| Axis Labels | ✅ Done | Title, xlabel, ylabel |
| Colormaps | ✅ Done | Viridis, Plasma, Inferno, Magma, Cividis, Coolwarm, RdBu, Spectral, Blues, Greens, Reds |
| Gradient Points | ✅ Done | ScatterPlot, BubbleChart, RadarChart |
| Arrow Annotations | ✅ Done | LinePlot |

---

## Matplotlib Gallery Coverage Analysis

### 1. lines_bars_and_markers (49 examples)
| Example | Status | Notes |
|---------|--------|-------|
| simple_plot.py | ✅ Ready | LinePlot |
| barchart.py | ✅ Ready | BarPlot |
| barh.py | ✅ Ready | BarPlot.set_horizontal(true) |
| bar_stacked.py | ✅ Ready | BarPlot.set_stacked(true) |
| bar_colors.py | ✅ Ready | BarPlot with colors |
| bar_label_demo.py | ✅ Ready | BarPlot.set_show_bar_labels(true) |
| scatter_demo2.py | ✅ Ready | ScatterPlot |
| scatter_with_legend.py | ✅ Ready | ScatterPlot + legend |
| fill.py | ✅ Ready | fill_between |
| fill_between_demo.py | ✅ Ready | fill_between |
| fill_between_alpha.py | ✅ Ready | fill_between with alpha |
| step_demo.py | ✅ Ready | StepPlot |
| stem_plot.py | ✅ Ready | StemPlot |
| stackplot_demo.py | ❌ Missing | Need stacked area chart |
| errorbar_*.py | ✅ Ready | Series.with_yerr(), with_xerr() |
| linestyles.py | ✅ Ready | LineStyle enum |
| marker_reference.py | ✅ Ready | MarkerStyle enum (8 types) |
| vline_hline_demo.py | ✅ Ready | axvline, axhline |
| span_regions.py | ✅ Ready | fill_between |
| eventplot.py | ❌ Missing | Need EventPlot |

### 2. statistics (22 examples)
| Example | Status | Notes |
|---------|--------|-------|
| hist.py | ✅ Ready | HistogramChart |
| histogram_*.py | ✅ Ready | HistogramChart variations |
| boxplot.py | ✅ Ready | BoxPlotChart |
| boxplot_demo.py | ✅ Ready | BoxPlotChart |
| boxplot_color.py | ✅ Ready | BoxPlotChart with colors |
| violinplot.py | ✅ Ready | ViolinPlot |
| errorbar.py | ✅ Ready | LinePlot with error bars |
| hexbin_demo.py | ❌ Missing | Need HexbinPlot |
| confidence_ellipse.py | ❌ Missing | Need ellipse drawing |
| bxp.py | ✅ Ready | BoxPlotChart |

### 3. pie_and_polar_charts (10 examples)
| Example | Status | Notes |
|---------|--------|-------|
| pie_features.py | ✅ Ready | PieChart |
| pie_label.py | ✅ Ready | PieChart with labels |
| pie_and_donut_labels.py | ✅ Ready | DonutChart |
| nested_pie.py | ⚠️ Partial | Need nested rings |
| bar_of_pie.py | ❌ Missing | Need compound chart |
| polar_demo.py | ✅ Ready | PolarPlot |
| polar_bar.py | ⚠️ Partial | PolarPlot (need bar mode) |
| polar_scatter.py | ✅ Ready | PolarPlot with markers |
| radar_chart.py | ✅ Ready | RadarChart |

### 4. images_contours_and_fields (48 examples)
| Example | Status | Notes |
|---------|--------|-------|
| image_annotated_heatmap.py | ✅ Ready | HeatmapChart |
| matshow.py | ✅ Ready | HeatmapChart |
| pcolor_demo.py | ✅ Ready | HeatmapChart |
| contour_demo.py | ✅ Ready | ContourPlot |
| contourf_demo.py | ✅ Ready | ContourPlot with set_filled(true) |
| quiver_demo.py | ✅ Ready | QuiverPlot |
| streamplot.py | ❌ Missing | Need StreamPlot |
| barb_demo.py | ❌ Missing | Need WindBarbPlot |
| tricontour_demo.py | ❌ Missing | Need triangulated contours |
| specgram.py | ❌ Missing | Need Spectrogram |

### 5. mplot3d (47 examples)
| Example | Status | Notes |
|---------|--------|-------|
| scatter3d.py | ✅ Ready | Scatter3D |
| surface3d.py | ✅ Ready | Surface3D |
| lines3d.py | ✅ Ready | Line3D |
| wireframe3d.py | ⚠️ Partial | Surface3D (need wireframe mode) |
| bars3d.py | ❌ Missing | Need Bar3D |
| contour3d.py | ❌ Missing | Need Contour3D |
| voxels.py | ❌ Missing | Need VoxelPlot |
| quiver3d.py | ❌ Missing | Need Quiver3D |

### 6. subplots_axes_and_figures
| Feature | Status | Notes |
|---------|--------|-------|
| Subplot grid | ✅ Ready | SubplotGrid |
| Shared axes | ⚠️ Partial | Need shared axis support |
| Inset axes | ❌ Missing | Need inset support |
| Secondary y-axis | ✅ Ready | LinePlotDual |
| Twin axes | ✅ Ready | LinePlotDual |

### 7. specialty_plots
| Example | Status | Notes |
|---------|--------|-------|
| radar_chart.py | ✅ Ready | RadarChart |
| sankey.py | ❌ Missing | Need SankeyDiagram |
| hinton.py | ❌ Missing | Need HintonDiagram |
| ishikawa.py | ❌ Missing | Need IshikawaDiagram |

### 8. animation
| Feature | Status | Notes |
|---------|--------|-------|
| Basic animation | ⚠️ Partial | Can use NextFrame + redraw |
| FuncAnimation API | ❌ Missing | Need animation framework |

---

## Remaining Gaps (Priority Order)

### High Priority - Common Use Cases
| Feature | Complexity | Notes |
|---------|-----------|-------|
| ~~Stackplot (stacked area)~~ | ~~Medium~~ | ✅ **DONE** |
| ~~HexbinPlot~~ | ~~Medium~~ | ✅ **DONE** |
| StreamPlot | High | Flow field visualization (different from Streamgraph) |
| ~~Filled ContourPlot~~ | ~~Low~~ | ✅ **DONE** - set_filled(true) |
| Bar3D | Medium | 3D bar chart |

### Medium Priority - Advanced Features
| Feature | Complexity | Notes |
|---------|-----------|-------|
| Spectrogram | High | Time-frequency analysis |
| EventPlot | Low | Event timeline |
| WindBarbPlot | Medium | Meteorological |
| Confidence Ellipse | Medium | Statistical |
| Wireframe mode | Low | Add to Surface3D |
| Polar bar mode | Low | Add to PolarPlot |
| Nested pie/donut | Medium | Multi-ring pie |
| Animation framework | High | Proper animation API |

### Low Priority - Specialized
| Feature | Complexity | Notes |
|---------|-----------|-------|
| ~~SankeyDiagram~~ | ~~High~~ | ✅ **DONE** |
| HintonDiagram | Medium | Matrix visualization |
| IshikawaDiagram | Medium | Fishbone/cause-effect |
| VoxelPlot | High | 3D voxels |
| Quiver3D | Medium | 3D vector field |
| Contour3D | Medium | 3D contours |
| TriContour | High | Triangulated mesh |

---

## Coverage Summary

| Category | Total | Implemented | Coverage |
|----------|-------|-------------|----------|
| lines_bars_and_markers | 49 | ~47 | ~96% |
| statistics | 22 | ~20 | ~91% |
| pie_and_polar_charts | 10 | ~8 | ~80% |
| images_contours_and_fields | 48 | ~22 | ~46% |
| mplot3d | 47 | ~15 | ~32% |
| subplots_axes_and_figures | 15 | ~10 | ~67% |
| specialty_plots | 12 | ~6 | ~50% |
| **Overall** | **203** | **~128** | **~63%** |

---

## API Comparison: Matplotlib vs Makepad Plot

### Fully Equivalent
| Matplotlib | Makepad Plot |
|------------|--------------|
| `plt.plot(x, y)` | `plot.add_series(Series::new("").with_data(x, y))` |
| `plt.plot(x, y, '--')` | `Series::new("").with_data(x,y).with_line_style(LineStyle::Dashed)` |
| `plt.plot(x, y, 'o')` | `Series::new("").with_data(x,y).with_marker(MarkerStyle::Circle)` |
| `plt.scatter(x, y)` | `scatter.add_series(Series::new("").with_data(x, y))` |
| `plt.bar(x, height)` | `bar.set_data(categories, values)` |
| `plt.barh(x, height)` | `bar.set_horizontal(true)` |
| `plt.pie(values)` | `pie.set_data(labels, values)` |
| `plt.hist(data, bins)` | `hist.set_values(data); hist.set_num_bins(bins)` |
| `plt.boxplot(data)` | `box.add_from_values(label, data)` |
| `plt.violinplot(data)` | `violin.add_item(ViolinItem::from_values(data))` |
| `plt.imshow(data)` | `heatmap.set_data(data)` |
| `plt.contour(X, Y, Z)` | `contour.set_data(x, y, z)` |
| `plt.quiver(X, Y, U, V)` | `quiver.set_data(x, y, u, v)` |
| `plt.fill_between(x, y1, y2)` | `plot.fill_between(x, y1, y2, color)` |
| `plt.errorbar(x, y, yerr)` | `Series::new("").with_data(x, y).with_yerr(yerr)` |
| `plt.step(x, y)` | `StepPlot` |
| `plt.stem(x, y)` | `StemPlot` |
| `plt.axvline(x)` | `plot.axvline(x, color, width, style)` |
| `plt.axhline(y)` | `plot.axhline(y, color, width, style)` |
| `plt.title("...")` | `plot.set_title("...")` |
| `plt.xlabel("...")` | `plot.set_xlabel("...")` |
| `plt.xlim(min, max)` | `plot.set_xlim(min, max)` |
| `plt.xscale("log")` | `plot.set_x_scale(ScaleType::Log)` |
| `plt.legend()` | `plot.set_legend(LegendPosition::TopRight)` |
| `ax.annotate(text, xy)` | `plot.annotate(text, x, y, color, size)` |
| `mplot3d.scatter3D` | `Scatter3D` |
| `mplot3d.plot_surface` | `Surface3D` |
| `mplot3d.plot3D` | `Line3D` |
| `plt.subplot(r,c,n)` | `SubplotGrid` |
| `ax.twinx()` | `LinePlotDual` |

### Still Missing
| Matplotlib | Priority | Notes |
|------------|----------|-------|
| `plt.stackplot()` | High | Stacked area |
| `plt.hexbin()` | High | 2D hex histogram |
| `plt.streamplot()` | Medium | Flow lines |
| `plt.specgram()` | Medium | Spectrogram |
| `plt.eventplot()` | Low | Event timeline |
| `plt.barbs()` | Low | Wind barbs |
| `FuncAnimation` | Medium | Animation API |

---

## Next Development Priorities

### Immediate (High Impact) - ✅ ALL DONE
1. ~~**Stackplot**~~ - ✅ Done
2. ~~**HexbinPlot**~~ - ✅ Done
3. ~~**Filled Contours**~~ - ✅ Done (set_filled exists)

### Short Term
4. **StreamPlot** - Flow visualization (vector field flow lines)
5. **Bar3D** - Complete 3D chart set
6. **Animation Framework** - Basic animation support

### Medium Term
7. **Spectrogram** - Audio/signal analysis
8. **Nested Pie** - Multi-ring donut
9. **Polar Bars** - Wind rose style charts

---

## Files Structure

```
src/
├── lib.rs           # Module exports
├── plot.rs          # All chart implementations (9000+ lines)
├── elements.rs      # Drawing primitives, shaders
└── text.rs          # Text rendering utilities

examples/
└── plot_demo.rs     # Interactive demo with all charts
```

## Demo App Pages

The demo app (`cargo run`) includes:
- **Overview**: Grid of all basic charts
- **Detail pages**: Each chart type with interactive controls
- **3D charts**: Surface3D, Scatter3D, Line3D
- **Financial**: CandlestickChart
- **Statistical**: ViolinPlot, BoxPlotChart
- **Specialty**: RadarChart, TreeMap, Sankey (partial)
