# Makepad Matplotlib Implementation - Detailed Development Plan

**Created**: January 2025
**Based on**: makepad-matplot-gap-claude.md analysis
**Total Estimated Effort**: ~103 developer days

---

## Priority Legend

| Priority | Description |
|----------|-------------|
| 🔴 P0 | Critical - Blocks other work |
| 🟠 P1 | High - Core functionality |
| 🟡 P2 | Medium - Important features |
| 🟢 P3 | Low - Nice to have |

---

## Phase 1: Text Rendering Foundation (P0)

**Why First**: Text rendering is a blocker for tooltips, legends, axis labels, and tick labels. Nothing else is truly usable without it.

### 1.1 Basic Text Label Component

#### Step 1.1.1: Create ChartTextLabel struct
**File**: `src/component/text_label.rs`
**Effort**: 0.5 day

```rust
#[derive(Live, LiveHook)]
pub struct ChartTextLabel {
    #[live] draw_text: DrawText,
    #[live] text: String,
    #[live] color: Vec4,
    #[live] font_size: f64,
}
```

**Test**:
- [ ] Create a simple widget that displays "Hello Chart" at position (100, 100)
- [ ] Verify text appears in chart_zoo example
- [ ] Change color to red, verify color changes

#### Step 1.1.2: Add TextAnchor enum
**File**: `src/component/text_label.rs`
**Effort**: 0.25 day

```rust
pub enum TextAnchor {
    TopLeft, TopCenter, TopRight,
    MiddleLeft, Center, MiddleRight,
    BottomLeft, BottomCenter, BottomRight,
}
```

**Test**:
- [ ] Draw text with `TopLeft` anchor at (200, 200) - text should extend right and down
- [ ] Draw text with `BottomRight` anchor at (200, 200) - text should extend left and up
- [ ] Draw text with `Center` anchor at (200, 200) - text should be centered on point

#### Step 1.1.3: Implement draw_at method
**File**: `src/component/text_label.rs`
**Effort**: 0.5 day

```rust
impl ChartTextLabel {
    pub fn draw_at(&mut self, cx: &mut Cx2d, pos: DVec2) { ... }
}
```

**Test**:
- [ ] Draw "Test" at 5 different positions, verify all appear correctly
- [ ] Measure text width matches expected (within 5px tolerance)

#### Step 1.1.4: Add text measurement
**File**: `src/component/text_label.rs`
**Effort**: 0.25 day

```rust
pub fn measure_text(&self, cx: &Cx) -> DVec2 { ... }
```

**Test**:
- [ ] Measure "Hello" returns non-zero width and height
- [ ] Longer text "Hello World" returns larger width than "Hello"
- [ ] Font size 20 returns larger dimensions than font size 10

---

### 1.2 Tick Labels

#### Step 1.2.1: Create TickLabelRenderer
**File**: `src/component/axis.rs`
**Effort**: 0.5 day

```rust
pub struct TickLabelRenderer {
    text_label: ChartTextLabel,
    padding: f64,
}
```

**Test**:
- [ ] Instantiate TickLabelRenderer with default settings
- [ ] Verify it compiles and can be created

#### Step 1.2.2: Implement draw_x_tick_labels
**File**: `src/component/axis.rs`
**Effort**: 0.5 day

```rust
pub fn draw_x_tick_labels(&mut self, cx: &mut Cx2d, ticks: &[Tick], y_position: f64) { ... }
```

**Test**:
- [ ] Draw 5 tick labels at x positions [100, 200, 300, 400, 500]
- [ ] Verify labels are centered below their tick positions
- [ ] Verify labels don't overlap (minimum spacing check)

#### Step 1.2.3: Implement draw_y_tick_labels
**File**: `src/component/axis.rs`
**Effort**: 0.5 day

```rust
pub fn draw_y_tick_labels(&mut self, cx: &mut Cx2d, ticks: &[Tick], x_position: f64) { ... }
```

**Test**:
- [ ] Draw 5 tick labels at y positions [100, 150, 200, 250, 300]
- [ ] Verify labels are right-aligned to the left of their tick positions
- [ ] Verify vertical spacing is consistent

#### Step 1.2.4: Integrate with BarChart
**File**: `src/chart/bar_chart.rs`
**Effort**: 0.5 day

**Test**:
- [ ] BarChart displays X-axis category labels ("Jan", "Feb", "Mar")
- [ ] BarChart displays Y-axis value labels (0, 20, 40, 60, 80, 100)
- [ ] Labels update when data changes

---

### 1.3 Axis Titles

#### Step 1.3.1: Add axis title to AxisOptions
**File**: `src/core/options.rs`
**Effort**: 0.25 day

