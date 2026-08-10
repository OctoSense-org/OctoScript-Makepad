//! Bar Chart Variants
//!
//! GPU-accelerated bar chart variants with smooth animations.

use makepad_widgets::*;
use makepad_d3::prelude::*;
use super::draw_primitives::{DrawTriangle, DrawChartLine, DrawPoint};
use super::animation::{ChartAnimator, EasingType};
use super::axis_renderer::DrawAxisText;
use super::legend_renderer::{render_legend, LegendItem, LegendConfig, LegendMarker, LegendPosition};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawTriangle;
    use super::draw_primitives::DrawChartLine;
    use super::draw_primitives::DrawPoint;
    use super::axis_renderer::DrawAxisText;

    pub HorizontalBarWidget = {{HorizontalBarWidget}} {
        width: Fill,
        height: Fill,
    }

    pub GroupedBarWidget = {{GroupedBarWidget}} {
        width: Fill,
        height: Fill,
    }

    pub StackedBarWidget = {{StackedBarWidget}} {
        width: Fill,
        height: Fill,
    }

    pub DivergingBarWidget = {{DivergingBarWidget}} {
        width: Fill,
        height: Fill,
    }

    pub RoundedBarWidget = {{RoundedBarWidget}} {
        width: Fill,
        height: Fill,
    }

    pub GradientBarWidget = {{GradientBarWidget}} {
        width: Fill,
        height: Fill,
    }
}

fn draw_rect(
    draw_bar: &mut DrawTriangle,
    draw_line: &mut DrawChartLine,
    cx: &mut Cx2d,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    color: Vec4,
    show_border: bool,
) {
    draw_bar.color = color;
    draw_bar.disable_gradient();

    let p1 = dvec2(x, y);
    let p2 = dvec2(x + width, y);
    let p3 = dvec2(x + width, y + height);
    let p4 = dvec2(x, y + height);

    draw_bar.draw_triangle(cx, p1, p2, p3);
    draw_bar.draw_triangle(cx, p1, p3, p4);

    if show_border {
        draw_line.color = vec4(0.1, 0.1, 0.12, 0.5);
        draw_line.draw_line(cx, p1, p2, 1.0);
        draw_line.draw_line(cx, p2, p3, 1.0);
        draw_line.draw_line(cx, p3, p4, 1.0);
        draw_line.draw_line(cx, p4, p1, 1.0);
    }
}

fn draw_rect_gradient(
    draw_bar: &mut DrawTriangle,
    draw_line: &mut DrawChartLine,
    cx: &mut Cx2d,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    color_top: Vec4,
    color_bottom: Vec4,
    show_border: bool,
) {
    draw_bar.set_vertical_gradient(color_top, color_bottom);

    let p1 = dvec2(x, y);
    let p2 = dvec2(x + width, y);
    let p3 = dvec2(x + width, y + height);
    let p4 = dvec2(x, y + height);

    draw_bar.draw_triangle(cx, p1, p2, p3);
    draw_bar.draw_triangle(cx, p1, p3, p4);

    if show_border {
        draw_line.color = vec4(0.1, 0.1, 0.12, 0.5);
        draw_line.draw_line(cx, p1, p2, 1.0);
        draw_line.draw_line(cx, p2, p3, 1.0);
        draw_line.draw_line(cx, p3, p4, 1.0);
        draw_line.draw_line(cx, p4, p1, 1.0);
    }
}

// Horizontal bar chart
#[derive(Live, LiveHook, Widget)]
pub struct HorizontalBarWidget {
    #[redraw] #[live] draw_bar: DrawTriangle,
    #[redraw] #[live] draw_line: DrawChartLine,
    #[walk] walk: Walk,
    #[rust] data: Vec<(String, f64)>,
    #[rust] animator: ChartAnimator,
    #[rust] initialized: bool,
    #[rust] area: Area,
}

impl Widget for HorizontalBarWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event {
            Event::NextFrame(_) => {
                if self.animator.is_running() {
                    let time = cx.seconds_since_app_start();
                    if self.animator.update(time) {
                        self.redraw(cx);
                    }
                    cx.new_next_frame();
                }
            }
            Event::WindowGeomChange(_) => {
                self.redraw(cx);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            if !self.initialized {
                self.initialize_data();
                self.start_animation(cx);
                self.initialized = true;
            }
            self.draw_chart(cx, rect);
        }
        DrawStep::done()
    }
}

