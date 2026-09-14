//! Semantic components retain native controls and every inspectable child.
//! Shared L0 templates own composition; this layer owns events and public state.
use super::makepad_widgets::*;
use serde_json::Value;

script_mod! {
    use mod.prelude.widgets.*
    mod.widgets.KitSelectionControl = RadioButton {
        padding: 0 text: ""
        icon_walk: Walk{width: 0 height: 0}
        label_walk: Walk{width: 0 height: 0 margin: Inset{}}
        draw_bg +: {pixel: fn(){return vec4(0.0)}}
    }
    mod.prelude.widgets.KitSelectionControl = mod.widgets.KitSelectionControl
    mod.widgets.KitButton = #(KitButton::register_widget(vm))
    mod.prelude.widgets.KitButton = mod.widgets.KitButton
    mod.widgets.KitFormField = #(KitFormField::register_widget(vm))
    mod.prelude.widgets.KitFormField = mod.widgets.KitFormField
    mod.widgets.KitTabBar = #(KitTabBar::register_widget(vm))
    mod.prelude.widgets.KitTabBar = mod.widgets.KitTabBar
    mod.widgets.KitBottomNavigation = #(KitBottomNavigation::register_widget(vm))
    mod.prelude.widgets.KitBottomNavigation = mod.widgets.KitBottomNavigation
    mod.widgets.TaskplanProjectCard = #(TaskplanProjectCard::register_widget(vm))
    mod.prelude.widgets.TaskplanProjectCard = mod.widgets.TaskplanProjectCard
    mod.widgets.CamoTrackRow = #(CamoTrackRow::register_widget(vm))
    mod.prelude.widgets.CamoTrackRow = mod.widgets.CamoTrackRow
}

#[derive(Clone, Debug, Default)]
pub enum KitAction {
    Activated,
    Action(String),
    Changed(String),
    Selected(usize),
    #[default]
    None,
}

#[derive(Default)]
struct Controller {
    source: String,
    config: Value,
    selected: i64,
    reconcile_selection: bool,
    draw_list: Option<DrawList2d>,
}

fn part(view: &View, value: &Value) -> WidgetRef {
    value
        .as_str()
        .map(|id| view.child_by_path(&[LiveId::from_str(id)]))
        .unwrap_or_else(WidgetRef::empty)
}

fn activated(widget: &WidgetRef, actions: &Actions) -> bool {
    widget
        .borrow::<Button>()
        .is_some_and(|w| w.clicked(actions))
        || widget
            .borrow::<RadioButton>()
            .is_some_and(|w| w.clicked(actions))
}

fn ink(value: &Value) -> Vec4f {
    let c = value.as_u64().unwrap_or(0xff4c5fef) as u32;
    vec4(
        ((c >> 16) & 255) as f32 / 255.,
        ((c >> 8) & 255) as f32 / 255.,
        (c & 255) as f32 / 255.,
        (c >> 24) as f32 / 255.,
    )
}

