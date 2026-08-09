//! Line Chart Variants
//!
//! GPU-accelerated line chart variants with smooth rendering and animations.

use makepad_widgets::*;
use makepad_d3::prelude::*;
use super::draw_primitives::{DrawChartLine, DrawPoint, DrawTriangle};
use super::animation::{ChartAnimator, EasingType, get_color};
use super::axis_renderer::DrawAxisText;
use super::legend_renderer::{render_legend, LegendItem, LegendConfig, LegendMarker, LegendPosition};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawChartLine;
    use super::draw_primitives::DrawPoint;
    use super::draw_primitives::DrawTriangle;
    use super::axis_renderer::DrawAxisText;

    pub SmoothLineWidget = {{SmoothLineWidget}} {
        width: Fill,
        height: Fill,
    }

    pub StepLineWidget = {{StepLineWidget}} {
        width: Fill,
        height: Fill,
    }

    pub DashedLineWidget = {{DashedLineWidget}} {
        width: Fill,
        height: Fill,
    }

    pub ThickLineWidget = {{ThickLineWidget}} {
        width: Fill,
        height: Fill,
    }

    pub MultiSeriesLineWidget = {{MultiSeriesLineWidget}} {
        width: Fill,
        height: Fill,
    }

    pub GradientLineWidget = {{GradientLineWidget}} {
        width: Fill,
        height: Fill,
    }
}

// ============ Smooth Line Widget ============
#[derive(Live, LiveHook, Widget)]
pub struct SmoothLineWidget {
    #[redraw] #[live] draw_line: DrawChartLine,
    #[redraw] #[live] draw_point: DrawPoint,
    #[walk] walk: Walk,
    #[rust] data: Vec<f64>,
    #[rust] animator: ChartAnimator,
    #[rust] initialized: bool,
    #[rust] area: Area,
}

impl Widget for SmoothLineWidget {
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

impl SmoothLineWidget {
    fn initialize_data(&mut self) {
        self.data = vec![20.0, 45.0, 28.0, 80.0, 55.0, 90.0, 70.0, 85.0];
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1200.0).with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        let progress = self.animator.get_progress();
        let padding = 40.0;
        let chart_rect = Rect {
            pos: dvec2(rect.pos.x + padding, rect.pos.y + 20.0),
            size: dvec2(rect.size.x - padding - 20.0, rect.size.y - padding - 20.0),
        };
        if chart_rect.size.x <= 0.0 || chart_rect.size.y <= 0.0 { return; }

        let max_val = self.data.iter().cloned().fold(0.0f64, f64::max);
        let x_scale = LinearScale::new().with_domain(0.0, (self.data.len() - 1) as f64).with_range(0.0, chart_rect.size.x);
        let y_scale = LinearScale::new().with_domain(0.0, max_val * 1.1).with_range(chart_rect.size.y, 0.0);

        // Collect points
        let points: Vec<DVec2> = self.data.iter().enumerate().map(|(i, &v)| {
            dvec2(
                chart_rect.pos.x + x_scale.scale(i as f64),
                chart_rect.pos.y + y_scale.scale(v),
            )
        }).collect();

        // Draw smooth curve using Catmull-Rom spline
        self.draw_line.color = vec4(0.26, 0.52, 0.96, 1.0);
        let total_segments = (points.len() - 1) * 16;
        let draw_segments = ((total_segments as f64 * progress) as usize).max(1);

        for seg in 0..draw_segments {
            let t1 = seg as f64 / total_segments as f64;
            let t2 = (seg + 1) as f64 / total_segments as f64;
            let p1 = catmull_rom_point(&points, t1);
            let p2 = catmull_rom_point(&points, t2);
            self.draw_line.draw_line(cx, p1, p2, 3.0);
        }

