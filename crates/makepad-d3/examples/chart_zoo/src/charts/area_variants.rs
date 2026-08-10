//! Area Chart Variants
//!
//! GPU-accelerated area chart variants with smooth animations.

use makepad_widgets::*;
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

    pub SmoothAreaWidget = {{SmoothAreaWidget}} {
        width: Fill,
        height: Fill,
    }

    pub SteppedAreaWidget = {{SteppedAreaWidget}} {
        width: Fill,
        height: Fill,
    }

    pub StackedAreaWidget = {{StackedAreaWidget}} {
        width: Fill,
        height: Fill,
    }

    pub GradientAreaWidget = {{GradientAreaWidget}} {
        width: Fill,
        height: Fill,
    }

    pub StreamGraphWidget = {{StreamGraphWidget}} {
        width: Fill,
        height: Fill,
    }

    pub MultiAreaWidget = {{MultiAreaWidget}} {
        width: Fill,
        height: Fill,
    }
}

// Draw filled area under a curve using triangles
fn draw_area_triangles(
    draw_bar: &mut DrawTriangle,
    cx: &mut Cx2d,
    points: &[(f64, f64)],
    color: Vec4,
    chart_x: f64,
    chart_y: f64,
    chart_w: f64,
    chart_h: f64,
    baseline: f64,
    progress: f64,
) {
    if points.len() < 2 {
        return;
    }

    draw_bar.color = color;
    draw_bar.disable_gradient();

    let base_y = chart_y + (1.0 - baseline) * chart_h;

    for i in 0..(points.len() - 1) {
        let (x1, y1) = points[i];
        let (x2, y2) = points[i + 1];

        let px1 = chart_x + x1 * chart_w;
        let py1 = chart_y + (1.0 - y1 * progress) * chart_h;
        let px2 = chart_x + x2 * chart_w;
        let py2 = chart_y + (1.0 - y2 * progress) * chart_h;

        let base_y1 = base_y;
        let base_y2 = base_y;

        // Draw two triangles for the trapezoid
        let p1 = dvec2(px1, py1);
        let p2 = dvec2(px2, py2);
        let p3 = dvec2(px2, base_y2);
        let p4 = dvec2(px1, base_y1);

        draw_bar.draw_triangle(cx, p1, p2, p3);
        draw_bar.draw_triangle(cx, p1, p3, p4);
    }
}

// Draw a single bar/rectangle using triangles
fn draw_rect(
    draw_bar: &mut DrawTriangle,
    cx: &mut Cx2d,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    color: Vec4,
) {
    draw_bar.color = color;
    draw_bar.disable_gradient();

    let p1 = dvec2(x, y);
    let p2 = dvec2(x + width, y);
    let p3 = dvec2(x + width, y + height);
    let p4 = dvec2(x, y + height);

    draw_bar.draw_triangle(cx, p1, p2, p3);
    draw_bar.draw_triangle(cx, p1, p3, p4);
}

fn draw_rect_gradient(
    draw_bar: &mut DrawTriangle,
    cx: &mut Cx2d,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    color_top: Vec4,
    color_bottom: Vec4,
) {
    draw_bar.set_vertical_gradient(color_top, color_bottom);

    let p1 = dvec2(x, y);
    let p2 = dvec2(x + width, y);
    let p3 = dvec2(x + width, y + height);
    let p4 = dvec2(x, y + height);

    draw_bar.draw_triangle(cx, p1, p2, p3);
    draw_bar.draw_triangle(cx, p1, p3, p4);
}

// ============ Smooth Area Widget ============
#[derive(Live, LiveHook, Widget)]
pub struct SmoothAreaWidget {
    #[redraw] #[live] draw_bar: DrawTriangle,
    #[redraw] #[live] draw_line: DrawChartLine,
    #[walk] walk: Walk,
    #[rust] animator: ChartAnimator,
    #[rust] initialized: bool,
    #[rust] area: Area,
}