impl HorizontalBarWidget {
    fn initialize_data(&mut self) {
        self.data = vec![
            ("Product A".into(), 85.0),
            ("Product B".into(), 65.0),
            ("Product C".into(), 92.0),
            ("Product D".into(), 48.0),
            ("Product E".into(), 73.0),
        ];
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1000.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        let progress = self.animator.get_progress();

        let padding = 80.0;
        let chart_x = rect.pos.x + padding;
        let chart_y = rect.pos.y + 20.0;
        let chart_width = rect.size.x - padding - 20.0;
        let chart_height = rect.size.y - 40.0;
        if chart_width <= 0.0 || chart_height <= 0.0 { return; }

        let max_val = self.data.iter().map(|d| d.1).fold(0.0f64, f64::max);
        let bar_height = chart_height / self.data.len() as f64 * 0.7;
        let spacing = chart_height / self.data.len() as f64;

        let data = self.data.clone();
        for (i, (_, value)) in data.iter().enumerate() {
            let bar_progress = ((progress - i as f64 * 0.1) / 0.5).clamp(0.0, 1.0);
            if bar_progress <= 0.0 { continue; }

            let full_width = (value / max_val) * chart_width;
            let bar_width = full_width * bar_progress;
            let y = chart_y + i as f64 * spacing + (spacing - bar_height) / 2.0;

            let t = i as f32 / self.data.len() as f32;
            let color = vec4(0.26 + t * 0.1, 0.52 + t * 0.1, 0.96 - t * 0.2, 1.0);

            draw_rect(&mut self.draw_bar, &mut self.draw_line, cx, chart_x, y, bar_width, bar_height, color, bar_progress > 0.7);
        }
    }
}

// Grouped bar chart
#[derive(Live, LiveHook, Widget)]
pub struct GroupedBarWidget {
    #[redraw] #[live] draw_bar: DrawTriangle,
    #[redraw] #[live] draw_line: DrawChartLine,
    #[redraw] #[live] draw_point: DrawPoint,
    #[redraw] #[live] draw_axis_text: DrawAxisText,
    #[walk] walk: Walk,
    #[rust] groups: Vec<Vec<f64>>,
    #[rust] series_labels: Vec<String>,
    #[rust] animator: ChartAnimator,
    #[rust] initialized: bool,
    #[rust] area: Area,
}

impl Widget for GroupedBarWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event {
            Event::NextFrame(_) => {
                if self.animator.is_running() {
                    let time = cx.seconds_since_app_start();
                    if self.animator.update(time) {
                        self.redraw(cx);
                    }
                    cx.new_next_frame();
                }
            }
            Event::WindowGeomChange(_) => {
                self.redraw(cx);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            if !self.initialized {
                self.initialize_data();
                self.start_animation(cx);
                self.initialized = true;
            }
            self.draw_chart(cx, rect);
        }
        DrawStep::done()
    }
}

impl GroupedBarWidget {
    fn initialize_data(&mut self) {
        self.groups = vec![
            vec![45.0, 55.0, 40.0],
            vec![60.0, 48.0, 65.0],
            vec![35.0, 70.0, 50.0],
            vec![75.0, 42.0, 58.0],
        ];
        self.series_labels = vec![
            "2022".to_string(),
            "2023".to_string(),
            "2024".to_string(),
        ];
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1200.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        let progress = self.animator.get_progress();

        let padding = 50.0;
        let legend_height = 30.0;
        let chart_x = rect.pos.x + padding;
        let chart_y = rect.pos.y + 20.0;
        let chart_width = rect.size.x - padding - 20.0;
        let chart_height = rect.size.y - padding - 20.0 - legend_height;
        if chart_width <= 0.0 || chart_height <= 0.0 { return; }

        let max_val = self.groups.iter().flat_map(|g| g.iter()).cloned().fold(0.0f64, f64::max);
        let group_width = chart_width / self.groups.len() as f64;
        let bars_per_group = self.groups.first().map(|g| g.len()).unwrap_or(0);
        let bar_width = group_width / (bars_per_group + 1) as f64;

        let colors = [
            vec4(0.26, 0.52, 0.96, 1.0),
            vec4(0.92, 0.26, 0.21, 1.0),
            vec4(0.20, 0.66, 0.33, 1.0),
        ];

        let groups = self.groups.clone();
        for (g_idx, group) in groups.iter().enumerate() {
            for (b_idx, &value) in group.iter().enumerate() {
                let bar_progress = ((progress - (g_idx * 3 + b_idx) as f64 * 0.05) / 0.4).clamp(0.0, 1.0);
                if bar_progress <= 0.0 { continue; }

                let x = chart_x + g_idx as f64 * group_width + (b_idx as f64 + 0.5) * bar_width;
                let full_height = (value / max_val) * chart_height;
                let bar_height = full_height * bar_progress;
                let y = chart_y + chart_height - bar_height;

                draw_rect(&mut self.draw_bar, &mut self.draw_line, cx, x, y, bar_width * 0.8, bar_height, colors[b_idx % colors.len()], bar_progress > 0.7);
            }
        }

        // Draw legend
        let series_labels = self.series_labels.clone();
        let legend_items: Vec<LegendItem> = series_labels.iter().enumerate()
            .map(|(i, label)| LegendItem::new(label, colors[i % colors.len()]).with_marker(LegendMarker::Square))
            .collect();

        let legend_config = LegendConfig::horizontal()
            .with_position(LegendPosition::Bottom);

        render_legend(
            cx,
            &mut self.draw_line,
            &mut self.draw_point,
            &mut self.draw_axis_text,
            &legend_items,
            rect,
            &legend_config,
        );
    }
}