impl Controller {
    fn refresh(&mut self, contract: &str) {
        if self.source == contract {
            return;
        }
        self.source = contract.into();
        // The design translator validates all child paths and native kinds.
        self.config = serde_json::from_str(contract).unwrap_or(Value::Null);
        self.selected = self.config["selected_index"].as_i64().unwrap_or(-1);
        self.reconcile_selection =
            self.config["selected_index"] != self.config["source_selected_index"];
    }
    fn content(&self, view: &View) -> WidgetRef {
        for role in ["input", "title", "label"] {
            let w = part(view, &self.config["bindings"][role]);
            if !w.is_empty() {
                return w;
            }
        }
        WidgetRef::empty()
    }
    fn select(&mut self, cx: &mut Cx, view: &View, selected: i64) -> bool {
        let Some(items) = self.config["items"].as_array() else {
            return false;
        };
        if selected < -1
            || selected >= items.len() as i64
            || (selected >= 0 && part(view, &items[selected as usize]["control"]).disabled(cx))
        {
            return false;
        }
        self.selected = selected;
        for (i, item) in items.iter().enumerate() {
            let root = part(view, &item["root"]);
            if let Some(mut w) = root.borrow_mut::<View>() {
                w.selected = Some(i as i64 == selected);
            }
            super::set_design_selection(&root, cx, i as i64 == selected);
            if let Some(mut w) = root.borrow_mut::<KitButton>() {
                w.view.selected = Some(i as i64 == selected);
            }
            let w = part(view, &item["control"]);
            if let Some(mut radio) = w.borrow_mut::<RadioButton>() {
                radio.set_active(cx, i as i64 == selected, Animate::No);
            }
            // A disabled item retains its source disabled paint; it must not
            // adopt the normal inactive palette when a sibling is selected.
            if w.disabled(cx) {
                continue;
            }
            for (parts, active, inactive) in [
                ("surfaces", "active_surface", "inactive_surface"),
                ("indicators", "active_indicator", "inactive_indicator"),
            ] {
                let surface = &self.config[if i as i64 == selected {
                    active
                } else {
                    inactive
                }];
                if surface.is_number() {
                    if let Some(surfaces) = item[parts].as_array() {
                        for id in surfaces {
                            let w = part(view, id);
                            let color = ink(surface);
                            if let Some(mut shape) = w.borrow_mut::<View>() {
                                shape.draw_bg.draw_vars.set_dyn_instance(
                                    cx,
                                    id!(color),
                                    &[color.x, color.y, color.z, color.w],
                                );
                            }
                            w.redraw(cx);
                        }
                    }
                }
            }
            let color = ink(&self.config[if i as i64 == selected {
                "active_color"
            } else {
                "inactive_color"
            }]);
            if let Some(paint) = item["paint"].as_array() {
                for id in paint {
                    let w = part(view, id);
                    if let Some(mut label) = w.borrow_mut::<Label>() {
                        label.draw_text.color = color;
                    }
                    if let Some(mut svg) = w.borrow_mut::<Svg>() {
                        svg.draw_svg.color = color;
                    }
                    w.redraw(cx);
                }
            }
        }
        true
    }
    fn event(
        &mut self,
        cx: &mut Cx,
        view: &mut View,
        event: &Event,
        scope: &mut Scope,
        enabled: bool,
    ) {
        if !enabled && event.requires_visibility() {
            return;
        }
        let before = self.content(view).text();
        let actions = cx.capture_actions(|cx| view.handle_event(cx, event, scope));
        if activated(&part(view, &self.config["bindings"]["control"]), &actions) {
            cx.widget_action(view.widget_uid(), KitAction::Activated);
        }
        if let Some(bindings) = self.config["action_bindings"].as_object() {
            for (name, id) in bindings {
                if activated(&part(view, id), &actions) {
                    cx.widget_action(view.widget_uid(), KitAction::Action(name.clone()));
                }
            }
        }
        let chosen = self.config["items"].as_array().and_then(|items| {
            items
                .iter()
                .position(|item| activated(&part(view, &item["control"]), &actions))
        });
        if let Some(index) = chosen {
            if self.select(cx, view, index as i64) {
                cx.widget_action(view.widget_uid(), KitAction::Selected(index));
            }
        }
        let after = self.content(view).text();
        if before != after && self.config["bindings"]["input"].is_string() {
            cx.widget_action(view.widget_uid(), KitAction::Changed(after));
        }
        // Retain native actions for normal application and inspection consumers.
        cx.extend_actions(actions);
    }
    fn draw(
        &mut self,
        cx: &mut Cx2d,
        view: &mut View,
        scope: &mut Scope,
        walk: Walk,
        glass: bool,
    ) -> DrawStep {
        let step = if glass && !cx.is_drawing_overlay() {
            self.draw_list
                .get_or_insert_with(|| DrawList2d::new(cx))
                .begin_overlay_reuse(cx);
            let step = view.draw_walk(cx, scope, walk);
            self.draw_list.as_mut().unwrap().end(cx);
            step
        } else {
            view.draw_walk(cx, scope, walk)
        };
        if self.reconcile_selection {
            self.reconcile_selection = false;
            self.select(cx, view, self.selected);
        }
        step
    }
}

