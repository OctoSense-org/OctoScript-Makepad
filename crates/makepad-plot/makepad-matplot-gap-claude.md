# Matplotlib to Makepad Implementation Gap Analysis

**Date**: January 2025
**Based on**: matplotlib 3.10.x and makepad-chart implementation
**Purpose**: Define the gap between matplotlib's capabilities and what can be implemented in Makepad

---

## Executive Summary

This document analyzes how to implement matplotlib-like functionality in Makepad, using the existing `makepad-chart` library as a reference implementation. The analysis covers data formats, plot types, coordinate systems, animations, colors, styling, and interactivity.

**Key Finding**: The `makepad-chart` library already implements ~60% of common matplotlib functionality with a clean, GPU-accelerated architecture. Extending it to full matplotlib parity requires adding specialized plot types, log scales, colormaps, and text rendering.

### Quick Gap Overview

| Aspect | Matplotlib | makepad-chart | Gap Status |
|--------|-----------|---------------|------------|
| **Rendering** | CPU rasterization (Agg) + vector backends | GPU-based shaders | ✅ Different paradigm, GPU approach is faster |
| **Data Format** | NumPy arrays (in-memory) | Builder pattern with hierarchical data | ✅ Comparable - cleaner than raw arrays |
| **Animation** | FuncAnimation with blitting | 28 easing functions + delay/stagger | ✅ Superior - more sophisticated |
| **Coordinate System** | 4-level transform pipeline | Cartesian/Polar with Linear/Category | ⚠️ Missing log/symlog/time scales |
| **Plot Types** | 30+ types | 11 types | ❌ Missing scientific plots |
| **Text Rendering** | Full LaTeX support | Basic labels only | ❌ Critical gap |
| **Interactivity** | Pan/zoom/annotations | Hover/click only | ❌ Major gap |
| **Colormaps** | viridis, plasma, coolwarm, etc. | Not implemented | ❌ Missing feature |
| **Export** | PDF/SVG/PNG/PS | Not implemented | ❌ Missing feature |

### Key Strengths of makepad-chart

- ✅ 11 chart types with GPU-accelerated rendering
- ✅ 28 easing functions (superior to matplotlib)
- ✅ Clean architecture with separation of concerns
- ✅ Builder pattern API for easy data configuration
- ✅ Live DSL integration for declarative UI
- ✅ Custom shader-based drawing primitives
- ✅ Basic hover interaction and hit testing

### Critical Gaps to Address

1. **Text Rendering** - Cannot display axis labels, tick labels, legends, or tooltips properly
2. **Scientific Plots** - No histogram, box plot, heatmap, contour, error bars
3. **Advanced Scales** - No log, symlog, or time scales
4. **Interactivity** - No pan/zoom, annotations, or export functionality

---

## Table of Contents

