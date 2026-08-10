//! Parallel Coordinates Chart Widget
//!
//! Multi-dimensional data visualization with smooth GPU-accelerated lines.
//! Features animated line drawing, hover effects, and axis highlighting.

use makepad_widgets::*;
use std::f64::consts::PI;
use super::draw_primitives::{DrawChartLine, DrawPoint};
use super::animation::{ChartAnimator, EasingType, get_color};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawChartLine;
    use super::draw_primitives::DrawPoint;

    pub ParallelCoordsWidget = {{ParallelCoordsWidget}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Clone)]
struct DataItem {
    values: Vec<f64>,  // Normalized 0-1 values for each dimension
    color: Vec4,
}

#[derive(Live, LiveHook, Widget)]
pub struct ParallelCoordsWidget {
    #[redraw]
    #[live]
    draw_line: DrawChartLine,

    #[redraw]
    #[live]
    draw_point: DrawPoint,

    #[walk]
    walk: Walk,

    #[rust]
    dimensions: Vec<String>,

    #[rust]
    items: Vec<DataItem>,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,

    #[rust]
    chart_rect: Rect,

    #[rust(true)]
    show_smooth_curves: bool,

    #[rust(true)]
    show_points: bool,

    #[rust]
    hovered_item: Option<usize>,
}

impl Widget for ParallelCoordsWidget {
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

            self.draw_parallel_coords(cx);
        }

        DrawStep::done()
    }
}