        // Draw points
        for (i, &point) in points.iter().enumerate() {
            let point_progress = ((progress - i as f64 * 0.08) / 0.3).clamp(0.0, 1.0);
            if point_progress > 0.0 {
                self.draw_point.color = vec4(0.26, 0.52, 0.96, 1.0);
                self.draw_point.draw_point(cx, point, 5.0 * point_progress);
            }
        }
    }
}

// ============ Step Line Widget ============
#[derive(Live, LiveHook, Widget)]
pub struct StepLineWidget {
    #[redraw] #[live] draw_line: DrawChartLine,
    #[redraw] #[live] draw_point: DrawPoint,
    #[walk] walk: Walk,
    #[rust] data: Vec<f64>,
    #[rust] animator: ChartAnimator,
    #[rust] initialized: bool,
    #[rust] area: Area,
}

impl Widget for StepLineWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event {
            Event::NextFrame(_) => {
                if self.animator.is_running() {
                    let time = cx.seconds_since_app_start();
                    if self.animator.update(time) { self.redraw(cx); }
                    cx.new_next_frame();
                }
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

impl StepLineWidget {
    fn initialize_data(&mut self) {
        self.data = vec![30.0, 50.0, 35.0, 70.0, 45.0, 80.0, 60.0];
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1000.0).with_easing(EasingType::EaseOutQuad);
        self.animator.start(time);
        cx.new_next_frame();
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        let progress = self.animator.get_progress();
        let padding = 40.0;
        let chart_rect = Rect {
            pos: dvec2(rect.pos.x + padding, rect.pos.y + 20.0),
            size: dvec2(rect.size.x - padding - 20.0, rect.size.y - padding - 20.0),
        };
        if chart_rect.size.x <= 0.0 || chart_rect.size.y <= 0.0 { return; }

        let max_val = self.data.iter().cloned().fold(0.0f64, f64::max);
        let x_scale = LinearScale::new().with_domain(0.0, (self.data.len() - 1) as f64).with_range(0.0, chart_rect.size.x);
        let y_scale = LinearScale::new().with_domain(0.0, max_val * 1.1).with_range(chart_rect.size.y, 0.0);

        self.draw_line.color = vec4(0.20, 0.66, 0.33, 1.0);

        let total_steps = (self.data.len() - 1) * 2;
        let draw_steps = ((total_steps as f64 * progress) as usize).max(1);

        for i in 0..self.data.len() - 1 {
            let x1 = chart_rect.pos.x + x_scale.scale(i as f64);
            let y1 = chart_rect.pos.y + y_scale.scale(self.data[i]);
            let x2 = chart_rect.pos.x + x_scale.scale((i + 1) as f64);
            let y2 = chart_rect.pos.y + y_scale.scale(self.data[i + 1]);

            let step_idx = i * 2;

            // Horizontal line
            if step_idx < draw_steps {
                let seg_progress = if step_idx == draw_steps - 1 {
                    (progress * total_steps as f64 - step_idx as f64).clamp(0.0, 1.0)
                } else { 1.0 };
                let end_x = x1 + (x2 - x1) * seg_progress;
                self.draw_line.draw_line(cx, dvec2(x1, y1), dvec2(end_x, y1), 2.5);
            }

            // Vertical line
            if step_idx + 1 < draw_steps {
                let seg_progress = if step_idx + 1 == draw_steps - 1 {
                    (progress * total_steps as f64 - (step_idx + 1) as f64).clamp(0.0, 1.0)
                } else { 1.0 };
                let end_y = y1 + (y2 - y1) * seg_progress;
                self.draw_line.draw_line(cx, dvec2(x2, y1), dvec2(x2, end_y), 2.5);
            }
        }

        // Draw points
        for (i, &val) in self.data.iter().enumerate() {
            let point_progress = ((progress - i as f64 * 0.1) / 0.2).clamp(0.0, 1.0);
            if point_progress > 0.0 {
                let x = chart_rect.pos.x + x_scale.scale(i as f64);
                let y = chart_rect.pos.y + y_scale.scale(val);
                self.draw_point.color = vec4(0.20, 0.66, 0.33, 1.0);
                self.draw_point.draw_point(cx, dvec2(x, y), 4.0 * point_progress);
            }
        }
    }
}

// ============ Dashed Line Widget ============
#[derive(Live, LiveHook, Widget)]
pub struct DashedLineWidget {
    #[redraw] #[live] draw_line: DrawChartLine,
    #[redraw] #[live] draw_point: DrawPoint,
    #[walk] walk: Walk,
    #[rust] data: Vec<f64>,
    #[rust] animator: ChartAnimator,
    #[rust] initialized: bool,
    #[rust] area: Area,
}

impl Widget for DashedLineWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event {
            Event::NextFrame(_) => {
                if self.animator.is_running() {
                    let time = cx.seconds_since_app_start();
                    if self.animator.update(time) { self.redraw(cx); }
                    cx.new_next_frame();
                }
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

impl DashedLineWidget {
    fn initialize_data(&mut self) {
        self.data = vec![40.0, 35.0, 60.0, 50.0, 75.0, 65.0, 85.0];
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1200.0).with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        let progress = self.animator.get_progress();
        let padding = 40.0;
        let chart_rect = Rect {
            pos: dvec2(rect.pos.x + padding, rect.pos.y + 20.0),
            size: dvec2(rect.size.x - padding - 20.0, rect.size.y - padding - 20.0),
        };
        if chart_rect.size.x <= 0.0 || chart_rect.size.y <= 0.0 { return; }

        let max_val = self.data.iter().cloned().fold(0.0f64, f64::max);
        let x_scale = LinearScale::new().with_domain(0.0, (self.data.len() - 1) as f64).with_range(0.0, chart_rect.size.x);
        let y_scale = LinearScale::new().with_domain(0.0, max_val * 1.1).with_range(chart_rect.size.y, 0.0);

        self.draw_line.color = vec4(0.92, 0.26, 0.21, 1.0);
        let dash_len = 12.0;
        let gap_len = 8.0;

        for i in 0..self.data.len() - 1 {
            let seg_progress = ((progress - i as f64 * 0.1) / 0.5).clamp(0.0, 1.0);
            if seg_progress <= 0.0 { continue; }

            let x1 = chart_rect.pos.x + x_scale.scale(i as f64);
            let y1 = chart_rect.pos.y + y_scale.scale(self.data[i]);
            let x2 = chart_rect.pos.x + x_scale.scale((i + 1) as f64);
            let y2 = chart_rect.pos.y + y_scale.scale(self.data[i + 1]);

            let dx = x2 - x1;
            let dy = y2 - y1;
            let len = (dx * dx + dy * dy).sqrt();
            let nx = dx / len;
            let ny = dy / len;

            let animated_len = len * seg_progress;
            let mut pos = 0.0;
            let mut drawing = true;

            while pos < animated_len {
                let seg_len = if drawing { dash_len } else { gap_len };
                let end_pos = (pos + seg_len).min(animated_len);

                if drawing {
                    let sx = x1 + nx * pos;
                    let sy = y1 + ny * pos;
                    let ex = x1 + nx * end_pos;
                    let ey = y1 + ny * end_pos;
                    self.draw_line.draw_line(cx, dvec2(sx, sy), dvec2(ex, ey), 3.0);
                }
                pos = end_pos;
                drawing = !drawing;
            }
        }

        // Draw points
        for (i, &val) in self.data.iter().enumerate() {
            let point_progress = ((progress - i as f64 * 0.1) / 0.3).clamp(0.0, 1.0);
            if point_progress > 0.0 {
                let x = chart_rect.pos.x + x_scale.scale(i as f64);
                let y = chart_rect.pos.y + y_scale.scale(val);
                self.draw_point.color = vec4(0.92, 0.26, 0.21, 1.0);
                self.draw_point.draw_point(cx, dvec2(x, y), 5.0 * point_progress);
            }
        }
    }
}

// ============ Thick Line Widget ============
#[derive(Live, LiveHook, Widget)]
pub struct ThickLineWidget {
    #[redraw] #[live] draw_line: DrawChartLine,
    #[redraw] #[live] draw_point: DrawPoint,
    #[walk] walk: Walk,
    #[rust] data: Vec<f64>,
    #[rust] animator: ChartAnimator,
    #[rust] initialized: bool,
    #[rust] area: Area,
}

impl Widget for ThickLineWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event {
            Event::NextFrame(_) => {
                if self.animator.is_running() {
                    let time = cx.seconds_since_app_start();
                    if self.animator.update(time) { self.redraw(cx); }
                    cx.new_next_frame();
                }
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

impl ThickLineWidget {
    fn initialize_data(&mut self) {
        self.data = vec![25.0, 55.0, 40.0, 70.0, 50.0, 85.0, 65.0];
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1000.0).with_easing(EasingType::EaseOutQuad);
        self.animator.start(time);
        cx.new_next_frame();
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        let progress = self.animator.get_progress();
        let padding = 40.0;
        let chart_rect = Rect {
            pos: dvec2(rect.pos.x + padding, rect.pos.y + 20.0),
            size: dvec2(rect.size.x - padding - 20.0, rect.size.y - padding - 20.0),
        };
        if chart_rect.size.x <= 0.0 || chart_rect.size.y <= 0.0 { return; }

        let max_val = self.data.iter().cloned().fold(0.0f64, f64::max);
        let x_scale = LinearScale::new().with_domain(0.0, (self.data.len() - 1) as f64).with_range(0.0, chart_rect.size.x);
        let y_scale = LinearScale::new().with_domain(0.0, max_val * 1.1).with_range(chart_rect.size.y, 0.0);

        // Draw thick line with shadow
        for i in 0..self.data.len() - 1 {
            let seg_progress = ((progress - i as f64 * 0.1) / 0.5).clamp(0.0, 1.0);
            if seg_progress <= 0.0 { continue; }

            let x1 = chart_rect.pos.x + x_scale.scale(i as f64);
            let y1 = chart_rect.pos.y + y_scale.scale(self.data[i]);
            let x2 = chart_rect.pos.x + x_scale.scale((i + 1) as f64);
            let y2 = chart_rect.pos.y + y_scale.scale(self.data[i + 1]);

            let end_x = x1 + (x2 - x1) * seg_progress;
            let end_y = y1 + (y2 - y1) * seg_progress;

            // Shadow
            self.draw_line.color = vec4(0.4, 0.1, 0.5, 0.3);
            self.draw_line.draw_line(cx, dvec2(x1 + 2.0, y1 + 2.0), dvec2(end_x + 2.0, end_y + 2.0), 10.0);

            // Main line
            self.draw_line.color = vec4(0.61, 0.15, 0.69, 1.0);
            self.draw_line.draw_line(cx, dvec2(x1, y1), dvec2(end_x, end_y), 8.0);
        }

        // Draw points
        for (i, &val) in self.data.iter().enumerate() {
            let point_progress = ((progress - i as f64 * 0.1) / 0.3).clamp(0.0, 1.0);
            if point_progress > 0.0 {
                let x = chart_rect.pos.x + x_scale.scale(i as f64);
                let y = chart_rect.pos.y + y_scale.scale(val);

                // Outer glow
                self.draw_point.color = vec4(0.61, 0.15, 0.69, 0.4);
                self.draw_point.draw_point(cx, dvec2(x, y), 12.0 * point_progress);

                // Inner point
                self.draw_point.color = vec4(0.61, 0.15, 0.69, 1.0);
                self.draw_point.draw_point(cx, dvec2(x, y), 6.0 * point_progress);
            }
        }
    }
}

// ============ Multi-Series Line Widget ============
#[derive(Live, LiveHook, Widget)]
pub struct MultiSeriesLineWidget {
    #[redraw] #[live] draw_line: DrawChartLine,
    #[redraw] #[live] draw_point: DrawPoint,
    #[redraw] #[live] draw_axis_text: DrawAxisText,
    #[walk] walk: Walk,
    #[rust] series: Vec<Vec<f64>>,
    #[rust] series_labels: Vec<String>,
    #[rust] animator: ChartAnimator,
    #[rust] initialized: bool,
    #[rust] area: Area,
}

impl Widget for MultiSeriesLineWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event {
            Event::NextFrame(_) => {
                if self.animator.is_running() {
                    let time = cx.seconds_since_app_start();
                    if self.animator.update(time) { self.redraw(cx); }
                    cx.new_next_frame();
                }
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

impl MultiSeriesLineWidget {
    fn initialize_data(&mut self) {
        self.series = vec![
            vec![20.0, 40.0, 35.0, 50.0, 45.0, 60.0, 55.0, 70.0],
            vec![30.0, 25.0, 45.0, 40.0, 55.0, 50.0, 70.0, 65.0],
            vec![15.0, 35.0, 30.0, 55.0, 35.0, 45.0, 40.0, 50.0],
            vec![40.0, 45.0, 50.0, 35.0, 60.0, 55.0, 65.0, 75.0],
        ];
        self.series_labels = vec![
            "Series A".to_string(),
            "Series B".to_string(),
            "Series C".to_string(),
            "Series D".to_string(),
        ];
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1500.0).with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        let progress = self.animator.get_progress();
        let padding = 40.0;
        // Reserve space for legend on the right
        let legend_width = 90.0;
        let chart_rect = Rect {
            pos: dvec2(rect.pos.x + padding, rect.pos.y + 20.0),
            size: dvec2(rect.size.x - padding - legend_width - 10.0, rect.size.y - padding - 20.0),
        };
        if chart_rect.size.x <= 0.0 || chart_rect.size.y <= 0.0 { return; }

        let max_val = self.series.iter().flat_map(|s| s.iter()).cloned().fold(0.0f64, f64::max);
        let n = self.series.first().map(|s| s.len()).unwrap_or(0);
        if n == 0 { return; }

        let x_scale = LinearScale::new().with_domain(0.0, (n - 1) as f64).with_range(0.0, chart_rect.size.x);
        let y_scale = LinearScale::new().with_domain(0.0, max_val * 1.1).with_range(chart_rect.size.y, 0.0);

        // Clone series for iteration
        let series = self.series.clone();
        let series_labels = self.series_labels.clone();

        // Draw each series with staggered animation
        for (s_idx, data) in series.iter().enumerate() {
            let series_progress = ((progress - s_idx as f64 * 0.15) / 0.6).clamp(0.0, 1.0);
            if series_progress <= 0.0 { continue; }

            let color = get_color(s_idx);

            // Collect points for this series
            let points: Vec<DVec2> = data.iter().enumerate().map(|(i, &v)| {
                dvec2(
                    chart_rect.pos.x + x_scale.scale(i as f64),
                    chart_rect.pos.y + y_scale.scale(v),
                )
            }).collect();

            // Draw smooth curve
            self.draw_line.color = color;
            let total_segments = (points.len() - 1) * 12;
            let draw_segments = ((total_segments as f64 * series_progress) as usize).max(1);

            for seg in 0..draw_segments {
                let t1 = seg as f64 / total_segments as f64;
                let t2 = (seg + 1) as f64 / total_segments as f64;
                let p1 = catmull_rom_point(&points, t1);
                let p2 = catmull_rom_point(&points, t2);
                self.draw_line.draw_line(cx, p1, p2, 2.5);
            }

            // Draw points
            for (i, &point) in points.iter().enumerate() {
                let point_progress = ((series_progress - i as f64 * 0.08) / 0.25).clamp(0.0, 1.0);
                if point_progress > 0.0 {
                    // Outer glow
                    self.draw_point.color = vec4(color.x, color.y, color.z, 0.3);
                    self.draw_point.draw_point(cx, point, 8.0 * point_progress);

                    // Inner point
                    self.draw_point.color = color;
                    self.draw_point.draw_point(cx, point, 4.0 * point_progress);

                    // Highlight
                    self.draw_point.color = vec4(1.0, 1.0, 1.0, 0.5);
                    self.draw_point.draw_point(
                        cx,
                        dvec2(point.x - 1.0, point.y - 1.0),
                        1.5 * point_progress,
                    );
                }
            }
        }

        // Draw legend
        let legend_items: Vec<LegendItem> = series_labels.iter().enumerate()
            .map(|(i, label)| LegendItem::new(label, get_color(i)).with_marker(LegendMarker::Line))
            .collect();

        let legend_config = LegendConfig::vertical()
            .with_position(LegendPosition::TopRight);

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

// ============ Gradient Line Widget ============
#[derive(Live, LiveHook, Widget)]
pub struct GradientLineWidget {
    #[redraw] #[live] draw_line: DrawChartLine,
    #[redraw] #[live] draw_point: DrawPoint,
    #[redraw] #[live] draw_fill: DrawTriangle,
    #[walk] walk: Walk,
    #[rust] data: Vec<f64>,
    #[rust] animator: ChartAnimator,
    #[rust] initialized: bool,
    #[rust] area: Area,
}

impl Widget for GradientLineWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event {
            Event::NextFrame(_) => {
                if self.animator.is_running() {
                    let time = cx.seconds_since_app_start();
                    if self.animator.update(time) { self.redraw(cx); }
                    cx.new_next_frame();
                }
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

impl GradientLineWidget {
    fn initialize_data(&mut self) {
        self.data = vec![30.0, 45.0, 38.0, 65.0, 55.0, 80.0, 70.0, 90.0];
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1200.0).with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        let progress = self.animator.get_progress();
        let padding = 40.0;
        let chart_rect = Rect {
            pos: dvec2(rect.pos.x + padding, rect.pos.y + 20.0),
            size: dvec2(rect.size.x - padding - 20.0, rect.size.y - padding - 20.0),
        };
        if chart_rect.size.x <= 0.0 || chart_rect.size.y <= 0.0 { return; }

        let max_val = self.data.iter().cloned().fold(0.0f64, f64::max);
        let x_scale = LinearScale::new().with_domain(0.0, (self.data.len() - 1) as f64).with_range(0.0, chart_rect.size.x);
        let y_scale = LinearScale::new().with_domain(0.0, max_val * 1.1).with_range(chart_rect.size.y, 0.0);

        let baseline_y = chart_rect.pos.y + chart_rect.size.y;

        // Draw filled area first (behind line)
        let data_len = self.data.len();
        for i in 0..data_len - 1 {
            let seg_progress = ((progress - i as f64 * 0.08) / 0.5).clamp(0.0, 1.0);
            if seg_progress <= 0.0 { continue; }

            let x1 = chart_rect.pos.x + x_scale.scale(i as f64);
            let y1 = chart_rect.pos.y + y_scale.scale(self.data[i]);
            let x2 = chart_rect.pos.x + x_scale.scale((i + 1) as f64);
            let y2 = chart_rect.pos.y + y_scale.scale(self.data[i + 1]);

            let animated_x2 = x1 + (x2 - x1) * seg_progress;
            let animated_y2 = y1 + (y2 - y1) * seg_progress;

            // Gradient fill color
            let t = i as f32 / (data_len - 1) as f32;
            let fill_color = vec4(
                0.26 + t * 0.66,
                0.52 - t * 0.26,
                0.96 - t * 0.75,
                0.25,
            );

            self.draw_fill.color = fill_color;
            self.draw_fill.disable_gradient();

            // Draw fill triangles
            self.draw_fill.draw_triangle(cx, dvec2(x1, y1), dvec2(animated_x2, animated_y2), dvec2(x1, baseline_y));
            self.draw_fill.draw_triangle(cx, dvec2(animated_x2, animated_y2), dvec2(animated_x2, baseline_y), dvec2(x1, baseline_y));
        }

        // Draw gradient line
        for i in 0..data_len - 1 {
            let seg_progress = ((progress - i as f64 * 0.08) / 0.5).clamp(0.0, 1.0);
            if seg_progress <= 0.0 { continue; }

            let t = i as f32 / (data_len - 1) as f32;
            self.draw_line.color = vec4(
                0.26 + t * 0.66,
                0.52 - t * 0.26,
                0.96 - t * 0.75,
                1.0,
            );

            let x1 = chart_rect.pos.x + x_scale.scale(i as f64);
            let y1 = chart_rect.pos.y + y_scale.scale(self.data[i]);
            let x2 = chart_rect.pos.x + x_scale.scale((i + 1) as f64);
            let y2 = chart_rect.pos.y + y_scale.scale(self.data[i + 1]);

            let end_x = x1 + (x2 - x1) * seg_progress;
            let end_y = y1 + (y2 - y1) * seg_progress;

            self.draw_line.draw_line(cx, dvec2(x1, y1), dvec2(end_x, end_y), 4.0);
        }

        // Draw points
        for (i, &val) in self.data.iter().enumerate() {
            let point_progress = ((progress - i as f64 * 0.08) / 0.3).clamp(0.0, 1.0);
            if point_progress > 0.0 {
                let x = chart_rect.pos.x + x_scale.scale(i as f64);
                let y = chart_rect.pos.y + y_scale.scale(val);

                let t = i as f32 / (data_len - 1) as f32;
                let color = vec4(
                    0.26 + t * 0.66,
                    0.52 - t * 0.26,
                    0.96 - t * 0.75,
                    1.0,
                );

                // Outer glow
                self.draw_point.color = vec4(color.x, color.y, color.z, 0.3);
                self.draw_point.draw_point(cx, dvec2(x, y), 10.0 * point_progress);

                // Main point
                self.draw_point.color = color;
                self.draw_point.draw_point(cx, dvec2(x, y), 5.0 * point_progress);
            }
        }
    }
}

// ============ Helper Functions ============

fn catmull_rom_point(points: &[DVec2], t: f64) -> DVec2 {
    let n = points.len();
    if n < 2 {
        return points.get(0).copied().unwrap_or(dvec2(0.0, 0.0));
    }

    let segment_count = n - 1;
    let segment_t = t * segment_count as f64;
    let segment_idx = (segment_t as usize).min(segment_count - 1);
    let local_t = segment_t - segment_idx as f64;

    let p0 = if segment_idx > 0 { points[segment_idx - 1] } else { points[0] };
    let p1 = points[segment_idx];
    let p2 = points[(segment_idx + 1).min(n - 1)];
    let p3 = if segment_idx + 2 < n { points[segment_idx + 2] } else { points[n - 1] };

    let t2 = local_t * local_t;
    let t3 = t2 * local_t;

    let x = 0.5 * ((2.0 * p1.x)
        + (-p0.x + p2.x) * local_t
        + (2.0 * p0.x - 5.0 * p1.x + 4.0 * p2.x - p3.x) * t2
        + (-p0.x + 3.0 * p1.x - 3.0 * p2.x + p3.x) * t3);

    let y = 0.5 * ((2.0 * p1.y)
        + (-p0.y + p2.y) * local_t
        + (2.0 * p0.y - 5.0 * p1.y + 4.0 * p2.y - p3.y) * t2
        + (-p0.y + 3.0 * p1.y - 3.0 * p2.y + p3.y) * t3);

    dvec2(x, y)
}
