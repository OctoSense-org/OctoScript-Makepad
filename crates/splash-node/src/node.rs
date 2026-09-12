//! Backend-agnostic UI node model.
//!
//! The Splash DSL evaluates (in the makepad-script VM) to a tree of plain data
//! objects `{t: "...", <attrs>, c: [...]}`. [`crate::build`] walks that into this
//! `UiNode` tree, which carries **no renderer dependency**. Each backend (ArkUI,
//! makepad, …) turns a `UiNode` tree into its own widgets — that is what makes
//! makepad just *one* render backend rather than *the* renderer.

/// Every node type the DSL can name. A backend maps each to one of its widgets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeKind {
    Column,
    Row,
    Stack,
    Scroll,
    List,
    Grid,
    Waterflow,
    Refresh,
    Swiper,
    Text,
    Image,
    /// Source SVG geometry, rendered with a native vector widget.
    Svg,
    Button,
    Toggle,
    Checkbox,
    Radio,
    Slider,
    Progress,
    Loading,
    Input,
    Textarea,
    DatePicker,
    TimePicker,
    TextPicker,
    /// An OpenStreetMap view — makepad ships a full vector-tile renderer
    /// (`widgets/src/map`, ~12k lines) with rotation and tilt, so a real map is
    /// not a platform view here, it is a widget.
    Map,
    /// The two fragment-shader samples, as compiled MPSL variants in
    /// `splash-widgets`. A DSL node cannot carry shader source — MPSL compiles
    /// at build time — but it can select a shader that was compiled.
    Shader,
    Sdf,
    // ---- Material components -------------------------------------------
    // The reference catalog (Splash-Android) states components *semantically*
    // and lets the renderer produce the real Material widget. These are those
    // nodes; a backend reads `variant` to pick which one of the family it is.
    /// Floating action button — `small` / `regular` / `large` / `extended`.
    Fab,
    /// Icon-only button — `standard` / `filled` / `tonal` / `outlined`.
    IconButton,
    /// A joined single-select strip, choices in `items`, chosen in `selected`.
    Segmented,
    /// `assist` / `filter` / `input` / `suggestion`.
    Chip,
    /// `elevated` / `filled` / `outlined`, optionally checkable.
    Card,
    /// A list row: `label` over `supporting`, one/two/three line by content.
    ListItem,
    /// A rule — `full` / `inset` / `vertical`.
    Divider,
    /// Fixed empty space.
    Spacer,
    /// A row that wraps its children onto further lines.
    Flow,
    /// A shape-scale swatch (`radius`), including the cut-corner family.
    ShapeBox,
    /// A colour-role swatch with its label.
    ColorSwatch,
    /// An icon carrying a count badge.
    BadgeIcon,
    /// The M3 search bar / search view.
    SearchBar,
    /// An exposed dropdown menu.
    Dropdown,
    /// Tab strip — `fixed` / `scrollable`, optionally with icons or badges.
    Tabs,
    /// Bottom navigation bar.
    NavBar,
    /// Navigation rail.
    NavRail,
    /// A carousel strip — `hero` / `multibrowse` / `uncontained` / `fullscreen`.
    Carousel,
    /// A group of radio buttons with one selection.
    RadioGroup,
    /// Two-thumb slider.
    RangeSlider,
    // ---- demo hosts -----------------------------------------------------
    // Composite screens the reference builds as one widget rather than out of
    // parts. They still have to draw something here, or the screens that use
    // them come out blank.
    /// A top app bar — `small` / `medium` / `large` / `center`.
    AppBarDemo,
    /// A bottom app bar with actions and a docked FAB.
    BottomBarDemo,
    /// A docked / floating / vertical toolbar.
    ToolbarDemo,
    /// An adaptive pane layout — `listdetail` / `supporting` / `feed`.
    AdaptiveDemo,
    /// The stage a motion demo animates.
    TransitionHost,
    /// octos-one's own widgets, ported to Android views in the reference.
    WeatherIcon,
    // ---- data visualisations -------------------------------------------------
    // `ui-profile-l0.md` §1.1: six roles are small data visualisations rather
    // than compositions of boxes and text. A gradient temperature bar is a
    // fragment shader parameterised by data, and a DSL node cannot carry shader
    // SOURCE — MPSL compiles at build time — but it can select a shader that was
    // compiled.
    //
    // Adding kinds obliges every backend at once, which is why §1.1 left this
    // unsettled for so long. It is settled this way because the alternative —
    // `Shader` plus a `variant` string — makes the parameters untyped: a
    // temperature bar and a moon phase would ride the same anonymous fields, and
    // nothing would catch a card that passed a latitude where a phase belongs.
    TempBar,
    SunArc,
    MoonPhase,
    AqiContour,
    StockPlot,
    IndicatorPlot,
    NavMap,
    GlassPanel,
    /// A web surface positioned into the native tree. The host reserves the
    /// space and puts a real WebView there — the hybrid Splash-OH's own cards
    /// use, reached from the DSL.
    Web,
}

