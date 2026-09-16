//! flutter-samples — the flutter/samples catalog as native makepad widgets.
//!
//! Same shell as `kit-host`, pointed at `components/flutter/` instead of the
//! Material kit: assemble the kit, inject `st`, run the Octoscript pipeline, feed
//! makepad's dialect into the mounted `Splash` widget.
//!
//! Two pieces of state, because that is all the ports need: the current route
//! and the light/dark flag. A screen navigates by setting `tapto`, which emits
//! an `on_click` writing the target into `nav_signal`; this app reads that
//! label each frame and re-mounts.
//!
//! Hot reload: dropping an assembled kit at `DEVICE_PATH` overrides the baked
//! one, so screens can be edited without a rebuild. Assemble one with
//! `cargo run -p octoscript-makepad --example assemble -- components/flutter`.
//!
//! The kit is baked by `include_str!` rather than generated into `OUT_DIR`,
//! because `cargo-makepad` compiles the app inside a generated wrapper crate
//! that never runs this crate's build script — `OUT_DIR` is undefined there and
//! the Android build fails to compile. A relative `include_str!` resolves
//! against the source file, so it works identically on desktop and on device.
//! `baked_kit_matches_the_directory` below fails if this list drifts from
//! `components/flutter/`.

pub use makepad_widgets;
use makepad_widgets::*;

app_main!(App);

/// One `.octoscript` per flutter/samples directory, in the order
/// [`octoscript_makepad::kit`] fixes: `_kit.octoscript` first (tokens and helpers),
/// the samples sorted, `_index.octoscript` last (the index and the router).
macro_rules! kit {
    ($($name:literal),* $(,)?) => {
        concat!($(include_str!(concat!("../../../components/flutter/", $name, ".octoscript")), "\n"),*)
    };
}

const BAKED: &str = kit![
    "_kit",
    "add_to_app",
    "analysis_defaults",
    "android_splash_screen",
    "animations",
    "asset_transformation",
    "background_isolate_channels",
    "compass_app",
    "cupertino_gallery",
    "date_planner",
    "desktop_photo_search",
    "docs",
    "dynamic_theme",
    "form_app",
    "google_maps",
    "ios_app_clip",
    "material_3_demo",
    "navigation_and_routing",
    "pedometer",
    "platform_channels",
    "platform_design",
    "platform_view_swift",
    "simple_sdf",
    "simple_shader",
    "testing_app",
    "tool",
    "veggieseasons",
    "web_embedding",
    "_index",
];

const DEVICE_PATH: &str = "/data/local/tmp/flutter_samples.octoscript";

/// A route written here is picked up within a frame or two and mounted. Exists
/// so the visual-QA sweep can drive all 108 screens on a real device without
/// tapping through them:
///
/// ```text
/// adb shell "echo compass_app/booking > /data/local/tmp/flutter_samples.route"
/// adb exec-out screencap -p > booking.png
/// ```
///
/// Appending ` dark` to the line switches the palette for that shot.
const ROUTE_PATH: &str = "/data/local/tmp/flutter_samples.route";

/// Where `mount` reports what it did, for when there is no log to read.
///
/// On OpenHarmony the app produces no hilog output — `log!` does not reach it —
/// so a mount that silently produces nothing is undiagnosable from the host
/// side. The app already reads two files from this directory, so it can write
/// one: `hdc shell cat` is then the log.
const DIAG_PATH: &str = "/data/local/tmp/flutter_samples.diag";