```rust
pub struct AxisTitleOptions {
    pub display: bool,
    pub text: String,
    pub color: Vec4,
    pub font_size: f64,
    pub padding: f64,
}
```

**Test**:
- [ ] Create AxisTitleOptions with text "Revenue ($)"
- [ ] Verify all fields are accessible

#### Step 1.3.2: Implement draw_x_axis_title
**File**: `src/component/axis.rs`
**Effort**: 0.5 day

**Test**:
- [ ] X-axis title "Month" appears centered below tick labels
- [ ] Title respects padding setting
- [ ] Title color matches option

#### Step 1.3.3: Implement draw_y_axis_title
**File**: `src/component/axis.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Y-axis title "Value" appears rotated 90° to the left of tick labels
- [ ] Title is vertically centered
- [ ] Title respects padding setting

---

### 1.4 Chart Title

#### Step 1.4.1: Implement draw_chart_title
**File**: `src/component/title.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Chart title "Sales Report 2024" appears centered above chart
- [ ] Title font size is larger than axis labels (default 16px vs 11px)
- [ ] Subtitle appears below title in smaller font

#### Step 1.4.2: Integrate title into all chart types
**File**: Multiple chart files
**Effort**: 0.5 day

**Test**:
- [ ] BarChart shows title
- [ ] LineChart shows title
- [ ] PieChart shows title
- [ ] All 11 chart types show title when options.title.display = true

---

**Phase 1 Checkpoint**:
- [ ] All charts can display tick labels
- [ ] All charts can display axis titles
- [ ] All charts can display chart title and subtitle
- [ ] Text is readable and properly positioned

---

## Phase 2: Tooltip Component (P0)

**Why Second**: Tooltips are essential for data exploration and require text rendering.

### 2.1 Basic Tooltip

#### Step 2.1.1: Create Tooltip background shader
**File**: `src/component/tooltip.rs`
**Effort**: 0.5 day

```rust
live_design! {
    DrawTooltipBg = {{DrawTooltipBg}} {
        fn pixel(self) -> vec4 {
            // Rounded rectangle with shadow
        }
    }
}
```

**Test**:
- [ ] Draw a 150x80 tooltip background at (100, 100)
- [ ] Verify rounded corners (radius 6px)
- [ ] Verify semi-transparent black background

#### Step 2.1.2: Create ChartTooltip widget
**File**: `src/component/tooltip.rs`
**Effort**: 0.5 day

```rust
#[derive(Live, LiveHook, Widget)]
pub struct ChartTooltip {
    #[live] visible: bool,
    #[live] position: DVec2,
    #[live] draw_bg: DrawTooltipBg,
    // ...
}
```

**Test**:
- [ ] Create tooltip, verify it's hidden by default
- [ ] Set visible = true, verify tooltip appears
- [ ] Change position, verify tooltip moves

#### Step 2.1.3: Implement show/hide methods
**File**: `src/component/tooltip.rs`
**Effort**: 0.25 day

```rust
impl ChartTooltip {
    pub fn show(&mut self, cx: &mut Cx, pos: DVec2, content: TooltipContent) { ... }
    pub fn hide(&mut self, cx: &mut Cx) { ... }
}
```

**Test**:
- [ ] Call show() - tooltip becomes visible
- [ ] Call hide() - tooltip disappears
- [ ] Call show() again - tooltip reappears at new position

#### Step 2.1.4: Implement screen edge detection
**File**: `src/component/tooltip.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Mouse near right edge - tooltip appears to the left of cursor
- [ ] Mouse near bottom edge - tooltip appears above cursor
- [ ] Mouse near corner - tooltip adjusts both axes
- [ ] Mouse in center - tooltip appears bottom-right of cursor

---

### 2.2 Tooltip Content

#### Step 2.2.1: Create TooltipItem struct
**File**: `src/component/tooltip.rs`
**Effort**: 0.25 day

```rust
pub struct TooltipItem {
    pub color: Vec4,
    pub label: String,
    pub value: String,
}
```

**Test**:
- [ ] Create TooltipItem with color, label "Sales", value "1,234"
- [ ] Verify all fields accessible

#### Step 2.2.2: Implement multi-item tooltip rendering
**File**: `src/component/tooltip.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Tooltip with 1 item displays correctly
- [ ] Tooltip with 3 items displays all items vertically
- [ ] Each item shows color box + label + value
- [ ] Tooltip height adjusts to content

#### Step 2.2.3: Add title to tooltip
**File**: `src/component/tooltip.rs`
**Effort**: 0.25 day

**Test**:
- [ ] Tooltip shows title "January" at top
- [ ] Divider line appears below title
- [ ] Items appear below divider