impl Widget for SmoothAreaWidget {
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
                self.start_animation(cx);
                self.initialized = true;
            }
            self.draw_chart(cx, rect);
        }
        DrawStep::done()
    }
}

impl SmoothAreaWidget {
    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1000.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        let progress = self.animator.get_progress();

        let padding = 40.0;
        let chart_x = rect.pos.x + padding;
        let chart_y = rect.pos.y + padding;
        let chart_w = rect.size.x - padding * 2.0;
        let chart_h = rect.size.y - padding * 2.0;

        // Smooth curve data with interpolation
        let base_points = vec![0.2, 0.35, 0.3, 0.5, 0.45, 0.6, 0.55, 0.7, 0.65, 0.8];
        let mut smooth_points = Vec::new();

        for i in 0..50 {
            let t = i as f64 / 49.0;
            let idx = (t * (base_points.len() - 1) as f64) as usize;
            let frac = t * (base_points.len() - 1) as f64 - idx as f64;
            let y = if idx < base_points.len() - 1 {
                base_points[idx] * (1.0 - frac) + base_points[idx + 1] * frac
            } else {
                base_points[idx]
            };
            smooth_points.push((t, y));
        }

        draw_area_triangles(&mut self.draw_bar, cx, &smooth_points, vec4(0.26, 0.52, 0.96, 0.6), chart_x, chart_y, chart_w, chart_h, 0.0, progress);

        // Draw the line on top
        if progress > 0.5 {
            let line_alpha = ((progress - 0.5) * 2.0).min(1.0) as f32;
            self.draw_line.color = vec4(0.20, 0.45, 0.90, line_alpha);
            for i in 0..(smooth_points.len() - 1) {
                let (x1, y1) = smooth_points[i];
                let (x2, y2) = smooth_points[i + 1];
                let p1 = dvec2(chart_x + x1 * chart_w, chart_y + (1.0 - y1 * progress) * chart_h);
                let p2 = dvec2(chart_x + x2 * chart_w, chart_y + (1.0 - y2 * progress) * chart_h);
                self.draw_line.draw_line(cx, p1, p2, 2.0);
            }
        }
    }
}

// ============ Stepped Area Widget ============
#[derive(Live, LiveHook, Widget)]
pub struct SteppedAreaWidget {
    #[redraw] #[live] draw_bar: DrawTriangle,
    #[redraw] #[live] draw_line: DrawChartLine,
    #[walk] walk: Walk,
    #[rust] animator: ChartAnimator,
    #[rust] initialized: bool,
    #[rust] area: Area,
}

impl Widget for SteppedAreaWidget {
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
                self.start_animation(cx);
                self.initialized = true;
            }
            self.draw_chart(cx, rect);
        }
        DrawStep::done()
    }
}

impl SteppedAreaWidget {
    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1000.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        let progress = self.animator.get_progress();

        let padding = 40.0;
        let chart_x = rect.pos.x + padding;
        let chart_y = rect.pos.y + padding;
        let chart_w = rect.size.x - padding * 2.0;
        let chart_h = rect.size.y - padding * 2.0;

        // Step function data
        let values = vec![0.3, 0.5, 0.4, 0.7, 0.6, 0.8, 0.5, 0.9];

        for (i, &y) in values.iter().enumerate() {
            let bar_progress = ((progress - i as f64 * 0.08) / 0.5).clamp(0.0, 1.0);
            if bar_progress <= 0.0 { continue; }

            let x = i as f64 / values.len() as f64;
            let width = chart_w / values.len() as f64;
            let px = chart_x + x * chart_w;
            let py = chart_y + (1.0 - y * bar_progress) * chart_h;
            let height = y * bar_progress * chart_h;

            draw_rect(&mut self.draw_bar, cx, px, py, width, height, vec4(0.20, 0.66, 0.33, 0.6));
        }
    }
}