// Each semantic role has its own native type in Studio. View's derive forwards
// child enumeration, lookup, area, layout, and redraw through the composition.
macro_rules! component {
    ($name:ident, $reference:ident) => {
        #[derive(Script, ScriptHook, Widget)]
        pub struct $name {
            #[deref]
            view: View,
            #[live]
            contract: String,
            #[live(true)]
            enabled: bool,
            #[live]
            glass: bool,
            #[rust]
            controller: Controller,
        }
        impl Widget for $name {
            fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
                self.controller.refresh(&self.contract);
                self.controller
                    .draw(cx, &mut self.view, scope, walk, self.glass)
            }
            fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
                self.controller.refresh(&self.contract);
                self.controller
                    .event(cx, &mut self.view, event, scope, self.enabled);
            }
            fn text(&self) -> String {
                self.controller.content(&self.view).text()
            }
            fn set_text(&mut self, cx: &mut Cx, text: &str) {
                self.controller.refresh(&self.contract);
                self.controller.content(&self.view).set_text(cx, text);
            }
            fn disabled(&self, cx: &Cx) -> bool {
                let bindings = &self.controller.config["bindings"];
                let primary = if bindings["input"].is_string() {
                    &bindings["input"]
                } else {
                    &bindings["control"]
                };
                !self.enabled || (primary.is_string() && part(&self.view, primary).disabled(cx))
            }
            fn set_disabled(&mut self, cx: &mut Cx, disabled: bool) {
                self.enabled = !disabled;
                self.controller.refresh(&self.contract);
                for role in ["control", "input"] {
                    part(&self.view, &self.controller.config["bindings"][role])
                        .set_disabled(cx, disabled);
                }
                if let Some(bindings) = self.controller.config["action_bindings"].as_object() {
                    for id in bindings.values() {
                        part(&self.view, id).set_disabled(cx, disabled);
                    }
                }
                if let Some(items) = self.controller.config["items"].as_array() {
                    for item in items {
                        part(&self.view, &item["control"]).set_disabled(
                            cx,
                            disabled || item["source_enabled"].as_bool() == Some(false),
                        );
                    }
                }
                self.view.redraw(cx);
            }
            fn selected_value(&self, _cx: &Cx) -> Option<String> {
                if self.controller.config["items"].is_array() {
                    Some(self.controller.selected.to_string())
                } else {
                    self.view.selected.map(|v| v.to_string())
                }
            }
            fn is_interactive(&self) -> bool {
                self.enabled
                    && (self.controller.config["items"].is_array()
                        || self.controller.config["bindings"]["control"].is_string()
                        || self.controller.config["bindings"]["input"].is_string())
            }
        }
        impl $reference {
            pub fn activated(&self, actions: &Actions) -> bool {
                matches!(
                    actions.find_widget_action_cast::<KitAction>(self.widget_uid()),
                    KitAction::Activated
                )
            }
            pub fn changed(&self, actions: &Actions) -> Option<String> {
                match actions.find_widget_action_cast::<KitAction>(self.widget_uid()) {
                    KitAction::Changed(value) => Some(value),
                    _ => None,
                }
            }
            pub fn action(&self, actions: &Actions) -> Option<String> {
                match actions.find_widget_action_cast::<KitAction>(self.widget_uid()) {
                    KitAction::Action(name) => Some(name),
                    _ => None,
                }
            }
            pub fn selected(&self, actions: &Actions) -> Option<usize> {
                match actions.find_widget_action_cast::<KitAction>(self.widget_uid()) {
                    KitAction::Selected(index) => Some(index),
                    _ => None,
                }
            }
            pub fn set_content(&self, cx: &mut Cx, role: &str, text: &str) -> bool {
                let Some(mut inner) = self.borrow_mut() else {
                    return false;
                };
                let contract = inner.contract.clone();
                inner.controller.refresh(&contract);
                let widget = part(&inner.view, &inner.controller.config["bindings"][role]);
                if widget.is_empty() {
                    return false;
                }
                widget.set_text(cx, text);
                true
            }
            pub fn select(&self, cx: &mut Cx, index: usize) -> bool {
                let Some(mut inner) = self.borrow_mut() else {
                    return false;
                };
                let inner = &mut *inner;
                inner.controller.refresh(&inner.contract);
                i64::try_from(index)
                    .is_ok_and(|index| inner.controller.select(cx, &inner.view, index))
            }
        }
    };
}
component!(KitButton, KitButtonRef);
component!(KitFormField, KitFormFieldRef);
component!(KitTabBar, KitTabBarRef);
component!(KitBottomNavigation, KitBottomNavigationRef);
component!(TaskplanProjectCard, TaskplanProjectCardRef);
component!(CamoTrackRow, CamoTrackRowRef);

#[cfg(test)]
mod tests {
    use super::Controller;

    #[test]
    fn changed_initial_selection_including_none_reconciles_source_paint() {
        for selected in [-1, 0, 2] {
            let mut controller = Controller::default();
            let contract = format!(r#"{{"selected_index":{selected},"source_selected_index":1}}"#);
            controller.refresh(&contract);
            assert_eq!(controller.selected, selected);
            assert!(controller.reconcile_selection);
            controller.reconcile_selection = false;
            controller.refresh(&contract);
            assert!(
                !controller.reconcile_selection,
                "unchanged contracts retain user selection"
            );
        }
    }
}

/// Release a retired component's overlay before another screen is mounted.
pub fn retire_overlay(cx: &mut Cx, widget: &WidgetRef) {
    macro_rules! clear {($($t:ty),*)=>{$(
        if let Some(w)=widget.borrow::<$t>() {
            if let Some(list)=&w.controller.draw_list {
                // The build-tool lane's retained draw lists are cleared by
                // redraw id alone. (port/appcard-on-octoscript keys recording
                // and uniform reuse on generation counters and takes two extra
                // generations from `Cx::next_uniform_gen` here.)
                let list_id = list.id();
                let redraw_id = cx.redraw_id;
                cx.draw_lists[list_id].clear_draw_items(redraw_id);
            }
        }
    )*};}
    clear!(
        KitButton,
        KitFormField,
        KitTabBar,
        KitBottomNavigation,
        TaskplanProjectCard,
        CamoTrackRow
    );
}
