# flutter/samples, on Octoscript + makepad

[English](README.md) | 简体中文

> ## 这些是示意图，不是移植。请先读这一段。
>
> 我们请一位独立审阅者（OpenAI Codex，只读权限）判断：这究竟是 Flutter 控件的移植，还是看起来像 Flutter 控件的静态图片。它的结论是正确的：
>
> > *"mostly static pictures resembling Flutter widgets, plus limited route
> > navigation — not faithful Flutter ports."*
> >
> > （大体上是看起来像 Flutter 控件的静态图片，外加有限的路由导航——并不是忠实的 Flutter 移植。）
>
> 这套组件中的大多数节点都是布局容器，而 DSL 没有 `onPressed`，没有状态模型，也没有 animator。这些界面在内容和几何上是忠实的，但它们不是能工作的控件。下文所有的"移植"都请读作"绘制"。
>
> 例外值得点名，因为它们是真实的：动画演示确实在动，地图是真正的 OpenStreetMap 渲染器，能力类界面读取的是设备上的实时数值。

## 真正能用的控件

审阅结果的中间一栏——十八个"仅绘制"的界面——其实不是十八个问题，而是一个。一个 `.octoscript` 界面每次挂载时被求值成一棵树，之后什么都不保留，一次点击唯一能做的就是改变路由。所以没有哪个复选框能保持勾选状态：根本没有地方存放"已勾选"。

现在有了。两个后端中的 `state.rs` 都持有一个 key→value 存储，DSL 用 `sget(key, default)` 读取它，而一次点击指定的是一个动作，而不是一个路由：

```text
{t: "row", tapto: "set:m3_c1=!",        c: [...]}   // toggle
{t: "row", tapto: "set:ca_guests=+1",   c: [...]}   // step
{t: "row", tapto: "set:dps_query=~4",   c: [...]}   // cycle
{t: "row", tapto: "set:m3_radio=2",     c: [...]}   // pick, for radio groups
```

动作与路由共用同一套 interning 机制，因此控件不需要新的节点属性，两个后端都从点击本来就会落到的那个地方拿到它。

十八个界面全部接上了这套机制，`a_control_changes_what_the_screen_renders` 用二十八个用例约束它：渲染一次，执行控件指定的动作，再渲染一次，两次结果必须不同。这是这套目录以前从来没有的测试，也正因为没有它，十八个界面才会在索引里被标成"已移植"，而其中没有一个控件能用。

其中八个本来就画出了控件，只是毫无反应——Material 3、Cupertino Gallery、Form App、Date Planner、Compass、Platform Design、Photo Search、Testing。另外十个一个控件都没有，于是每个都加上了原示例中真正具备的交互：

| 界面 | 现在能做什么 |
|---|---|
| `dynamic_theme` | 对话记录一轮一轮地到达，`change_text_scale_factor` 会缩放这个界面自己的文字——示例声明的三个函数中有两个是真实的 |
| `google_maps` | 镜头依次切换四个缩放级别，每一级都是一次不同的 OSM 请求 |
| `simple_sdf` / `simple_shader` | 以 4 种分辨率重新采样该场——这两个界面的要点就在于它是算术，不是图片 |
| `web_embedding` | 嵌入槽在三个页面之间导航（仅 ArkUI；makepad 上没有嵌入槽） |
| `platform_channels` | 再次调用，并缩小到单个通道。`device.notifications` 已注册却没有任何东西调用到它——现在它在列表里了 |
| `pedometer` | 重新读取传感器，并附带读取次数，以便区分新的结果 |
| `background_isolate_channels` | 重新执行非主线程调用 |
| `asset_transformation` | 逐个执行三个请求 |
| `platform_view_swift` | 示例自己的交互：切换两半，并把计数器传过去 |

写这部分时冒出了三个缺陷，此前每一个都看不见：

- `sget` 必须在第一次读取时**写入初始值**，而不只是返回默认值。`apply` 基于当前值做切换，无法知道某个界面认为某个控件默认是开启的，所以 `sget("m3_switch", 1)` 会从假定的 0 切换到 1，渲染结果完全相同。点击是生效的，界面却无法体现出来。
- `a or b` 在这个 VM 中不是运算符。它能通过解析，结果 Testing App 只数出了一个收藏，而不是三个。
- `txt()` 根据 `s.len()` 来确定盒子大小，而数字没有长度，于是这个节点整个消失了——Compass 的步进器画出了两个按钮，中间却什么都没有。

