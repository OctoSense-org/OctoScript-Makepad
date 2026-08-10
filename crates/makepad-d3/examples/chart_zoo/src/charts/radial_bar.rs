//! Radial Bar Chart Widget
//!
//! GPU-accelerated circular bar chart with smooth animations.
//! Features gradient fills, inner labels, and animated arc sweeps.

use makepad_widgets::*;
use std::f64::consts::PI;
use super::draw_primitives::{DrawChartLine, DrawTriangle, DrawPoint};
use super::animation::{ChartAnimator, EasingType};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawChartLine;
    use super::draw_primitives::DrawTriangle;
    use super::draw_primitives::DrawPoint;

    pub RadialBarWidget = {{RadialBarWidget}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Clone)]
struct RadialBarData {
    label: String,
    value: f64,
    color: Vec4,
}

#[derive(Live, LiveHook, Widget)]
pub struct RadialBarWidget {
    #[redraw]
    #[live]
    draw_arc: DrawTriangle,

    #[redraw]
    #[live]
    draw_line: DrawChartLine,

    #[redraw]
    #[live]
    draw_point: DrawPoint,

    #[walk]
    walk: Walk,

    #[rust]
    data: Vec<RadialBarData>,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,

    #[rust]
    chart_rect: Rect,

    #[rust(true)]
    show_grid: bool,

    #[rust(true)]
    gradient_bars: bool,

    #[rust]
    hovered_bar: Option<usize>,
}

impl Widget for RadialBarWidget {
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
            self.chart_rect = rect;

            if !self.initialized {
                self.initialize_data();
                self.start_animation(cx);
                self.initialized = true;
            }

            self.draw_radial_bars(cx);
        }

        DrawStep::done()
    }
}