impl NodeKind {
    /// Parse the DSL `t` tag. Unknown tags yield `None` (the node is dropped).
    pub fn from_tag(tag: &str) -> Option<Self> {
        Some(match tag {
            // The Android backend's catalog — the reference rendering, and the
            // only one authored against real Material widgets — spells these
            // three differently. They are the same nodes, so accept both rather
            // than fork the screens: `col` is by far the most used tag there (88
            // uses), and every one of them was dropped on the floor here.
            "col" => Self::Column,
            "switch" => Self::Toggle,
            "textfield" => Self::Input,
            "column" => Self::Column,
            "row" => Self::Row,
            "stack" => Self::Stack,
            "scroll" => Self::Scroll,
            "list" => Self::List,
            "grid" => Self::Grid,
            "waterflow" => Self::Waterflow,
            "refresh" => Self::Refresh,
            "swiper" => Self::Swiper,
            "text" => Self::Text,
            "image" => Self::Image,
            "svg" => Self::Svg,
            "button" => Self::Button,
            "toggle" => Self::Toggle,
            "checkbox" => Self::Checkbox,
            "radio" => Self::Radio,
            "slider" => Self::Slider,
            "progress" => Self::Progress,
            "loading" => Self::Loading,
            "input" => Self::Input,
            "textarea" => Self::Textarea,
            "datepicker" => Self::DatePicker,
            "timepicker" => Self::TimePicker,
            "textpicker" => Self::TextPicker,
            "map" => Self::Map,
            "shader" => Self::Shader,
            "sdf" => Self::Sdf,
            "fab" => Self::Fab,
            "iconbutton" => Self::IconButton,
            "segmented" => Self::Segmented,
            "chip" => Self::Chip,
            "card" => Self::Card,
            "listitem" => Self::ListItem,
            "divider" => Self::Divider,
            "spacer" => Self::Spacer,
            "flow" => Self::Flow,
            "shapebox" => Self::ShapeBox,
            "colorswatch" => Self::ColorSwatch,
            "badgeicon" => Self::BadgeIcon,
            "searchbar" => Self::SearchBar,
            "dropdown" => Self::Dropdown,
            "tabs" => Self::Tabs,
            "navbar" => Self::NavBar,
            "navrail" => Self::NavRail,
            "carousel" => Self::Carousel,
            "radiogroup" => Self::RadioGroup,
            "rangeslider" => Self::RangeSlider,
            "appbardemo" => Self::AppBarDemo,
            "bottombardemo" => Self::BottomBarDemo,
            "toolbardemo" => Self::ToolbarDemo,
            "adaptivedemo" => Self::AdaptiveDemo,
            "transitionhost" => Self::TransitionHost,
            "weathericon" => Self::WeatherIcon,
            "tempbar" => Self::TempBar,
            "sunarc" => Self::SunArc,
            "moonphase" => Self::MoonPhase,
            "aqicontour" => Self::AqiContour,
            "stockplot" => Self::StockPlot,
            "indicatorplot" => Self::IndicatorPlot,
            "navmap" => Self::NavMap,
            "glasspanel" => Self::GlassPanel,
            "web" => Self::Web,
            _ => return None,
        })
    }