// ============ Stacked Area Widget ============
#[derive(Live, LiveHook, Widget)]
pub struct StackedAreaWidget {
    #[redraw] #[live] draw_bar: DrawTriangle,
    #[redraw] #[live] draw_line: DrawChartLine,
    #[redraw] #[live] draw_point: DrawPoint,
    #[redraw] #[live] draw_axis_text: DrawAxisText,
    #[walk] walk: Walk,
    #[rust] series_labels: Vec<String>,
    #[rust] animator: ChartAnimator,
    #[rust] initialized: bool,
    #[rust] area: Area,
}

impl Widget for StackedAreaWidget {
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
                self.start_animation(cx);
                self.initialized = true;
            }
            self.draw_chart(cx, rect);
        }
        DrawStep::done()
    }
}

impl StackedAreaWidget {
    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1200.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        self.series_labels = vec![
            "Series A".to_string(),
            "Series B".to_string(),
            "Series C".to_string(),
        ];
        cx.new_next_frame();
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        let progress = self.animator.get_progress();

        let padding = 40.0;
        let legend_height = 30.0;
        let chart_x = rect.pos.x + padding;
        let chart_y = rect.pos.y + padding;
        let chart_w = rect.size.x - padding * 2.0;
        let chart_h = rect.size.y - padding * 2.0 - legend_height;

        // Three stacked series
        let series1 = vec![0.15, 0.18, 0.16, 0.20, 0.19, 0.22, 0.20, 0.18];
        let series2 = vec![0.12, 0.15, 0.14, 0.16, 0.18, 0.15, 0.17, 0.14];
        let series3 = vec![0.10, 0.12, 0.11, 0.14, 0.13, 0.15, 0.12, 0.13];
        let colors = [
            vec4(0.26, 0.52, 0.96, 0.8),
            vec4(0.92, 0.26, 0.21, 0.8),
            vec4(0.20, 0.66, 0.33, 0.8),
        ];

        let n = series1.len();
        let bar_width = chart_w / n as f64;

        // Draw from bottom to top
        for i in 0..n {
            let bar_progress = ((progress - i as f64 * 0.06) / 0.5).clamp(0.0, 1.0);
            if bar_progress <= 0.0 { continue; }

            let x = chart_x + i as f64 * bar_width;

            // Bottom layer (series 3)
            let h3 = series3[i] * chart_h * 2.0 * bar_progress;
            draw_rect(&mut self.draw_bar, cx, x, chart_y + chart_h - h3, bar_width, h3, colors[2]);

            // Middle layer (series 2)
            let h2 = series2[i] * chart_h * 2.0 * bar_progress;
            draw_rect(&mut self.draw_bar, cx, x, chart_y + chart_h - h3 - h2, bar_width, h2, colors[1]);

            // Top layer (series 1)
            let h1 = series1[i] * chart_h * 2.0 * bar_progress;
            draw_rect(&mut self.draw_bar, cx, x, chart_y + chart_h - h3 - h2 - h1, bar_width, h1, colors[0]);
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

// ============ Gradient Area Widget ============
#[derive(Live, LiveHook, Widget)]
pub struct GradientAreaWidget {
    #[redraw] #[live] draw_bar: DrawTriangle,
    #[redraw] #[live] draw_line: DrawChartLine,
    #[walk] walk: Walk,
    #[rust] animator: ChartAnimator,
    #[rust] initialized: bool,
    #[rust] area: Area,
}

impl Widget for GradientAreaWidget {
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
                self.start_animation(cx);
                self.initialized = true;
            }
            self.draw_chart(cx, rect);
        }
        DrawStep::done()
    }
}

impl GradientAreaWidget {
    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1000.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        let progress = self.animator.get_progress();

        let padding = 40.0;
        let chart_x = rect.pos.x + padding;
        let chart_y = rect.pos.y + padding;
        let chart_w = rect.size.x - padding * 2.0;
        let chart_h = rect.size.y - padding * 2.0;