点击之后滚动位置会保留。重建会替换 Scroll 节点，所以视图过去会跳回顶部——在界面中间勾选一个复选框，结果人被带回了顶部。现在会在旧节点被丢弃之前读取其偏移量，并写回到新节点上，但仅在路由不变时这样做：勾选复选框的点击应当让你仍然看着那个复选框，而触发导航的点击应当让新界面从顶部开始。为此需要 shim 唯一的 getter：`octoscript_get_f32`。

## 外部审阅发现了什么

索引过去给全部 27 个都打了勾。那是错的，这一节就是更正。我们把两个仓库以只读方式交给 gpt-5.6-terra，要求它按代码实际*做了什么*给每个界面分类，忽略注释——在这个代码库里，注释又长又有说服力，本身就是问题的一部分。它的结论：

| | 数量 | 含义 |
|---|---|---|
| 可用 | 3 | 使用宿主的实时数据，或者真的会动 |
| 仅绘制 | 18 | 从示例誊写过来的几何与 token，背后什么都没有 |
| 说明 | 6 | 文字说明，没有移植 |

索引现在如实这样写，分成三个部分，只有那三个可用的打勾。界面本身在这方面没有变化：变化的是目录不再做出相反的声称。

它发现的四个问题不只是夸大其词：

- **`web_embedding` 会让应用崩溃。** 索引标为已完成的界面发生了 SIGSEGV。`web` 节点的 `src` 被当作 Column 上的图片源来应用，它的嵌入槽宽度——一个设备像素数——被当作以 vp 为单位的节点宽度，而 ArkUI 在 `SetWidth` 内部解引用了一个空的 frame node，而不是对数值做截断。
- **Web 覆盖层泄漏。** 嵌入槽从不在每次构建时重置，所以访问过那个界面之后，WebView 会浮在之后的每个界面上方。
- **`google_maps` 在 ArkUI 上什么都没画**，而它自己的文字却写着"rendered from OpenStreetMap vector tiles"。`map` 标签落入了遍历器的未知标签分支。
- **`compass_app` 从未触及定位栈**，而定位栈就放在 Octoscript-OH 里，完整却无法调用——权限已声明，`location::get` 已写好，DSL 界面却没有任何途径调用它。

这四个问题都已修复并在设备上验证。地图现在在 ArkUI 上加载真实的 OSM 栅格瓦片；Web 界面是真正的 ArkWeb 组件，并说明了它能做什么、不能做什么；指南针读取平台的定位开关和位置。

## "没有对应物"的界面已经不存在了

27 个目录中的每一个，现在都有一个就这套技术栈说出真实情况的界面。旧的横幅——*"No Octoscript+makepad analogue"*（没有 Octoscript+makepad 对应物）——错了六次，而且每次原因都一样：它评判的是 **makepad DSL** 能表达什么，而不是这个项目实际拥有什么。

它错在哪里，实际情况又是什么：

| 示例 | 原来的说法 | 实际情况 |
|---|---|---|
| `google_maps` | "是平台视图；没有哪棵控件树描述地图" | makepad 自带一个 1.2 万行、支持倾斜的 OpenStreetMap 渲染器 |
| `platform_channels` | "渲染管线里没有通道" | `build` 一直接受 `register` 钩子；桥接层有约 45 项能力 |
| `pedometer` | "需要平台传感器 API" | Octoscript-OH 有 `sensor::list`/`sample`/`stream` |
| `asset_transformation` | "Cargo 没有资源管线" | `octoscript://` 把请求解析为字节，其中一个是生成的 |
| `add_to_app` | "没有与 FlutterEngine 对应的东西" | 这个应用本身*就是* add-to-app，只是方向反过来 |
| `web_embedding` | "没有可嵌入的 hostElement" | `webslot::declare` 把 WebView 合成进原生树 |

其余的在**两个**仓库里都是配置、工具或文字说明——共享的 lint、启动窗口、Xcode target、仓库文档、CI 工具、一个已在上游删除的示例。它们现在会说出各自的对应物（HarmonyOS 元服务就是 App Clip 的概念；`tests/flutter_samples.rs` 就是 CI 遍历器），而不是被一笔带过。