impl RadialBarWidget {
    fn initialize_data(&mut self) {
        // Monthly data in radial format with gradient colors
        self.data = vec![
            RadialBarData { label: "Jan".to_string(), value: 65.0, color: vec4(0.26, 0.52, 0.96, 1.0) },
            RadialBarData { label: "Feb".to_string(), value: 72.0, color: vec4(0.30, 0.56, 0.98, 1.0) },
            RadialBarData { label: "Mar".to_string(), value: 80.0, color: vec4(0.34, 0.60, 1.00, 1.0) },
            RadialBarData { label: "Apr".to_string(), value: 88.0, color: vec4(0.20, 0.66, 0.33, 1.0) },
            RadialBarData { label: "May".to_string(), value: 95.0, color: vec4(0.24, 0.70, 0.37, 1.0) },
            RadialBarData { label: "Jun".to_string(), value: 100.0, color: vec4(0.28, 0.74, 0.41, 1.0) },
            RadialBarData { label: "Jul".to_string(), value: 92.0, color: vec4(1.0, 0.76, 0.03, 1.0) },
            RadialBarData { label: "Aug".to_string(), value: 85.0, color: vec4(1.0, 0.70, 0.00, 1.0) },
            RadialBarData { label: "Sep".to_string(), value: 78.0, color: vec4(1.0, 0.60, 0.00, 1.0) },
            RadialBarData { label: "Oct".to_string(), value: 70.0, color: vec4(0.92, 0.26, 0.21, 1.0) },
            RadialBarData { label: "Nov".to_string(), value: 62.0, color: vec4(0.88, 0.22, 0.17, 1.0) },
            RadialBarData { label: "Dec".to_string(), value: 58.0, color: vec4(0.84, 0.18, 0.13, 1.0) },
        ];
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1500.0)
            .with_easing(EasingType::EaseOutElastic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    pub fn replay_animation(&mut self, cx: &mut Cx) {
        self.animator.reset();
        self.start_animation(cx);
        self.redraw(cx);
    }

    fn draw_radial_bars(&mut self, cx: &mut Cx2d) {
        let progress = self.animator.get_progress();
        let rect = self.chart_rect;

        let center = dvec2(
            rect.pos.x + rect.size.x / 2.0,
            rect.pos.y + rect.size.y / 2.0,
        );
        let max_radius = (rect.size.x.min(rect.size.y) / 2.0 - 30.0).max(10.0);
        let inner_radius = max_radius * 0.3;

        if self.data.is_empty() {
            return;
        }

        // Draw grid circles if enabled
        if self.show_grid {
            self.draw_grid_circles(cx, center, inner_radius, max_radius, progress);
        }

        let max_value = self.data.iter().map(|d| d.value).fold(0.0f64, f64::max);
        let n = self.data.len();
        let bar_angle = (2.0 * PI) / n as f64;
        let gap_angle = bar_angle * 0.08;

        // Clone data to avoid borrow issues
        let data: Vec<_> = self.data.clone();

        // Draw each bar
        for (i, bar) in data.iter().enumerate() {
            // Stagger animation per bar
            let bar_progress = ((progress - i as f64 * 0.05) / 0.7).clamp(0.0, 1.0);
            if bar_progress <= 0.0 {
                continue;
            }

            let start_angle = i as f64 * bar_angle - PI / 2.0;
            let sweep = (bar_angle - gap_angle) * bar_progress;
            let bar_radius = inner_radius + (max_radius - inner_radius) * (bar.value / max_value) * bar_progress;

            let is_hovered = self.hovered_bar == Some(i);
            let scale = if is_hovered { 1.05 } else { 1.0 };

            // Draw the arc using triangles
            self.draw_arc_segment(
                cx,
                center,
                inner_radius * scale,
                bar_radius * scale,
                start_angle,
                sweep,
                bar.color,
                bar_progress,
            );

            // Draw outer edge highlight
            if bar_progress > 0.5 {
                let highlight_alpha = ((bar_progress - 0.5) * 2.0) as f32;
                self.draw_line.color = vec4(1.0, 1.0, 1.0, 0.4 * highlight_alpha);
                self.draw_arc_outline(cx, center, bar_radius * scale, start_angle, sweep, 1.5);
            }
        }

        // Draw center circle
        self.draw_center_circle(cx, center, inner_radius * 0.9, progress);
    }

    fn draw_grid_circles(
        &mut self,
        cx: &mut Cx2d,
        center: DVec2,
        inner_radius: f64,
        max_radius: f64,
        progress: f64,
    ) {
        let grid_progress = (progress * 2.0).clamp(0.0, 1.0);

        // Draw concentric circles
        self.draw_line.color = vec4(0.85, 0.85, 0.85, 0.4 * grid_progress as f32);
        for i in 1..=4 {
            let r = inner_radius + (max_radius - inner_radius) * (i as f64 / 4.0);
            self.draw_circle_outline(cx, center, r * grid_progress, 1.0);
        }

        // Draw radial lines
        let n_lines = 12;
        self.draw_line.color = vec4(0.85, 0.85, 0.85, 0.3 * grid_progress as f32);
        for i in 0..n_lines {
            let angle = i as f64 * (2.0 * PI / n_lines as f64) - PI / 2.0;
            let outer = dvec2(
                center.x + max_radius * grid_progress * angle.cos(),
                center.y + max_radius * grid_progress * angle.sin(),
            );
            let inner = dvec2(
                center.x + inner_radius * grid_progress * angle.cos(),
                center.y + inner_radius * grid_progress * angle.sin(),
            );
            self.draw_line.draw_line(cx, inner, outer, 1.0);
        }
    }

    fn draw_arc_segment(
        &mut self,
        cx: &mut Cx2d,
        center: DVec2,
        inner_r: f64,
        outer_r: f64,
        start_angle: f64,
        sweep: f64,
        color: Vec4,
        progress: f64,
    ) {
        // Number of segments for smooth arc
        let segments = ((sweep.abs() * 20.0) as usize).max(8);

        // Set up gradient if enabled
        if self.gradient_bars {
            let inner_color = vec4(
                (color.x + 0.2).min(1.0),
                (color.y + 0.2).min(1.0),
                (color.z + 0.2).min(1.0),
                color.w,
            );
            let outer_color = vec4(
                color.x * 0.8,
                color.y * 0.8,
                color.z * 0.8,
                color.w,
            );
            self.draw_arc.set_radial_gradient(inner_color, outer_color);
        } else {
            self.draw_arc.color = color;
            self.draw_arc.disable_gradient();
        }

        // Draw arc as triangle fan
        for i in 0..segments {
            let a1 = start_angle + sweep * (i as f64 / segments as f64);
            let a2 = start_angle + sweep * ((i + 1) as f64 / segments as f64);

            // Inner points
            let inner1 = dvec2(
                center.x + inner_r * a1.cos(),
                center.y + inner_r * a1.sin(),
            );
            let inner2 = dvec2(
                center.x + inner_r * a2.cos(),
                center.y + inner_r * a2.sin(),
            );

            // Outer points
            let outer1 = dvec2(
                center.x + outer_r * a1.cos(),
                center.y + outer_r * a1.sin(),
            );
            let outer2 = dvec2(
                center.x + outer_r * a2.cos(),
                center.y + outer_r * a2.sin(),
            );

            // Draw two triangles for each segment (quad)
            self.draw_arc.draw_triangle(cx, inner1, outer1, outer2);
            self.draw_arc.draw_triangle(cx, inner1, outer2, inner2);
        }

        // Draw end caps for rounded appearance
        if sweep.abs() > 0.01 {
            self.draw_end_cap(cx, center, inner_r, outer_r, start_angle, color);
            self.draw_end_cap(cx, center, inner_r, outer_r, start_angle + sweep, color);
        }
    }

    fn draw_end_cap(
        &mut self,
        cx: &mut Cx2d,
        center: DVec2,
        inner_r: f64,
        outer_r: f64,
        angle: f64,
        color: Vec4,
    ) {
        let mid_r = (inner_r + outer_r) / 2.0;
        let cap_r = (outer_r - inner_r) / 2.0 * 0.8;

        let cap_center = dvec2(
            center.x + mid_r * angle.cos(),
            center.y + mid_r * angle.sin(),
        );

        // Draw small circle at end
        self.draw_point.color = color;
        self.draw_point.draw_point(cx, cap_center, cap_r);
    }

    fn draw_arc_outline(
        &mut self,
        cx: &mut Cx2d,
        center: DVec2,
        radius: f64,
        start_angle: f64,
        sweep: f64,
        thickness: f64,
    ) {
        let segments = ((sweep.abs() * 20.0) as usize).max(8);

        for i in 0..segments {
            let a1 = start_angle + sweep * (i as f64 / segments as f64);
            let a2 = start_angle + sweep * ((i + 1) as f64 / segments as f64);

            let p1 = dvec2(
                center.x + radius * a1.cos(),
                center.y + radius * a1.sin(),
            );
            let p2 = dvec2(
                center.x + radius * a2.cos(),
                center.y + radius * a2.sin(),
            );

            self.draw_line.draw_line(cx, p1, p2, thickness);
        }
    }

    fn draw_circle_outline(
        &mut self,
        cx: &mut Cx2d,
        center: DVec2,
        radius: f64,
        thickness: f64,
    ) {
        self.draw_arc_outline(cx, center, radius, 0.0, 2.0 * PI, thickness);
    }

    fn draw_center_circle(
        &mut self,
        cx: &mut Cx2d,
        center: DVec2,
        radius: f64,
        progress: f64,
    ) {
        let circle_progress = ((progress - 0.3) / 0.4).clamp(0.0, 1.0);
        if circle_progress <= 0.0 {
            return;
        }

        let animated_radius = radius * circle_progress;

        // Draw filled center
        self.draw_arc.color = vec4(0.98, 0.98, 0.98, 0.95);
        self.draw_arc.disable_gradient();

        // Draw as triangle fan from center
        let segments = 32;
        for i in 0..segments {
            let a1 = 2.0 * PI * (i as f64 / segments as f64);
            let a2 = 2.0 * PI * ((i + 1) as f64 / segments as f64);

            let p1 = dvec2(
                center.x + animated_radius * a1.cos(),
                center.y + animated_radius * a1.sin(),
            );
            let p2 = dvec2(
                center.x + animated_radius * a2.cos(),
                center.y + animated_radius * a2.sin(),
            );

            self.draw_arc.draw_triangle(cx, center, p1, p2);
        }

        // Draw border
        self.draw_line.color = vec4(0.7, 0.7, 0.7, 0.6 * circle_progress as f32);
        self.draw_circle_outline(cx, center, animated_radius, 2.0);

        // Draw inner highlight
        self.draw_point.color = vec4(1.0, 1.0, 1.0, 0.4 * circle_progress as f32);
        self.draw_point.draw_point(
            cx,
            dvec2(center.x - animated_radius * 0.3, center.y - animated_radius * 0.3),
            animated_radius * 0.3,
        );
    }

    pub fn set_data(&mut self, data: Vec<(String, f64, Vec4)>) {
        self.data = data
            .into_iter()
            .map(|(label, value, color)| RadialBarData { label, value, color })
            .collect();
        self.initialized = false;
    }
}