---

### 2.3 Chart Integration

#### Step 2.3.1: Add tooltip to BarChart
**File**: `src/chart/bar_chart.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Hover over bar - tooltip appears
- [ ] Tooltip shows category label as title
- [ ] Tooltip shows dataset label and value
- [ ] Move mouse away - tooltip disappears

#### Step 2.3.2: Add tooltip to LineChart
**File**: `src/chart/line_chart.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Hover near point - tooltip appears
- [ ] Tooltip shows X value as title
- [ ] Tooltip shows Y value for each dataset
- [ ] Nearest point is highlighted

#### Step 2.3.3: Add tooltip to PieChart
**File**: `src/chart/pie_chart.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Hover over slice - tooltip appears
- [ ] Tooltip shows slice label and value
- [ ] Tooltip shows percentage
- [ ] Hovered slice is slightly offset

#### Step 2.3.4: Add tooltip to remaining chart types
**File**: Multiple files
**Effort**: 1 day

**Test**:
- [ ] ScatterChart tooltip shows (x, y) values
- [ ] BubbleChart tooltip shows (x, y, r) values
- [ ] RadarChart tooltip shows axis and value
- [ ] All 11 chart types have working tooltips

---

**Phase 2 Checkpoint**:
- [ ] All charts show tooltips on hover
- [ ] Tooltips display correct data
- [ ] Tooltips position correctly at screen edges
- [ ] Tooltips disappear when mouse leaves

---

## Phase 3: Legend Component (P1)

### 3.1 Basic Legend

#### Step 3.1.1: Create LegendItem struct
**File**: `src/component/legend.rs`
**Effort**: 0.25 day

```rust
pub struct LegendItem {
    pub color: Vec4,
    pub label: String,
    pub dataset_index: usize,
    pub hidden: bool,
}
```

**Test**:
- [ ] Create LegendItem, verify fields accessible
- [ ] Create Vec<LegendItem> from ChartData

#### Step 3.1.2: Implement horizontal legend layout
**File**: `src/component/legend.rs`
**Effort**: 0.5 day

**Test**:
- [ ] 3 items layout horizontally with spacing
- [ ] Color box (12x12) appears before label
- [ ] Total width is sum of items + spacing
- [ ] Legend wraps to next line if too wide

#### Step 3.1.3: Implement vertical legend layout
**File**: `src/component/legend.rs`
**Effort**: 0.5 day

**Test**:
- [ ] 3 items layout vertically
- [ ] Items are left-aligned
- [ ] Consistent vertical spacing

#### Step 3.1.4: Implement legend positioning
**File**: `src/component/legend.rs`
**Effort**: 0.5 day

**Test**:
- [ ] `LegendPosition::Top` - legend above chart area
- [ ] `LegendPosition::Bottom` - legend below chart area
- [ ] `LegendPosition::Left` - legend left of chart area (vertical)
- [ ] `LegendPosition::Right` - legend right of chart area (vertical)

---

### 3.2 Interactive Legend

#### Step 3.2.1: Add click handler to legend items
**File**: `src/component/legend.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Click on legend item - fires LegendClickAction
- [ ] Action contains dataset_index
- [ ] Cursor changes to pointer on hover

#### Step 3.2.2: Implement dataset toggle
**File**: `src/chart/bar_chart.rs` (then others)
**Effort**: 0.5 day

**Test**:
- [ ] Click legend item - dataset hides
- [ ] Click again - dataset shows
- [ ] Hidden item appears faded in legend
- [ ] Chart Y-axis rescales when dataset hidden

#### Step 3.2.3: Add hover effect to legend items
**File**: `src/component/legend.rs`
**Effort**: 0.25 day

**Test**:
- [ ] Hover over legend item - item highlights
- [ ] Corresponding data in chart highlights
- [ ] Mouse leave - highlight removed

---

**Phase 3 Checkpoint**:
- [ ] All charts display legend when enabled
- [ ] Legend items can be clicked to toggle datasets
- [ ] Legend position options all work
- [ ] Hidden datasets show in faded legend items

---

## Phase 4: Scientific Plot Types (P1)

### 4.1 Histogram Chart

#### Step 4.1.1: Create HistogramData struct
**File**: `src/core/data.rs`
**Effort**: 0.25 day

```rust
pub struct HistogramData {
    pub values: Vec<f64>,
    pub bins: Option<usize>,
    pub range: Option<(f64, f64)>,
    pub density: bool,
}
```

**Test**:
- [ ] Create HistogramData with 1000 random values
- [ ] Verify bins defaults to auto-calculate

