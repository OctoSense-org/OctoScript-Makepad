//! Bar Chart Widget
//!
//! GPU-accelerated bar chart with smooth animations and axis support.

use makepad_widgets::*;
use makepad_d3::prelude::*;
use super::draw_primitives::{DrawTriangle, DrawChartLine};
use super::animation::{ChartAnimator, EasingType, get_color};
use super::axis_renderer::{DrawAxisText, AxisRendererConfig, render_axis};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawTriangle;
    use super::draw_primitives::DrawChartLine;
    use super::axis_renderer::DrawAxisText;

    pub BarChartWidget = {{BarChartWidget}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BarChartWidget {
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
    data: Vec<f64>,

    #[rust]
    labels: Vec<String>,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,

    #[rust]
    chart_rect: Rect,

    #[rust]
    hovered_bar: Option<usize>,
}

impl Widget for BarChartWidget {
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
                self.initialize_demo_data();
                self.start_animation(cx);
                self.initialized = true;
            }

            self.draw_chart(cx, rect);
        }

        DrawStep::done()
    }
}

impl BarChartWidget {
    fn initialize_demo_data(&mut self) {
        self.data = vec![65.0, 45.0, 78.0, 52.0, 88.0, 36.0, 70.0];
        self.labels = vec![
            "Mon".to_string(),
            "Tue".to_string(),
            "Wed".to_string(),
            "Thu".to_string(),
            "Fri".to_string(),
            "Sat".to_string(),
            "Sun".to_string(),
        ];
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1000.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    pub fn replay_animation(&mut self, cx: &mut Cx) {
        self.animator.reset();
        self.start_animation(cx);
        self.redraw(cx);
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
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

        if chart_width <= 0.0 || chart_height <= 0.0 || self.data.is_empty() {
            return;
        }

        // Create scales
        let max_val = self.data.iter().cloned().fold(0.0f64, f64::max);
        let y_scale = LinearScale::new()
            .with_domain(0.0, max_val * 1.1)
            .with_range(chart_height as f64, 0.0);

        let x_scale = LinearScale::new()
            .with_domain(0.0, self.data.len() as f64)
            .with_range(0.0, chart_width as f64);

        // Draw axes first (behind bars)
        self.draw_axes(cx, chart_x, chart_y, chart_width, chart_height, &x_scale, &y_scale);

        // Draw bars
        self.draw_bars(cx, chart_x, chart_y, chart_width, chart_height, &y_scale, progress);
    }

    fn draw_axes(
        &mut self,
        cx: &mut Cx2d,
        chart_x: f64,
        chart_y: f64,
        chart_width: f64,
        chart_height: f64,
        _x_scale: &LinearScale,
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
        let mut y_axis = Axis::with_config(y_config);
        y_axis.set_scale(y_scale);

        // For x-axis, we'll use category labels (manually position)
        let mut x_axis = Axis::with_config(x_config);

        // Create band scale for x-axis to center labels under bars
        let bar_count = self.data.len();
        let band_width = chart_width / bar_count as f64;

        // Manually create ticks for category labels
        let x_ticks: Vec<Tick> = self.labels.iter().enumerate().map(|(i, label)| {
            Tick {
                value: i as f64,
                position: band_width * (i as f64 + 0.5),  // Center of each band
                label: label.clone(),
            }
        }).collect();
        x_axis.set_ticks(x_ticks);
        x_axis.set_range((0.0, chart_width));

        // Compute layouts
        let x_layout = x_axis.compute_layout(chart_height);
        let y_layout = y_axis.compute_layout(0.0);

        // Render config
        let axis_config = AxisRendererConfig::with_grid()
            .grid_color(vec4(0.92, 0.92, 0.92, 0.6));

        // Offset for chart position
        let offset = dvec2(chart_x, chart_y);

        // Render Y-axis
        render_axis(cx, &mut self.draw_line, &mut self.draw_axis_text, &y_layout, offset, &axis_config);

        // Render X-axis (without grid for categorical axis)
        let x_axis_config = AxisRendererConfig::without_grid();
        render_axis(cx, &mut self.draw_line, &mut self.draw_axis_text, &x_layout, offset, &x_axis_config);
    }

    fn draw_bars(
        &mut self,
        cx: &mut Cx2d,
        chart_x: f64,
        chart_y: f64,
        chart_width: f64,
        chart_height: f64,
        y_scale: &LinearScale,
        progress: f64,
    ) {
        let bar_count = self.data.len();
        let bar_spacing = 0.2;
        let band_width = chart_width / bar_count as f64;
        let bar_width = band_width * (1.0 - bar_spacing);
        let bar_offset = band_width * bar_spacing / 2.0;

        // Clone data to avoid borrow issues
        let data: Vec<f64> = self.data.clone();

        // Draw each bar with animation
        for (i, &value) in data.iter().enumerate() {
            let bar_progress = ((progress - i as f64 * 0.08) / 0.5).clamp(0.0, 1.0);
            if bar_progress <= 0.0 {
                continue;
            }

            let x = chart_x + i as f64 * band_width + bar_offset;
            let full_bar_height = chart_height - y_scale.scale(value);
            let bar_height = full_bar_height * bar_progress;
            let y = chart_y + chart_height - bar_height;

            let is_hovered = self.hovered_bar == Some(i);
            let color = get_color(i);
            let draw_color = if is_hovered {
                vec4(
                    (color.x + 0.15).min(1.0),
                    (color.y + 0.15).min(1.0),
                    (color.z + 0.15).min(1.0),
                    color.w,
                )
            } else {
                color
            };

            // Draw bar as two triangles
            self.draw_bar.color = draw_color;
            self.draw_bar.disable_gradient();

            let p1 = dvec2(x, y);
            let p2 = dvec2(x + bar_width, y);
            let p3 = dvec2(x + bar_width, y + bar_height);
            let p4 = dvec2(x, y + bar_height);

            self.draw_bar.draw_triangle(cx, p1, p2, p3);
            self.draw_bar.draw_triangle(cx, p1, p3, p4);

            // Draw border
            if bar_progress > 0.5 {
                let border_alpha = ((bar_progress - 0.5) * 2.0).min(1.0) as f32;
                self.draw_line.color = vec4(0.1, 0.1, 0.12, 0.6 * border_alpha);
                self.draw_line.draw_line(cx, p1, p2, 1.0);
                self.draw_line.draw_line(cx, p2, p3, 1.0);
                self.draw_line.draw_line(cx, p3, p4, 1.0);
                self.draw_line.draw_line(cx, p4, p1, 1.0);
            }
        }
    }

    pub fn set_data(&mut self, data: Vec<f64>) {
        self.data = data;
        self.initialized = false;
    }

    pub fn set_data_with_labels(&mut self, data: Vec<f64>, labels: Vec<String>) {
        self.data = data;
        self.labels = labels;
        self.initialized = false;
    }
}