最后两个，`simple_shader` 和 `simple_sdf`，也已完成，而它们的收尾方式与上面六个说明的是同一件事。片段着色器是一个从坐标到颜色的函数，SDF 是算术——所以由 DSL 自己来求值，每个单元格算一次而不是每个像素算一次，然后输出一个由普通节点组成的网格。`sdHeart` 需要 `sqrt`、`min`、`sign` 和 `smoothstep`，而 VM 没有这些；它们在 `_kit.octoscript` 里约 12 行（`sqrt` 用牛顿法实现）。同样的数学，同样的颜色，同样的图像，而且在完全没有片段着色器路径的 ArkUI 上也能运行。

编译型 MPSL 变体（`octoscript-widgets` 中的 `FlutterShader`/`FlutterSdf`）在 makepad 上仍然是正确答案，也仍在构建。它们能编译，节点也会输出，但什么都画不出来；怀疑原因是 Octoscript isolate 没有解析这个 crate 加入 prelude 的控件。尚未确认，也不再阻塞任何事。

## 被错误放弃的三个

`google_maps`、`platform_channels` 和 `pedometer` 各自都有一个"没有对应物"的界面。三个都错了，而且错法一样：它们孤立地评判 **makepad DSL** 能表达什么，而不是这个项目实际拥有什么。

- **google_maps** —— makepad 自带一个约 1.2 万行、支持旋转和倾斜的 OpenStreetMap 矢量瓦片渲染器（`widgets/src/map`）。在这里地图是一个控件，而不是平台视图。现在它是一张真实的地图，使用示例自己的镜头参数，另外还有一个 2.5D 视图。
- **platform_channels** —— `octoscript_render::build` 一直接受一个用于注入宿主函数的 `register` 钩子，Octoscript-OH 的天气卡片早已在用它。桥接层承载约 45 项能力。`invoke(tool)` 现在可以访问这个注册表，由桥接层在挂载时安装，因此渲染器仍然不依赖它。在设备上，这个界面显示的是真实的返回结果。
- **pedometer** —— FFIgen/JNIgen 那一半没有对应物，但这个应用本质上是基于平台传感器的计步器，而 Octoscript-OH 有 `sensor::list` / `sample` / `stream`。

在我看来，剩下的十三个确实没有可做的——lint 配置、一个 Android 启动屏、一个 Xcode target、一种 UIKit 技巧、仓库文档、CI 工具、一个已在上游删除的示例。有两个差一点就能做、但我还没做：`simple_sdf` 和 `simple_shader` 需要在 `octoscript-widgets` 中提供一个编译型 MPSL 变体，由 DSL 节点按名字选用；`web_embedding` 可以使用 Octoscript-OH 的 Web 嵌入槽（`webslot::declare`）。请把这个数字理解为"尚未"，而不是"不可能"。

## 视觉 QA

每个界面都在真实设备（OnePlus 6T，Android）上跑过并亲眼看过。`tools/visual-qa.sh` 把每个路由写入应用轮询的一个文件，通过 adb 截取结果，并生成带标注的缩略图拼版：

```sh
cargo makepad android run -p flutter-samples --release   # once
tools/visual-qa.sh                                       # 108 screens
tools/visual-qa.sh cupertino                             # or a subset
DARK=1 tools/visual-qa.sh                                # dark palette
```

这很重要，因为路由遍历测试对这些都视而不见。一个控件从未绑定着色器的界面、一个容器塌缩到零高度的界面、一个 Label 裁掉了自己下伸部的界面，都能完美地完成转换，并通过所有断言。**靠亲眼查看发现了九个没有任何测试捕获到的缺陷：**

| 现象 | 原因 |
|---|---|
| 所有界面空白 | `page()` 在宿主的 `Splash{height: Fit}` 内部要求 `height: Fill` |
| 每个标签的下伸部都被切掉 | 手动挑选的文字高度刚好比字体的行框小一点 |
| 段落在句子中间被截断 | 没有宽度的 Label 不会换行；高度靠猜的 Label 会裁掉换行后的行 |
| 三个选择器全是空白框 | makepad 没有选择器控件，所以 `datepicker`/`timepicker`/`textpicker` 在转换器中落空，变成一个光秃秃的 `View` |
| 聊天气泡只有一像素宽 | `fitw` 列中放了一个 `fillw` 段落——Fill 以 Fit 为基准来解析 |
| 列表行相互重叠 | 行高固定，而其中的文字开始自行测量尺寸 |
| 长标题被截断 | 应用栏和导航栏的标题是单行的 |
| 项目符号短横浮在句子中间 | 行默认会让子元素居中 |
| `icon()` 调用悄悄丢了颜色 | 我自己做的一次错误的机械式编辑——见参数个数测试 |