script_mod! {
    use mod.prelude.widgets.*

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.inner_size: vec2(440, 1400)
                body +: {
                    flow: Down
                    ScrollYView{
                        width: Fill
                        height: Fill
                        flow: Down
                        show_bg: true
                        draw_bg +: { color: #fef7ffff }
                        // Upstream `Splash` always allocates an isolate VM; the
                        // light theme and the shared heap live on the app's main
                        // VM. See the repo README on the one upstream PR.
                        // Fit, not Fill — and this is load-bearing.
                        //
                        // The kit's `{t: "scroll"}` emits a plain View on this
                        // backend, so this ScrollYView is the only scrolling in
                        // the app. A Fill child would exactly match it and
                        // never scroll. See `page()` in `_kit.octoscript` for the
                        // measurement, and why mapping Scroll to ScrollYView
                        // makes it worse rather than better.
                        host := Splash{ width: Fill, height: Fit }
                    }
                    // PROBE: a plain makepad Label, drawn by the host rather
                    // than through the Splash mount. If this is visible on
                    // OpenHarmony and the kit is not, the mount is at fault; if
                    // neither is, makepad's text/shader path is not working on
                    // that platform at all. Remove once answered.
                    probe := Label{
                        text: "PROBE host label"
                        draw_text.text_style.font_size: 24
                        draw_text.color: #ff0000ff
                    }
                    // The routing signal the mounted kit writes.
                    nav_signal := Label{ text: "" height: 0 draw_text.text_style.font_size: 1 }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    next_frame: NextFrame,
    #[rust]
    last_src: String,
    #[rust]
    route: String,
    #[rust]
    dark: bool,
    /// Viewport in vp, from WindowGeomChange. 0 until the first one arrives.
    #[rust]
    vw: f64,
    #[rust]
    vh: f64,
    #[rust]
    tick: u32,
    #[rust]
    started: bool,
    #[rust]
    last_route_file: String,
    /// Seconds since startup, fed to the kit as `st.t`.
    #[rust]
    clock: f64,
    /// SELF-DRIVE: commands loaded from /data/local/tmp/fs_cmd.txt — the phone
    /// taps and captures ITSELF; no adb input, no adb screencap. `tap X Y`
    /// (physical px), `shot <path.png>`, `wait <ticks>`.
    #[rust]
    drive_cmds: Vec<String>,
    #[rust]
    drive_at: usize,
    #[rust]
    drive_delay: u32,
    /// A Stop touch waiting to follow its Start.
    #[rust]
    drive_stop: Option<(f64, f64, u64)>,
    #[rust]
    drive_uid: u64,
    #[rust]
    drive_last_file: String,
    /// Frames remaining on an active capture (env var set).
    #[rust]
    drive_shot: u32,
}

/// One synthetic touch through the REAL input pipeline: the same channel and
/// coalescing the JNI layer uses, physical coords, converted by the same
/// handler. The phone touching its own screen.
#[cfg(target_os = "android")]
fn self_tap(x: f64, y: f64, uid: u64, down: bool) {
    use makepad_widgets::makepad_platform::os::linux::android::android_jni;
    use makepad_widgets::makepad_platform::event::{TouchPoint, TouchState};
    use makepad_widgets::makepad_platform::Area;
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0);
    let touch = TouchPoint {
        state: if down { TouchState::Start } else { TouchState::Stop },
        abs: makepad_widgets::makepad_platform::math_f64::dvec2(x, y),
        time: t,
        uid,
        rotation_angle: 0.0,
        force: 1.0,
        radius: makepad_widgets::makepad_platform::math_f64::dvec2(1.0, 1.0),
        handled: std::cell::Cell::new(Area::Empty),
        sweep_lock: std::cell::Cell::new(Area::Empty),
    };
    android_jni::send_from_java_message(android_jni::FromJavaMessage::Touch(vec![touch]));
}
#[cfg(not(target_os = "android"))]
fn self_tap(_x: f64, _y: f64, _uid: u64, _down: bool) {}

impl App {
    fn current_source() -> String {
        std::fs::read_to_string(DEVICE_PATH).unwrap_or_else(|_| BAKED.to_string())
    }

    /// The QA route override, if one has been written. `<route>[ dark]`.
    fn route_override() -> Option<String> {
        let raw = std::fs::read_to_string(ROUTE_PATH).ok()?;
        let line = raw.trim();
        if line.is_empty() {
            return None;
        }
        Some(line.to_string())
    }

    /// Translate + mount the active route.
    fn mount(&mut self, cx: &mut Cx) {
        let src = Self::current_source();
        self.last_src = src.clone();
        let route = if self.route.is_empty() {
            "index"
        } else {
            &self.route
        };
        let full = octoscript_makepad::kit::with_state_sized(
            route, self.dark, self.clock, self.vw, self.vh, &src,
        );
        let built = octoscript_render::build(&full, octoscript_makepad::kit::register_stub_capabilities);
        let mut diag = format!(
            "route={route} src_len={} vw={} vh={} built={}",
            src.len(),
            self.vw,
            self.vh,
            built.is_some()
        );
        if let Some(node) = built {
            let ui = octoscript_makepad::to_makepad_ui(&node);
            diag.push_str(&format!(" nodes={} ui_len={}", node.count(), ui.len()));
            // On the APP VM, with THIS crate as the module identity — not as
            // text into the `Splash` isolate. Two reasons, both found blank on
            // a real 6T and both already documented in kit-host's mount:
            //   · `crate_resource("self:…")` resolves against the module that
            //     evaluated the body. On the isolate that is makepad_widgets,
            //     which ships no Roboto — so every text node silently dropped
            //     while the chips and sliders drew fine.
            //   · the emitted `on_click` calls the `NAV` global, which only
            //     the app VM registers — on the isolate every tap was dead.
            let code = format!("use mod.prelude.widgets.*\nView{{height:Fit, {ui}");
            let script_mod = ScriptMod {
                cargo_manifest_path: env!("CARGO_MANIFEST_DIR").to_string(),
                module_path: module_path!().to_string(),
                file: file!().to_string(),
                line: 1,
                column: 0,
                code: String::new(),
                values: Vec::new(),
            };
            let built_view = cx.with_vm(|vm| {
                let value = vm.eval_with_append_source(script_mod, &code, octoscript_render::makepad_script::ScriptValue::NIL.into());
                (!value.is_err() && !value.is_nil()).then(|| View::script_from_value(vm, value))
            });
            if let Some(view) = built_view {
                if let Some(mut host) = self.ui.widget(cx, ids!(host)).borrow_mut::<Splash>() {
                    host.view = view;
                }
            }
        }
        // `log!`, not a file write: the OHOS app sandbox blocks /data/local/tmp,
        // so the earlier file-based diagnostic produced nothing.
        log!("MOUNT {}", diag);
    }
}

/// Taps land here. The emitted body's `on_click` calls `NAV(t: "…")` — the
/// same channel kit-host uses, because the body now evaluates on the APP VM
/// (see `mount`), where `ui.nav_signal` does not exist.
static TAPS: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn take_tap() -> Option<String> {
    TAPS.lock().ok().and_then(|mut q| if q.is_empty() { None } else { Some(q.remove(0)) })
}

fn register_nav(vm: &mut ScriptVm) {
    let f_nav = octoscript_render::add_global_fn(
        vm,
        &[(live_id!(t), octoscript_render::makepad_script::ScriptValue::NIL)],
        |vm, a| {
            let t = octoscript_render::string_prop(vm, a, live_id!(t)).unwrap_or_default();
            if let Ok(mut q) = TAPS.lock() {
                q.push(t);
            }
            octoscript_render::makepad_script::ScriptValue::NIL
        },
    );
    vm.set_injected_global(live_id!(NAV), f_nav);
}

impl MatchEvent for App {}

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        crate::makepad_widgets::theme_mod(vm);
        script_eval!(vm, {
            mod.theme = mod.themes.light
        });
        // Fork-free themed widgets, against upstream makepad.
        octoscript_widgets::widgets_mod(vm);
        // The body mounts on THIS VM now (see `mount`), so its tap handlers
        // need `NAV` here — unregistered, every tap raises "variable NAV not
        // found" and dies silently.
        register_nav(vm);
        self::script_mod(vm)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        // The viewport, so a page can name its own height. `Splash` wraps its
        // mount in `View{height:Fit}`, so filling is not available — see
        // `with_state_sized`. inner_size is physical; vp is what the kit's
        // lengths are in.
        if let Event::WindowGeomChange(e) = event {
            let g = &e.new_geom;
            // `inner_size` is already in vp — measured on the 6T: 384x787.5
            // against a 1080x2340 screen at dpi_factor 2.8125, and
            // 384 * 2.8125 = 1080 exactly. Dividing by dpi_factor again gave
            // 136x280, which is why the first attempt at a self-sized page came
            // out about a third of the screen with its content clipped.
            let (vw, vh) = (g.inner_size.x, g.inner_size.y);
            if (vw - self.vw).abs() > 0.5 || (vh - self.vh).abs() > 0.5 {
                self.vw = vw;
                self.vh = vh;
                self.last_src.clear();
                self.mount(cx);
            }
        }
        if matches!(event, Event::Startup) {
            log!("STARTUP seen, started={}", self.started);
        }
        if matches!(event, Event::Startup) && !self.started {
            self.started = true;
            // Start on a named screen instead of the index, so one can be opened
            // directly for a look (desktop only — on device, tap through):
            //   OCTOSCRIPT_ROUTE=date_planner/maya cargo run -p flutter-samples
            self.route = std::env::var("OCTOSCRIPT_ROUTE").unwrap_or_else(|_| "index".to_string());
            self.next_frame = cx.new_next_frame();
            self.mount(cx);
        }
        if self.next_frame.is_event(event).is_some() {
            self.tick = self.tick.wrapping_add(1);
            // SELF-DRIVE executor: one command at a time, frame-paced.
            if let Some((x, y, uid)) = self.drive_stop.take() {
                self_tap(x, y, uid, false);
            } else if self.drive_shot > 0 {
                self.drive_shot -= 1;
                if self.drive_shot == 0 {
                    std::env::remove_var("MAKEPAD_WRITE_FRAMEBUFFER_PNG");
                }
            } else if self.drive_delay > 0 {
                self.drive_delay -= 1;
            } else if self.drive_at < self.drive_cmds.len() {
                let cmd = self.drive_cmds[self.drive_at].clone();
                self.drive_at += 1;
                let parts: Vec<&str> = cmd.split_whitespace().collect();
                match parts.as_slice() {
                    ["tap", x, y] => {
                        if let (Ok(x), Ok(y)) = (x.parse::<f64>(), y.parse::<f64>()) {
                            self.drive_uid += 1;
                            self_tap(x, y, self.drive_uid, true);
                            self.drive_stop = Some((x, y, self.drive_uid));
                            self.drive_delay = 30;
                        }
                    }
                    ["shot", path] => {
                        std::env::set_var("MAKEPAD_WRITE_FRAMEBUFFER_PNG", path);
                        self.drive_shot = 3;
                        self.drive_delay = 6;
                    }
                    ["wait", n] => self.drive_delay = n.parse().unwrap_or(30),
                    _ => {}
                }
                let done = self.drive_at >= self.drive_cmds.len();
                let _ = std::fs::write(
                    "/storage/emulated/0/Android/data/dev.makepad.flutter_samples/files/fs_cmd_done.txt",
                    format!("{} {}/{}{}\n", cmd, self.drive_at, self.drive_cmds.len(),
                            if done { " DONE" } else { "" }),
                );
                if done { log!("SELFDRIVE done"); }
            }
            let nav_raw = take_tap().unwrap_or_default();
            let nav = nav_raw.trim();
            if !nav.is_empty() {
                if nav == "theme:toggle" {
                    self.dark = !self.dark;
                } else if octoscript_render::state::apply(nav) {
                    // A control, not a link. The kit names these `set:key=!`,
                    // `set:key=+1`, `set:key=~n`, and `state::apply` is the same
                    // parser the ArkUI backend uses, so a checkbox behaves
                    // identically on both.
                    //
                    // Without this branch every control target fell through to
                    // the route case below, matched no screen, and did nothing —
                    // so on makepad no checkbox, toggle, radio, slider, chip or
                    // stepper in the kit had ever worked. The state store was
                    // only ever driven from the ArkUI side.
                } else {
                    self.route = nav.to_string();
                }
                self.mount(cx);
            } else if self.tick % 10 == 0 {
                // SELF-DRIVE: load a new command file (mtime-independent — the
                // file's CONTENT is the identity; clear it to re-arm).
                if let Ok(txt) = std::fs::read_to_string("/data/local/tmp/fs_cmd.txt") {
                    if !txt.trim().is_empty() && txt != self.drive_last_file {
                        self.drive_last_file = txt.clone();
                        self.drive_cmds =
                            txt.lines().map(|l| l.trim().to_string()).filter(|l| !l.is_empty()).collect();
                        self.drive_at = 0;
                        self.drive_delay = 0;
                        let _ = std::fs::write("/storage/emulated/0/Android/data/dev.makepad.flutter_samples/files/fs_cmd_done.txt", "LOADED\n");
                        log!("SELFDRIVE loaded {} cmds", self.drive_cmds.len());
                    }
                }
                // A route written to ROUTE_PATH wins over whatever was tapped,
                // so the QA sweep can drive every screen from adb. Checked
                // independently of the source, so pushing an edited kit still
                // hot-reloads while a route override is in place — otherwise the
                // QA loop would need a reinstall for every screen tweak.
                let mut remount = false;
                match Self::route_override() {
                    Some(line) => {
                        if line != self.last_route_file {
                            self.last_route_file = line.clone();
                            let (route, dark) = match line.strip_suffix(" dark") {
                                Some(r) => (r.trim().to_string(), true),
                                None => (line, false),
                            };
                            self.route = route;
                            self.dark = dark;
                            remount = true;
                        }
                    }
                    // Removing the file forgets the last route, so writing the
                    // same route again re-mounts. Without this the QA sweep
                    // silently skipped any screen that matched the previous
                    // run's last route — it photographed whatever was on screen.
                    None => self.last_route_file.clear(),
                }
                if !remount && Self::current_source() != self.last_src {
                    remount = true;
                }
                if remount {
                    self.mount(cx);
                }
            }
            // Animation: a screen can only move if the tree is recomputed
            // against a changing value, so advance the clock and re-mount while
            // an animated route is on screen. Everything else stays static and
            // costs nothing.
            if self.route.starts_with("animations/") {
                self.clock += 1.0 / 60.0;
                self.mount(cx);
            }
            cx.redraw_all();
            self.next_frame = cx.new_next_frame();
        }
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

#[cfg(test)]
mod tests {
    use super::BAKED;
    use std::path::PathBuf;

    /// The baked list is spelled out above because the Android wrapper crate
    /// cannot run a build script. That means it can drift: add a `.octoscript` to
    /// `components/flutter/` and forget this list, and the screen is missing
    /// from the app while every test in `octoscript-makepad` still passes, because
    /// those assemble from the directory. This pins the two together.
    #[test]
    fn baked_kit_matches_the_directory() {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../components/flutter");
        let assembled = octoscript_makepad::kit::concat_kit(&dir).expect("kit assembles");
        // `concat_kit` interleaves `// ---- <file>` markers; the baked const has
        // none. Strip them and the two must be byte-identical.
        let stripped: String = assembled
            .lines()
            .filter(|l| !(l.starts_with("// ---- ") && l.ends_with(".octoscript")))
            .map(|l| format!("{l}\n"))
            .collect();
        assert_eq!(
            stripped, BAKED,
            "the baked kit in main.rs has drifted from components/flutter/"
        );
    }
}