#### Step 4.1.2: Implement bin calculation
**File**: `src/chart/histogram_chart.rs`
**Effort**: 0.5 day

```rust
fn compute_bins(&self) -> Vec<HistogramBin> {
    // Freedman-Diaconis or Sturges rule
}
```

**Test**:
- [ ] 100 values → reasonable number of bins (5-15)
- [ ] 10000 values → more bins than 100 values
- [ ] Explicit bins=20 → exactly 20 bins
- [ ] All values fall into exactly one bin

#### Step 4.1.3: Create HistogramChart widget
**File**: `src/chart/histogram_chart.rs`
**Effort**: 1 day

**Test**:
- [ ] Display histogram of normal distribution
- [ ] Bars have correct heights (counts)
- [ ] X-axis shows bin edges
- [ ] Y-axis shows counts

#### Step 4.1.4: Add density mode
**File**: `src/chart/histogram_chart.rs`
**Effort**: 0.5 day

**Test**:
- [ ] density=true → Y-axis shows density (area sums to 1)
- [ ] density=false → Y-axis shows counts
- [ ] Visual shape is same, only Y scale differs

#### Step 4.1.5: Add cumulative mode
**File**: `src/chart/histogram_chart.rs`
**Effort**: 0.5 day

**Test**:
- [ ] cumulative=true → bars increase monotonically
- [ ] Final bar reaches total count (or 1.0 for density)

---

### 4.2 Error Bars

#### Step 4.2.1: Add error data to DataPoint
**File**: `src/core/data.rs`
**Effort**: 0.25 day

```rust
pub struct DataPoint {
    // ... existing fields
    pub x_err: Option<(f64, f64)>,  // (lower, upper)
    pub y_err: Option<(f64, f64)>,
}
```

**Test**:
- [ ] Create DataPoint with y_err = Some((2.0, 3.0))
- [ ] Verify backward compatible (existing code still works)

#### Step 4.2.2: Create DrawErrorBar shader
**File**: `src/element/error_bar.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Draw vertical error bar with caps
- [ ] Draw horizontal error bar with caps
- [ ] Cap width is configurable

#### Step 4.2.3: Integrate error bars with ScatterChart
**File**: `src/chart/scatter_chart.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Points with y_err show vertical error bars
- [ ] Points with x_err show horizontal error bars
- [ ] Points with both show cross pattern
- [ ] Error bars use same color as points

#### Step 4.2.4: Integrate error bars with LineChart
**File**: `src/chart/line_chart.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Each point can have individual error bars
- [ ] Error bars don't interfere with line drawing
- [ ] Error bars animate with points

---

### 4.3 Box Plot

#### Step 4.3.1: Create BoxPlotData struct
**File**: `src/core/data.rs`
**Effort**: 0.25 day

```rust
pub struct BoxPlotStats {
    pub min: f64,
    pub q1: f64,
    pub median: f64,
    pub q3: f64,
    pub max: f64,
    pub outliers: Vec<f64>,
}
```

**Test**:
- [ ] Create BoxPlotStats from raw data
- [ ] Verify quartiles calculated correctly
- [ ] Verify outliers identified (1.5 * IQR rule)

#### Step 4.3.2: Implement statistics calculation
**File**: `src/chart/box_plot_chart.rs`
**Effort**: 0.5 day

```rust
fn calculate_stats(values: &[f64]) -> BoxPlotStats { ... }
```

**Test**:
- [ ] Known dataset: [1,2,3,4,5] → median=3, q1=1.5, q3=4.5
- [ ] Dataset with outlier: [1,2,3,4,100] → outlier=[100]
- [ ] Empty dataset → error handled gracefully

#### Step 4.3.3: Create DrawBoxPlot shader
**File**: `src/element/box_plot.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Draw box from q1 to q3
- [ ] Draw median line inside box
- [ ] Draw whiskers from box to min/max
- [ ] Draw whisker caps

#### Step 4.3.4: Create BoxPlotChart widget
**File**: `src/chart/box_plot_chart.rs`
**Effort**: 1 day

**Test**:
- [ ] Display single box plot
- [ ] Display multiple box plots side by side
- [ ] Category labels on X-axis
- [ ] Y-axis auto-scales to data range + outliers