这些修复都归结为一条规则：**让内容自己测量尺寸**。`txt`、`para` 和 `icon` 不带高度，容纳它们的行是 `fith`，纵向间距来自父元素的 `spacing`/`pad`。现在只有在盒子确实是固定尺寸的地方才使用固定高度——一块 30px 的色块、一张 96px 的缩略图。

### 仍与 Flutter 不同的地方，以及原因

这些是结构性问题，无法通过修改 `.octoscript` 解决：

- **控件是 makepad 的，不是 Material 的。** `CheckBox`、`RadioButton`、`Toggle` 和 `Slider` 由 makepad 自己的 MPSL 着色器绘制。它们能渲染，也能工作，但看起来不像 Material 或 Cupertino。重新设计它们的样式正是 `octoscript-widgets` 的用途——也就是本仓库已经演示过的免 fork 主题化，只是还没有应用到这些组件上。
- **没有水波纹，没有 elevation 着色叠层，没有状态层。** `elevation` 只映射为一个投影。
- **图标是 Font Awesome**，而不是 Material Symbols 或 SF Symbols，因为主题自带的是这套字体。
- **选择器是画出来的，不是原生的**——见 `_kit.octoscript` 中的 `c_wheel`。它们不会滚动。
- **除了两个着色器依据绘制时间运行的 makepad 控件之外，没有任何东西会动。**

## 让点击生效

组件中没有任何东西可以点击，原因是两个互不相干的缺陷，每一个都会悄无声息地吞掉点击。

**1. `View` 会忽略 `on_click`。** `on_click` 是 `Button`、`CheckBox` 和 `GlassPanel` 上的 `ScriptFnRef` 字段，其他地方都没有。行、卡片或列表项上的每个 `tapto` 都会输出一个属性，最终生成的 `View` 解析了它然后直接丢弃。而 `Button` 又不接受子元素，所以可点击区域不能简单地*就是*一个 Button，否则会丢掉图标、两行文字和右侧箭头。

转换器现在会把任何可点击的容器包进一个 `Overlay` 视图，其中包含原始内容，并在上面叠放一个透明、与内容同尺寸的 `Button`——由 Button 负责点击区域和回调，下面的内容保持不变。由 `a_tappable_container_gets_a_button_over_it` 固定下来。

所输出的这个 Button 有两个细节至关重要，都是吃过苦头才发现的：每行一个属性（用逗号连接的形式无法解析），以及**不要覆盖 `draw_bg`**——把 `border_size` 合并进带主题的按钮着色器（它没有这个 instance）会悄无声息地让整个控件失效。

**2. `nav_signal` 必须位于挂载的树内部。** 上游的 `Splash` 挂载在 isolate VM 上，而 makepad 注入到该 VM 中的 `ui` 是*相对于 octoscript 自己的视图根解析的*（`widgets/src/widget_async.rs` 中的 `inject_splash_ui_handle`，对主 VM 会提前返回）。所以处理函数中的 `ui.nav_signal` 永远看不到宿主的 `nav_signal` Label，因为它是 `Splash` 控件的兄弟节点，而不在其内部。`page()` 现在会输出自己的隐藏 `nav_signal`；宿主的控件查找确实会深入挂载的子树，所以仍然能读到它。