impl ParallelCoordsWidget {
    fn initialize_data(&mut self) {
        // Example: Car comparison data (normalized)
        self.dimensions = vec![
            "Price".to_string(),
            "MPG".to_string(),
            "Horsepower".to_string(),
            "Weight".to_string(),
            "Acceleration".to_string(),
        ];

        // Sample data items (cars) with vibrant colors
        self.items = vec![
            DataItem {
                values: vec![0.3, 0.8, 0.4, 0.5, 0.7],
                color: vec4(0.26, 0.52, 0.96, 0.85),
            },
            DataItem {
                values: vec![0.7, 0.4, 0.9, 0.8, 0.3],
                color: vec4(0.92, 0.26, 0.21, 0.85),
            },
            DataItem {
                values: vec![0.5, 0.6, 0.6, 0.6, 0.5],
                color: vec4(0.20, 0.66, 0.33, 0.85),
            },
            DataItem {
                values: vec![0.2, 0.9, 0.3, 0.3, 0.8],
                color: vec4(1.0, 0.76, 0.03, 0.85),
            },
            DataItem {
                values: vec![0.9, 0.2, 0.95, 0.9, 0.2],
                color: vec4(0.61, 0.15, 0.69, 0.85),
            },
            DataItem {
                values: vec![0.4, 0.7, 0.5, 0.4, 0.6],
                color: vec4(0.10, 0.74, 0.61, 0.85),
            },
            DataItem {
                values: vec![0.6, 0.5, 0.7, 0.7, 0.4],
                color: vec4(0.95, 0.61, 0.07, 0.85),
            },
            DataItem {
                values: vec![0.8, 0.3, 0.85, 0.85, 0.25],
                color: vec4(0.56, 0.27, 0.68, 0.85),
            },
        ];
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1500.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    pub fn replay_animation(&mut self, cx: &mut Cx) {
        self.animator.reset();
        self.start_animation(cx);
        self.redraw(cx);
    }

    fn draw_parallel_coords(&mut self, cx: &mut Cx2d) {
        let progress = self.animator.get_progress();
        let rect = self.chart_rect;

        let padding_x = 60.0;
        let padding_y = 50.0;
        let chart_x = rect.pos.x + padding_x;
        let chart_y = rect.pos.y + padding_y;
        let chart_w = rect.size.x - padding_x * 2.0;
        let chart_h = rect.size.y - padding_y * 2.0;

        if chart_w <= 0.0 || chart_h <= 0.0 || self.dimensions.is_empty() {
            return;
        }

        let n_dims = self.dimensions.len();
        let axis_spacing = chart_w / (n_dims - 1) as f64;

        // Draw background grid
        self.draw_grid(cx, chart_x, chart_y, chart_w, chart_h, n_dims, axis_spacing);

        // Draw axes
        self.draw_axes(cx, chart_x, chart_y, chart_h, n_dims, axis_spacing, progress);

        // Clone items to avoid borrow issues
        let items: Vec<_> = self.items.clone();

        // Draw data lines with animation
        for (item_idx, item) in items.iter().enumerate() {
            // Stagger animation per item
            let item_progress = ((progress - item_idx as f64 * 0.08) / 0.6).clamp(0.0, 1.0);
            if item_progress <= 0.0 {
                continue;
            }

            let is_hovered = self.hovered_item == Some(item_idx);
            let line_alpha = if is_hovered { 1.0 } else { 0.7 };
            let line_width = if is_hovered { 3.0 } else { 2.0 };

            let color = vec4(item.color.x, item.color.y, item.color.z, line_alpha as f32);

            if self.show_smooth_curves {
                self.draw_smooth_polyline(
                    cx, &item.values, chart_x, chart_y, chart_w, chart_h,
                    axis_spacing, item_progress, color, line_width,
                );
            } else {
                self.draw_straight_polyline(
                    cx, &item.values, chart_x, chart_y, chart_w, chart_h,
                    axis_spacing, item_progress, color, line_width,
                );
            }

            // Draw data points on axes
            if self.show_points {
                self.draw_data_points(
                    cx, &item.values, chart_x, chart_y, chart_w, chart_h,
                    axis_spacing, item_progress, color, is_hovered,
                );
            }
        }
    }

    fn draw_grid(
        &mut self,
        cx: &mut Cx2d,
        chart_x: f64,
        chart_y: f64,
        chart_w: f64,
        chart_h: f64,
        n_dims: usize,
        axis_spacing: f64,
    ) {
        // Draw horizontal grid lines
        self.draw_line.color = vec4(0.9, 0.9, 0.9, 0.4);
        for i in 0..=5 {
            let y = chart_y + (i as f64 / 5.0) * chart_h;
            self.draw_line.draw_line(
                cx,
                dvec2(chart_x, y),
                dvec2(chart_x + chart_w, y),
                1.0,
            );
        }
    }

    fn draw_axes(
        &mut self,
        cx: &mut Cx2d,
        chart_x: f64,
        chart_y: f64,
        chart_h: f64,
        n_dims: usize,
        axis_spacing: f64,
        progress: f64,
    ) {
        for i in 0..n_dims {
            let axis_progress = ((progress - i as f64 * 0.05) / 0.3).clamp(0.0, 1.0);
            if axis_progress <= 0.0 {
                continue;
            }

            let x = chart_x + i as f64 * axis_spacing;

            // Draw vertical axis line with animation
            let animated_height = chart_h * axis_progress;
            self.draw_line.color = vec4(0.4, 0.4, 0.5, 0.9);
            self.draw_line.draw_line(
                cx,
                dvec2(x, chart_y + chart_h),
                dvec2(x, chart_y + chart_h - animated_height),
                2.5,
            );

            // Draw tick marks
            if axis_progress > 0.5 {
                self.draw_line.color = vec4(0.5, 0.5, 0.6, 0.7);
                for t in 0..=5 {
                    let tick_progress = ((axis_progress - 0.5) * 2.0 - t as f64 * 0.1).clamp(0.0, 1.0);
                    if tick_progress <= 0.0 {
                        continue;
                    }

                    let y = chart_y + (t as f64 / 5.0) * chart_h;
                    let tick_width = 6.0 * tick_progress;
                    self.draw_line.draw_line(
                        cx,
                        dvec2(x - tick_width, y),
                        dvec2(x + tick_width, y),
                        1.5,
                    );
                }
            }

            // Draw axis endpoint circles
            self.draw_point.color = vec4(0.4, 0.4, 0.5, 0.9);
            self.draw_point.draw_point(cx, dvec2(x, chart_y), 4.0 * axis_progress);
            self.draw_point.draw_point(cx, dvec2(x, chart_y + chart_h), 4.0 * axis_progress);
        }
    }

    fn draw_smooth_polyline(
        &mut self,
        cx: &mut Cx2d,
        values: &[f64],
        chart_x: f64,
        chart_y: f64,
        chart_w: f64,
        chart_h: f64,
        axis_spacing: f64,
        progress: f64,
        color: Vec4,
        line_width: f64,
    ) {
        let n_dims = values.len();
        if n_dims < 2 {
            return;
        }

        // Calculate control points for smooth curve
        let points: Vec<DVec2> = values.iter().enumerate().map(|(i, &v)| {
            let x = chart_x + i as f64 * axis_spacing;
            let y = chart_y + chart_h - v * chart_h;
            dvec2(x, y)
        }).collect();

        // Draw smooth curve through all points using Catmull-Rom spline
        let total_segments = (n_dims - 1) * 16;
        let draw_segments = ((total_segments as f64 * progress) as usize).max(1);

        self.draw_line.color = color;

        for seg in 0..draw_segments {
            let t_global = seg as f64 / total_segments as f64;
            let t_global_next = (seg + 1) as f64 / total_segments as f64;

            let p1 = self.catmull_rom_point(&points, t_global);
            let p2 = self.catmull_rom_point(&points, t_global_next);

            self.draw_line.draw_line(cx, p1, p2, line_width);
        }
    }

    fn catmull_rom_point(&self, points: &[DVec2], t: f64) -> DVec2 {
        let n = points.len();
        if n < 2 {
            return points.get(0).copied().unwrap_or(dvec2(0.0, 0.0));
        }

        // Find which segment we're in
        let segment_count = n - 1;
        let segment_t = t * segment_count as f64;
        let segment_idx = (segment_t as usize).min(segment_count - 1);
        let local_t = segment_t - segment_idx as f64;

        // Get the four control points
        let p0 = if segment_idx > 0 { points[segment_idx - 1] } else { points[0] };
        let p1 = points[segment_idx];
        let p2 = points[(segment_idx + 1).min(n - 1)];
        let p3 = if segment_idx + 2 < n { points[segment_idx + 2] } else { points[n - 1] };

        // Catmull-Rom interpolation
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

    fn draw_straight_polyline(
        &mut self,
        cx: &mut Cx2d,
        values: &[f64],
        chart_x: f64,
        chart_y: f64,
        _chart_w: f64,
        chart_h: f64,
        axis_spacing: f64,
        progress: f64,
        color: Vec4,
        line_width: f64,
    ) {
        let n_dims = values.len();
        if n_dims < 2 {
            return;
        }

        self.draw_line.color = color;

        let total_segments = n_dims - 1;
        let draw_segments = ((total_segments as f64 * progress) as usize).max(1);

        for i in 0..draw_segments {
            let x1 = chart_x + i as f64 * axis_spacing;
            let y1 = chart_y + chart_h - values[i] * chart_h;
            let x2 = chart_x + (i + 1) as f64 * axis_spacing;
            let y2 = chart_y + chart_h - values[i + 1] * chart_h;

            // Partial last segment
            let seg_progress = if i == draw_segments - 1 {
                (progress * total_segments as f64 - i as f64).clamp(0.0, 1.0)
            } else {
                1.0
            };

            let end_x = x1 + (x2 - x1) * seg_progress;
            let end_y = y1 + (y2 - y1) * seg_progress;

            self.draw_line.draw_line(cx, dvec2(x1, y1), dvec2(end_x, end_y), line_width);
        }
    }

    fn draw_data_points(
        &mut self,
        cx: &mut Cx2d,
        values: &[f64],
        chart_x: f64,
        chart_y: f64,
        _chart_w: f64,
        chart_h: f64,
        axis_spacing: f64,
        progress: f64,
        color: Vec4,
        is_hovered: bool,
    ) {
        for (i, &value) in values.iter().enumerate() {
            let point_progress = ((progress - i as f64 * 0.1) / 0.3).clamp(0.0, 1.0);
            if point_progress <= 0.0 {
                continue;
            }

            let x = chart_x + i as f64 * axis_spacing;
            let y = chart_y + chart_h - value * chart_h;

            let base_radius = if is_hovered { 6.0 } else { 4.0 };
            let radius = base_radius * point_progress;

            // Outer glow
            self.draw_point.color = vec4(color.x, color.y, color.z, 0.3);
            self.draw_point.draw_point(cx, dvec2(x, y), radius * 1.8);

            // Main point
            self.draw_point.color = color;
            self.draw_point.draw_point(cx, dvec2(x, y), radius);

            // Inner highlight
            self.draw_point.color = vec4(1.0, 1.0, 1.0, 0.5);
            self.draw_point.draw_point(cx, dvec2(x - radius * 0.2, y - radius * 0.2), radius * 0.4);
        }
    }
}
