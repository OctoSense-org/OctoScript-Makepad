//! Native, chrome-free preview of the lab's L0 cards through splash-makepad.
pub use makepad_widgets;
use makepad_widgets::*;
use splash_widgets::design::*;
mod beauty_semantics;
use beauty_semantics::Session as SemanticSession;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    startup() do #(App::script_component(vm)) {
        ui: Root {
            main_window := Window {
                show_caption_bar: false
                window.inner_size: vec2(393, 852)
                body +: {
                    flow: Overlay
                    host := Splash { width: Fill height: Fill }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    // Keep the previous subtree's GPU resources alive through the replacement
    // draw. Its old draw lists may still be consumed by an in-flight frame.
    #[rust]
    retired_view: Option<View>,
    #[rust]
    timer: Timer,
    #[rust]
    last_request: String,
    #[rust]
    inspection_ids: Vec<String>,
    #[rust]
    layout_path: String,
    #[rust]
    pending_focus: Option<String>,
    #[rust]
    action_log: String,
    #[rust]
    semantic_actions: Vec<serde_json::Value>,
    #[rust]
    semantic: SemanticSession,
}

impl App {
    fn mount_request(&mut self, cx: &mut Cx) -> Result<(), String> {
        let path = std::env::var("BEAUTY_REQUEST").map_err(|e| e.to_string())?;
        let request = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        if request == self.last_request {
            return Ok(());
        }
        // A failed request is recorded too, so a bad input cannot spam a log.
        self.last_request = request.clone();
        let r: serde_json::Value = serde_json::from_str(&request).map_err(|e| e.to_string())?;
        let read = |key: &str| -> Result<String, String> {
            std::fs::read_to_string(r[key].as_str().ok_or_else(|| format!("missing {key}"))?)
                .map_err(|e| e.to_string())
        };
        let card = read("card")?;
        let first_mount = self.inspection_ids.is_empty();
        let data = serde_json::from_str(&read("data")?).map_err(|e| e.to_string())?;
        let mut measured = r["format"].as_str() == Some("design");
        let mut tree = if measured {
            splash_makepad::design::prepare(&card)?
        } else {
            let prepared = splash_makepad::l0::prepare(
                &card,
                &data,
                std::path::Path::new(r["kit_dir"].as_str().ok_or("missing kit_dir")?),
            )?;
            measured = prepared.native_components;
            prepared.tree
        };
        let elements = splash_makepad::l0::inspectable(&mut tree);
        let focused: Vec<_> = elements
            .iter()
            .filter(|e| e["focused"].as_i64() == Some(1))
            .collect();
        if focused.len() > 1 {
            return Err("a design cannot focus multiple native inputs".into());
        }
        self.pending_focus = focused
            .first()
            .and_then(|e| e["id"].as_str().map(str::to_owned));
        cx.set_key_focus(Area::Empty);
        self.inspection_ids = elements
            .iter()
            .filter_map(|e| e["id"].as_str().map(str::to_owned))
            .collect();
        self.layout_path = r["layout"].as_str().unwrap_or_default().to_owned();
        self.action_log = r["actions"].as_str().unwrap_or_default().to_owned();
        self.semantic_actions.clear();
        if !self.action_log.is_empty() {std::fs::write(&self.action_log,"[]").map_err(|e|e.to_string())?;}
        let ui = if measured {
            splash_makepad::design::to_makepad_ui(&tree)?
        } else {
            splash_makepad::to_makepad_l0_ui(&tree)
        };
        let code = format!(
            "use mod.prelude.widgets.*\nlet root = View{{width:Fill height:Fill flow:Overlay {ui}}}\nroot"
        );
        let sm = ScriptMod {
            cargo_manifest_path: env!("CARGO_MANIFEST_DIR").into(),
            module_path: module_path!().into(),
            file: file!().into(),
            line: 1,
            column: 0,
            code,
            values: Vec::new(),
        };
        let view = cx.with_vm(|vm| {
            let value = vm
                .eval_checked(sm, 2_000_000)
                .ok_or("native VM rejected the widget tree")?;
            Ok::<_, String>(View::script_from_value(vm, value))
        })?;
        let width = r["width"].as_f64().ok_or("missing width")?;
        let height = r["height"].as_f64().ok_or("missing height")?;
        self.ui.window(cx, ids!(main_window)).configure_window(
            cx,
            dvec2(width, height),
            dvec2(80.0, 80.0),
            false,
            "Splash beauty preview".into(),
        );
        if !first_mount {
            self.ui
                .window(cx, ids!(main_window))
                .resize(cx, dvec2(width, height));
        }
        let host = self.ui.widget(cx, ids!(host));
        let mut host = host
            .borrow_mut::<Splash>()
            .ok_or("missing native Splash mount")?;
        cx.set_key_focus(Area::Empty);
        self.retired_view=Some(std::mem::replace(&mut host.view,view));
        // Retain GPU resources through the replacement frame, but detach their
        // overlay drawing immediately. Nested old overlays can otherwise keep
        // the same stale parent redraw id and survive into the next screen.
        fn retire_overlay(cx:&mut Cx,widget:WidgetRef) {
            splash_widgets::kit::retire_overlay(cx,&widget);
            let list=widget.borrow::<DesignOverlay>().and_then(|v|v.draw_list.as_ref().map(|l|l.id()))
                .or_else(||widget.borrow::<DesignGlassSvg>().and_then(|v|v.draw_list.as_ref().map(|l|l.id())));
            if let Some(id)=list {cx.draw_lists[id].clear_draw_items(cx.redraw_id);}
            let mut children=Vec::new();
            widget.children(&mut |_,child|children.push(child));
            for child in children {retire_overlay(cx,child);}
        }
        if let Some(retired)=&self.retired_view {
            for (_,child) in &retired.children {retire_overlay(cx,child.clone());}
        }
        let uid = host.widget_uid();
        let mut children = Vec::new();
        host.children(&mut |id, child| children.push((id, child)));
        drop(host);
        // Replacing a dynamic subtree must invalidate Studio's cached index.
        // Deep registration also reparents nodes created under the temporary
        // standalone View during script evaluation before mounting into Splash.
        for (id, child) in children {
            cx.widget_tree_insert_child_deep(uid, id, child);
        }
        cx.widget_tree_mark_dirty(uid);
        for element in &elements {
            if element["kind"].as_str()==Some("Radio") {
                if let Some(id)=element["id"].as_str() {
                    self.ui.radio_button(cx,&[LiveId::from_str(id)]).set_active(
                        cx,element["on"].as_i64()==Some(1),Animate::No);
                }
            }
            if let (Some(id),Some(enabled))=(element["id"].as_str(),element["enabled"].as_i64()) {
                self.ui.widget(cx,&[LiveId::from_str(id)]).set_disabled(cx,enabled==0);
            }
        }
        if let Some(updates)=r["updates"].as_array() {
            for update in updates {
                let id=update["id"].as_str().ok_or("missing component update ID")?;
                if !self.inspection_ids.iter().any(|known|known==id) {return Err("unknown component update ID".into());}
                let text=update["text"].as_str().ok_or("missing component text")?;
                self.ui.widget(cx,&[LiveId::from_str(id)]).set_text(cx,text);
            }
        }
        self.semantic.mount(cx,&self.ui,&r,&elements)?;
        cx.redraw_all();
        let evidence = serde_json::json!({"ok":true, "request":r,
            "nodes":tree.count(), "texts":splash_makepad::l0::texts(&tree),
            "elements": elements});
        if let Some(path) = r["result"].as_str() {
            std::fs::write(path, evidence.to_string()).map_err(|e| e.to_string())?;
        }
        log!("BEAUTY_MOUNT {}", evidence);
        Ok(())
    }

    fn measure_layout(&mut self, cx: &mut Cx) {
        if self.layout_path.is_empty() {
            return;
        }
        let mut rows = Vec::new();
        let mut pending = Vec::new();
        for (index,id) in self.inspection_ids.iter().enumerate() {
            let widget = self.ui.widget(cx, &[LiveId::from_str(id)]);
            let area = widget.area();
            if !area.is_valid(cx) {
                pending.push(serde_json::json!({"id":id,"reason":"missing drawn area"}));
                continue;
            }
            let raw = area.rect(cx);
            let clipped = area.clipped_rect(cx);
            if index==0 && (clipped.pos.x>raw.pos.x+0.5 || clipped.pos.y>raw.pos.y+0.5 ||
                clipped.size.x<raw.size.x-0.5 || clipped.size.y<raw.size.y-0.5) {
                pending.push(serde_json::json!({"id":id,"reason":"Studio viewport has not settled at the artboard size",
                    "expected":[raw.size.x,raw.size.y],"clipped":[clipped.size.x,clipped.size.y]}));
            }
            let mut image_pixels = None;
            let vector_ready = widget.borrow::<Svg>().map(|svg| {
                svg.draw_svg.svg_doc.is_some() && !svg.draw_svg.cached_indices.is_empty()
            }).or_else(||widget.borrow::<DesignGlassSvg>().map(|glass|glass.svg.draw_svg.svg_doc.is_some()&&!glass.svg.draw_svg.cached_indices.is_empty()));
            let glass_ready=widget.borrow::<makepad_widgets::gauss_view::GaussRoundedView>().map(|glass|glass.snapshot_ready)
                .or_else(||widget.borrow::<DesignGlassSvg>().map(|glass|glass.snapshot_ready));
            if glass_ready==Some(false) {
                pending.push(serde_json::json!({"id":id,"reason":"native backdrop texture not ready"}));
            }
            if vector_ready == Some(false) {
                pending.push(serde_json::json!({"id":id,"reason":"SVG geometry not ready"}));
            }
            let text_layout = widget.borrow::<Label>().map(|label| {
                let rect = label.text_layout_rect;
                [rect.pos.x, rect.pos.y, rect.size.x, rect.size.y]
            }).or_else(||widget.borrow::<DesignRotatedLabel>().map(|label| {
                let r=label.text_layout_rect;[r.pos.x,r.pos.y,r.size.x,r.size.y]
            }));
            let text_clip = widget.borrow::<Label>().map(|label| [label.clip_x, label.clip_y]);
            let password=widget.borrow::<TextInput>().map(|input|input.is_password());
            let rotation_degrees=widget.borrow::<DesignRotatedLabel>().map(|label|label.draw_text.rotation.to_degrees());
            if let Some(image) = widget.borrow::<Image>() {
                image_pixels = image.size_in_pixels(cx);
                if image_pixels.is_none() {
                    // Receiving an HTTP response precedes asynchronous image
                    // decoding. Do not certify geometry before pixels exist.
                    pending.push(serde_json::json!({"id":id,"reason":"image texture not decoded"}));
                }
            }
            rows.push(serde_json::json!({"id":id,
                "bounds":[raw.pos.x,raw.pos.y,raw.size.x,raw.size.y],
                "clipped_bounds":[clipped.pos.x,clipped.pos.y,clipped.size.x,clipped.size.y],
                "focused":cx.has_key_focus(area), "password":password, "image_pixels":image_pixels,
                "text_layout":text_layout,"text_clip":text_clip,"rotation_degrees":rotation_degrees,"vector_ready":vector_ready,"glass_ready":glass_ready}));
        }
        if !pending.is_empty() {
            let r: serde_json::Value = serde_json::from_str(&self.last_request).unwrap_or_default();
            let status = serde_json::json!({"nonce":r["nonce"],"pending":pending});
            let _ = std::fs::write(
                format!("{}.pending.json", self.layout_path),
                status.to_string(),
            );
            cx.redraw_all();
            return;
        }
        if let Some(id) = self.pending_focus.take() {
            let widget = self.ui.widget(cx, &[LiveId::from_str(&id)]);
            if let Some(mut input) = widget.borrow_mut::<TextInput>() {
                input.set_key_focus(cx);
                input.move_cursor_text_end(cx, false);
                input.reset_blink_timer(cx);
                input.redraw(cx);
                return;
            }
            log!("BEAUTY_ERROR focused widget is not a native TextInput: {id}");
            return;
        }
        let r: serde_json::Value = serde_json::from_str(&self.last_request).unwrap_or_default();
        let evidence = serde_json::json!({"nonce":r["nonce"], "elements":rows});
        if std::fs::write(&self.layout_path, evidence.to_string()).is_ok() {
            let _ = std::fs::remove_file(format!("{}.pending.json", self.layout_path));
            self.layout_path.clear();
        }
    }
}

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        crate::makepad_widgets::theme_mod(vm);
        splash_widgets::widgets_mod(vm);
        splash_widgets::design::script_mod(vm);
        splash_widgets::kit::script_mod(vm);
        splash_widgets::progress::script_mod(vm);
        makepad_plot::script_mod(vm);
        self::script_mod(vm)
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Actions(actions)=event {
            self.semantic.actions(cx,&self.ui,actions);
            for action in actions {
                let Some(action)=action.downcast_ref::<WidgetAction>() else {continue;};
                let action_kind=match action.cast::<splash_widgets::kit::KitAction>() {
                    splash_widgets::kit::KitAction::Activated=>serde_json::json!({"kind":"activated"}),
                    splash_widgets::kit::KitAction::Action(name)=>serde_json::json!({"kind":"action","name":name}),
                    splash_widgets::kit::KitAction::Changed(value)=>serde_json::json!({"kind":"changed","value":value}),
                    splash_widgets::kit::KitAction::Selected(index)=>serde_json::json!({"kind":"selected","index":index}),
                    _=>continue,
                };
                let id=self.inspection_ids.iter().find(|id|
                    self.ui.widget(cx,&[LiveId::from_str(id)]).widget_uid()==action.widget_uid);
                self.semantic_actions.push(serde_json::json!({"id":id,"action":action_kind}));
            }
            if !self.action_log.is_empty() {
                let _=std::fs::write(&self.action_log,serde_json::to_vec(&self.semantic_actions).unwrap());
            }
        }
        if matches!(event, Event::Startup) {
            self.timer = cx.start_interval(0.25);
        }
        if matches!(event, Event::Startup) || self.timer.is_event(event).is_some() {
            if let Err(error) = self.mount_request(cx) {
                log!("BEAUTY_ERROR {error}");
            }
        }
        self.ui.handle_event(cx, event, &mut Scope::empty());
        if self.timer.is_event(event).is_some() {
            self.semantic.poll(cx,&self.ui);
            self.measure_layout(cx);
        }
    }
}