#### Step 4.3.5: Add outlier rendering
**File**: `src/chart/box_plot_chart.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Outliers shown as individual points
- [ ] Outliers use different marker style
- [ ] Outliers included in tooltip

---

### 4.4 Heatmap

#### Step 4.4.1: Create GridData struct
**File**: `src/core/data.rs`
**Effort**: 0.25 day

```rust
pub struct GridData {
    pub values: Vec<Vec<f64>>,
    pub x_labels: Option<Vec<String>>,
    pub y_labels: Option<Vec<String>>,
}
```

**Test**:
- [ ] Create 10x10 grid of random values
- [ ] Access value at (row=5, col=3)

#### Step 4.4.2: Create DrawHeatmapCell shader
**File**: `src/element/heatmap.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Draw cell with color from colormap
- [ ] value=0 → colormap start color
- [ ] value=1 → colormap end color
- [ ] value=0.5 → interpolated color

#### Step 4.4.3: Create HeatmapChart widget
**File**: `src/chart/heatmap_chart.rs`
**Effort**: 1 day

**Test**:
- [ ] Display 10x10 heatmap
- [ ] All cells visible and colored
- [ ] Cell borders optional

#### Step 4.4.4: Add colorbar
**File**: `src/component/colorbar.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Colorbar shows gradient from vmin to vmax
- [ ] Tick labels on colorbar show values
- [ ] Colorbar positioned to right of heatmap

#### Step 4.4.5: Add cell annotations
**File**: `src/chart/heatmap_chart.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Option to show value text in each cell
- [ ] Text color auto-contrasts with cell color
- [ ] Text size adjusts to cell size

---

**Phase 4 Checkpoint**:
- [ ] HistogramChart works with various bin counts
- [ ] Error bars work on scatter and line charts
- [ ] BoxPlotChart displays statistics correctly
- [ ] HeatmapChart displays grid data with colorbar

---

## Phase 5: Advanced Scales (P1)

### 5.1 Log Scale

#### Step 5.1.1: Create LogScale struct
**File**: `src/scale/log.rs`
**Effort**: 0.5 day

```rust
pub struct LogScale {
    base: f64,
    data_min: f64,
    data_max: f64,
    pixel_start: f64,
    pixel_end: f64,
}
```

**Test**:
- [ ] Create LogScale with base=10
- [ ] Create LogScale with base=2
- [ ] Verify data_min must be > 0

#### Step 5.1.2: Implement Scale trait for LogScale
**File**: `src/scale/log.rs`
**Effort**: 0.5 day

**Test**:
- [ ] get_pixel_for_value(10) with range [1, 100] → midpoint
- [ ] get_pixel_for_value(1) → pixel_start
- [ ] get_pixel_for_value(100) → pixel_end
- [ ] Logarithmic spacing verified

#### Step 5.1.3: Implement log scale tick generation
**File**: `src/scale/log.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Range [1, 1000] base=10 → ticks at 1, 10, 100, 1000
- [ ] Range [1, 100] base=10 → ticks at 1, 10, 100
- [ ] Tick labels formatted as powers (10⁰, 10¹, 10², 10³)

#### Step 5.1.4: Add ScaleType::Log variant
**File**: `src/scale/mod.rs`
**Effort**: 0.25 day

```rust
pub enum ScaleType {
    Linear(LinearScale),
    Category(CategoryScale),
    Log(LogScale),  // NEW
}
```

**Test**:
- [ ] ScaleType::Log can be created
- [ ] All Scale trait methods dispatch correctly

#### Step 5.1.5: Integrate log scale with charts
**File**: `src/chart/line_chart.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Set Y-axis to log scale
- [ ] Exponential data [1, 10, 100, 1000] appears linear
- [ ] Grid lines at log intervals
- [ ] Tick labels show powers

---

### 5.2 SymLog Scale

#### Step 5.2.1: Create SymLogScale struct
**File**: `src/scale/symlog.rs`
**Effort**: 0.5 day

```rust
pub struct SymLogScale {
    base: f64,
    linthresh: f64,  // Linear threshold
    data_min: f64,
    data_max: f64,
    pixel_start: f64,
    pixel_end: f64,
}
```

**Test**:
- [ ] Create SymLogScale with linthresh=10
- [ ] Verify it accepts negative values

#### Step 5.2.2: Implement symlog transform
**File**: `src/scale/symlog.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Values in [-linthresh, linthresh] → linear mapping
- [ ] Values > linthresh → log mapping
- [ ] Values < -linthresh → -log mapping
- [ ] Transform is continuous at boundaries

#### Step 5.2.3: Implement symlog tick generation
**File**: `src/scale/symlog.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Ticks include 0
- [ ] Positive and negative ticks symmetric
- [ ] Linear ticks near zero, log ticks far from zero

---

### 5.3 Time Scale

#### Step 5.3.1: Create TimeScale struct
**File**: `src/scale/time.rs`
**Effort**: 0.5 day

```rust
pub struct TimeScale {
    data_min: i64,  // Unix timestamp
    data_max: i64,
    pixel_start: f64,
    pixel_end: f64,
    timezone: Option<String>,
}
```