        let values = vec![0.3, 0.45, 0.4, 0.6, 0.55, 0.7, 0.65, 0.8, 0.75, 0.85];
        let n = values.len();
        let bar_width = chart_w / n as f64;

        for (i, &y) in values.iter().enumerate() {
            let bar_progress = ((progress - i as f64 * 0.06) / 0.5).clamp(0.0, 1.0);
            if bar_progress <= 0.0 { continue; }

            let x = chart_x + i as f64 * bar_width;
            let py = chart_y + (1.0 - y * bar_progress) * chart_h;
            let height = y * bar_progress * chart_h;

            // Gradient from top (light) to bottom (darker)
            draw_rect_gradient(
                &mut self.draw_bar,
                cx,
                x, py, bar_width, height,
                vec4(0.26, 0.52, 0.96, 0.9),
                vec4(0.15, 0.35, 0.75, 0.3),
            );
        }
    }
}

// ============ Stream Graph Widget ============
#[derive(Live, LiveHook, Widget)]
pub struct StreamGraphWidget {
    #[redraw] #[live] draw_bar: DrawTriangle,
    #[redraw] #[live] draw_line: DrawChartLine,
    #[redraw] #[live] draw_point: DrawPoint,
    #[redraw] #[live] draw_axis_text: DrawAxisText,
    #[walk] walk: Walk,
    #[rust] series_labels: Vec<String>,
    #[rust] animator: ChartAnimator,
    #[rust] initialized: bool,
    #[rust] area: Area,
}

impl Widget for StreamGraphWidget {
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
                self.start_animation(cx);
                self.initialized = true;
            }
            self.draw_chart(cx, rect);
        }
        DrawStep::done()
    }
}

impl StreamGraphWidget {
    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1200.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        self.series_labels = vec![
            "Category A".to_string(),
            "Category B".to_string(),
            "Category C".to_string(),
        ];
        cx.new_next_frame();
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        let progress = self.animator.get_progress();

        let padding = 40.0;
        let legend_height = 30.0;
        let chart_x = rect.pos.x + padding;
        let chart_y = rect.pos.y + padding;
        let chart_w = rect.size.x - padding * 2.0;
        let chart_h = rect.size.y - padding * 2.0 - legend_height;

        // Stream graph layers centered around middle
        let series = [
            (vec![0.08, 0.10, 0.12, 0.15, 0.12, 0.10, 0.08, 0.10, 0.12, 0.10], vec4(0.26, 0.52, 0.96, 0.8)),
            (vec![0.10, 0.12, 0.15, 0.18, 0.15, 0.12, 0.10, 0.13, 0.15, 0.12], vec4(0.92, 0.26, 0.21, 0.8)),
            (vec![0.06, 0.08, 0.10, 0.12, 0.10, 0.08, 0.07, 0.09, 0.10, 0.08], vec4(0.20, 0.66, 0.33, 0.8)),
        ];

        let n = series[0].0.len();
        let bar_width = chart_w / n as f64;
        let center_y = chart_y + chart_h / 2.0;

        for i in 0..n {
            let bar_progress = ((progress - i as f64 * 0.06) / 0.5).clamp(0.0, 1.0);
            if bar_progress <= 0.0 { continue; }

            let x = chart_x + i as f64 * bar_width;

            // Calculate total height for centering
            let total_height: f64 = series.iter().map(|(s, _)| s[i]).sum::<f64>() * chart_h * bar_progress;
            let mut current_y = center_y - total_height / 2.0;

            for (values, color) in &series {
                let height = values[i] * chart_h * bar_progress;
                draw_rect(&mut self.draw_bar, cx, x, current_y, bar_width, height, *color);
                current_y += height;
            }
        }