// Stacked bar chart
#[derive(Live, LiveHook, Widget)]
pub struct StackedBarWidget {
    #[redraw] #[live] draw_bar: DrawTriangle,
    #[redraw] #[live] draw_line: DrawChartLine,
    #[redraw] #[live] draw_point: DrawPoint,
    #[redraw] #[live] draw_axis_text: DrawAxisText,
    #[walk] walk: Walk,
    #[rust] stacks: Vec<Vec<f64>>,
    #[rust] series_labels: Vec<String>,
    #[rust] animator: ChartAnimator,
    #[rust] initialized: bool,
    #[rust] area: Area,
}

impl Widget for StackedBarWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event {
            Event::NextFrame(_) => {
                if self.animator.is_running() {
                    let time = cx.seconds_since_app_start();
                    if self.animator.update(time) {
                        self.redraw(cx);
                    }
                    cx.new_next_frame();
                }
            }
            Event::WindowGeomChange(_) => {
                self.redraw(cx);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            if !self.initialized {
                self.initialize_data();
                self.start_animation(cx);
                self.initialized = true;
            }
            self.draw_chart(cx, rect);
        }
        DrawStep::done()
    }
}

impl StackedBarWidget {
    fn initialize_data(&mut self) {
        self.stacks = vec![
            vec![30.0, 25.0, 20.0],
            vec![40.0, 30.0, 15.0],
            vec![25.0, 35.0, 25.0],
            vec![35.0, 20.0, 30.0],
            vec![45.0, 25.0, 20.0],
        ];
        self.series_labels = vec![
            "Products".to_string(),
            "Services".to_string(),
            "Other".to_string(),
        ];
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1200.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        let progress = self.animator.get_progress();

        let padding = 50.0;
        let legend_height = 30.0;
        let chart_x = rect.pos.x + padding;
        let chart_y = rect.pos.y + 20.0;
        let chart_width = rect.size.x - padding - 20.0;
        let chart_height = rect.size.y - padding - 20.0 - legend_height;
        if chart_width <= 0.0 || chart_height <= 0.0 { return; }

        let max_total = self.stacks.iter().map(|s| s.iter().sum::<f64>()).fold(0.0f64, f64::max);
        let bar_width = chart_width / self.stacks.len() as f64 * 0.7;
        let spacing = chart_width / self.stacks.len() as f64;

        let colors = [
            vec4(0.26, 0.52, 0.96, 1.0),
            vec4(0.92, 0.26, 0.21, 1.0),
            vec4(0.20, 0.66, 0.33, 1.0),
        ];

        let stacks = self.stacks.clone();
        for (s_idx, stack) in stacks.iter().enumerate() {
            let stack_progress = ((progress - s_idx as f64 * 0.08) / 0.5).clamp(0.0, 1.0);
            if stack_progress <= 0.0 { continue; }

            let x = chart_x + s_idx as f64 * spacing + (spacing - bar_width) / 2.0;
            let mut y_offset = 0.0;

            for (layer_idx, &value) in stack.iter().enumerate() {
                let full_height = (value / max_total) * chart_height;
                let bar_height = full_height * stack_progress;
                let y = chart_y + chart_height - y_offset - bar_height;

                draw_rect(&mut self.draw_bar, &mut self.draw_line, cx, x, y, bar_width, bar_height, colors[layer_idx % colors.len()], stack_progress > 0.7);

                y_offset += bar_height;
            }
        }

        // Draw legend
        let series_labels = self.series_labels.clone();
        let legend_items: Vec<LegendItem> = series_labels.iter().enumerate()
            .map(|(i, label)| LegendItem::new(label, colors[i % colors.len()]).with_marker(LegendMarker::Square))
            .collect();

        let legend_config = LegendConfig::horizontal()
            .with_position(LegendPosition::Bottom);

        render_legend(
            cx,
            &mut self.draw_line,
            &mut self.draw_point,
            &mut self.draw_axis_text,
            &legend_items,
            rect,
            &legend_config,
        );
    }
}