**Test**:
- [ ] Create TimeScale with Jan 1 to Dec 31
- [ ] Handle millisecond timestamps

#### Step 5.3.2: Implement time unit selection
**File**: `src/scale/time.rs`
**Effort**: 0.5 day

**Test**:
- [ ] 1 hour range → minute ticks
- [ ] 1 day range → hour ticks
- [ ] 1 month range → day ticks
- [ ] 1 year range → month ticks

#### Step 5.3.3: Implement time tick formatting
**File**: `src/scale/time.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Minute ticks: "10:30"
- [ ] Hour ticks: "10:00"
- [ ] Day ticks: "Jan 15"
- [ ] Month ticks: "Jan 2024"

#### Step 5.3.4: Add timestamp support to DataPoint
**File**: `src/core/data.rs`
**Effort**: 0.25 day

**Test**:
- [ ] Create DataPoint with timestamp
- [ ] Verify backward compatible

---

**Phase 5 Checkpoint**:
- [ ] Log scale works for positive data
- [ ] SymLog scale works for data crossing zero
- [ ] Time scale auto-selects appropriate units
- [ ] All scales integrate with existing charts

---

## Phase 6: Pan/Zoom Interactivity (P1)

### 6.1 Pan Handler

#### Step 6.1.1: Create PanZoomHandler struct
**File**: `src/interaction/pan_zoom.rs`
**Effort**: 0.25 day

```rust
pub struct PanZoomHandler {
    is_panning: bool,
    pan_start: DVec2,
    pan_start_range: ((f64, f64), (f64, f64)),
}
```

**Test**:
- [ ] Create handler, verify default state (not panning)

#### Step 6.1.2: Implement pan start/update/end
**File**: `src/interaction/pan_zoom.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Middle mouse down → start panning
- [ ] Drag 100px right → data range shifts left
- [ ] Mouse up → stop panning
- [ ] Data range persists after pan

#### Step 6.1.3: Integrate pan with LineChart
**File**: `src/chart/line_chart.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Ctrl+drag pans the view
- [ ] Pan beyond data range allowed (empty space visible)
- [ ] Grid and axis labels update during pan

---

### 6.2 Zoom Handler

#### Step 6.2.1: Implement scroll wheel zoom
**File**: `src/interaction/pan_zoom.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Scroll up → zoom in (data range shrinks)
- [ ] Scroll down → zoom out (data range expands)
- [ ] Zoom centered on mouse position
- [ ] Zoom factor is configurable

#### Step 6.2.2: Implement rectangle zoom
**File**: `src/interaction/pan_zoom.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Right-drag draws selection rectangle
- [ ] Release → zoom to selected area
- [ ] Rectangle visible during drag
- [ ] Small rectangles ignored (< 10px)

#### Step 6.2.3: Implement reset zoom
**File**: `src/interaction/pan_zoom.rs`
**Effort**: 0.25 day

**Test**:
- [ ] Double-click resets to original view
- [ ] Reset restores exact original range
- [ ] Works after multiple zoom operations

#### Step 6.2.4: Integrate with all cartesian charts
**File**: Multiple files
**Effort**: 1 day

**Test**:
- [ ] BarChart supports pan/zoom
- [ ] LineChart supports pan/zoom
- [ ] ScatterChart supports pan/zoom
- [ ] HeatmapChart supports pan/zoom

---

### 6.3 Zoom Constraints

#### Step 6.3.1: Add zoom limits
**File**: `src/interaction/pan_zoom.rs`
**Effort**: 0.25 day

**Test**:
- [ ] Cannot zoom in beyond min_range
- [ ] Cannot zoom out beyond max_range
- [ ] Limits are configurable

#### Step 6.3.2: Add axis lock options
**File**: `src/interaction/pan_zoom.rs`
**Effort**: 0.25 day

**Test**:
- [ ] lock_x_axis → only Y zooms/pans
- [ ] lock_y_axis → only X zooms/pans
- [ ] Useful for time series (lock Y)

---

**Phase 6 Checkpoint**:
- [ ] Pan with middle mouse or Ctrl+drag
- [ ] Zoom with scroll wheel
- [ ] Rectangle selection zoom
- [ ] Double-click to reset
- [ ] Zoom limits enforced

---

## Phase 7: Colormaps (P2)

### 7.1 Colormap Implementation

#### Step 7.1.1: Create Colormap struct
**File**: `src/core/colormap.rs`
**Effort**: 0.25 day

```rust
pub struct Colormap {
    name: String,
    colors: Vec<Vec4>,
}
```

**Test**:
- [ ] Create colormap with 256 colors
- [ ] Access color at index 0, 127, 255

#### Step 7.1.2: Implement colormap sampling
**File**: `src/core/colormap.rs`
**Effort**: 0.5 day

```rust
pub fn sample(&self, t: f64) -> Vec4 { ... }
```

**Test**:
- [ ] sample(0.0) → first color
- [ ] sample(1.0) → last color
- [ ] sample(0.5) → interpolated middle color
- [ ] sample(-0.1) → clamped to first
- [ ] sample(1.1) → clamped to last

#### Step 7.1.3: Add standard colormaps
**File**: `src/core/colormap.rs`
**Effort**: 1 day

**Test**:
- [ ] Colormap::viridis() - perceptually uniform
- [ ] Colormap::plasma() - perceptually uniform
- [ ] Colormap::coolwarm() - diverging
- [ ] Colormap::jet() - rainbow (legacy)
- [ ] Visual comparison with matplotlib output

---

### 7.2 Normalization

#### Step 7.2.1: Create Normalize struct
**File**: `src/core/colormap.rs`
**Effort**: 0.25 day

```rust
pub struct Normalize {
    vmin: f64,
    vmax: f64,
    clip: bool,
}
```

**Test**:
- [ ] normalize(vmin) → 0.0
- [ ] normalize(vmax) → 1.0
- [ ] normalize((vmin+vmax)/2) → 0.5

#### Step 7.2.2: Create LogNorm struct
**File**: `src/core/colormap.rs`
**Effort**: 0.25 day

**Test**:
- [ ] LogNorm with vmin=1, vmax=1000
- [ ] normalize(10) → 0.5 (log scale)
- [ ] normalize(0) → error or special value

#### Step 7.2.3: Integrate with HeatmapChart
**File**: `src/chart/heatmap_chart.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Heatmap uses Normalize by default
- [ ] Option to switch to LogNorm
- [ ] Colorbar shows correct scale