    /// Whether this kind lays its children out along the main axis vertically
    /// (column-like) — a convenience for simple backends.
    pub fn is_vertical_stack(self) -> bool {
        matches!(self, Self::Column | Self::Scroll | Self::List | Self::Card | Self::GlassPanel)
    }
}

/// All attributes a node can carry. Every field is optional; a backend applies
/// the ones it understands and ignores the rest. Colours are `0xAARRGGBB`.
/// `on` / `tap` are left for the backend to resolve against [`NodeKind`] (e.g.
/// `on` means checkbox-select vs toggle-value depending on the kind).
#[derive(Clone, Default, Debug)]
pub struct Attrs {
    /// Validated semantic widget contract: native type and child index paths.
    pub kit: Option<String>,
    pub kit_index: Option<i32>,
    pub text: Option<String>,
    pub label: Option<String>,
    pub placeholder: Option<String>,
    /// Initial keyboard focus and password masking for native input fields.
    pub focused: Option<i32>,
    pub password: Option<i32>,
    /// Makepad widget id (`name := Widget{…}`) so the widget is addressable
    /// (e.g. a signal Label the host reads, or a target of `ui.<id>.set_text`).
    pub id: Option<String>,
    /// Navigate on tap: emits `on_click` that writes the target route into the
    /// `nav_signal` widget, which the host app reads to switch screens.
    pub tapto: Option<String>,
    /// Image source: a resource ref or an `https://` URL.
    pub src: Option<String>,
    /// Decoded raster dimensions, independent of logical layout size.
    pub image_width: Option<f32>,
    pub image_height: Option<f32>,
    /// ObjectFit-style enum for images.
    pub fit: Option<i32>,
    pub w: Option<f32>,
    pub h: Option<f32>,
    /// Force Fit (hug-content) sizing on an axis, overriding the container
    /// default of Fill — for content-sized items like chips and buttons.
    pub fitw: Option<i32>,
    pub fith: Option<i32>,
    /// Force Fill sizing on width even for non-containers (e.g. a full-width
    /// Button used as a navigation list row).
    pub fillw: Option<i32>,
    /// Force Fill sizing on height (e.g. a themed page that must cover the
    /// viewport, not just hug its content).
    pub fillh: Option<i32>,
    pub size: Option<f32>,
    pub weight: Option<i32>,
    /// Render this text in the theme's icon font (Font Awesome) so a codepoint
    /// like `\u{f002}` paints a monochrome Material-style icon, not a colour emoji.
    pub icon: Option<i32>,
    /// The same `icon` key read as a *name* (`"add"`, `"favorite"`, …). The
    /// reference catalog names its icons rather than spelling codepoints, so the
    /// backend resolves the name to a glyph in whatever icon font it has.
    pub icon_name: Option<String>,
    pub color: Option<u32>,
    pub bg: Option<u32>,
    /// The far end of a two-stop gradient fill (makepad's `draw_bg.color_2`).
    /// The reference's shapeable images are named gradients, not bitmaps.
    pub bg2: Option<u32>,
    /// A tiled surface grain laid over this node — the NAME of a bundled
    /// texture (`paper`, `linen`, …), never a path. The theme owns the asset,
    /// so a card names an intent and never a file, exactly as it does for
    /// colour. L0 refuses a literal in a data position (profile §4), and a
    /// texture is no more the card's to choose than a hex value is.
    ///
    /// The assets are greyscale by construction: a texture carrying its own hue
    /// would fight whatever accent the card asked for — the same defect as an
    /// accent that never reached the ink.
    pub texture: Option<String>,
    /// The shadow's INK. Absent means the derived soft shadow that `elevation`
    /// alone produces; present means the theme chose, which is the difference
    /// between a Material lift and a neubrutalist offset block.
    ///
    /// `elevation` was always able to say how FAR a surface sits off the page.
    /// It could never say what the shadow is made of, so every school that
    /// wants a hard coloured drop — memphis, neubrutalist, punk — was
    /// unreachable no matter what elevation it asked for.
    pub shadowcolor: Option<u32>,
    /// Blur radius. Zero is a hard edge; large with no offset is a glow.
    pub shadowblur: Option<f32>,
    /// Offset. A hard shadow is defined by having one; a glow by having none.
    pub shadowdx: Option<f32>,
    pub shadowdy: Option<f32>,
    /// The type FAMILY this run is drawn in — `"sans"` (the default) or
    /// `"serif"`. A role, not a file: the theme picks the family and the
    /// backend owns which face answers it at each weight, exactly as the
    /// backend already owns Roboto-Thin versus Roboto-Bold.
    pub family: Option<String>,
    /// Explicit font resource and line height for source-measured designs.
    pub font_src: Option<String>,
    pub font_asc: Option<f32>,
    pub font_desc: Option<f32>,
    pub line_height: Option<f32>,
    /// Letter spacing, in ems. The text stack has always had `letter_spacing`
    /// in its shaper; nothing above ever reached it, so the eyebrow role fakes
    /// tracking by inserting thin spaces into the STRING — which cannot work on
    /// a live value, because its text does not exist at lowering time.
    pub tracking: Option<f32>,
    /// Run the two-stop fill ACROSS rather than down.
    ///
    /// `gradient_fill_horizontal` has been a uniform on every view shader all
    /// along and nothing has ever set it, so every gradient in the product ran
    /// top-to-bottom because that is the branch the default takes — not because
    /// anything chose it. A second direction is the cheapest axis in the whole
    /// list: no shader work, one uniform.
    pub gradient_across: Option<i32>,



