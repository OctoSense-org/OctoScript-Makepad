//! Box Plot Widget
//!
//! GPU-accelerated box-and-whisker plots with smooth animations and axis support.

use makepad_widgets::*;
use makepad_d3::prelude::*;
use super::draw_primitives::{DrawTriangle, DrawChartLine, DrawPoint};
use super::animation::{ChartAnimator, EasingType};
use super::axis_renderer::{DrawAxisText, AxisRendererConfig, render_axis};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawTriangle;
    use super::draw_primitives::DrawChartLine;
    use super::draw_primitives::DrawPoint;
    use super::axis_renderer::DrawAxisText;

    pub BoxPlotWidget = {{BoxPlotWidget}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BoxPlotWidget {
    #[redraw]
    #[live]
    draw_box: DrawTriangle,

    #[redraw]
    #[live]
    draw_line: DrawChartLine,

    #[redraw]
    #[live]
    draw_point: DrawPoint,

    #[redraw]
    #[live]
    draw_axis_text: DrawAxisText,

    #[walk]
    walk: Walk,

    #[rust]
    datasets: Vec<BoxPlotData>,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,
}

#[derive(Clone)]
struct BoxPlotData {
    label: String,
    min: f64,
    q1: f64,
    median: f64,
    q3: f64,
    max: f64,
    outliers: Vec<f64>,
    color: Vec4,
}

impl Widget for BoxPlotWidget {
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

            self.draw_box_plots(cx, rect);
        }

        DrawStep::done()
    }
}

