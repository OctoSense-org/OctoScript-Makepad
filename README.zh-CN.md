# Octoscript-Makepad

[English](README.md) | 简体中文

面向 **Octoscript DSL → makepad 原生控件**渲染器所构建应用的主题化、跨平台**组件套件**。

用纯数据形式的 Octoscript DSL 编写一次 UI：它在 makepad-script VM 中求值，被翻译成 makepad 自身的控件方言，并在运行时挂载为**真正的 makepad 原生控件**（支持设备上热重载）。本仓库既存放渲染管线，**也**存放基于这条管线的各套主题组件：目前是 Material 3，**iOS** 和 **liquid-glass** 在计划中。

## 渲染管线

```
Octoscript DSL  ──►  octoscript-render  ──►  UiNode tree  ──►  octoscript-makepad  ──►  makepad dialect string
{t:"column",       (makepad-script VM,   (backend-       (pure translation)     View{…}/Label{…}/…
 c:[ … ]}           renderer-free)        agnostic)                              │
                                                                                 ▼
                                                          makepad `Splash` widget .set_text() → live native widgets
```

- **`crates/octoscript-render`**：与后端无关的核心。在 makepad-script VM 中对 Octoscript DSL 求值，并遍历生成 `UiNode` 树。只依赖 `makepad-script`，有单元测试。
- **`crates/octoscript-makepad`**：makepad 后端。`to_makepad_ui(&UiNode) -> String` 把这棵树转换成 makepad 的 `View{}/Label{}/…` 方言。纯函数，有单元测试，构建和测试都不需要 makepad-platform/draw。
- **`crates/octoscript-widgets`**：**主题化原生控件套件**（目前是 Material 3，之后是 iOS / liquid-glass），以**外部 `script_mod!` 变体的形式扩展 makepad 控件**（见下文*无需 fork 的主题化*）。
- **`crates/makepad-d3`**：**以原生控件实现的 d3 语法**（比例尺、形状、布局、层级、地理、3D），在 VM 中注册到 `mod.d3.*` 下。于 2026-08-09 连同提交历史一起并入（原为 `mofa-org/makepad-d3`）。
- **`crates/makepad-plot`**：**matplotlib 图表集**（32 个控件：折线、柱状、散点、饼图、箱线、小提琴、热力图、等高线、向量场、3D 曲面/散点/折线、仪表盘、矩形树图……），位于 `mod.plot.*` 下。于 2026-08-09 连同提交历史一起并入（原为 `mofa-org/makepad-matplot`）。两者都基于与 `octoscript-widgets` **同一份** makepad 检出构建：每个工作区只能有一份 makepad，否则两份 `makepad-widgets` 会出现在同一个二进制中，各自注册到自己的堆里。它们的命名空间互不重叠，所以一张卡片可以使用其中之一，也可以两者都用。
- **`components/<theme>/`**：每个主题的**组件库**，以 `.octoscript` 编写（例如 `components/material/catalog.octoscript`，约 35 个 Material 组件加演示页面）。纯数据，可热重载，无需重新构建。
- **`components/flutter/`**：**flutter/samples 移植**，每个示例目录对应一个 `.octoscript`，共 108 个路由（见下文）。

## flutter/samples 移植

> **这些是静态示意，不是控件移植。** 套件中 86% 的节点是布局容器，没有一个是按钮；DSL 没有 `onPressed`、没有状态模型、也没有动画，因此 Flutter 控件无法被复现，只能被描绘出来。确认这一点的独立评审以及它发现的两处缺陷，见[套件 README](components/flutter/README.zh-CN.md)。