// Diverging bar chart
#[derive(Live, LiveHook, Widget)]
pub struct DivergingBarWidget {
    #[redraw] #[live] draw_bar: DrawTriangle,
    #[redraw] #[live] draw_line: DrawChartLine,
    #[walk] walk: Walk,
    #[rust] data: Vec<f64>,
    #[rust] animator: ChartAnimator,
    #[rust] initialized: bool,
    #[rust] area: Area,
}

impl Widget for DivergingBarWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event {
            Event::NextFrame(_) => {
                if self.animator.is_running() {
                    let time = cx.seconds_since_app_start();
                    if self.animator.update(time) {
                        self.redraw(cx);
                    }
                    cx.new_next_frame();
                }
            }
            Event::WindowGeomChange(_) => {
                self.redraw(cx);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            if !self.initialized {
                self.initialize_data();
                self.start_animation(cx);
                self.initialized = true;
            }
            self.draw_chart(cx, rect);
        }
        DrawStep::done()
    }
}

impl DivergingBarWidget {
    fn initialize_data(&mut self) {
        self.data = vec![-35.0, 45.0, -20.0, 60.0, -50.0, 30.0, 55.0];
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1000.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        let progress = self.animator.get_progress();

        let padding = 50.0;
        let chart_x = rect.pos.x + padding;
        let chart_y = rect.pos.y + 20.0;
        let chart_width = rect.size.x - padding - 20.0;
        let chart_height = rect.size.y - padding - 20.0;
        if chart_width <= 0.0 || chart_height <= 0.0 { return; }

        let max_abs = self.data.iter().map(|v| v.abs()).fold(0.0f64, f64::max);
        let bar_width = chart_width / self.data.len() as f64 * 0.7;
        let spacing = chart_width / self.data.len() as f64;
        let center_y = chart_y + chart_height / 2.0;

        // Draw center line
        self.draw_line.color = vec4(0.6, 0.6, 0.6, 1.0);
        self.draw_line.draw_line(cx, dvec2(chart_x, center_y), dvec2(chart_x + chart_width, center_y), 2.0);

        let data = self.data.clone();
        for (i, &value) in data.iter().enumerate() {
            let bar_progress = ((progress - i as f64 * 0.08) / 0.5).clamp(0.0, 1.0);
            if bar_progress <= 0.0 { continue; }

            let x = chart_x + i as f64 * spacing + (spacing - bar_width) / 2.0;
            let full_height = (value.abs() / max_abs) * (chart_height / 2.0 - 10.0);
            let bar_height = full_height * bar_progress;

            let (color, y) = if value >= 0.0 {
                (vec4(0.20, 0.66, 0.33, 1.0), center_y - bar_height)
            } else {
                (vec4(0.92, 0.26, 0.21, 1.0), center_y)
            };

            draw_rect(&mut self.draw_bar, &mut self.draw_line, cx, x, y, bar_width, bar_height, color, bar_progress > 0.7);
        }
    }
}

// Rounded bar chart (visual effect through gradients)
#[derive(Live, LiveHook, Widget)]
pub struct RoundedBarWidget {
    #[redraw] #[live] draw_bar: DrawTriangle,
    #[redraw] #[live] draw_line: DrawChartLine,
    #[walk] walk: Walk,
    #[rust] data: Vec<f64>,
    #[rust] animator: ChartAnimator,
    #[rust] initialized: bool,
    #[rust] area: Area,
}

impl Widget for RoundedBarWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event {
            Event::NextFrame(_) => {
                if self.animator.is_running() {
                    let time = cx.seconds_since_app_start();
                    if self.animator.update(time) {
                        self.redraw(cx);
                    }
                    cx.new_next_frame();
                }
            }
            Event::WindowGeomChange(_) => {
                self.redraw(cx);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            if !self.initialized {
                self.initialize_data();
                self.start_animation(cx);
                self.initialized = true;
            }
            self.draw_chart(cx, rect);
        }
        DrawStep::done()
    }
}