    /// How strongly the grain reads, 0..1. Small, because this is a surface and
    /// not a picture.
    pub texture_alpha: Option<f32>,
    /// How many times the tile repeats across the node. Larger is finer.
    pub texture_scale: Option<f32>,
    pub radius: Option<f32>,
    /// Material elevation (dp). Non-zero promotes a filled container to a
    /// shadow-casting view and scales its drop shadow.
    pub elevation: Option<f32>,
    pub pad: Option<f32>,
    /// Asymmetric padding: horizontal (`padx`) / vertical (`pady`), each
    /// overriding `pad` on its axis — for M3 insets like a button's 24dp
    /// horizontal / 6dp vertical padding that a uniform `pad` can't express.
    pub padx: Option<f32>,
    pub pady: Option<f32>,
    /// Explicit leading inset for source-measured native text fields.
    pub padleft: Option<f32>,
    /// Asymmetric vertical padding, where `pady` cannot say it.
    ///
    /// A page's top padding clears the status bar and its bottom clears the
    /// gesture bar, and those are different numbers on every device. Expressed
    /// as one symmetric `pady` the page sat 30px too high, which shifted every
    /// row below it and read as a layout bug rather than as a missing inset.
    pub padtop: Option<f32>,
    pub padbottom: Option<f32>,
    pub spacing: Option<f32>,
    pub margin: Option<f32>,
    /// Per-axis margin, overriding `margin` on its axis. Every section heading in
    /// the reference carries `marginy: 4`; dropping it made each screen sit a
    /// little tighter than the reference and drift further down the page.
    pub marginx: Option<f32>,
    pub marginy: Option<f32>,
    /// Asymmetric vertical margin, for the same reason as `padtop`.
    ///
    /// A panel separates itself from what is ABOVE it; repeating that below
    /// doubles the gap between two stacked panels and leaves a dead strip under
    /// the last one.
    pub margintop: Option<f32>,
    pub marginbottom: Option<f32>,
    pub border: Option<f32>,
    pub bordercolor: Option<u32>,
    /// Which Material variant this node is — `filled`/`tonal`/`outlined`/`text`/
    /// `elevated` on a button, `small`/`large`/`extended` on a FAB, the type role
    /// on a text node. The reference catalog (Splash-Android, rendered with real
    /// `com.google.android.material.*` views) states components *semantically*
    /// this way rather than drawing look-alikes, and it is the single most used
    /// attribute there. Without it every variant collapses to one appearance.
    pub variant: Option<String>,
    /// Disabled state — the reference devotes a whole section per component to it.
    pub enabled: Option<i32>,
    /// The state slot this widget reads and writes. The reference screens are
    /// state-driven: a widget event writes `key`, the DSL is re-evaluated, and
    /// the new tree rebuilds the views — the DSL, not the host, decides what the
    /// screen says. Without it a catalog is a picture rather than a demo.
    pub key: Option<String>,
    /// What this widget asks the host to do when tapped — `"alert"`, `"modal"`,
    /// `"range"`. The reference writes it into `key`, and the host turns it into
    /// a real dialog / sheet / picker. Paired with [`Attrs::key`].
    pub action: Option<String>,
    /// `;`-separated choices, for segmented buttons and tab strips.
    pub items: Option<String>,
    /// Index of the chosen item in `items`.
    pub selected: Option<i32>,
    /// A field's floating label, and the supporting/error text beneath it.
    pub hint: Option<String>,
    /// A card's headline, and a list row's line count when content alone is
    /// ambiguous (M3: 56 / 72 / 88dp for one / two / three lines).
    pub title: Option<String>,
    pub lines: Option<i32>,
    /// A badge's contents; `count` is the numeric form.
    pub badge: Option<String>,
    pub count: Option<i32>,
    pub supporting: Option<String>,
    /// A field's supporting line (the reference spells it `helper`).
    pub helper: Option<String>,
    /// A checkbox in its third state — neither on nor off.
    pub indeterminate: Option<i32>,
    /// The Material role a swatch names (`"primary"`, `"onSurfaceVariant"`).
    pub group: Option<String>,
    /// A slider's range and step, and a range slider's second thumb.
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub step: Option<f32>,
    pub value2: Option<f32>,
    pub error: Option<String>,
    /// A native control's selected/active role (M3 primary): the checked box, the
    /// radio dot, the switch's on-track, the slider's value track and handle, a
    /// focused field's outline. Distinct from `bg`, which stays the *container*
    /// (an unchecked track, a filled field) as it does on every other node.
    ///
    /// Controls need this because they are drawn by their own shaders, whose
    /// colours are not reachable from `bg`/`color` alone. The theming a widget
    /// kit registers on the app VM never reaches them either: `Splash` mounts its
    /// body on an isolate VM that only ever gets makepad's own `script_mod`
    /// (`widget_async.rs`), so the kit's variants are simply absent there. What
    /// does arrive is whatever the mounted dialect string carries — verified on
    /// device — so a control states its Material roles per instance.
    pub accent: Option<u32>,
    /// The ink drawn *on* `accent` (M3 on-primary): a checkbox's tick, a switch's
    /// thumb when on.
    pub markcolor: Option<u32>,
    pub value: Option<f32>,
    /// Background blur radius in logical pixels for measured native designs.
    pub blur: Option<f32>,
    pub total: Option<f32>,
    pub align: Option<i32>,
    /// Child alignment within a container, 0.0..=1.0 on each axis.
    pub alignx: Option<f32>,
    pub aligny: Option<f32>,
    /// The ink descendant text is read with, where the mood's own ink cannot
    /// be read on this node's fill.
    ///
    /// A card's fill belongs to the PACK — CaMo's black slab, Atro's indigo
    /// gradient — and it does not flip when the pack's light variant flips
    /// `l0_text`; the theme cannot re-answer a role per subtree. CaMo light
    /// therefore drew near-black headlines on a pure black card, on every
    /// rail. This was an `inkdark` FLAG, which no renderer ever read and which
    /// the evaluator never even parsed, so it fixed nothing. Carrying the
    /// colour lets a backend apply it, and apply it only where a descendant's
    /// own colour fails contrast — an accent that already reads keeps its own.
    pub ink: Option<u32>,
    pub on: Option<i32>,
    pub tap: Option<i32>,
    /// Map camera. `tilt` is what makes the view 2.5D; `rotation` is the bearing.
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub zoom: Option<f64>,
    pub tilt: Option<f64>,
    pub rotation: Option<f64>,
    /// The route a map draws, as an encoded polyline5.
    ///
    /// Geometry rather than a query, because the tree carries values and not
    /// requests: whoever built this node already resolved the route, and a
    /// backend that re-fetched from an origin and destination would fetch again
    /// on every rebuild. Which member of the map family this is — route preview,
    /// chase camera, flat — travels in `variant`, like every other family.
    /// Where a field sends what has been typed SO FAR, per keystroke.
    ///
    /// Separate from `tapto`, which a field uses for its commit: the two carry
    /// different events and fire at different moments. A search box wants both —
    /// results while you type, a destination when you press return.
    pub changeto: Option<String>,
    pub polyline: Option<String>,
    /// The pins a map stands on its route: `"lat,lon,kind;…"`, kind 0 origin,
    /// 1 an intermediate stop, 2 the destination.
    ///
    /// A VALUE for the same reason `polyline` is one. The endpoints were already
    /// resolved to draw the route, so a backend that re-derived them would resolve
    /// the same places twice and could disagree with the line on screen.
    pub markers: Option<String>,
    /// What the drawn route COSTS, labelled on the path: two lines separated by a
    /// pipe, in practice a duration over a distance. On the route rather than in a
    /// sheet, because that is where it answers the question being asked of it.
    ///
    /// `route_badge`, not `badge`: this struct already has one, for the count on a
    /// component, and a route's cost is not that. Two meanings under one name is how
    /// a field ends up carrying whichever the last writer meant.
    pub route_badge: Option<String>,
    /// Absolute position for a surface the host composites (a web slot). The
    /// tree does not know where a node lands, so a screen that wants one says.
    pub x: Option<f64>,
    pub y: Option<f64>,