---

**Phase 7 Checkpoint**:
- [ ] All standard colormaps available
- [ ] Linear and log normalization work
- [ ] Heatmap renders correctly with colormaps
- [ ] Colorbar displays colormap gradient

---

## Phase 8: Annotations (P2)

### 8.1 Text Annotations

#### Step 8.1.1: Create Annotation enum
**File**: `src/component/annotation.rs`
**Effort**: 0.25 day

```rust
pub enum Annotation {
    Text { position: DVec2, text: String, ... },
    Arrow { start: DVec2, end: DVec2, ... },
    Line { start: DVec2, end: DVec2, ... },
}
```

**Test**:
- [ ] Create text annotation
- [ ] Create arrow annotation
- [ ] Create line annotation

#### Step 8.1.2: Implement text annotation rendering
**File**: `src/component/annotation.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Text appears at specified data coordinates
- [ ] Text converts from data to pixel coordinates
- [ ] Text respects anchor setting

---

### 8.2 Arrow Annotations

#### Step 8.2.1: Implement arrow rendering
**File**: `src/component/annotation.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Arrow from (1, 2) to (3, 4) draws correctly
- [ ] Arrowhead points in direction of arrow
- [ ] Arrow color and width configurable

#### Step 8.2.2: Implement text + arrow combination
**File**: `src/component/annotation.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Text annotation with arrow pointing to data
- [ ] Arrow connects text to target point
- [ ] Text positioned to not overlap arrow

---

### 8.3 Reference Lines

#### Step 8.3.1: Implement horizontal reference line
**File**: `src/component/annotation.rs`
**Effort**: 0.25 day

**Test**:
- [ ] Horizontal line at y=50 spans entire X range
- [ ] Line can be dashed
- [ ] Line has label

#### Step 8.3.2: Implement vertical reference line
**File**: `src/component/annotation.rs`
**Effort**: 0.25 day

**Test**:
- [ ] Vertical line at x=10 spans entire Y range
- [ ] Commonly used for "today" marker on time series

#### Step 8.3.3: Implement shaded region
**File**: `src/component/annotation.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Shaded region from y=40 to y=60
- [ ] Semi-transparent fill
- [ ] Useful for confidence intervals

---

**Phase 8 Checkpoint**:
- [ ] Text annotations with various anchors
- [ ] Arrow annotations pointing to data
- [ ] Horizontal and vertical reference lines
- [ ] Shaded regions for ranges

---

## Phase 9: Export (P2)

### 9.1 PNG Export

#### Step 9.1.1: Implement render-to-texture
**File**: `src/export/png.rs`
**Effort**: 1 day

**Test**:
- [ ] Render chart to offscreen texture
- [ ] Texture dimensions match requested size
- [ ] Content matches on-screen rendering

