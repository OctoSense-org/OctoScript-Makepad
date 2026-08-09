//! Histogram Widget
//!
//! GPU-accelerated histogram with animated bar reveal and axis support.

use makepad_widgets::*;
use makepad_d3::prelude::*;
use super::draw_primitives::{DrawTriangle, DrawChartLine};
use super::animation::{ChartAnimator, EasingType};
use super::axis_renderer::{DrawAxisText, AxisRendererConfig, render_axis};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawTriangle;
    use super::draw_primitives::DrawChartLine;
    use super::axis_renderer::DrawAxisText;

    pub HistogramWidget = {{HistogramWidget}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct HistogramWidget {
    #[redraw]
    #[live]
    draw_bar: DrawTriangle,

    #[redraw]
    #[live]
    draw_line: DrawChartLine,

    #[redraw]
    #[live]
    draw_axis_text: DrawAxisText,

    #[walk]
    walk: Walk,

    #[rust]
    raw_data: Vec<f64>,

    #[rust]
    bins: Vec<HistogramBin>,

    #[rust]
    bin_count: usize,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,
}

#[derive(Clone)]
struct HistogramBin {
    x0: f64,
    x1: f64,
    count: usize,
}

impl Widget for HistogramWidget {
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
                self.initialize_demo_data();
                self.start_animation(cx);
                self.initialized = true;
            }

            self.draw_histogram(cx, rect);
        }

        DrawStep::done()
    }
}

impl HistogramWidget {
    fn initialize_demo_data(&mut self) {
        self.raw_data = Self::generate_normal_distribution(500, 50.0, 15.0);
        self.bin_count = 20;
        self.compute_bins();
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1200.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    fn generate_normal_distribution(n: usize, mean: f64, std_dev: f64) -> Vec<f64> {
        let mut data = Vec::with_capacity(n);
        for i in 0..n {
            let u1 = (i as f64 + 1.0) / (n as f64 + 2.0);
            let u2 = ((i * 7 + 3) % n) as f64 / n as f64;
            let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
            data.push(mean + z * std_dev);
        }
        data
    }

    fn compute_bins(&mut self) {
        if self.raw_data.is_empty() {
            return;
        }

        let min_val = self.raw_data.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_val = self.raw_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let range = max_val - min_val;
        let bin_width = range / self.bin_count as f64;

        self.bins = (0..self.bin_count)
            .map(|i| {
                let x0 = min_val + i as f64 * bin_width;
                let x1 = x0 + bin_width;
                let count = self.raw_data
                    .iter()
                    .filter(|&&v| v >= x0 && (v < x1 || (i == self.bin_count - 1 && v <= x1)))
                    .count();
                HistogramBin { x0, x1, count }
            })
            .collect();
    }

    fn draw_histogram(&mut self, cx: &mut Cx2d, rect: Rect) {
        let progress = self.animator.get_progress();

        // Larger padding for axes
        let padding_left = 50.0;
        let padding_bottom = 40.0;
        let padding_top = 20.0;
        let padding_right = 20.0;

        let chart_x = rect.pos.x + padding_left;
        let chart_y = rect.pos.y + padding_top;
        let chart_width = rect.size.x - padding_left - padding_right;
        let chart_height = rect.size.y - padding_top - padding_bottom;

        if chart_width <= 0.0 || chart_height <= 0.0 || self.bins.is_empty() {
            return;
        }

        let max_count = self.bins.iter().map(|b| b.count).max().unwrap_or(1);
        let min_x = self.bins.first().map(|b| b.x0).unwrap_or(0.0);
        let max_x = self.bins.last().map(|b| b.x1).unwrap_or(100.0);

        let x_scale = LinearScale::new()
            .with_domain(min_x, max_x)
            .with_range(0.0, chart_width as f64);

        let y_scale = LinearScale::new()
            .with_domain(0.0, max_count as f64 * 1.1)
            .with_range(chart_height as f64, 0.0);

        // Draw axes first
        self.draw_axes(cx, chart_x, chart_y, chart_width, chart_height, &x_scale, &y_scale);

        // Clone bins to avoid borrow issues
        let bins: Vec<_> = self.bins.clone();

        // Draw bars with staggered animation
        let bar_gap = 1.0;
        for (i, bin) in bins.iter().enumerate() {
            let bar_progress = ((progress - i as f64 * 0.02) / 0.4).clamp(0.0, 1.0);
            if bar_progress <= 0.0 {
                continue;
            }

            let x = chart_x as f64 + x_scale.scale(bin.x0);
            let width = x_scale.scale(bin.x1) - x_scale.scale(bin.x0) - bar_gap;
            let full_height = chart_height as f64 - y_scale.scale(bin.count as f64);
            let height = full_height * bar_progress;
            let y = chart_y as f64 + chart_height as f64 - height;

            // Color based on density
            let intensity = bin.count as f32 / max_count as f32;
            let color = vec4(
                0.26 + intensity * 0.1,
                0.52 + intensity * 0.1,
                0.96,
                0.85 + intensity * 0.15,
            );

            // Draw bar
            self.draw_bar.color = color;
            self.draw_bar.disable_gradient();

            let p1 = dvec2(x, y);
            let p2 = dvec2(x + width.max(1.0), y);
            let p3 = dvec2(x + width.max(1.0), y + height);
            let p4 = dvec2(x, y + height);

            self.draw_bar.draw_triangle(cx, p1, p2, p3);
            self.draw_bar.draw_triangle(cx, p1, p3, p4);

            // Draw border
            if bar_progress > 0.7 {
                self.draw_line.color = vec4(0.15, 0.35, 0.75, 0.5);
                self.draw_line.draw_line(cx, p1, p2, 1.0);
                self.draw_line.draw_line(cx, p2, p3, 1.0);
                self.draw_line.draw_line(cx, p3, p4, 1.0);
                self.draw_line.draw_line(cx, p4, p1, 1.0);
            }
        }
    }

    fn draw_axes(
        &mut self,
        cx: &mut Cx2d,
        chart_x: f64,
        chart_y: f64,
        chart_width: f64,
        chart_height: f64,
        x_scale: &LinearScale,
        y_scale: &LinearScale,
    ) {
        // Create axis configurations
        let x_config = AxisConfig::bottom()
            .with_tick_size(6.0)
            .with_tick_padding(8.0);

        let y_config = AxisConfig::left()
            .with_tick_size(6.0)
            .with_tick_padding(8.0)
            .with_grid(chart_width);

        // Create and configure axes
        let mut x_axis = Axis::with_config(x_config);
        x_axis.set_scale(x_scale);

        let mut y_axis = Axis::with_config(y_config);
        y_axis.set_scale(y_scale);

        // Compute layouts
        let x_layout = x_axis.compute_layout(chart_height);
        let y_layout = y_axis.compute_layout(0.0);

        // Render config
        let axis_config = AxisRendererConfig::with_grid()
            .grid_color(vec4(0.92, 0.92, 0.92, 0.6));

        // Offset for chart position
        let offset = dvec2(chart_x, chart_y);

        // Render Y-axis with grid
        render_axis(cx, &mut self.draw_line, &mut self.draw_axis_text, &y_layout, offset, &axis_config);

        // Render X-axis without grid (bins already form a visual rhythm)
        let x_axis_config = AxisRendererConfig::without_grid();
        render_axis(cx, &mut self.draw_line, &mut self.draw_axis_text, &x_layout, offset, &x_axis_config);
    }

    pub fn set_data(&mut self, data: Vec<f64>, bin_count: usize) {
        self.raw_data = data;
        self.bin_count = bin_count;
        self.compute_bins();
        self.initialized = false;
    }
}