impl RoundedBarWidget {
    fn initialize_data(&mut self) {
        self.data = vec![65.0, 80.0, 45.0, 90.0, 55.0, 70.0];
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1000.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        let progress = self.animator.get_progress();

        let padding = 50.0;
        let chart_x = rect.pos.x + padding;
        let chart_y = rect.pos.y + 20.0;
        let chart_width = rect.size.x - padding - 20.0;
        let chart_height = rect.size.y - padding - 20.0;
        if chart_width <= 0.0 || chart_height <= 0.0 { return; }

        let max_val = self.data.iter().cloned().fold(0.0f64, f64::max);
        let bar_width = chart_width / self.data.len() as f64 * 0.6;
        let spacing = chart_width / self.data.len() as f64;

        let data = self.data.clone();
        for (i, &value) in data.iter().enumerate() {
            let bar_progress = ((progress - i as f64 * 0.08) / 0.5).clamp(0.0, 1.0);
            if bar_progress <= 0.0 { continue; }

            let x = chart_x + i as f64 * spacing + (spacing - bar_width) / 2.0;
            let full_height = (value / max_val) * chart_height;
            let bar_height = full_height * bar_progress;
            let y = chart_y + chart_height - bar_height;

            let t = i as f32 / self.data.len() as f32;
            let color = vec4(0.26 + t * 0.4, 0.52, 0.96 - t * 0.3, 1.0);
            let light = vec4(0.36 + t * 0.4, 0.62, 1.0 - t * 0.2, 1.0);

            draw_rect_gradient(&mut self.draw_bar, &mut self.draw_line, cx, x, y, bar_width, bar_height, light, color, bar_progress > 0.7);
        }
    }
}

// Gradient bar chart
#[derive(Live, LiveHook, Widget)]
pub struct GradientBarWidget {
    #[redraw] #[live] draw_bar: DrawTriangle,
    #[redraw] #[live] draw_line: DrawChartLine,
    #[walk] walk: Walk,
    #[rust] data: Vec<f64>,
    #[rust] animator: ChartAnimator,
    #[rust] initialized: bool,
    #[rust] area: Area,
}

impl Widget for GradientBarWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event {
            Event::NextFrame(_) => {
                if self.animator.is_running() {
                    let time = cx.seconds_since_app_start();
                    if self.animator.update(time) {
                        self.redraw(cx);
                    }
                    cx.new_next_frame();
                }
            }
            Event::WindowGeomChange(_) => {
                self.redraw(cx);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            if !self.initialized {
                self.initialize_data();
                self.start_animation(cx);
                self.initialized = true;
            }
            self.draw_chart(cx, rect);
        }
        DrawStep::done()
    }
}

impl GradientBarWidget {
    fn initialize_data(&mut self) {
        self.data = vec![50.0, 75.0, 60.0, 85.0, 40.0, 95.0];
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1000.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        let progress = self.animator.get_progress();

        let padding = 50.0;
        let chart_x = rect.pos.x + padding;
        let chart_y = rect.pos.y + 20.0;
        let chart_width = rect.size.x - padding - 20.0;
        let chart_height = rect.size.y - padding - 20.0;
        if chart_width <= 0.0 || chart_height <= 0.0 { return; }

        let max_val = self.data.iter().cloned().fold(0.0f64, f64::max);
        let bar_width = chart_width / self.data.len() as f64 * 0.7;
        let spacing = chart_width / self.data.len() as f64;

        let data = self.data.clone();
        for (i, &value) in data.iter().enumerate() {
            let bar_progress = ((progress - i as f64 * 0.08) / 0.5).clamp(0.0, 1.0);
            if bar_progress <= 0.0 { continue; }

            let x = chart_x + i as f64 * spacing + (spacing - bar_width) / 2.0;
            let full_height = (value / max_val) * chart_height;
            let bar_height = full_height * bar_progress;
            let y = chart_y + chart_height - bar_height;

            // Rainbow gradient across bars
            let t = i as f32 / self.data.len() as f32;
            let hue = t * 360.0;
            let (r, g, b) = hsl_to_rgb(hue, 0.7, 0.5);
            let (r2, g2, b2) = hsl_to_rgb(hue, 0.8, 0.65);

            draw_rect_gradient(&mut self.draw_bar, &mut self.draw_line, cx, x, y, bar_width, bar_height, vec4(r2, g2, b2, 1.0), vec4(r, g, b, 1.0), bar_progress > 0.7);
        }
    }
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 60.0 { (c, x, 0.0) }
    else if h < 120.0 { (x, c, 0.0) }
    else if h < 180.0 { (0.0, c, x) }
    else if h < 240.0 { (0.0, x, c) }
    else if h < 300.0 { (x, 0.0, c) }
    else { (c, 0.0, x) };

    (r + m, g + m, b + m)
}