#### Step 9.1.2: Implement texture-to-PNG
**File**: `src/export/png.rs`
**Effort**: 0.5 day

**Test**:
- [ ] Export 800x600 chart to PNG file
- [ ] PNG file is valid and opens in image viewer
- [ ] Colors match on-screen display

#### Step 9.1.3: Add export options
**File**: `src/export/png.rs`
**Effort**: 0.5 day

**Test**:
- [ ] DPI setting affects output size
- [ ] Transparent background option
- [ ] Scale factor for retina displays

---

### 9.2 SVG Export (Advanced)

#### Step 9.2.1: Create SVG writer
**File**: `src/export/svg.rs`
**Effort**: 1 day

**Test**:
- [ ] Generate valid SVG header
- [ ] SVG dimensions match chart size

#### Step 9.2.2: Export basic shapes to SVG
**File**: `src/export/svg.rs`
**Effort**: 2 days

**Test**:
- [ ] Rectangles export as <rect>
- [ ] Lines export as <line>
- [ ] Text exports as <text>

#### Step 9.2.3: Export chart to SVG
**File**: `src/export/svg.rs`
**Effort**: 2 days

**Test**:
- [ ] Bar chart exports to valid SVG
- [ ] SVG renders correctly in browser
- [ ] File size reasonable

---

**Phase 9 Checkpoint**:
- [ ] PNG export works for all chart types
- [ ] SVG export works for basic charts
- [ ] Exported files are valid and look correct

---

## Phase 10: Remaining Plot Types (P3)

### 10.1 Violin Plot
**Effort**: 4 days

**Steps**:
1. Implement kernel density estimation (KDE)
2. Create DrawViolin shader
3. Create ViolinPlotChart widget
4. Add box plot overlay option

### 10.2 Contour Plot
**Effort**: 5 days

**Steps**:
1. Implement marching squares algorithm
2. Create DrawContourLine shader
3. Create ContourChart widget (lines only)
4. Add filled contour option

### 10.3 Quiver Plot
**Effort**: 3 days

**Steps**:
1. Create VectorFieldData struct
2. Create DrawArrow shader
3. Create QuiverChart widget

### 10.4 Step Chart
**Effort**: 1 day

**Steps**:
1. Add step mode to LineChart
2. Support "pre", "post", "mid" step styles

### 10.5 Fill Between
**Effort**: 2 days

**Steps**:
1. Create DrawFillArea shader
2. Add fill_between method to LineChart

---

## Testing Strategy

### Unit Tests

Each component should have unit tests for:
- [ ] Default construction
- [ ] All public methods
- [ ] Edge cases (empty data, single point, etc.)
- [ ] Error handling

### Visual Tests

Create `chart_zoo` examples for:
- [ ] Each chart type with sample data
- [ ] Each configuration option
- [ ] Edge cases (large data, small data, negative values)

### Integration Tests

- [ ] All charts render without crashing
- [ ] Animation completes without errors
- [ ] Interaction events fire correctly
- [ ] Export produces valid files

---

## Definition of Done

A feature is considered complete when:

1. **Code Complete**: All implementation steps done
2. **Tests Pass**: Unit tests and visual tests pass
3. **Documentation**: README updated with usage example
4. **Example**: Added to chart_zoo demo
5. **Review**: Code reviewed and approved

---

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Text rendering performance | Use batched text drawing |
| Large dataset performance | Implement data decimation |
| Memory usage | Stream data instead of loading all |
| Cross-platform issues | Test on Windows, Mac, Linux early |
| Shader compatibility | Test on multiple GPU vendors |

---

## Summary

| Phase | Priority | Effort | Dependencies |
|-------|----------|--------|--------------|
| 1. Text Rendering | P0 | 5 days | None |
| 2. Tooltips | P0 | 4 days | Phase 1 |
| 3. Legend | P1 | 3 days | Phase 1 |
| 4. Scientific Plots | P1 | 12 days | Phase 1 |
| 5. Advanced Scales | P1 | 5 days | None |
| 6. Pan/Zoom | P1 | 4 days | None |
| 7. Colormaps | P2 | 3 days | None |
| 8. Annotations | P2 | 3 days | Phase 1 |
| 9. Export | P2 | 7 days | All rendering |
| 10. Remaining Plots | P3 | 15 days | Phase 4 |
| **Total** | | **61 days** | |

**Critical Path**: Phase 1 → Phase 2 → Phase 3 → Phase 4

**Parallel Work Possible**:
- Phase 5, 6, 7 can start immediately
- Phase 8 can start after Phase 1
- Phase 9 can start after Phase 4
