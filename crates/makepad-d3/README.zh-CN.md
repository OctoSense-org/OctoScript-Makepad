# Makepad D3

[English](README.md) | 简体中文

一个兼容 D3.js 的数据可视化库，基于 [Makepad](https://github.com/makepad/makepad) 的 GPU 加速渲染。

> **Makepad 2.0 / Octoscript 状态（2026-07）：** 本库现已面向 **Makepad 2.0**，其 Script/**Octoscript** 运行时取代了旧的 Live 系统（`live_design!`）。makepad-d3 向 Octoscript VM 注册了一个可脚本化的 **`d3.*` 组件命名空间**，因此可以直接用 Octoscript DSL 编写图表，包括通过 `d3.Octoscript` 宿主组件在沙箱化的 `runsplash` 风格小应用中使用。设计与迁移记录见：[`docs/OCTOSCRIPT_INTEGRATION_DESIGN.md`](docs/OCTOSCRIPT_INTEGRATION_DESIGN.md)。

## 快速运行

Makepad 2.0 来自 Octoscript-Makepad 工作区根目录（`[workspace.dependencies]`）中固定的 `OctoSense-org/makepad` 版本，与 OctoSense-org/octosense 使用的是同一个固定版本：

```bash
# NOTE: this crate now lives in the Octoscript-Makepad workspace; build it from there.
# The standalone instructions below are kept for building it outside that workspace.
git clone https://github.com/mofa-org/makepad-d3.git
cd makepad-d3
cargo run --example octoscript_demo
```

这会打开一个整个界面都由 Octoscript DSL 编写的仪表盘：带声明式 `data:` 的 `d3.BarChart`、`d3.PieChart`、`d3.LineChart`、`d3.ScatterChart`，由脚本驱动 `ui.chart.set_data(...)` 的按钮，`on_click`/`on_hover` 闭包，以及一个运行自身 octoscript 主体的沙箱化 `d3.Octoscript` 隔离实例。

## 在 Octoscript DSL 中使用 d3 图表

### 1. 注册 `d3.*` 命名空间（一行 Rust）

```rust
use makepad_widgets::*;

app_main!(App);

#[derive(Script, ScriptHook)]
pub struct App {
    #[live] ui: WidgetRef,
}

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        makepad_widgets::script_mod(vm);
        makepad_d3::script_mod(vm);      // <- adds the d3.* namespace
        self::script_mod(vm)             // your app's script_mod! block
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
```

`makepad_d3::script_mod` 还会把 `d3` 注入到 widgets prelude 中，因此任何以 `use mod.prelude.widgets.*` 开头的作用域（包括每一个由 `Splash` 承载的主体）都可以直接写 `d3.BarChart{...}`，无需额外导入。

### 2. 图表组件：全部 21 个

所有图表遵循同一套约定：声明式属性、脚本方法和事件闭包。完整的逐组件参考（数据格式、属性、方法、事件）见 **[`docs/d3-octoscript.md`](docs/d3-octoscript.md)**；命名空间一览：

| 类别 | 组件 |
|---|---|
| 基础 | `d3.BarChart` `d3.LineChart` `d3.AreaChart` `d3.ScatterChart` `d3.PieChart` |
| 统计 | `d3.Histogram` `d3.Heatmap` `d3.RadarChart` `d3.BoxPlot` |
| 层级 | `d3.Treemap` `d3.Sunburst` `d3.CirclePack` `d3.TreeChart` |
| 流向 | `d3.Sankey` `d3.ChordDiagram` `d3.ArcDiagram` |
| 网络 / 密度 | `d3.ForceGraph` `d3.Hexbin` `d3.Ridgeline` `d3.Horizon` `d3.Contour` |
| 地理 | `d3.Globe`（拖动旋转） |
| 3D | `d3.Surface3D` `d3.Scatter3D` `d3.Bar3D`（拖动环绕，滚轮缩放） |
| 沙箱宿主 | `d3.Octoscript` |

通用属性：`width`/`height`、`plot_margin: Inset{...}`、`grid_color`、`label_color`。没有 `data:` 的图表会渲染一小份演示数据。

**脚本方法**（通过 `ui.` 在 `:=` id 上调用）：

| 方法 | 作用 |
|---|---|
| `ui.chart.set_data(values)` | 替换数据，重新适配并重绘 |
| `ui.chart.set_labels(labels)` | 替换类别/扇区标签 |
| `ui.chart.set_domain(min, max)` | 固定 y 轴定义域（关闭自动适配） |
| `ui.chart.data()` | 以脚本数组形式读回数值 |

**事件**：闭包接收图元索引：

```octoscript
d3.BarChart{
    on_click: |i| ui.status.set_text("clicked bar " + i)
    on_hover: |i| ui.status.set_text("hovering " + i)
}
```

### 3. 一个完整的 Octoscript 片段

```octoscript
View{ width: Fill height: Fit flow: Down spacing: 10
    chart := d3.BarChart{
        height: 300
        data: [30 86 168 281 303 365]
        labels: ["Jan" "Feb" "Mar" "Apr" "May" "Jun"]
        on_click: |i| ui.status.set_text("clicked bar " + i)
    }
    View{ width: Fill height: Fit flow: Right spacing: 10
        Button{text: "Update" on_click: || ui.chart.set_data([2 7 1 8 2 8])}
        Button{text: "Pin 0..400" on_click: || ui.chart.set_domain(0, 400)}
        status := Label{text: "-"}
    }
    // x/y charts take pairs or objects — note the commas between pairs
    d3.LineChart{ height: 260 data: [[0 5], [1 18], [2 12], [3 40]] }
}
```

### 4. 沙箱化的 octoscript 应用（`d3.Octoscript`）

原生的 `runsplash` 沙箱只能看到内置组件。`d3.Octoscript` 是一个可直接替换的宿主，它在一个**注册了 `d3.*` 的隔离脚本 VM** 中执行一段主体字符串。从 Rust 向它输入内容的方式，与 Markdown 组件流式处理 ```` ```runsplash ```` 代码块完全相同：

```octoscript
host := d3.Octoscript{ width: Fill height: Fit }
```

```rust
// e.g. in MatchEvent::handle_startup, or as a markdown code-block template
if let Some(mut host) = self.ui.widget(cx, ids!(host))
    .borrow_mut::<makepad_d3::octoscript::D3Octoscript>()
{
    host.set_text(cx, "flow: Right spacing: 12 \
        d3.PieChart{width: 240 height: 180 data: [4 3 2 1]} \
        d3.AreaChart{width: Fill height: 180 data: [3 7 4 9 6 12 8]}");
}
```

沙箱说明：内联的 `on_click` 处理函数可以使用 `ui.`，但主体层级的辅助 `fn` 不行（没有 `ui` 全局变量，需要 makepad 侧提供钩子，见设计文档 §8.3/§14）；网络访问处于关闭状态；主体前会加上 `View{height:Fit, ` 前缀，因此它以该视图的属性/子元素开头。

### 5. 注意事项

- **嵌套数组字面量需要逗号**：`[[0 5], [1 18]]`；相邻的 `[..] [..]` 会被解析为索引操作。
- **在 `flow: Right` 行中，把固定尺寸的图表放在 `Fill` 兄弟元素之前**，否则延后布局的 `Fill` 子元素会在固定尺寸元素绘制之后才确定位置，二者会重叠。
- 已知的上游缺口：在当前 makepad dev 最新版本上，`DrawText::draw_abs`（坐标轴/扇区标签）不会渲染；内置的 `mod.widgets` 图表也有同样的表现；网格、图元和 `Label` 组件不受影响。
- 2.0 之前的 Chart Zoo（40 多种图表）属于待移植项，目前还无法针对 2.0 编译（`docs/OCTOSCRIPT_INTEGRATION_DESIGN.md` §11 Phase 4）。

## 功能

- **比例尺**：Linear、Log、Pow、Symlog、Time、Category、Band、Quantize、Quantile、Threshold
- **形状**：Line、Area、Arc、Pie、Stack 生成器，支持 7 种曲线插值
- **布局**：力导向图、Treemap、Tree、圆形打包（Circle packing）
- **地理**：Mercator、Orthographic、Equirectangular、Albers 投影
- **颜色**：RGB、HSL、LAB、HCL 色彩空间，支持感知均匀插值
- **交互**：Zoom、Brush、Tooltip 行为
- **组件**：Legend、Crosshair、Annotations、参考线（Reference lines）

## 安装

添加到你的 `Cargo.toml`：

```toml
[dependencies]
makepad-d3 = { git = "https://github.com/mofa-org/makepad-d3.git" }
```

## 快速上手

```rust
use makepad_d3::prelude::*;

// Create chart data
let data = ChartData::new()
    .with_labels(vec!["Jan", "Feb", "Mar", "Apr"])
    .add_dataset(
        Dataset::new("Revenue")
            .with_data(vec![100.0, 200.0, 150.0, 300.0])
            .with_hex_color(0x4285F4)
    );

// Create scales
let x_scale = CategoryScale::new()
    .with_labels(data.labels.clone())
    .with_range(50.0, 550.0);

let y_scale = LinearScale::new()
    .with_domain(0.0, 300.0)
    .with_range(350.0, 50.0);  // Inverted for screen coordinates
```

## 模块概览

| 模块 | 说明 |
|--------|-------------|
| `data` | ChartData、Dataset、DataPoint 数据结构 |
| `scale` | 数据到像素的映射函数 |
| `axis` | 坐标轴生成、刻度、标签、格式化 |
| `shape` | Path、Line、Area、Arc、Pie、Stack 生成器 |
| `shape::curve` | Linear、Catmull-Rom、Natural、Monotone、Basis、Cardinal、Step |
| `color` | 色彩空间、插值、配色方案 |
| `layout::force` | 力导向图模拟 |
| `layout::hierarchy` | Tree、Treemap、Pack 布局 |
| `geo` | 地理投影、GeoJSON 支持 |
| `interaction` | Zoom、Brush、Tooltip 行为 |
| `component` | Legend、Crosshair、Annotation、ReferenceLine |

## Chart Zoo 示例

运行包含 40 多种图表类型的完整图表画廊：

```bash
cargo run --example chart_zoo
```

### 可用图表

| 类别 | 图表 |
|----------|--------|
| **基础** | 柱状图、折线图、面积图、散点图、饼图、环形图 |
| **统计** | 直方图、箱线图、小提琴图、蜂群图 |
| **层级** | Treemap、旭日图、圆形打包、树图 |
| **网络** | 力导向图、桑基图、弦图、弧线图 |
| **时间序列** | K 线图、地平线图、日历图 |
| **地理** | 地球仪地图、分级统计地图 |
| **专用** | 热力图、等高线图、六边形分箱图、雷达图、平行坐标图 |

## 比例尺类型

### 线性比例尺

```rust
let scale = LinearScale::new()
    .with_domain(0.0, 100.0)
    .with_range(0.0, 500.0)
    .with_nice(true)        // Round to nice values
    .with_clamp(true);      // Clamp out-of-domain values

let pixel = scale.scale(50.0);  // -> 250.0
```

### 类别比例尺

```rust
let scale = CategoryScale::new()
    .with_labels(vec!["A", "B", "C", "D"])
    .with_range(0.0, 400.0)
    .with_padding(0.1);     // Gap between bands

let x = scale.scale_index(1);   // -> position for "B"
let width = scale.bandwidth();  // -> width of each band
```

### 时间比例尺

```rust
let scale = TimeScale::new()
    .with_domain(start_date, end_date)
    .with_range(0.0, 800.0);

let ticks = scale.ticks(&TickOptions::default());
```

## 曲线插值

```rust
use makepad_d3::shape::curve::*;

// Straight lines
let linear = LinearCurve;

// Smooth curves passing through all points
let catmull = CatmullRomCurve::new().with_alpha(0.5);

// Natural cubic spline
let natural = NaturalCurve;

// Preserves monotonicity (no overshoots)
let monotone = MonotoneCurve::x();

// Step function
let step = StepCurve::after();
```

## 力导向布局

```rust
use makepad_d3::layout::force::*;

let mut simulation = ForceSimulation::new()
    .with_nodes(nodes)
    .with_links(links)
    .add_force("charge", ManyBodyForce::new().with_strength(-30.0))
    .add_force("link", LinkForce::new().with_distance(50.0))
    .add_force("center", CenterForce::new(width / 2.0, height / 2.0));

// Run simulation
while simulation.tick() {
    // Update positions
}
```

## 桑基图

桑基图的实现遵循 D3 的算法：

- **节点值**：汇点 = 流入量，源点 = 流出量，中间节点 = max(流入, 流出)
- **全局缩放因子（ky）**：由最密集的层计算得出
- **SankeyJustify**：根据源点所在层数智能放置汇点
- **松弛（Relaxation）**：D3 风格的迭代位置优化

## 颜色插值

```rust
use makepad_d3::color::*;

// Perceptually uniform interpolation in LAB space
let color = lerp_color(
    rgba(0.2, 0.4, 0.8, 1.0),
    rgba(0.9, 0.3, 0.3, 1.0),
    0.5
);

// Sequential color scale
let scale = SequentialScale::new(vec![
    rgba(1.0, 1.0, 1.0, 1.0),
    rgba(0.0, 0.0, 1.0, 1.0),
]);
let color = scale.get_color(0.7);
```

## 地理投影

```rust
use makepad_d3::geo::*;

let projection = MercatorProjection::new()
    .with_center(-98.0, 39.0)
    .with_scale(1000.0)
    .with_translate(width / 2.0, height / 2.0);

// Project coordinates
if let Some((x, y)) = projection.project(lon, lat) {
    // Draw at (x, y)
}

// Render GeoJSON
let path = GeoPath::new(projection);
let segments = path.render(&geometry);
```

## 许可证

MIT

## 参考

- [D3.js](https://d3js.org/) - 原始 JavaScript 库
- [Makepad](https://github.com/makepad/makepad) - GPU UI 框架
- [D3 Sankey](https://github.com/d3/d3-sankey) - 桑基布局算法
- [D3 Force](https://github.com/d3/d3-force) - 力模拟