[flutter/samples](https://github.com/flutter/samples) 的每个目录在 `components/flutter/` 中都有一个 `.octoscript` 文件：共 27 个，108 个路由，全部由 `cargo test` 遍历检查。完整说明见 [`components/flutter/README.zh-CN.md`](components/flutter/README.zh-CN.md)。

其中十一个目录是有界面可画的应用，已完成移植：92 个页面，承载示例中的真实内容，包括按实际 sp 值排布的 M3 字号体系、六个海拔层级及其 dp 值和表面着色百分比、`date_planner` 的全部九个事件及其任务列表、`libraryInstance` 的四本书，以及 `destinations.json` 中的真实条目。

另外十六个目录用于演示 Flutter 的**平台集成**：`add_to_app`、`platform_channels`、`pedometer` 的 FFIgen 绑定、GLSL 着色器示例、构建工具。它们没有可画的界面，所以每个目录对应一个页面，说明该示例教的是什么、为什么无法移植，而不是凭空编一个 UI。

这些是**视觉移植**：管线在每次挂载时把 DSL 求值成一棵树，因此没有组件级状态、没有异步、没有 HTTP、没有导航栈，也没有动画。`animations` 移植了它的索引页和全部 20 个标题，但没有移植动画本身；`compass_app` 移植了五个页面，但没有移植占据该示例大部分内容的架构。凡是页面无法如实渲染的内容，都会在页面上直接说明。

套件分布在许多文件中，而 DSL 没有 `import`，因此由 `octoscript_makepad::kit` 按固定顺序**拼接**：`_kit.octoscript` 在最前，各示例按名称排序，`_index.octoscript`（路由器）在最后。测试和 `assemble` 示例调用这个函数；应用则用 `include_str!` 把同样的文件打包进去，因为 `cargo-makepad` 在一个生成的包装 crate 中构建 Android，而这个 crate 从不运行应用的构建脚本。有一个测试把打包列表与目录绑定，保证两者不会出现偏差。

```sh
cargo test -p octoscript-makepad     # sweep all 108 routes — no device needed
cargo run  -p flutter-samples    # run the catalog on desktop
cargo makepad android run -p flutter-samples --release    # …or on a phone
tools/visual-qa.sh               # screenshot all 108 on the device
```

每个页面都在真机（OnePlus 6T）上运行并人工查看过，而不只是断言检查：`tools/visual-qa.sh` 通过 adb 逐个驱动路由、截图并生成缩略图总览。由此发现了九处路由遍历在结构上无法察觉的渲染缺陷：页面根节点塌缩、下行字母被裁切、段落不换行、三个空的选择器、只有一像素宽的聊天气泡，全部已修复。与 Flutter 之间剩余的差异是结构性的，列在套件的 README 中。

在此基础上，点击交互还需要两处修复，都记录在套件的 README 中：`View` 会忽略 `on_click`（只有 `Button`/`CheckBox`/`GlassPanel` 支持），因此翻译器现在会在任何可点击的容器上覆盖一个透明 Button；另外，由于 `Splash` isolate 会以它自己的视图根来解析 `ui`，处理函数写入的 `nav_signal` 标签必须位于挂载的树*内部*。

## 无需 fork 的主题化（关键设计点）

makepad 的原生控件（复选框、开关、单选、滑块、文本框）由它们各自的 MPSL 着色器绘制；它们的外观**无法**从 Octoscript DSL 触及。外部 crate **可以**触及，但只有一种方式可行：

| 机制 | 结果 |
|---|---|
| 运行时用 `script_eval!` 覆盖 `mod.prelude.widgets.*` | 着色器**丢失** → 控件渲染为空白 ❌ |
| **编译期 `script_mod!`**：扩展基础控件（`mod.widgets.CheckBox = mod.widgets.CheckBoxFlat{ draw_bg +: {…} }`），再引用到 prelude 中 | 着色器**保留** ✅ |

`script_mod!` 宏在构建时编译 MPSL；运行时字符串永远不会被编译。因此 `octoscript-widgets` 可以基于**上游 makepad** 重新设计 makepad 控件的样式，**无需 fork**：主题化本身不需要任何上游改动。（已在设备上验证：编译期变体能正常渲染；运行时覆盖渲染为空白。）

每个新主题只是在 `octoscript-widgets` 中多加一些变体，再加一个 `.octoscript` 组件库，不需要为每个主题 fork 一次 makepad。

### 唯一一个上游 PR

基于上游构建并运行 `kit-host` 时，只暴露出**一处**上游缺少的东西：**`Splash` 在主 VM 上挂载的选项**。上游的 `Splash` 总是分配一个 *isolate* VM（`alloc_splash_vm_with_network(allow_net)`），但浅色主题和共享堆都位于应用的**主** VM 上。修复方法是本项目的 fork 在 `widgets/src/splash.rs` 中新增的一个小字段 `isolate: false`；把它合入上游后，受信任的、由应用生成的套件就能挂载到主 VM 上（主题正确，也不会出现 isolate 堆上的 animator panic）。在此之前，套件挂载在 isolate 上（默认深色主题）。这是*唯一*需要的上游改动；其他一切都可以直接基于上游 `dev` 运行。

## OctoSense 应用的共享运行时

本仓库负责 AppCards、Mail、OctoSense 桌面端与移动端、Android、OpenHarmony 以及浏览器宿主的共享运行时。`runtime.json` 锁定一个底层 `OctoSense-org/makepad` 修订版本和一个 `OctoSense-org/Octoscript` 修订版本。根 Cargo 工作区中出现的是同样的固定版本；`tools/runtime.py` 会拒绝任何偏差。每个 Cargo 工作区只声明它实际用到的同级目录覆盖，从而保证锁定构建可复现。Mail 的滚动、文本输入和原生 HTML WebView 支持都放在这里以及锁定的 Makepad 源码中，而不是放在各个应用的补丁里。

把这几个独立仓库作为同级目录放置，分别命名为 `octoscript-makepad`、`octoscript` 和 `makepad`。在本仓库中运行 `python3 tools/runtime.py prepare`，或者使用 AppCards 的 `tools/setup-native.py`（它还会选择框架版本）。已有的本地修改会被保留；`--update` 只会移动干净的依赖检出。`python3 tools/runtime.py verify --cargo-manifest Cargo.toml` 会检查源码集合，并拒绝在解析后的 Cargo 依赖图中出现多个 Makepad 实例。

## 构建

### 原生应用验证

在本工作区中运行 `cargo build --release -p kit-host --bin beauty-host` 进行构建。用 `--remote` 启动独立宿主；自动化流程会设置 `MAKEPAD_HIDE_WINDOWS=1`，并在原生 GPU 后端上使用内置的 HTTP 探测接口。用 `/snap`、输入路由和 `/g` 检查所控制的应用；最后调用 `/gq` 并确认进程已退出。Studio 不属于这个流程。

宿主接受经过测量的 AppCard 设计和 L0 卡片，在重新挂载时保留文本选区和滚动状态，并在导航时回收平台 WebView。在 macOS 上，它的自定义事件探针会暴露所控制 WebView 的内容、滚动范围、快照和生命周期，供应用验收检查使用。

`cargo test --release -p octoscript-node -p octoscript-render -p octoscript-makepad` 检查可移植的渲染管线和组件契约。

## 状态

- ✅ `octoscript-render` + `octoscript-makepad`：可移植渲染管线；**基于上游 `makepad-script` 编译并通过测试**（修订版本 `e1c2164b`），无 fork
- ✅ **Material 3 套件**：`components/material/catalog.octoscript`，约 35 个组件（按钮、FAB、卡片、纸片、导航栏/导航轨/抽屉、应用栏，以**真正可交互的浮层**实现的对话框/菜单/面板，选择器、标签页、徽标、工具栏），M3 设计令牌（颜色、字号体系 + Medium 字重、形状、海拔、表面色调），Font-Awesome 单色图标，以及真正的动画（环形加载指示器 + 形状变形加载指示器）
- ✅ `octoscript-widgets`：Material 3 原生控件变体（复选框/开关/单选/滑块/文本框）+ `LoadingMorph`，无需 fork；**基于上游 `makepad-widgets` 编译通过**
- ✅ **`apps/kit-host`**：通用应用外壳，**基于上游 makepad 构建并运行**（桌面端，约 37 MB 二进制），无需 fork，通过 `octoscript_widgets::widgets_mod` 挂载 Material 套件
- ⏳ **唯一一个上游 PR：** `Splash` 主 VM 挂载选项（见上文），这是正确渲染浅色主题所需的唯一改动
- ⏳ **下一步：** 提交该 PR（或者修复 isolate VM 的主题/堆问题，让隔离挂载也能正常工作）；通过 `cargo-makepad` 构建 Android；以 `RippleButton` 变体实现按钮**触摸涟漪**；**iOS** + **liquid-glass** 套件

## 许可证

Apache-2.0（见 [LICENSE](LICENSE) 和 [NOTICE](NOTICE)）。并入的 `crates/makepad-d3`（MIT OR Apache-2.0）和 `crates/makepad-plot`（MIT）保留各自原有的许可证，随附的字体也保留其各自的许可证。