        // Draw legend
        let series_labels = self.series_labels.clone();
        let colors = [
            vec4(0.26, 0.52, 0.96, 0.8),
            vec4(0.92, 0.26, 0.21, 0.8),
            vec4(0.20, 0.66, 0.33, 0.8),
        ];
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

// ============ Multi Area Widget (overlapping) ============
#[derive(Live, LiveHook, Widget)]
pub struct MultiAreaWidget {
    #[redraw] #[live] draw_bar: DrawTriangle,
    #[redraw] #[live] draw_line: DrawChartLine,
    #[redraw] #[live] draw_point: DrawPoint,
    #[redraw] #[live] draw_axis_text: DrawAxisText,
    #[walk] walk: Walk,
    #[rust] series_labels: Vec<String>,
    #[rust] animator: ChartAnimator,
    #[rust] initialized: bool,
    #[rust] area: Area,
}

impl Widget for MultiAreaWidget {
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
                self.start_animation(cx);
                self.initialized = true;
            }
            self.draw_chart(cx, rect);
        }
        DrawStep::done()
    }
}

impl MultiAreaWidget {
    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1000.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        self.series_labels = vec![
            "Revenue".to_string(),
            "Expenses".to_string(),
        ];
        cx.new_next_frame();
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        let progress = self.animator.get_progress();

        let padding = 40.0;
        let legend_height = 30.0;
        let chart_x = rect.pos.x + padding;
        let chart_y = rect.pos.y + padding;
        let chart_w = rect.size.x - padding * 2.0;
        let chart_h = rect.size.y - padding * 2.0 - legend_height;

        let colors = [
            vec4(0.26, 0.52, 0.96, 0.5),
            vec4(0.92, 0.26, 0.21, 0.5),
        ];

        // Two overlapping series
        let series1: Vec<(f64, f64)> = vec![
            (0.0, 0.3), (0.15, 0.45), (0.3, 0.4), (0.45, 0.55), (0.6, 0.5),
            (0.75, 0.65), (0.9, 0.6), (1.0, 0.7)
        ];
        let series2: Vec<(f64, f64)> = vec![
            (0.0, 0.2), (0.15, 0.35), (0.3, 0.5), (0.45, 0.45), (0.6, 0.6),
            (0.75, 0.55), (0.9, 0.7), (1.0, 0.65)
        ];

        // Draw series 1 (back)
        draw_area_triangles(&mut self.draw_bar, cx, &series1, colors[0], chart_x, chart_y, chart_w, chart_h, 0.0, progress);

        // Draw series 2 (front)
        draw_area_triangles(&mut self.draw_bar, cx, &series2, colors[1], chart_x, chart_y, chart_w, chart_h, 0.0, progress);

        // Draw lines on top
        if progress > 0.5 {
            let line_alpha = ((progress - 0.5) * 2.0).min(1.0) as f32;

            // Series 1 line
            self.draw_line.color = vec4(0.20, 0.45, 0.90, line_alpha);
            for i in 0..(series1.len() - 1) {
                let (x1, y1) = series1[i];
                let (x2, y2) = series1[i + 1];
                let p1 = dvec2(chart_x + x1 * chart_w, chart_y + (1.0 - y1 * progress) * chart_h);
                let p2 = dvec2(chart_x + x2 * chart_w, chart_y + (1.0 - y2 * progress) * chart_h);
                self.draw_line.draw_line(cx, p1, p2, 2.0);
            }

            // Series 2 line
            self.draw_line.color = vec4(0.85, 0.20, 0.15, line_alpha);
            for i in 0..(series2.len() - 1) {
                let (x1, y1) = series2[i];
                let (x2, y2) = series2[i + 1];
                let p1 = dvec2(chart_x + x1 * chart_w, chart_y + (1.0 - y1 * progress) * chart_h);
                let p2 = dvec2(chart_x + x2 * chart_w, chart_y + (1.0 - y2 * progress) * chart_h);
                self.draw_line.draw_line(cx, p1, p2, 2.0);
            }
        }

        // Draw legend
        let series_labels = self.series_labels.clone();
        let legend_items: Vec<LegendItem> = series_labels.iter().enumerate()
            .map(|(i, label)| LegendItem::new(label, colors[i % colors.len()]).with_marker(LegendMarker::Line))
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