impl BoxPlotWidget {
    fn initialize_demo_data(&mut self) {
        self.datasets = vec![
            BoxPlotData {
                label: "Group A".to_string(),
                min: 10.0, q1: 25.0, median: 35.0, q3: 45.0, max: 60.0,
                outliers: vec![5.0, 70.0],
                color: vec4(0.26, 0.52, 0.96, 1.0),
            },
            BoxPlotData {
                label: "Group B".to_string(),
                min: 20.0, q1: 35.0, median: 50.0, q3: 62.0, max: 80.0,
                outliers: vec![15.0],
                color: vec4(0.92, 0.26, 0.21, 1.0),
            },
            BoxPlotData {
                label: "Group C".to_string(),
                min: 15.0, q1: 30.0, median: 42.0, q3: 55.0, max: 70.0,
                outliers: vec![],
                color: vec4(0.20, 0.66, 0.33, 1.0),
            },
            BoxPlotData {
                label: "Group D".to_string(),
                min: 25.0, q1: 40.0, median: 48.0, q3: 58.0, max: 75.0,
                outliers: vec![20.0, 85.0],
                color: vec4(1.0, 0.76, 0.03, 1.0),
            },
            BoxPlotData {
                label: "Group E".to_string(),
                min: 5.0, q1: 22.0, median: 38.0, q3: 52.0, max: 68.0,
                outliers: vec![],
                color: vec4(0.61, 0.15, 0.69, 1.0),
            },
        ];
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1200.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    fn draw_box_plots(&mut self, cx: &mut Cx2d, rect: Rect) {
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

        if chart_width <= 0.0 || chart_height <= 0.0 || self.datasets.is_empty() {
            return;
        }

        // Find global min/max
        let mut global_min = f64::INFINITY;
        let mut global_max = f64::NEG_INFINITY;

        for data in &self.datasets {
            global_min = global_min.min(data.min);
            global_max = global_max.max(data.max);
            for &outlier in &data.outliers {
                global_min = global_min.min(outlier);
                global_max = global_max.max(outlier);
            }
        }

        let y_padding = (global_max - global_min) * 0.1;
        global_min -= y_padding;
        global_max += y_padding;

        let y_scale = LinearScale::new()
            .with_domain(global_min, global_max)
            .with_range(chart_height as f64, 0.0);

        let n = self.datasets.len();
        let box_width = (chart_width as f64 / n as f64) * 0.6;
        let spacing = chart_width as f64 / n as f64;

        // Draw axes first
        self.draw_axes(cx, chart_x, chart_y, chart_width, chart_height, &y_scale);

        // Clone datasets
        let datasets: Vec<_> = self.datasets.clone();

        for (i, data) in datasets.iter().enumerate() {
            let box_progress = ((progress - i as f64 * 0.1) / 0.5).clamp(0.0, 1.0);
            if box_progress <= 0.0 {
                continue;
            }

            let center_x = chart_x as f64 + spacing * (i as f64 + 0.5);

            // Animate from median outward
            let median_y = chart_y as f64 + y_scale.scale(data.median);
            let q1_y = chart_y as f64 + y_scale.scale(data.q1);
            let q3_y = chart_y as f64 + y_scale.scale(data.q3);
            let min_y = chart_y as f64 + y_scale.scale(data.min);
            let max_y = chart_y as f64 + y_scale.scale(data.max);

            // Animated positions
            let anim_q1_y = median_y + (q1_y - median_y) * box_progress;
            let anim_q3_y = median_y + (q3_y - median_y) * box_progress;
            let anim_min_y = median_y + (min_y - median_y) * box_progress;
            let anim_max_y = median_y + (max_y - median_y) * box_progress;

            // Draw whisker line
            self.draw_line.color = vec4(0.4, 0.4, 0.4, 0.8);
            self.draw_line.draw_line(
                cx,
                dvec2(center_x, anim_max_y),
                dvec2(center_x, anim_min_y),
                2.0,
            );

            // Draw whisker caps
            let cap_width = box_width * 0.4;
            self.draw_line.draw_line(
                cx,
                dvec2(center_x - cap_width / 2.0, anim_min_y),
                dvec2(center_x + cap_width / 2.0, anim_min_y),
                2.0,
            );
            self.draw_line.draw_line(
                cx,
                dvec2(center_x - cap_width / 2.0, anim_max_y),
                dvec2(center_x + cap_width / 2.0, anim_max_y),
                2.0,
            );

            // Draw box (IQR)
            let box_x = center_x - box_width / 2.0;

            self.draw_box.color = data.color;
            self.draw_box.disable_gradient();

            let p1 = dvec2(box_x, anim_q3_y);
            let p2 = dvec2(box_x + box_width, anim_q3_y);
            let p3 = dvec2(box_x + box_width, anim_q1_y);
            let p4 = dvec2(box_x, anim_q1_y);

            self.draw_box.draw_triangle(cx, p1, p2, p3);
            self.draw_box.draw_triangle(cx, p1, p3, p4);

            // Draw box border
            self.draw_line.color = vec4(0.2, 0.2, 0.25, 0.8);
            self.draw_line.draw_line(cx, p1, p2, 1.5);
            self.draw_line.draw_line(cx, p2, p3, 1.5);
            self.draw_line.draw_line(cx, p3, p4, 1.5);
            self.draw_line.draw_line(cx, p4, p1, 1.5);

            // Draw median line
            self.draw_line.color = vec4(0.1, 0.1, 0.1, 1.0);
            self.draw_line.draw_line(
                cx,
                dvec2(box_x, median_y),
                dvec2(box_x + box_width, median_y),
                3.0,
            );

            // Draw outliers
            if box_progress > 0.7 {
                let outlier_progress = ((box_progress - 0.7) / 0.3).min(1.0);
                for &outlier in &data.outliers {
                    let outlier_y = chart_y as f64 + y_scale.scale(outlier);
                    self.draw_point.color = data.color;
                    self.draw_point.draw_point(cx, dvec2(center_x, outlier_y), 5.0 * outlier_progress);
                }
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

        // For x-axis, we'll use category labels
        let mut x_axis = Axis::with_config(x_config);
        let n = self.datasets.len();
        let spacing = chart_width / n as f64;

        let x_ticks: Vec<Tick> = self.datasets.iter().enumerate().map(|(i, data)| {
            Tick {
                value: i as f64,
                position: spacing * (i as f64 + 0.5),
                label: data.label.clone(),
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

        // Render Y-axis with grid
        render_axis(cx, &mut self.draw_line, &mut self.draw_axis_text, &y_layout, offset, &axis_config);

        // Render X-axis without grid (categorical)
        let x_axis_config = AxisRendererConfig::without_grid();
        render_axis(cx, &mut self.draw_line, &mut self.draw_axis_text, &x_layout, offset, &x_axis_config);
    }

    pub fn set_data(&mut self, datasets: Vec<(String, f64, f64, f64, f64, f64, Vec<f64>, Vec4)>) {
        self.datasets = datasets
            .into_iter()
            .map(|(label, min, q1, median, q3, max, outliers, color)| BoxPlotData {
                label, min, q1, median, q3, max, outliers, color,
            })
            .collect();
        self.initialized = false;
    }
}
