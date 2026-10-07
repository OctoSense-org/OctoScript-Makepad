# OctoScript-Makepad

[English](README.md) | 简体中文

> **要开发 OctoSense 应用？** 你不需要在这个仓库里工作。它是所有 OctoSense 外壳和 `card-host` 共同依赖的 UI 运行时；OctoScript-App-Design-Flow 的 `tools/setup-native.py` 会按固定版本把它检出为同级目录 `octoscript-makepad/`。请按 [OctoSense 组织主页](https://github.com/OctoSense-org)给出的顺序阅读：[OctoScript-App-Design-Flow `AGENTS.md`](https://github.com/OctoSense-org/OctoScript-App-Design-Flow/blob/main/AGENTS.md) → [`flows/README.md`](https://github.com/OctoSense-org/OctoScript-App-Design-Flow/blob/main/flows/README.md) → [`docs/QUICKSTART.md`](https://github.com/OctoSense-org/OctoScript-App-Design-Flow/blob/main/docs/QUICKSTART.md)。

面向 **OctoScript DSL → makepad 原生控件**渲染器所构建应用的主题化、跨平台**组件套件**。

用纯数据形式的 OctoScript DSL 编写一次 UI：它在 makepad-script VM 中求值，被翻译成 makepad 自身的控件方言，并在运行时挂载为**真正的 makepad 原生控件**（支持设备上热重载）。本仓库既存放渲染管线，**也**存放基于这条管线的各套主题组件：目前是 Material 3，**iOS** 和 **liquid-glass** 在计划中。

[源码导读与运行步骤](docs/architecture-walkthrough.md)先用一个文本标签说明源码如何变成原生控件，
再介绍 L0 卡片、预览命令、状态与 agent 的边界。贡献约定见 [AGENTS.md](AGENTS.md)。

## 已安装应用卡片中的字体

`font_src` 是字体资源字符串。App Hub 会在转换前把应用包内的
`assets/fonts/Body.ttf` 等路径改写为自己的、经过允许列表检查的资源服务器 URL。
普通、L0 和测量设计渲染器使用 `http_resource` 加载这些宿主提供的 URL，因此字体
可以异步到达。编译进程序的字体仍使用 `crate_resource`；测量设计的 `file_resource`
只适用于准入策略允许该路径的宿主。渲染器本身不授予应用文件系统或网络权限。

测量设计中的标签和 Markdown 现在除了主字体、符号和表情字体，还带有延迟加载的
内置 CJK 后备字体。主字体只有拉丁字符时，中文不再依赖开发者机器的系统字体。
宿主需要包含 CJK 字体资源，例如通过 International 字体集打包；省略该资源的包
不能提供该后备字体。原生组件包中的
`font_src: {"$token": "key"}` 在进入渲染器之前由组件层解析成字体字符串；任意字体
来源对象不是渲染器节点合同的一部分。

`kit-host` 的原生字体测试执行生成的控件代码，提供确定性的 HTTP 完成事件，检查
包内字体成为主字体，并在禁用系统后备字体时排版中英文混合文本。这验证原生资源
与字体引擎，不代表完成了真实网络或手机截图验收。

## 渲染管线

```
OctoScript DSL  ──►  octoscript-render  ──►  UiNode tree  ──►  octoscript-makepad  ──►  makepad dialect string
{t:"column",       (makepad-script VM,   (backend-       (pure translation)     View{…}/Label{…}/…
 c:[ … ]}           renderer-free)        agnostic)                              │
                                                                                 ▼
                                                          host main VM → View → Splash.view → live native widgets
```

- **`crates/octoscript-node`**：无依赖的 `UiNode`/`Attrs` 数据模型，供各渲染后端共享。
- **`crates/octoscript-render`**：与后端无关的核心。在 makepad-script VM 中对 OctoScript DSL 求值，并遍历生成 `UiNode` 树。依赖 `makepad-script`、便携节点模型与 `serde_json`，不依赖 platform/draw/widgets；有单元测试。
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

makepad 的原生控件（复选框、开关、单选、滑块、文本框）由它们各自的 MPSL 着色器绘制；它们的外观**无法**从 OctoScript DSL 触及。外部 crate **可以**触及，但只有一种方式可行：

| 机制 | 结果 |
|---|---|
| 运行时用 `script_eval!` 覆盖 `mod.prelude.widgets.*` | 着色器**丢失** → 控件渲染为空白 ❌ |
| **编译期 `script_mod!`**：扩展基础控件（`mod.widgets.CheckBox = mod.widgets.CheckBoxFlat{ draw_bg +: {…} }`），再引用到 prelude 中 | 着色器**保留** ✅ |

`script_mod!` 宏在构建时编译 MPSL；运行时字符串永远不会被编译。因此 `octoscript-widgets` 可以基于**上游 makepad** 重新设计 makepad 控件的样式，**无需 fork**：主题化本身不需要任何上游改动。（已在设备上验证：编译期变体能正常渲染；运行时覆盖渲染为空白。）

每个新主题只是在 `octoscript-widgets` 中多加一些变体，再加一个 `.octoscript` 组件库，不需要为每个主题 fork 一次 makepad。

### 当前的挂载方式

`kit-host::App::mount` 与 `beauty-host::App::mount_request` 通过 `cx.with_vm`
在宿主的**主 VM** 上求值生成的控件源码，构建 `View` 后赋给 `Splash.view`。
字体与主题控件因此位于同一个 VM；这两个宿主当前不通过 `Splash::set_text` 挂载。

早期记录中的 isolate 挂载和拟议的 `isolate: false` 字段不是当前前提。
锁定的 Makepad 没有该字段，但宿主已经自行实现主 VM 挂载。复制旧示例前请读
[源码导读](docs/architecture-walkthrough.md)；其他消费者仍需单独核对挂载路径。

## OctoSense 应用的共享运行时

本仓库负责 OctoSense 外壳（[OctoSense](https://github.com/OctoSense-org/OctoSense) 中的桌面端与手机端）、App Hub 的 `card-host`、[OctoScript-App-Design-Flow](https://github.com/OctoSense-org/OctoScript-App-Design-Flow) 中的 App Card 与流程、Mail、Android、OpenHarmony 以及浏览器宿主的共享运行时。`runtime.json` 锁定一个底层 `OctoSense-org/makepad` 修订版本和一个 `OctoSense-org/Octoscript` 修订版本。根 Cargo 工作区中出现的是同样的固定版本；`tools/runtime.py` 会拒绝任何偏差。每个 Cargo 工作区只声明它实际用到的同级目录覆盖，从而保证锁定构建可复现。Mail 的滚动、文本输入和原生 HTML WebView 支持都放在这里以及锁定的 Makepad 源码中，而不是放在各个应用的补丁里。

把这几个独立仓库作为同级目录放置，分别命名为 `octoscript-makepad`、`octoscript` 和 `makepad`。在本仓库中运行 `python3 tools/runtime.py prepare`，或者使用 [OctoScript-App-Design-Flow](https://github.com/OctoSense-org/OctoScript-App-Design-Flow) 的 `tools/setup-native.py`（它的 `native-runtime.lock.json` 还会选择框架版本）。已有的本地修改会被保留；`--update` 只会移动干净的依赖检出。`python3 tools/runtime.py verify --cargo-manifest Cargo.toml` 会检查源码集合，并拒绝在解析后的 Cargo 依赖图中出现多个 Makepad 实例。

## 构建

### 原生应用验证

先准备并验证上文的锁定依赖。运行组件目录使用
`cargo run -p kit-host --bin kit-host`。预览宿主使用
`cargo build --release -p kit-host --bin beauty-host` 构建，并设置 `BEAUTY_REQUEST`
指向包含 card/data 文件路径和视口尺寸的 JSON；完整步骤见[源码导读](docs/architecture-walkthrough.md)。用 `--remote` 启动独立宿主；自动化流程会设置 `MAKEPAD_HIDE_WINDOWS=1`，并在原生 GPU 后端上使用内置的 HTTP 探测接口。用 `/snap`、输入路由和 `/g` 检查所控制的应用；最后调用 `/gq` 并确认进程已退出。Studio 不属于这个流程。

宿主接受经过测量的 AppCard 设计和 L0 卡片，在重新挂载时保留文本选区和滚动状态，并在导航时回收平台 WebView。在 macOS 上，它的自定义事件探针会暴露所控制 WebView 的内容、滚动范围、快照和生命周期，供应用验收检查使用。

`cargo test --release -p octoscript-node -p octoscript-render -p octoscript-makepad` 检查可移植的渲染管线和组件契约。

## 状态与验证范围

源码包含有执行预算的求值器、主题与设计稿翻译器、Material 控件、L0 原生套件、
图表以及原生目录/预览宿主。当前依赖以 [runtime.json](runtime.json) 为准。
上文设备结果早于当前锁定版本；发布前请重新运行便携测试和原生验收。

## 许可证

Apache-2.0（见 [LICENSE](LICENSE) 和 [NOTICE](NOTICE)）。并入的 `crates/makepad-d3`（MIT OR Apache-2.0）和 `crates/makepad-plot`（MIT）保留各自原有的许可证，随附的字体也保留其各自的许可证。