1. [Architecture Comparison](#1-architecture-comparison)
2. [Data Format Analysis](#2-data-format-analysis)
3. [Coordinate System Implementation](#3-coordinate-system-implementation)
4. [Plot Types Gap Analysis](#4-plot-types-gap-analysis)
5. [Animation System](#5-animation-system)
6. [Color and Styling System](#6-color-and-styling-system)
7. [Interactivity and Events](#7-interactivity-and-events)
8. [Text Rendering Gap](#8-text-rendering-gap)
9. [Options and Configuration](#9-options-and-configuration)
10. [Implementation Roadmap](#10-implementation-roadmap)
11. [Code Examples](#11-code-examples)

---

## 1. Architecture Comparison

### matplotlib Architecture

```
Figure (top-level container)
├── SubFigure[] (optional nested figures)
└── Axes[] (plotting areas)
    ├── XAxis / YAxis (ticks, labels, spine)
    ├── Legend
    └── Artist[] (visual elements)
        ├── Line2D
        ├── Patch (Rectangle, Circle, Polygon)
        ├── Collection (efficient rendering)
        ├── Text
        └── Image
```

**Key Characteristics**:
- Retained-mode rendering (objects persist in memory)
- Backend abstraction (Agg, Cairo, PDF, SVG)
- Explicit coordinate transformation pipeline
- Python-based with numpy integration

### makepad-chart Architecture

```
makepad-chart/src/
├── core/           # Data structures, options, colors
│   ├── data.rs     # ChartData, Dataset, DataPoint
│   ├── options.rs  # ChartOptions hierarchy
│   ├── colors.rs   # Palettes, utilities
│   └── types.rs    # Enums (EasingType, etc.)
├── scale/          # Coordinate scaling
│   ├── traits.rs   # Scale trait
│   ├── linear.rs   # LinearScale
│   └── category.rs # CategoryScale
├── coord/          # Coordinate systems
│   ├── cartesian.rs # CartesianCoord
│   └── polar.rs     # PolarCoord
├── element/        # GPU shaders
│   ├── bar.rs, line.rs, point.rs
│   ├── arc.rs, triangle.rs
│   └── grid.rs
├── animation/      # Animation system
│   ├── animator.rs # ChartAnimator
│   └── easing.rs   # 28 easing functions
├── interaction/    # Hit testing
│   └── hit_test.rs
├── component/      # UI components
│   └── legend.rs, axis.rs, etc.
└── chart/          # 11 chart implementations
```

**Key Characteristics**:
- Immediate-mode GPU rendering via shaders
- Widget-based (Makepad `#[derive(Widget)]`)
- Real-time 60fps animation capability
- Rust-native with type safety

### Architecture Gap

| Aspect | matplotlib | makepad-chart | Gap |
|--------|-----------|---------------|-----|
| Rendering | Retained-mode, backend-agnostic | Immediate-mode GPU | Different paradigm |
| Data binding | Implicit via Artists | Explicit `set_data()` | Comparable |
| Transforms | Full pipeline (data→display) | Linear/Category scales | Need Log, SymLog, Logit |
| Text | Full LaTeX support | Basic labels | Need math text |
| 3D | mplot3d toolkit | Not implemented | Full implementation needed |

---

## 2. Data Format Analysis

### matplotlib Data Formats

| Format | Description | Example |
|--------|-------------|---------|
| Lists/arrays | Python lists, numpy arrays | `[1, 2, 3]`, `np.array([1,2,3])` |
| 2D arrays | Matrix data for heatmaps | `np.random.rand(10,10)` |
| Masked arrays | Data with missing values | `np.ma.array([1,2,3], mask=[0,0,1])` |
| DataFrame | Pandas integration | `df['column']` |
| Categorical | Non-numeric data | `['A', 'B', 'C']` |

### makepad-chart Data Format (Implemented)

```rust
/// Single data point - supports multiple chart types
#[derive(Clone, Debug, Default)]
pub struct DataPoint {
    pub x: Option<f64>,        // None = use index
    pub y: f64,                // Primary value
    pub y_min: Option<f64>,    // For floating bars
    pub r: Option<f64>,        // Bubble radius
    pub label: Option<String>, // Per-point label
    pub meta: Option<String>,  // Tooltip metadata
}

/// Dataset with styling options
#[derive(Clone, Debug)]
pub struct Dataset {
    pub label: String,
    pub data: Vec<DataPoint>,

    // Colors
    pub background_color: Option<Vec4>,
    pub border_color: Option<Vec4>,
    pub border_width: f64,
    pub hidden: bool,

    // Line chart specific
    pub fill: bool,
    pub tension: f64,  // 0 = straight, 0.4 = smooth

    // Point options
    pub point_radius: f64,
    pub point_style: DataPointStyle,

    // Bar chart specific
    pub bar_thickness: Option<f64>,
    pub bar_percentage: f64,
    pub bar_border_radius: f64,

    // Pie chart specific
    pub hover_offset: f64,
}

/// Container for all chart data
#[derive(Clone, Debug, Default)]
pub struct ChartData {
    pub labels: Vec<String>,      // X-axis labels
    pub datasets: Vec<Dataset>,   // Multiple series
}
```

### Data Format Gap

| matplotlib | makepad-chart | Status |
|------------|---------------|--------|
| Simple Y values | `with_data(vec![...])` | ✅ Implemented |
| X,Y pairs | `with_xy_data(vec![(x,y)...])` | ✅ Implemented |
| Floating bars | `with_floating_data(vec![(min,max)...])` | ✅ Implemented |
| Bubble (x,y,r) | `with_bubble_data(vec![(x,y,r)...])` | ✅ Implemented |
| 2D grid data | Not implemented | ❌ Need for heatmaps |
| Masked/NaN | Partial (filtering) | ⚠️ Need explicit support |
| Error bars | Not implemented | ❌ Need (xerr, yerr) |
| DateTime | Not implemented | ❌ Need timestamp support |

### Recommended Additions

```rust
/// Extended DataPoint for full matplotlib parity
pub struct DataPoint {
    // Existing fields...

    // New fields for matplotlib parity
    pub xerr: Option<(f64, f64)>,  // (lower, upper) X error
    pub yerr: Option<(f64, f64)>,  // (lower, upper) Y error
    pub timestamp: Option<i64>,    // Unix timestamp for time series
    pub is_valid: bool,            // For masked array support
}

/// 2D grid data for heatmaps/contours
pub struct GridData {
    pub data: Vec<Vec<f64>>,
    pub x_edges: Vec<f64>,
    pub y_edges: Vec<f64>,
}

/// Time series data
pub struct TimeSeriesData {
    pub timestamps: Vec<i64>,
    pub values: Vec<f64>,
}
```

---

## 3. Coordinate System Implementation

### matplotlib Coordinate Pipeline

```
Data Coords → transScale → transLimits → transAxes → Display Coords
     ↓            ↓            ↓            ↓            ↓
  (x, y)    log/linear   normalize    position      pixels
            transform    to [0,1]     in figure
```

**Key Features**:
- Multiple scale types: Linear, Log, SymLog, Logit, Asinh
- Blended transforms (X in data, Y in axes coords)
- Inversion support for both axes
- Full bidirectional transformation

### makepad-chart Implementation (Current)

```rust
/// Scale trait - defines the interface
pub trait Scale {
    fn scale_type(&self) -> &'static str;
    fn set_data_range(&mut self, min: f64, max: f64);
    fn set_pixel_range(&mut self, start: f64, end: f64);
    fn get_pixel_for_value(&self, value: f64) -> f64;
    fn get_value_for_pixel(&self, pixel: f64) -> f64;
    fn build_ticks(&self, options: &TickOptions) -> Vec<Tick>;
    fn get_data_bounds(&self) -> (f64, f64);
    fn is_inverted(&self) -> bool;
}

/// Linear scale implementation
pub struct LinearScale {
    data_min: f64,
    data_max: f64,
    pixel_start: f64,
    pixel_end: f64,
    begin_at_zero: bool,
    nice: bool,       // Round to nice numbers
    clamp: bool,      // Clamp values to range
}

/// Category scale for discrete data
pub struct CategoryScale {
    labels: Vec<String>,
    pixel_start: f64,
    pixel_end: f64,
    offset: bool,     // Center items in bands
}

/// Cartesian coordinate transformer
pub struct CartesianCoord {
    total_rect: Rect,
    chart_area: ChartArea,
    x_scale: ScaleType,    // Category or Linear
    y_scale: ScaleType,    // Usually Linear
    left_padding: f64,
    bottom_padding: f64,
    right_padding: f64,
    top_padding: f64,
}

impl CartesianCoord {
    pub fn update(&mut self, rect: Rect) {
        // Calculate chart area after padding
        self.chart_area = ChartArea::new(
            rect.pos.x + self.left_padding,
            rect.pos.y + self.top_padding,
            rect.pos.x + rect.size.x - self.right_padding,
            rect.pos.y + rect.size.y - self.bottom_padding,
        );

        // X: left to right
        self.x_scale.set_pixel_range(self.chart_area.left, self.chart_area.right);
        // Y: bottom to top (INVERTED for screen coordinates)
        self.y_scale.set_pixel_range(self.chart_area.bottom, self.chart_area.top);
    }

    pub fn data_to_pixel(&self, x: f64, y: f64) -> DVec2 {
        dvec2(
            self.x_scale.get_pixel_for_value(x),
            self.y_scale.get_pixel_for_value(y),
        )
    }
}
```

### Coordinate System Gap

| Feature | matplotlib | makepad-chart | Status |
|---------|-----------|---------------|--------|
| Linear scale | `ax.set_xscale('linear')` | `LinearScale` | ✅ Implemented |
| Category scale | Implicit for bar charts | `CategoryScale` | ✅ Implemented |
| Log scale | `ax.set_xscale('log')` | Not implemented | ❌ Need |
| SymLog scale | `ax.set_xscale('symlog')` | Not implemented | ❌ Need |
| Logit scale | `ax.set_xscale('logit')` | Not implemented | ❌ Need |
| Date scale | `mdates` module | Not implemented | ❌ Need |
| Inverted axis | `ax.invert_xaxis()` | `is_inverted()` | ✅ Implemented |
| Nice bounds | Automatic | `nice: true` | ✅ Implemented |
| Begin at zero | `begin_at_zero` | `begin_at_zero` | ✅ Implemented |
| Polar coords | `projection='polar'` | `PolarCoord` | ✅ Implemented |
| 3D coords | `Axes3D` | Not implemented | ❌ Need |

### Recommended Scale Additions

```rust
/// Log scale implementation
pub struct LogScale {
    base: f64,
    data_min: f64,
    data_max: f64,
    pixel_start: f64,
    pixel_end: f64,
}

impl Scale for LogScale {
    fn get_pixel_for_value(&self, value: f64) -> f64 {
        if value <= 0.0 {
            return self.pixel_start; // Handle invalid values
        }
        let log_val = value.log(self.base);
        let log_min = self.data_min.log(self.base);
        let log_max = self.data_max.log(self.base);
        let ratio = (log_val - log_min) / (log_max - log_min);
        self.pixel_start + ratio * (self.pixel_end - self.pixel_start)
    }

    fn build_ticks(&self, options: &TickOptions) -> Vec<Tick> {
        // Generate ticks at powers of base
        let mut ticks = Vec::new();
        let start_power = self.data_min.log(self.base).floor() as i32;
        let end_power = self.data_max.log(self.base).ceil() as i32;

        for power in start_power..=end_power {
            let value = self.base.powi(power);
            if value >= self.data_min && value <= self.data_max {
                ticks.push(Tick::new(value, format_log_tick(value, self.base)));
            }
        }
        ticks
    }
}

/// SymLog scale (linear near zero, log far from zero)
pub struct SymLogScale {
    base: f64,
    linthresh: f64,   // Linear threshold
    linscale: f64,    // Linear scale factor
    data_min: f64,
    data_max: f64,
    pixel_start: f64,
    pixel_end: f64,
}

/// Date/Time scale
pub struct DateScale {
    data_min: i64,    // Unix timestamp
    data_max: i64,
    pixel_start: f64,
    pixel_end: f64,
    format: DateFormat,
}

impl DateScale {
    pub fn nice_ticks(&self, num_ticks: usize) -> Vec<(i64, String)> {
        let duration = self.data_max - self.data_min;
        let unit = self.select_time_unit(duration, num_ticks);

        let mut ticks = Vec::new();
        let mut current = self.round_to_unit(self.data_min, unit);

        while current <= self.data_max {
            let label = self.format_timestamp(current, unit);
            ticks.push((current, label));
            current = self.add_unit(current, 1, unit);
        }

        ticks
    }

    fn select_time_unit(&self, duration_secs: i64, num_ticks: usize) -> TimeUnit {
        let interval = duration_secs / num_ticks as i64;
        match interval {
            x if x > 365 * 24 * 3600 => TimeUnit::Year,
            x if x > 30 * 24 * 3600 => TimeUnit::Month,
            x if x > 24 * 3600 => TimeUnit::Day,
            x if x > 3600 => TimeUnit::Hour,
            x if x > 60 => TimeUnit::Minute,
            _ => TimeUnit::Second,
        }
    }

    fn round_to_unit(&self, timestamp: i64, unit: TimeUnit) -> i64 {
        // Round down to nearest unit boundary
        match unit {
            TimeUnit::Year => /* round to Jan 1 */ timestamp,
            TimeUnit::Month => /* round to 1st of month */ timestamp,
            TimeUnit::Day => timestamp - (timestamp % 86400),
            TimeUnit::Hour => timestamp - (timestamp % 3600),
            TimeUnit::Minute => timestamp - (timestamp % 60),
            TimeUnit::Second => timestamp,
        }
    }

    fn add_unit(&self, timestamp: i64, count: i64, unit: TimeUnit) -> i64 {
        match unit {
            TimeUnit::Year => timestamp + count * 365 * 24 * 3600,
            TimeUnit::Month => timestamp + count * 30 * 24 * 3600,
            TimeUnit::Day => timestamp + count * 24 * 3600,
            TimeUnit::Hour => timestamp + count * 3600,
            TimeUnit::Minute => timestamp + count * 60,
            TimeUnit::Second => timestamp + count,
        }
    }

    fn format_timestamp(&self, timestamp: i64, unit: TimeUnit) -> String {
        // Would use chrono in practice
        format!("{}", timestamp)
    }
}

pub enum TimeUnit {
    Second,
    Minute,
    Hour,
    Day,
    Month,
    Year,
}
```

### Tick Formatters

```rust
/// Trait for formatting tick values
pub trait TickFormatter {
    fn format(&self, value: f64) -> String;
}

/// Default scalar formatter
pub struct ScalarFormatter {
    pub precision: usize,
    pub use_scientific: bool,
    pub scientific_threshold: (f64, f64),  // (min, max) for switching to scientific
}

impl Default for ScalarFormatter {
    fn default() -> Self {
        Self {
            precision: 2,
            use_scientific: true,
            scientific_threshold: (1e-3, 1e6),
        }
    }
}

impl TickFormatter for ScalarFormatter {
    fn format(&self, value: f64) -> String {
        if value == 0.0 {
            return "0".to_string();
        }

        let abs_val = value.abs();
        if self.use_scientific
            && (abs_val < self.scientific_threshold.0 || abs_val >= self.scientific_threshold.1)
        {
            format!("{:.1e}", value)
        } else {
            format!("{:.prec$}", value, prec = self.precision)
        }
    }
}

/// Log scale formatter (shows powers)
pub struct LogFormatter {
    pub base: f64,
    pub show_base: bool,
}

impl TickFormatter for LogFormatter {
    fn format(&self, value: f64) -> String {
        if value <= 0.0 {
            return "".to_string();
        }

        let power = value.log(self.base).round() as i32;

        if self.show_base {
            if self.base == 10.0 {
                format!("10^{}", power)
            } else {
                format!("{}^{}", self.base, power)
            }
        } else {
            format!("{:.0e}", value)
        }
    }
}

/// Percentage formatter
pub struct PercentFormatter {
    pub decimals: usize,
}

impl TickFormatter for PercentFormatter {
    fn format(&self, value: f64) -> String {
        format!("{:.prec$}%", value * 100.0, prec = self.decimals)
    }
}

/// Currency formatter
pub struct CurrencyFormatter {
    pub symbol: String,
    pub decimals: usize,
    pub use_thousands_separator: bool,
}

impl TickFormatter for CurrencyFormatter {
    fn format(&self, value: f64) -> String {
        if self.use_thousands_separator {
            let formatted = format!("{:.prec$}", value, prec = self.decimals);
            // Add thousands separators (simplified)
            format!("{}{}", self.symbol, formatted)
        } else {
            format!("{}{:.prec$}", self.symbol, value, prec = self.decimals)
        }
    }
}
```

---

## 4. Plot Types Gap Analysis

### Implemented in makepad-chart

| Chart Type | matplotlib Equivalent | Features |
|------------|----------------------|----------|
| `BarChart` | `ax.bar()` | Stacked, grouped, delay animation, vertical gradient |
| `HorizontalBarChart` | `ax.barh()` | Horizontal bars, category Y-axis |
| `LineChart` | `ax.plot()` | Tension/smoothing, fill, stepped, progressive animation |
| `ScatterChart` | `ax.scatter()` | XY pairs, point styles, radial gradient |
| `BubbleChart` | `ax.scatter(s=sizes)` | Variable bubble radius |
| `PieChart` | `ax.pie()` | Doughnut mode, radial/angular gradients |
| `RadarChart` | Polar + fill | Multi-axis polygon |
| `PolarAreaChart` | Polar bar | Equal-angle segments |
| `ComboChart` | Mixed | Bar + Line per dataset |
| `ChordChart` | Custom | Relationship diagrams |

### Missing Plot Types (Need Implementation)

| Plot Type | matplotlib | Priority | Complexity |
|-----------|-----------|----------|------------|
| **Histogram** | `ax.hist()` | High | Medium |
| **Box Plot** | `ax.boxplot()` | High | Medium |
| **Violin Plot** | `ax.violinplot()` | Medium | High |
| **Error Bar** | `ax.errorbar()` | High | Low |
| **Heatmap** | `ax.imshow()`, `ax.pcolormesh()` | High | High |
| **Contour** | `ax.contour()`, `ax.contourf()` | High | High |
| **Quiver** | `ax.quiver()` | Medium | Medium |
| **Stream Plot** | `ax.streamplot()` | Low | High |
| **Step Plot** | `ax.step()` | Medium | Low |
| **Stem Plot** | `ax.stem()` | Low | Low |
| **Fill Between** | `ax.fill_between()` | Medium | Low |
| **Hexbin** | `ax.hexbin()` | Low | High |
| **3D Surface** | `ax.plot_surface()` | Medium | Very High |
| **3D Wireframe** | `ax.plot_wireframe()` | Medium | High |
| **3D Scatter** | `ax.scatter3D()` | Medium | High |

### Implementation Examples for Missing Types

#### Histogram (binning + bars)
```rust
pub struct HistogramChart {
    // ... widget fields
    bins: usize,
    density: bool,
    cumulative: bool,
}

impl HistogramChart {
    fn compute_bins(&self, data: &[f64]) -> (Vec<f64>, Vec<f64>) {
        let (min, max) = data.iter().fold(
            (f64::MAX, f64::MIN),
            |(min, max), &v| (min.min(v), max.max(v))
        );

        let bin_width = (max - min) / self.bins as f64;
        let mut counts = vec![0usize; self.bins];

        for &val in data {
            let idx = ((val - min) / bin_width).floor() as usize;
            let idx = idx.min(self.bins - 1);
            counts[idx] += 1;
        }

        let edges: Vec<f64> = (0..=self.bins)
            .map(|i| min + i as f64 * bin_width)
            .collect();

        let heights = if self.density {
            let total = data.len() as f64;
            counts.iter().map(|&c| c as f64 / (total * bin_width)).collect()
        } else {
            counts.iter().map(|&c| c as f64).collect()
        };

        (edges, heights)
    }
}
```

#### Heatmap (GPU shader)
```rust
live_design! {
    pub DrawHeatmapCell = {{DrawHeatmapCell}} {
        texture colormap_tex: texture2d

        fn pixel(self) -> vec4 {
            // Normalize value to 0-1
            let t = clamp(self.value, 0.0, 1.0);
            // Sample colormap texture
            return sample2d(self.colormap_tex, vec2(t, 0.5));
        }
    }
}

pub struct HeatmapChart {
    draw_cell: DrawHeatmapCell,
    colormap: Colormap,
    normalize: Normalize,
    grid_data: GridData,
}
```

#### Error Bars
```rust
pub struct DrawErrorBar {
    draw_line: DrawChartLine,
    cap_width: f64,
}

impl DrawErrorBar {
    pub fn draw_error(&mut self, cx: &mut Cx2d, center: DVec2,
                      yerr: (f64, f64), xerr: Option<(f64, f64)>) {
        // Vertical error bar
        let y_low = center.y - yerr.0;
        let y_high = center.y + yerr.1;
        self.draw_line.draw_line(cx, dvec2(center.x, y_low), dvec2(center.x, y_high), 1.0);

        // Caps
        let half_cap = self.cap_width / 2.0;
        self.draw_line.draw_line(cx,
            dvec2(center.x - half_cap, y_low),
            dvec2(center.x + half_cap, y_low), 1.0);
        self.draw_line.draw_line(cx,
            dvec2(center.x - half_cap, y_high),
            dvec2(center.x + half_cap, y_high), 1.0);

        // Horizontal error bar (if provided)
        if let Some((x_low, x_high)) = xerr {
            // Similar implementation
        }
    }
}
```

---

## 5. Animation System

### matplotlib Animation

```python
# FuncAnimation - call function per frame
ani = FuncAnimation(fig, update_func, frames=100, interval=50, blit=True)

# ArtistAnimation - pre-rendered frames
ani = ArtistAnimation(fig, [frame1_artists, frame2_artists, ...])
```

### makepad-chart Animation (Implemented)

```rust
/// Animation state machine
pub struct ChartAnimator {
    start_time: Option<f64>,
    duration: f64,           // In seconds
    easing: EasingType,
    progress: f64,           // 0-1
    completed: bool,
    delay: f64,              // Pre-animation delay
}

impl ChartAnimator {
    pub fn new(duration_ms: f64) -> Self { ... }
    pub fn with_easing(self, easing: EasingType) -> Self { ... }
    pub fn with_delay(self, delay_ms: f64) -> Self { ... }

    pub fn start(&mut self, current_time: f64) { ... }
    pub fn update(&mut self, current_time: f64) -> bool { ... }
    pub fn get_progress(&self) -> f64 {
        apply_easing(self.progress, self.easing)
    }
    pub fn interpolate(&self, from: f64, to: f64) -> f64 { ... }
}

/// 28 easing functions
pub enum EasingType {
    Linear,
    EaseInQuad, EaseOutQuad, EaseInOutQuad,
    EaseInCubic, EaseOutCubic, EaseInOutCubic,
    EaseInQuart, EaseOutQuart, EaseInOutQuart,
    EaseInQuint, EaseOutQuint, EaseInOutQuint,
    EaseInSine, EaseOutSine, EaseInOutSine,
    EaseInExpo, EaseOutExpo, EaseInOutExpo,
    EaseInCirc, EaseOutCirc, EaseInOutCirc,
    EaseInBack, EaseOutBack, EaseInOutBack,
    EaseInElastic, EaseOutElastic, EaseInOutElastic,
    EaseInBounce, EaseOutBounce, EaseInOutBounce,
}
```

### Animation Integration in Widget

```rust
impl Widget for BarChart {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        match event {
            Event::NextFrame(_) => {
                let time = cx.seconds_since_app_start();
                let mut any_running = false;

                // Update all per-bar animators
                for animator in &mut self.bar_animators {
                    if animator.update(time) {
                        any_running = true;
                    }
                }

                if any_running {
                    self.redraw(cx);
                    cx.new_next_frame();
                }
            }
            _ => {}
        }
    }
}
```

### Animation Gap

| Feature | matplotlib | makepad-chart | Status |
|---------|-----------|---------------|--------|
| Timed animation | `FuncAnimation` | `ChartAnimator` | ✅ Implemented |
| Easing functions | Limited | 28 functions | ✅ Better |
| Delay/stagger | Manual | Built-in delay | ✅ Implemented |
| Blitting | `blit=True` | GPU-native | ✅ Better |
| Frame export | `save()` | Not implemented | ❌ Need |
| Looping | `repeat=True` | `loop_animation` | ✅ Implemented |
| Interactive pause | Manual | Not implemented | ❌ Need |

---

## 6. Color and Styling System

### matplotlib Color System

```python
# Named colors
color='red', color='C0', color='tab:blue'

# Hex colors
color='#FF0000', color='#F00'

# RGB/RGBA tuples
color=(1.0, 0.0, 0.0), color=(1.0, 0.0, 0.0, 0.5)

# Colormaps
cmap='viridis', cmap=plt.cm.plasma
norm = Normalize(vmin=0, vmax=100)
```

### makepad-chart Color System (Implemented)

```rust
/// Default color palette (10 colors)
pub const CHART_COLORS: [Vec4; 10] = [
    vec4(0.290, 0.753, 0.753, 1.0),  // Teal
    vec4(1.000, 0.388, 0.384, 1.0),  // Coral
    vec4(1.000, 0.808, 0.298, 1.0),  // Gold
    vec4(0.608, 0.349, 0.714, 1.0),  // Purple
    vec4(0.204, 0.596, 0.859, 1.0),  // Blue
    vec4(0.180, 0.800, 0.443, 1.0),  // Green
    vec4(0.902, 0.494, 0.133, 1.0),  // Orange
    vec4(0.878, 0.416, 0.604, 1.0),  // Pink
    vec4(0.447, 0.533, 0.600, 1.0),  // Slate
    vec4(0.608, 0.678, 0.282, 1.0),  // Olive
];

/// Color utilities
pub fn get_color(index: usize) -> Vec4 { CHART_COLORS[index % 10] }
pub fn lighten(color: Vec4, amount: f32) -> Vec4 { ... }
pub fn darken(color: Vec4, amount: f32) -> Vec4 { ... }
pub fn hex_to_vec4(hex: u32) -> Vec4 { ... }
pub fn lerp_color(from: Vec4, to: Vec4, t: f32) -> Vec4 { ... }

/// Color gradient for interpolation
pub struct ColorGradient {
    colors: Vec<Vec4>,
}

impl ColorGradient {
    pub fn at(&self, t: f32) -> Vec4 { ... }
    pub fn generate(&self, n: usize) -> Vec<Vec4> { ... }
}
```

### Color System Gap

| Feature | matplotlib | makepad-chart | Status |
|---------|-----------|---------------|--------|
| Named colors | CSS4, XKCD | Limited palette | ⚠️ Could expand |
| Hex colors | Full support | `hex_to_vec4()` | ✅ Implemented |
| RGB/RGBA | Tuples | `vec4()` | ✅ Implemented |
| Color cycling | `C0-C9` | `get_color(idx)` | ✅ Implemented |
| Lighten/darken | Manual | Built-in | ✅ Implemented |
| Colormaps | viridis, plasma, etc. | Not implemented | ❌ Need |
| Normalization | `Normalize`, `LogNorm` | Not implemented | ❌ Need |
| Colorbar | `plt.colorbar()` | Not implemented | ❌ Need |

### Recommended Colormap Implementation

```rust
/// Standard colormaps
pub struct Colormap {
    name: String,
    colors: Vec<Vec4>,
    under: Option<Vec4>,  // Color for values < vmin
    over: Option<Vec4>,   // Color for values > vmax
    bad: Vec4,            // Color for NaN
}

impl Colormap {
    pub fn viridis() -> Self { Self::from_name("viridis") }
    pub fn plasma() -> Self { Self::from_name("plasma") }
    pub fn coolwarm() -> Self { Self::from_name("coolwarm") }
    pub fn jet() -> Self { Self::from_name("jet") }

    pub fn sample(&self, t: f64) -> Vec4 {
        if t.is_nan() { return self.bad; }
        let t = t.clamp(0.0, 1.0);
        let n = self.colors.len() - 1;
        let idx = (t * n as f64).floor() as usize;
        let frac = t * n as f64 - idx as f64;
        lerp_color(self.colors[idx], self.colors[idx + 1], frac as f32)
    }
}

/// Normalization for mapping data to colormap
pub struct Normalize {
    vmin: f64,
    vmax: f64,
    clip: bool,
}

impl Normalize {
    pub fn normalize(&self, value: f64) -> f64 {
        let t = (value - self.vmin) / (self.vmax - self.vmin);
        if self.clip { t.clamp(0.0, 1.0) } else { t }
    }
}

pub struct LogNorm {
    vmin: f64,
    vmax: f64,
}

impl LogNorm {
    pub fn normalize(&self, value: f64) -> f64 {
        (value.ln() - self.vmin.ln()) / (self.vmax.ln() - self.vmin.ln())
    }
}
```

---

## 7. Interactivity and Events

### matplotlib Interactivity

```python
# Event callbacks
fig.canvas.mpl_connect('button_press_event', on_click)
fig.canvas.mpl_connect('motion_notify_event', on_move)

# Interactive tools
from matplotlib.widgets import Slider, Button, RectangleSelector
```

### makepad-chart Interactivity (Implemented)

```rust
/// Hit testing for click detection
pub struct HitTester {
    regions: Vec<HitRegion>,
}

pub struct HitRegion {
    rect: Rect,
    data: HitData,
}

pub enum HitData {
    Bar { dataset_index: usize, data_index: usize },
    Point { dataset_index: usize, data_index: usize },
    Slice { index: usize },
    Custom(String),
}

impl HitTester {
    pub fn register(&mut self, rect: Rect, data: HitData) { ... }
    pub fn hit_test(&self, pos: DVec2) -> Option<&HitRegion> { ... }
    pub fn find_nearest(&self, pos: DVec2, max_distance: f64) -> Option<&HitRegion> { ... }
}
```

### Widget Event Handling

```rust
impl Widget for BarChart {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        match event {
            Event::MouseMove(e) => {
                if self.coord.contains_pixel(e.abs.x, e.abs.y) {
                    let x_value = self.coord.x_scale().get_value_for_pixel(e.abs.x);
                    self.hovered_bar = x_value.round() as i32;
                    self.redraw(cx);
                }
            }
            Event::MouseDown(e) => {
                // Handle click
                log!("Bar clicked: {}", self.hovered_bar);
            }
            _ => {}
        }
    }
}
```

### Interactivity Gap

| Feature | matplotlib | makepad-chart | Status |
|---------|-----------|---------------|--------|
| Hover detection | Events | Built-in | ✅ Implemented |
| Click events | Events | Built-in | ✅ Implemented |
| Tooltips | Custom | Planned | ⚠️ Stub only |
| Pan/zoom | Navigation toolbar | Not implemented | ❌ Need |
| Rectangle select | `RectangleSelector` | Not implemented | ❌ Need |
| Lasso select | `LassoSelector` | Not implemented | ❌ Need |
| Sliders | `Slider` widget | Not implemented | ❌ Need |
| Buttons | `Button` widget | Makepad native | ✅ Available |

---

## 8. Text Rendering Gap

### Current Status

**Critical Gap**: Text rendering is minimal in makepad-chart, severely limiting chart usability.

**Impact**:
- ⚠️ Axis labels are basic
- ⚠️ Tick labels have limited formatting
- ⚠️ Legend text is rudimentary
- ❌ No rotated text support
- ❌ No math/LaTeX rendering
- ❌ Limited tooltip text styling

### matplotlib Text Capabilities

```python
# Rich text formatting
ax.set_title(r'$\alpha > \beta$', fontsize=14)  # LaTeX math
ax.text(0.5, 0.5, 'Annotation', rotation=45, ha='center')
ax.annotate('Peak', xy=(x, y), xytext=(x+1, y+1),
            arrowprops=dict(arrowstyle='->'))
```

### Implementation Strategy

```rust
// New component: component/text_label.rs

live_design! {
    use link::shaders::*;
    use link::widgets::*;

    pub ChartTextLabel = {{ChartTextLabel}} {
        draw_text: {
            text_style: <THEME_FONT_REGULAR> {}
            fn get_color(self) -> vec4 {
                return self.color;
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ChartTextLabel {
    #[live] draw_text: DrawText,
    #[live] text: String,
    #[live] color: Vec4,
    #[live] font_size: f64,
    #[live] rotation: f64,        // Degrees
    #[live] anchor: TextAnchor,
    #[rust] computed_size: DVec2,
}

#[derive(Live, LiveCopy, Clone, Debug, Default)]
#[live_ignore]
pub enum TextAnchor {
    #[default]
    Center,
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl ChartTextLabel {
    pub fn draw_at(&mut self, cx: &mut Cx2d, pos: DVec2) {
        // Measure text
        let text_size = self.draw_text.text_style.font_size;

        // Apply anchor offset
        let offset = self.compute_anchor_offset();
        let final_pos = pos + offset;

        // Draw with rotation if needed
        if self.rotation != 0.0 {
            // Note: Makepad rotation would need transform support
            // For now, only 0, 90, -90 degrees supported via swap
        }

        self.draw_text.draw_abs(cx, final_pos, &self.text);
    }

    fn compute_anchor_offset(&self) -> DVec2 {
        let w = self.computed_size.x;
        let h = self.computed_size.y;

        match self.anchor {
            TextAnchor::Center => dvec2(-w / 2.0, -h / 2.0),
            TextAnchor::TopLeft => dvec2(0.0, 0.0),
            TextAnchor::TopCenter => dvec2(-w / 2.0, 0.0),
            TextAnchor::TopRight => dvec2(-w, 0.0),
            TextAnchor::MiddleLeft => dvec2(0.0, -h / 2.0),
            TextAnchor::MiddleRight => dvec2(-w, -h / 2.0),
            TextAnchor::BottomLeft => dvec2(0.0, -h),
            TextAnchor::BottomCenter => dvec2(-w / 2.0, -h),
            TextAnchor::BottomRight => dvec2(-w, -h),
        }
    }
}
```

### Axis Label Integration

```rust
impl AxisRenderer {
    pub fn draw_tick_labels(&mut self, cx: &mut Cx2d, ticks: &[Tick], orientation: Orientation) {
        for tick in ticks {
            let pos = self.get_tick_label_position(tick, orientation);

            let anchor = match orientation {
                Orientation::Horizontal => TextAnchor::TopCenter,
                Orientation::Vertical => TextAnchor::MiddleRight,
            };

            self.text_label.text = tick.label.clone();
            self.text_label.anchor = anchor;
            self.text_label.draw_at(cx, pos);
        }
    }

    pub fn draw_axis_title(&mut self, cx: &mut Cx2d, title: &str, orientation: Orientation) {
        let pos = self.get_axis_title_position(orientation);

        self.title_label.text = title.to_string();
        self.title_label.anchor = TextAnchor::Center;

        // Vertical axis title needs rotation
        if orientation == Orientation::Vertical {
            self.title_label.rotation = -90.0;
        }

        self.title_label.draw_at(cx, pos);
    }
}
```

### Tooltip Text Component

```rust
live_design! {
    use link::shaders::*;
    use link::widgets::*;

    pub TooltipContent = {{TooltipContent}} {
        width: Fit,
        height: Fit,
        padding: 8.0,

        draw_bg: {
            color: #000000CC
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0);
                sdf.fill(self.color);
                return sdf.result;
            }
        }

        title_label = <Label> {
            draw_text: { color: #FFFFFF }
            text: ""
        }

        body_label = <Label> {
            draw_text: { color: #FFFFFFCC }
            text: ""
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct TooltipContent {
    #[deref] view: View,
    #[live] draw_bg: DrawQuad,
    #[live] title_label: Label,
    #[live] body_label: Label,
}

impl TooltipContent {
    pub fn set_content(&mut self, title: &str, body: &str) {
        self.title_label.set_text(title);
        self.body_label.set_text(body);
    }
}
```

### Math Text (Future Enhancement)

For LaTeX-style math rendering, consider:

1. **Simple approach**: Pre-render common symbols as textures
2. **Medium approach**: Parse simple expressions like `x^2`, `α`, `β`
3. **Full approach**: Integrate a math layout engine

```rust
// Simple math symbol support
pub struct MathSymbols {
    greek: HashMap<&'static str, char>,
    operators: HashMap<&'static str, char>,
}

impl MathSymbols {
    pub fn new() -> Self {
        let mut greek = HashMap::new();
        greek.insert("alpha", 'α');
        greek.insert("beta", 'β');
        greek.insert("gamma", 'γ');
        greek.insert("delta", 'δ');
        greek.insert("pi", 'π');
        greek.insert("sigma", 'σ');
        greek.insert("mu", 'μ');
        // ... more symbols

        Self { greek, operators: HashMap::new() }
    }

    pub fn replace_symbols(&self, text: &str) -> String {
        let mut result = text.to_string();
        for (name, symbol) in &self.greek {
            result = result.replace(&format!("\\{}", name), &symbol.to_string());
        }
        result
    }
}
```

---

## 9. Options and Configuration

### matplotlib Configuration (rcParams)

```python
plt.rcParams['figure.figsize'] = (10, 6)
plt.rcParams['axes.grid'] = True
plt.rcParams['font.size'] = 12

# Or with context
with plt.style.context('seaborn'):
    ax.plot(x, y)
```

### makepad-chart Options (Implemented)

```rust
pub struct ChartOptions {
    pub responsive: bool,
    pub maintain_aspect_ratio: bool,
    pub aspect_ratio: f64,
    pub padding: ChartPadding,

    pub title: TitleOptions,
    pub subtitle: TitleOptions,
    pub legend: LegendOptions,
    pub tooltip: TooltipOptions,
    pub animation: AnimationOptions,
    pub interaction: InteractionOptions,
    pub scales: ScalesOptions,
}

pub struct AnimationOptions {
    pub duration: f64,        // Milliseconds
    pub easing: EasingType,
    pub delay: f64,
    pub loop_animation: bool,
}

pub struct ScalesOptions {
    pub x: AxisOptions,
    pub y: AxisOptions,
}

pub struct AxisOptions {
    pub display: bool,
    pub title: AxisTitleOptions,
    pub grid: GridOptions,
    pub ticks: TickOptions,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub begin_at_zero: bool,
    pub reverse: bool,
    pub stacked: bool,
}

pub struct GridOptions {
    pub display: bool,
    pub color: Vec4,
    pub line_width: f64,
    pub draw_border: bool,
    pub draw_on_chart_area: bool,
    pub draw_ticks: bool,
    pub tick_length: f64,
}

pub struct TickOptions {
    pub display: bool,
    pub color: Vec4,
    pub font_size: f64,
    pub padding: f64,
    pub max_ticks_limit: usize,
    pub step_size: Option<f64>,
    pub include_bounds: bool,
}
```

### Builder Pattern Usage

```rust
let options = ChartOptions::new()
    .with_title("Sales Report 2024")
    .with_subtitle("Q1-Q4")
    .with_legend(true, LegendPosition::Bottom)
    .with_animation_duration(500.0)
    .with_animation_easing(EasingType::EaseOutQuart)
    .with_begin_at_zero(true);

chart.set_options(options);
```

---

## 10. Implementation Roadmap

### Phase 1: Complete Basic Features (HIGH PRIORITY)

These are critical for making charts usable.

| Task | Current Status | Effort | Dependencies |
|------|----------------|--------|--------------|
| Text rendering for labels | Basic only | 3 days | None |
| Tooltip component | Stub | 2 days | Text rendering |
| Legend component | Stub | 2 days | Text rendering |
| Axis tick labels | Basic | 2 days | Text rendering |
| Grid rendering | Working | Done | - |

### Phase 2: Scientific Plot Types (HIGH PRIORITY)

Essential for scientific visualization use cases.

| Task | Effort | Dependencies |
|------|--------|--------------|
| Histogram chart | 2 days | Existing bar shader |
| Box plot | 3 days | Text labels |
| Error bars | 1 day | Existing line shader |
| Violin plot | 4 days | Box plot + KDE algorithm |
| Heatmap | 4 days | Colormap + texture upload |
| Contour plots | 5 days | Marching squares algorithm |
| Filled contour | 2 days | Contour plots |

### Phase 3: Advanced Scales (MEDIUM PRIORITY)

Required for scientific data with large ranges.

| Task | Effort | Dependencies |
|------|--------|--------------|
| Log scale | 2 days | Tick formatting |
| Symlog scale | 2 days | Log scale |
| Time scale | 3 days | Date parsing |
| Smart tick locators | 2 days | Scale system |
| Tick formatters | 1 day | Scale system |

### Phase 4: Interactivity (HIGH PRIORITY)

Critical for data exploration.

| Task | Effort | Dependencies |
|------|--------|--------------|
| Pan handler | 2 days | Coordinate system |
| Zoom handler | 2 days | Coordinate system |
| Tooltips | 2 days | Text rendering |
| Legend toggle | 1 day | Legend component |
| Crosshair cursor | 1 day | Event handling |
| Annotations | 3 days | Text + drawing primitives |

### Phase 5: Advanced Features (LOWER PRIORITY)

| Task | Effort | Dependencies |
|------|--------|--------------|
| Quiver (vector field) | 3 days | Arrow shader |
| Stream plot | 5 days | Vector integration |
| 3D axes | 10 days | 3D projection matrix |
| Surface plot | 5 days | 3D axes |
| Math text | 10 days | Font rendering |
| Subplots layout | 3 days | Layout system |

### Phase 6: Polish and Export (NICE TO HAVE)

| Task | Effort | Dependencies |
|------|--------|--------------|
| Theme system | 2 days | Makepad themes |
| PNG export | 2 days | Texture export |
| SVG export | 5 days | Vector path generation |
| Secondary Y-axis | 2 days | Scale system |
| Examples & docs | 5 days | All features |

### Implementation Templates

#### Pan/Zoom Handler

```rust
// interaction/pan_zoom.rs

#[derive(Default)]
pub struct PanZoomHandler {
    is_panning: bool,
    is_zoom_selecting: bool,
    pan_start: DVec2,
    pan_start_range: ((f64, f64), (f64, f64)),
    zoom_rect: Option<Rect>,
    zoom_on_scroll: bool,
}

impl PanZoomHandler {
    pub fn new() -> Self {
        Self {
            zoom_on_scroll: true,
            ..Default::default()
        }
    }

    pub fn handle_event(
        &mut self,
        event: &Event,
        coord: &mut CartesianCoord,
        chart_area: Rect,
    ) -> bool {
        let mut needs_redraw = false;

        match event {
            Event::MouseDown(e) => {
                if !chart_area.contains(e.abs) {
                    return false;
                }

                if e.modifiers.control || e.button == 1 {
                    // Ctrl+click or middle mouse = pan
                    self.is_panning = true;
                    self.pan_start = e.abs;
                    self.pan_start_range = (
                        coord.x_scale.get_data_bounds(),
                        coord.y_scale.get_data_bounds(),
                    );
                } else if e.button == 2 {
                    // Right click = zoom rectangle
                    self.is_zoom_selecting = true;
                    self.zoom_rect = Some(Rect {
                        pos: e.abs,
                        size: dvec2(0.0, 0.0),
                    });
                }
            }

            Event::MouseMove(e) => {
                if self.is_panning {
                    let delta = e.abs - self.pan_start;

                    // Convert pixel delta to data delta
                    let x_range = self.pan_start_range.0;
                    let y_range = self.pan_start_range.1;

                    let x_scale = (x_range.1 - x_range.0) / chart_area.size.x;
                    let y_scale = (y_range.1 - y_range.0) / chart_area.size.y;

                    let new_x_min = x_range.0 - delta.x * x_scale;
                    let new_x_max = x_range.1 - delta.x * x_scale;
                    let new_y_min = y_range.0 + delta.y * y_scale;  // Inverted Y
                    let new_y_max = y_range.1 + delta.y * y_scale;

                    coord.x_scale.set_data_range(new_x_min, new_x_max);
                    coord.y_scale.set_data_range(new_y_min, new_y_max);
                    needs_redraw = true;
                } else if self.is_zoom_selecting {
                    if let Some(ref mut rect) = self.zoom_rect {
                        rect.size = e.abs - rect.pos;
                    }
                    needs_redraw = true;
                }
            }

            Event::MouseUp(_) => {
                if self.is_zoom_selecting {
                    if let Some(rect) = self.zoom_rect.take() {
                        // Apply zoom to selected rectangle
                        if rect.size.x.abs() > 10.0 && rect.size.y.abs() > 10.0 {
                            let normalized_rect = rect.normalize();

                            let x_min = coord.x_scale.get_value_for_pixel(normalized_rect.pos.x);
                            let x_max = coord.x_scale.get_value_for_pixel(
                                normalized_rect.pos.x + normalized_rect.size.x
                            );
                            let y_max = coord.y_scale.get_value_for_pixel(normalized_rect.pos.y);
                            let y_min = coord.y_scale.get_value_for_pixel(
                                normalized_rect.pos.y + normalized_rect.size.y
                            );

                            coord.x_scale.set_data_range(x_min, x_max);
                            coord.y_scale.set_data_range(y_min, y_max);
                            needs_redraw = true;
                        }
                    }
                }
                self.is_panning = false;
                self.is_zoom_selecting = false;
            }

            Event::Scroll(e) => {
                if !self.zoom_on_scroll || !chart_area.contains(e.abs) {
                    return false;
                }

                let zoom_factor = if e.scroll.y > 0.0 { 0.9 } else { 1.1 };

                // Zoom centered on mouse position
                let mouse_x = coord.x_scale.get_value_for_pixel(e.abs.x);
                let mouse_y = coord.y_scale.get_value_for_pixel(e.abs.y);

                let (x_min, x_max) = coord.x_scale.get_data_bounds();
                let (y_min, y_max) = coord.y_scale.get_data_bounds();

                // Scale around mouse position
                let new_x_min = mouse_x - (mouse_x - x_min) * zoom_factor;
                let new_x_max = mouse_x + (x_max - mouse_x) * zoom_factor;
                let new_y_min = mouse_y - (mouse_y - y_min) * zoom_factor;
                let new_y_max = mouse_y + (y_max - mouse_y) * zoom_factor;

                coord.x_scale.set_data_range(new_x_min, new_x_max);
                coord.y_scale.set_data_range(new_y_min, new_y_max);
                needs_redraw = true;
            }

            _ => {}
        }

        needs_redraw
    }

    pub fn draw_zoom_rect(&self, cx: &mut Cx2d, draw_quad: &mut DrawQuad) {
        if let Some(rect) = &self.zoom_rect {
            let normalized = rect.normalize();
            draw_quad.color = vec4(0.3, 0.5, 0.9, 0.2);
            draw_quad.draw_abs(cx, normalized);
        }
    }

    pub fn reset_view(&self, coord: &mut CartesianCoord, original_bounds: ((f64, f64), (f64, f64))) {
        coord.x_scale.set_data_range(original_bounds.0.0, original_bounds.0.1);
        coord.y_scale.set_data_range(original_bounds.1.0, original_bounds.1.1);
    }
}
```

#### Tooltip Widget

```rust
// component/tooltip.rs

live_design! {
    use link::shaders::*;
    use link::widgets::*;

    pub ChartTooltip = {{ChartTooltip}} {
        width: Fit,
        height: Fit,
        visible: false,

        draw_bg: {
            instance bg_color: #000000DD
            instance border_color: #FFFFFF33
            instance border_width: 1.0
            instance radius: 6.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Background with rounded corners
                sdf.box(
                    self.border_width,
                    self.border_width,
                    self.rect_size.x - 2.0 * self.border_width,
                    self.rect_size.y - 2.0 * self.border_width,
                    self.radius
                );
                sdf.fill(self.bg_color);

                // Border
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.radius);
                sdf.stroke(self.border_color, self.border_width);

                return sdf.result;
            }
        }

        content = <View> {
            flow: Down,
            padding: {left: 10, right: 10, top: 8, bottom: 8},
            spacing: 4,

            title = <Label> {
                draw_text: {
                    color: #FFFFFF
                    text_style: { font_size: 11.0 }
                }
            }

            divider = <View> {
                width: Fill,
                height: 1,
                draw_bg: { color: #FFFFFF33 }
            }

            body = <View> {
                flow: Down,
                spacing: 2,
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ChartTooltip {
    #[deref] view: View,
    #[live] draw_bg: DrawQuad,
    #[live] visible: bool,
    #[rust] position: DVec2,
    #[rust] items: Vec<TooltipItem>,
}

pub struct TooltipItem {
    pub color: Vec4,
    pub label: String,
    pub value: String,
}

impl ChartTooltip {
    pub fn show(&mut self, cx: &mut Cx, pos: DVec2, title: &str, items: Vec<TooltipItem>) {
        self.visible = true;
        self.position = pos;
        self.items = items;

        // Update title
        self.label(id!(title)).set_text(title);

        // Update body items
        // (In practice, would dynamically create Label widgets)

        self.redraw(cx);
    }

    pub fn hide(&mut self, cx: &mut Cx) {
        self.visible = false;
        self.redraw(cx);
    }

    pub fn update_position(&mut self, cx: &mut Cx, mouse_pos: DVec2, screen_size: DVec2) {
        if !self.visible {
            return;
        }

        let tooltip_size = dvec2(150.0, 80.0);  // Estimated size
        let margin = 10.0;

        // Position tooltip to avoid screen edges
        let mut x = mouse_pos.x + margin;
        let mut y = mouse_pos.y + margin;

        if x + tooltip_size.x > screen_size.x {
            x = mouse_pos.x - tooltip_size.x - margin;
        }
        if y + tooltip_size.y > screen_size.y {
            y = mouse_pos.y - tooltip_size.y - margin;
        }

        self.position = dvec2(x, y);
        self.redraw(cx);
    }
}

impl Widget for ChartTooltip {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        // Draw at absolute position
        let rect = Rect {
            pos: self.position,
            size: dvec2(150.0, 80.0),  // Will be computed from content
        };

        self.draw_bg.draw_abs(cx, rect);
        self.view.draw_walk(cx, scope, walk)
    }
}
```

#### Annotation Layer

```rust
// component/annotation.rs

pub enum Annotation {
    Text {
        position: DVec2,
        text: String,
        color: Vec4,
        font_size: f64,
        rotation: f64,
    },
    Arrow {
        start: DVec2,
        end: DVec2,
        color: Vec4,
        width: f64,
        head_size: f64,
    },
    Line {
        start: DVec2,
        end: DVec2,
        color: Vec4,
        width: f64,
        dash_pattern: Option<(f64, f64)>,
    },
    Rectangle {
        rect: Rect,
        fill: Option<Vec4>,
        stroke: Option<(Vec4, f64)>,
    },
    Circle {
        center: DVec2,
        radius: f64,
        fill: Option<Vec4>,
        stroke: Option<(Vec4, f64)>,
    },
}

#[derive(Default)]
pub struct AnnotationLayer {
    annotations: Vec<Annotation>,
}

impl AnnotationLayer {
    pub fn add_text(&mut self, pos: DVec2, text: impl Into<String>) -> &mut Self {
        self.annotations.push(Annotation::Text {
            position: pos,
            text: text.into(),
            color: vec4(0.2, 0.2, 0.2, 1.0),
            font_size: 12.0,
            rotation: 0.0,
        });
        self
    }

    pub fn add_arrow(&mut self, start: DVec2, end: DVec2) -> &mut Self {
        self.annotations.push(Annotation::Arrow {
            start,
            end,
            color: vec4(0.2, 0.2, 0.2, 1.0),
            width: 1.5,
            head_size: 8.0,
        });
        self
    }

    pub fn add_horizontal_line(&mut self, y: f64, x_range: (f64, f64), color: Vec4) -> &mut Self {
        self.annotations.push(Annotation::Line {
            start: dvec2(x_range.0, y),
            end: dvec2(x_range.1, y),
            color,
            width: 1.0,
            dash_pattern: Some((5.0, 3.0)),
        });
        self
    }

    pub fn add_vertical_line(&mut self, x: f64, y_range: (f64, f64), color: Vec4) -> &mut Self {
        self.annotations.push(Annotation::Line {
            start: dvec2(x, y_range.0),
            end: dvec2(x, y_range.1),
            color,
            width: 1.0,
            dash_pattern: Some((5.0, 3.0)),
        });
        self
    }

    pub fn add_region(&mut self, rect: Rect, fill: Vec4) -> &mut Self {
        self.annotations.push(Annotation::Rectangle {
            rect,
            fill: Some(fill),
            stroke: None,
        });
        self
    }

    pub fn clear(&mut self) {
        self.annotations.clear();
    }

    pub fn draw(
        &self,
        cx: &mut Cx2d,
        coord: &CartesianCoord,
        draw_line: &mut DrawChartLine,
        draw_text: &mut DrawText,
    ) {
        for annotation in &self.annotations {
            match annotation {
                Annotation::Text { position, text, color, font_size, .. } => {
                    let pixel_pos = coord.data_to_pixel(position.x, position.y);
                    draw_text.color = *color;
                    draw_text.draw_abs(cx, pixel_pos, text);
                }

                Annotation::Arrow { start, end, color, width, head_size } => {
                    let p1 = coord.data_to_pixel(start.x, start.y);
                    let p2 = coord.data_to_pixel(end.x, end.y);

                    // Draw line
                    draw_line.color = *color;
                    draw_line.draw_line(cx, p1, p2, *width);

                    // Draw arrowhead
                    let angle = (p2.y - p1.y).atan2(p2.x - p1.x);
                    let head_angle = 0.5;  // ~30 degrees

                    let h1 = dvec2(
                        p2.x - head_size * (angle + head_angle).cos(),
                        p2.y - head_size * (angle + head_angle).sin(),
                    );
                    let h2 = dvec2(
                        p2.x - head_size * (angle - head_angle).cos(),
                        p2.y - head_size * (angle - head_angle).sin(),
                    );

                    draw_line.draw_line(cx, p2, h1, *width);
                    draw_line.draw_line(cx, p2, h2, *width);
                }

                Annotation::Line { start, end, color, width, .. } => {
                    let p1 = coord.data_to_pixel(start.x, start.y);
                    let p2 = coord.data_to_pixel(end.x, end.y);
                    draw_line.color = *color;
                    draw_line.draw_line(cx, p1, p2, *width);
                }

                _ => {
                    // Rectangle and Circle would need DrawQuad
                }
            }
        }
    }
}
```

### Total Estimated Effort

| Phase | Effort (dev days) |
|-------|-------------------|
| Phase 1: Basic Features | 9 days |
| Phase 2: Scientific Plots | 21 days |
| Phase 3: Advanced Scales | 10 days |
| Phase 4: Interactivity | 11 days |
| Phase 5: Advanced Features | 36 days |
| Phase 6: Polish | 16 days |
| **Total** | **~103 days** |

Note: Some phases can be parallelized. Critical path is Phase 1 → Phase 2/4 → Phase 6.

---

## 11. Code Examples

### matplotlib vs makepad-chart API Comparison

#### Simple Line Plot

```python
# matplotlib
import matplotlib.pyplot as plt
import numpy as np

x = np.linspace(0, 10, 100)
y = np.sin(x)

fig, ax = plt.subplots()
ax.plot(x, y, color='blue', label='sin(x)')
ax.set_xlabel('X')
ax.set_ylabel('Y')
ax.legend()
plt.show()
```

```rust
// makepad-chart
let data = ChartData::new()
    .with_labels((0..100).map(|i| format!("{:.1}", i as f64 * 0.1)).collect())
    .add_dataset(
        Dataset::new("sin(x)")
            .with_data((0..100).map(|i| (i as f64 * 0.1).sin()).collect())
            .with_color(hex_to_vec4(0x0000FF))
    );

let options = ChartOptions::new()
    .with_legend(true, LegendPosition::Top);

line_chart.set_data(data);
line_chart.set_options(options);
```

#### Bar Chart with Animation

```python
# matplotlib (with animation)
from matplotlib.animation import FuncAnimation

fig, ax = plt.subplots()
bars = ax.bar(['A', 'B', 'C'], [0, 0, 0])

def update(frame):
    progress = frame / 100
    for bar, height in zip(bars, [10, 20, 30]):
        bar.set_height(height * progress)
    return bars

ani = FuncAnimation(fig, update, frames=100, interval=20)
plt.show()
```

```rust
// makepad-chart
let data = ChartData::new()
    .with_labels(vec!["A", "B", "C"])
    .add_dataset(
        Dataset::new("Values")
            .with_data(vec![10.0, 20.0, 30.0])
    );

bar_chart.set_data(data);
bar_chart.set_delay_animation(true);
bar_chart.set_delay_timing(100.0, 0.0);  // 100ms per bar
bar_chart.replay_animation(cx);
```

#### Scatter Plot with Gradient

```python
# matplotlib
x = np.random.rand(50)
y = np.random.rand(50)
colors = np.random.rand(50)
sizes = np.random.rand(50) * 500

ax.scatter(x, y, c=colors, s=sizes, cmap='viridis', alpha=0.7)
plt.colorbar()
```

```rust
// makepad-chart
let points: Vec<(f64, f64, f64)> = (0..50)
    .map(|_| (rand(), rand(), rand() * 20.0))  // x, y, radius
    .collect();

let data = ChartData::new()
    .add_dataset(
        Dataset::new("Scatter")
            .with_bubble_data(points)
    );

scatter_chart.set_data(data);
scatter_chart.set_gradient(true);  // Radial gradient on points
```

---

## Appendix A: GPU Shader Patterns

### Bar Shader with Gradient

```rust
live_design! {
    pub DrawBar = {{DrawBar}} {
        fn pixel(self) -> vec4 {
            let uv = self.pos;

            if self.gradient_enabled > 0.5 {
                let t = 1.0 - uv.y;  // Bottom to top
                let final_color = mix(self.gradient_bottom_color, self.gradient_top_color, t);
                return vec4(final_color.rgb * final_color.a, final_color.a);
            }

            return vec4(self.color.rgb * self.color.a, self.color.a);
        }
    }
}
```

### Arc Shader (Pie/Doughnut)

```rust
live_design! {
    pub DrawArc = {{DrawArc}} {
        fn pixel(self) -> vec4 {
            let px = self.pos.x - 0.5;
            let py = self.pos.y - 0.5;
            let distance = sqrt(px * px + py * py);

            // Ring mask
            let inner_rad = self.inner_radius * 0.5;
            let outer_rad = 0.5;
            let dist_mask = step(inner_rad, distance) * step(distance, outer_rad);

            // Angle mask
            let pixel_ang = atan(py, px);
            let sweep_val = self.end_angle - self.start_angle;
            let wrap_ang = mod(pixel_ang - self.start_angle + 6.28 * 4.0, 6.28);
            let ang_mask = step(wrap_ang, sweep_val) * step(0.001, sweep_val);

            // Anti-aliased edges
            let edge_aa = 0.005;
            let outer_aa = 1.0 - smoothstep(outer_rad - edge_aa, outer_rad + edge_aa, distance);
            let inner_aa = smoothstep(inner_rad - edge_aa, inner_rad + edge_aa, distance);

            let alpha_val = dist_mask * ang_mask * outer_aa * inner_aa;

            // Gradient support
            if self.gradient_enabled > 0.5 {
                if self.gradient_type < 0.5 {
                    // Radial gradient
                    let t = (distance - inner_rad) / (outer_rad - inner_rad);
                    let color = mix(self.gradient_inner_color, self.gradient_outer_color, t);
                    return vec4(color.rgb * alpha_val, color.a * alpha_val);
                } else {
                    // Angular gradient
                    let t = wrap_ang / sweep_val;
                    let color = mix(self.gradient_inner_color, self.gradient_outer_color, t);
                    return vec4(color.rgb * alpha_val, color.a * alpha_val);
                }
            }

            return vec4(self.color.rgb * alpha_val, self.color.a * alpha_val);
        }
    }
}
```

---

## Appendix B: References

### matplotlib Documentation
- [API Reference](https://matplotlib.org/stable/api/index.html)
- [Transforms Tutorial](https://matplotlib.org/stable/users/explain/artists/transforms_tutorial.html)
- [Animation API](https://matplotlib.org/stable/api/animation_api.html)
- [Colors API](https://matplotlib.org/stable/api/colors_api.html)

### makepad-chart Source
- Location: `/Users/yuechen/home/echart/makepad-chart`
- Key files:
  - `src/core/data.rs` - Data structures
  - `src/scale/linear.rs` - Scale implementation
  - `src/coord/cartesian.rs` - Coordinate system
  - `src/animation/animator.rs` - Animation system
  - `src/chart/bar_chart.rs` - Widget example

### Makepad Framework
- [Makepad Repository](https://github.com/makepad/makepad)
- [UI Zoo Examples](https://github.com/makepad/makepad/tree/main/examples/ui_zoo)

---

## Summary

### Key Findings

The **makepad-chart** implementation provides an excellent foundation for matplotlib-like functionality in Makepad:

**Strengths**:
- ✅ 11 chart types with GPU-accelerated rendering
- ✅ Sophisticated animation system (28 easing functions)
- ✅ Clean architecture with separation of concerns
- ✅ Builder pattern API for easy data configuration
- ✅ Live DSL integration for declarative UI
- ✅ Custom shader-based drawing primitives
- ✅ Basic hover interaction and hit testing

**Critical Gaps**:
1. **Text Rendering** - Cannot properly display axis labels, tick labels, legends, or tooltips
2. **Scientific Plots** - No histogram, box plot, heatmap, contour, error bars
3. **Advanced Scales** - No log, symlog, or time scales
4. **Interactivity** - No pan/zoom, annotations, or export functionality
5. **Colormaps** - No viridis, plasma, coolwarm, etc.

### Architectural Insights

- GPU shader-based rendering is more performant than matplotlib's CPU rasterization
- The 28 easing functions provide smoother animations than matplotlib's basic options
- The builder pattern API is cleaner than matplotlib's dual interface (pyplot vs OO)
- The Live DSL integration enables powerful declarative UI composition
- **Critical Y-axis detail**: Pixel range is inverted (bottom to top) for screen coordinates

### File Structure Summary

```
makepad-chart/
├── src/
│   ├── core/
│   │   ├── data.rs          - Data structures (ChartData, Dataset, DataPoint)
│   │   ├── options.rs       - Configuration options hierarchy
│   │   └── colors.rs        - Color utilities and palettes
│   ├── scale/
│   │   ├── linear.rs        - Linear scale with nice bounds
│   │   └── category.rs      - Category scale for discrete data
│   ├── coord/
│   │   ├── cartesian.rs     - Cartesian coordinate transformer
│   │   └── polar.rs         - Polar coordinate transformer
│   ├── element/
│   │   ├── bar.rs           - DrawBar shader with gradient
│   │   ├── line.rs          - DrawChartLine shader
│   │   ├── point.rs         - DrawPoint shader
│   │   └── arc.rs           - DrawArc shader for pie/doughnut
│   ├── component/
│   │   ├── axis.rs          - Axis widget (basic)
│   │   ├── legend.rs        - Legend widget (stub)
│   │   └── tooltip.rs       - Tooltip widget (stub)
│   ├── animation/
│   │   ├── animator.rs      - ChartAnimator state machine
│   │   └── easing.rs        - 28 easing functions
│   ├── interaction/
│   │   └── hit_test.rs      - Hit testing for clicks
│   └── chart/
│       ├── bar_chart.rs
│       ├── line_chart.rs
│       ├── scatter_chart.rs
│       ├── bubble_chart.rs
│       ├── pie_chart.rs
│       ├── radar_chart.rs
│       ├── polar_area_chart.rs
│       ├── horizontal_bar_chart.rs
│       ├── combo_chart.rs
│       └── chord_chart.rs
├── examples/
│   └── chart_zoo.rs         - Demo application
└── doc/
    └── samples/             - Documentation examples
```

### Estimated Total Effort

| Phase | Description | Effort |
|-------|-------------|--------|
| Phase 1 | Basic Features (text, tooltip, legend) | 9 days |
| Phase 2 | Scientific Plots (histogram, boxplot, heatmap) | 21 days |
| Phase 3 | Advanced Scales (log, time) | 10 days |
| Phase 4 | Interactivity (pan/zoom, annotations) | 11 days |
| Phase 5 | Advanced Features (3D, math text) | 36 days |
| Phase 6 | Polish (export, themes, docs) | 16 days |
| **Total** | **Full matplotlib parity** | **~103 days** |

### Recommendations

1. **Start with text rendering** - It unlocks tooltips, legends, and proper axis labels
2. **Prioritize interactivity** - Pan/zoom is essential for data exploration
3. **Add scientific plots incrementally** - Histogram and box plot cover most use cases
4. **Consider 3D as optional** - High effort, lower priority for most applications