    // ---- data-visualisation parameters ---------------------------------------
    // Named, not generic. A `Shader` kind with anonymous slots would let a card
    // pass a latitude where a moon phase belongs and nothing would notice.
    // `min`/`max` (a bar's range) and `lat`/`lon` (a contour's centre) already
    // exist above and mean exactly this, so they are reused rather than doubled.
    /// TempBar: the day's low and high, against `min`/`max` for the week.
    pub lo: Option<f32>,
    pub hi: Option<f32>,
    /// SunArc: sunrise, sunset and now, as fractional hours.
    pub rise: Option<f32>,
    pub set: Option<f32>,
    pub now: Option<f32>,
    /// MoonPhase: 0..1 through the cycle, and percent illuminated.
    pub phase: Option<f32>,
    pub illum: Option<f32>,
    /// AqiContour: degrees of latitude the field covers, around `lat`/`lon`.
    pub span: Option<f32>,
    /// StockPlot: which series, and over what window.
    pub symbol: Option<String>,
    pub range: Option<String>,
    /// IndicatorPlot: which countries (ISO3, comma-separated, in the order the
    /// card named them), which World Bank indicator, and how many years back.
    pub countries: Option<String>,
    pub indicator: Option<String>,
    pub years: Option<f32>,
}

/// One node in the backend-agnostic tree.
#[derive(Clone, Debug)]
pub struct UiNode {
    pub kind: NodeKind,
    pub attrs: Attrs,
    pub children: Vec<UiNode>,
}

impl UiNode {
    /// Total node count including self — handy for tests and diagnostics.
    pub fn count(&self) -> usize {
        1 + self.children.iter().map(UiNode::count).sum::<usize>()
    }
}