这与[唯一一个上游 PR](../../README.zh-CN.md#唯一一个上游-pr) 是同一个 isolate VM 限制。设置 `isolate: false` 之后，这个变通方案就不再是必需的，但无论哪种情况它都仍然正确。

> 给调试这个问题的人的提示：热重载路径只替换 `.octoscript` **数据**。转换器是编译进二进制文件的 Rust 代码，所以对输出方言的修改需要重新构建并重新安装——推送新的组件包是看不到效果的。为此我们花了一个小时去追查一个其实早已正确的修复。

## 这次移植发现的 bug

每个路由都针对只有该界面才会输出的字符串做断言，因为路由器对任何无法识别的路由都会回落到索引页——所以"它渲染出来了"什么也证明不了。这项检查抓到了转换器里的一个真实缺陷，而不是这些文件里的：

> 设置了 `elevation` 的节点会被提升为 `RoundedShadowView`，但 `emit` 是通过把控件名与 `"View"` 或 `"RoundedView"` 匹配来决定是否递归处理子元素的。`RoundedShadowView` 两个都不匹配，所以**每个有高度的容器都悄无声息地丢掉了子元素**——一张带内容的 Material 凸起卡片被渲染成一个空的阴影盒子。

Material 目录从未触发过它，因为那里只在空的色调色块上使用 `elevation`。现在是否输出子元素由节点的*种类*决定，而不是由具体控件决定；`crates/octoscript-makepad/src/lib.rs` 中的 `a_raised_container_keeps_its_children` 把这一点固定了下来。

## 在 HarmonyOS 上运行组件（已尝试，未成功）

`cargo makepad ohos` 是存在的，组件也能一路做到一个已签名、可在 HarmonyOS 手机上安装并运行的 HAP（在 Mate 70 Air 上验证过）。但它除了背景什么都不渲染。这里记录它能走到哪一步、卡在哪里，免得下一次尝试又要重新发现一遍。

**可用，已通过设备日志验证：** XComponent 回调已注册，EGL 上下文和窗口 surface 已创建，vsync 已注册，已进入主循环，surface 为 1320x2523、密度 3.25。`Event::Startup` 触发，组件完成求值（`built=true nodes=236 ui_len=71753`），`Splash` 控件接受了方言并无错误地生成了视图。

**不可用：** 没有字形。`Cx::get_dependency` 会在一个依赖映射表中查找，未命中时回退到平台资源读取。这个映射表在任何平台上都从未被填充——Android 通过其 `to_java_load_asset` 回退满足所有查找——而 OpenHarmony 没有这样的回退，所以每个字体请求都会悄无声息地失败。没有字体数据，文字测量结果为零，每个 `Fit` 容器都会塌缩，窗口只显示它后面的 `Fill` 背景。这一个原因就解释了空白屏幕，也解释了为什么放在宿主界面外框中的一个普通 `Label` 同样不可见。

**在能够编译或打包之前，必须先修复九处构建错误。** 其中四处是同一个错误：OpenHarmony 报告 `target_os = "linux"`，所以桌面 Linux 的代码路径会被选中。

| 构建错误 | 原因 |
|---|---|
| `linux_video_playback` 无法解析 | 调用处的条件是 `linux, not(android)`；模块的条件是 `not(ohos)` |
| `no field opengl_cx`（x2） | `create_gl_render_bridge` 被编译进了 OHOS，而 OHOS 的 `CxOs` 没有自己的 EGL 上下文 |
| 找不到 `-lxkbcommon` | `build.rs` 在 OS 为 `linux` 时链接它；OHOS sysroot 中没有 |
| 找不到 `-lssl` / `-lcrypto` | Linux 网络后端按名字链接 OpenSSL。OHOS 提供的是 `libnet_ssl.so` / `libohcrypto.so`，它们不导出任何 OpenSSL 符号——不只是改了名字 |
| 没有 hilog 写入器 | `log_with_level` 通过函数指针分发；没有任何代码为 OHOS 安装它 |
| `Cx::init_log()` 从未被调用 | 桌面、Android 和 wasm 的入口都会调用它；OHOS 的 napi 入口没有 |
| `signingConfigs: []` | 生成的 DevEco 项目未签名 |
| bundle 名称不匹配 | provisioning profile 绑定到单个 bundle 名称 |
| `PackageHap` 需要 JRE | DevEco 在 `Contents/jbr` 自带了一个 |

在日志能用之前，这个平台完全没有任何输出，导致上面每一种失败都与下一种无法区分。

**还缺少：** `WindowGeomChange` 在 OHOS 上从不触发，所以 `st.vw`/`st.vh` 始终为 0，`page()` 回退为 `Fit`；另外应用沙箱会阻止访问 `/data/local/tmp`，所以宿主的热重载文件和路由覆盖文件在那里无法访问。

这些修复都不在本仓库中——它们是在一个本地 makepad fork 中完成的，且没有提交。请把 OpenHarmony 支持当作一项独立的工作，而不是移植某个示例时顺带的一步。
