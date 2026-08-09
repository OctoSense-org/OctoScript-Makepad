//! Heatmap Widget
//!
//! GPU-accelerated matrix visualization with smooth animations.
//! Features wave reveal effect, Viridis-like colormap, and cell highlighting.

use makepad_widgets::*;
use super::draw_primitives::{DrawChartLine, DrawTriangle, DrawPoint};
use super::animation::{ChartAnimator, EasingType};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawChartLine;
    use super::draw_primitives::DrawTriangle;
    use super::draw_primitives::DrawPoint;

    pub HeatmapWidget = {{HeatmapWidget}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum HeatmapColorScale {
    #[default]
    Viridis,
    Plasma,
    Inferno,
    BlueRed,
    YellowGreen,
}

#[derive(Live, LiveHook, Widget)]
pub struct HeatmapWidget {
    #[redraw]
    #[live]
    draw_cell: DrawTriangle,

    #[redraw]
    #[live]
    draw_line: DrawChartLine,

    #[redraw]
    #[live]
    draw_point: DrawPoint,

    #[walk]
    walk: Walk,

    #[rust]
    data: Vec<Vec<f64>>,

    #[rust]
    row_labels: Vec<String>,

    #[rust]
    col_labels: Vec<String>,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,

    #[rust]
    chart_rect: Rect,

    #[rust]
    color_scale: HeatmapColorScale,

    #[rust]
    hovered_cell: Option<(usize, usize)>,

    #[rust(true)]
    show_grid: bool,

    #[rust(true)]
    rounded_cells: bool,
}

impl Widget for HeatmapWidget {
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

            self.draw_heatmap(cx);
        }

        DrawStep::done()
    }
}

impl HeatmapWidget {
    fn initialize_demo_data(&mut self) {
        // Activity heatmap: hours vs days
        let days = vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
        let hours = vec!["6am", "9am", "12pm", "3pm", "6pm", "9pm", "12am"];

        self.row_labels = hours.iter().map(|s| s.to_string()).collect();
        self.col_labels = days.iter().map(|s| s.to_string()).collect();

        // Activity data (normalized 0-1)
        self.data = vec![
            vec![0.15, 0.20, 0.18, 0.22, 0.15, 0.05, 0.05],  // 6am
            vec![0.85, 0.92, 0.88, 0.90, 0.82, 0.25, 0.18],  // 9am
            vec![0.72, 0.78, 0.82, 0.75, 0.68, 0.55, 0.42],  // 12pm
            vec![0.65, 0.70, 0.72, 0.68, 0.55, 0.62, 0.58],  // 3pm
            vec![0.52, 0.58, 0.55, 0.62, 0.45, 0.75, 0.65],  // 6pm
            vec![0.35, 0.38, 0.42, 0.40, 0.32, 0.85, 0.78],  // 9pm
            vec![0.12, 0.15, 0.18, 0.15, 0.10, 0.45, 0.38],  // 12am
        ];

        self.color_scale = HeatmapColorScale::Viridis;
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1200.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    pub fn replay_animation(&mut self, cx: &mut Cx) {
        self.animator.reset();
        self.start_animation(cx);
        self.redraw(cx);
    }

    fn value_to_color(&self, value: f64) -> Vec4 {
        let t = value.clamp(0.0, 1.0) as f32;

        match self.color_scale {
            HeatmapColorScale::Viridis => {
                // Viridis colormap approximation
                if t < 0.25 {
                    let s = t * 4.0;
                    vec4(
                        0.267 + s * 0.01,
                        0.004 + s * 0.17,
                        0.329 + s * 0.13,
                        1.0,
                    )
                } else if t < 0.5 {
                    let s = (t - 0.25) * 4.0;
                    vec4(
                        0.277 - s * 0.05,
                        0.174 + s * 0.21,
                        0.459 + s * 0.05,
                        1.0,
                    )
                } else if t < 0.75 {
                    let s = (t - 0.5) * 4.0;
                    vec4(
                        0.227 + s * 0.22,
                        0.384 + s * 0.22,
                        0.509 - s * 0.12,
                        1.0,
                    )
                } else {
                    let s = (t - 0.75) * 4.0;
                    vec4(
                        0.447 + s * 0.55,
                        0.604 + s * 0.25,
                        0.389 - s * 0.15,
                        1.0,
                    )
                }
            }
            HeatmapColorScale::Plasma => {
                // Plasma colormap approximation
                if t < 0.5 {
                    let s = t * 2.0;
                    vec4(
                        0.05 + s * 0.70,
                        0.03 + s * 0.20,
                        0.53 - s * 0.10,
                        1.0,
                    )
                } else {
                    let s = (t - 0.5) * 2.0;
                    vec4(
                        0.75 + s * 0.20,
                        0.23 + s * 0.60,
                        0.43 - s * 0.40,
                        1.0,
                    )
                }
            }
            HeatmapColorScale::Inferno => {
                // Inferno colormap approximation
                if t < 0.33 {
                    let s = t * 3.0;
                    vec4(
                        0.0 + s * 0.42,
                        0.0 + s * 0.12,
                        0.02 + s * 0.33,
                        1.0,
                    )
                } else if t < 0.67 {
                    let s = (t - 0.33) * 3.0;
                    vec4(
                        0.42 + s * 0.50,
                        0.12 + s * 0.28,
                        0.35 - s * 0.20,
                        1.0,
                    )
                } else {
                    let s = (t - 0.67) * 3.0;
                    vec4(
                        0.92 + s * 0.08,
                        0.40 + s * 0.55,
                        0.15 + s * 0.65,
                        1.0,
                    )
                }
            }
            HeatmapColorScale::BlueRed => {
                // Diverging blue-white-red
                if t < 0.5 {
                    let s = t * 2.0;
                    vec4(
                        0.20 + s * 0.80,
                        0.40 + s * 0.60,
                        0.90 - s * 0.15,
                        1.0,
                    )
                } else {
                    let s = (t - 0.5) * 2.0;
                    vec4(
                        1.0,
                        1.0 - s * 0.70,
                        0.75 - s * 0.55,
                        1.0,
                    )
                }
            }
            HeatmapColorScale::YellowGreen => {
                // Yellow to green
                vec4(
                    0.97 - t * 0.65,
                    0.97 - t * 0.20,
                    0.25 + t * 0.35,
                    1.0,
                )
            }
        }
    }

    fn draw_heatmap(&mut self, cx: &mut Cx2d) {
        let progress = self.animator.get_progress();
        let rect = self.chart_rect;

        let padding_left = 60.0;
        let padding_top = 40.0;
        let padding_right = 30.0;
        let padding_bottom = 30.0;

        let chart_x = rect.pos.x + padding_left;
        let chart_y = rect.pos.y + padding_top;
        let chart_width = rect.size.x - padding_left - padding_right;
        let chart_height = rect.size.y - padding_top - padding_bottom;

        if chart_width <= 0.0 || chart_height <= 0.0 {
            return;
        }

        let rows = self.data.len();
        let cols = if rows > 0 { self.data[0].len() } else { 0 };

        if rows == 0 || cols == 0 {
            return;
        }

        let cell_width = chart_width / cols as f64;
        let cell_height = chart_height / rows as f64;
        let gap = 2.5;

        // Clone data for iteration
        let data: Vec<Vec<f64>> = self.data.clone();

        // Draw background
        self.draw_cell.color = vec4(0.12, 0.12, 0.14, 1.0);
        self.draw_cell.disable_gradient();
        let bg_p1 = dvec2(chart_x - 5.0, chart_y - 5.0);
        let bg_p2 = dvec2(chart_x + chart_width + 5.0, chart_y - 5.0);
        let bg_p3 = dvec2(chart_x + chart_width + 5.0, chart_y + chart_height + 5.0);
        let bg_p4 = dvec2(chart_x - 5.0, chart_y + chart_height + 5.0);
        self.draw_cell.draw_triangle(cx, bg_p1, bg_p2, bg_p3);
        self.draw_cell.draw_triangle(cx, bg_p1, bg_p3, bg_p4);

        // Calculate max distance for wave animation
        let max_dist = ((rows as f64).powi(2) + (cols as f64).powi(2)).sqrt();

        // Draw cells with wave reveal effect
        for (row_idx, row_data) in data.iter().enumerate() {
            for (col_idx, &value) in row_data.iter().enumerate() {
                // Wave reveal: cells closer to top-left appear first
                let dist = ((row_idx as f64).powi(2) + (col_idx as f64).powi(2)).sqrt();
                let cell_delay = dist / max_dist * 0.4;
                let cell_progress = ((progress - cell_delay) / 0.5).clamp(0.0, 1.0);

                if cell_progress <= 0.0 {
                    continue;
                }

                let x = chart_x + col_idx as f64 * cell_width + gap / 2.0;
                let y = chart_y + row_idx as f64 * cell_height + gap / 2.0;
                let w = (cell_width - gap) * cell_progress;
                let h = (cell_height - gap) * cell_progress;

                // Center the cell during animation
                let offset_x = (cell_width - gap - w) / 2.0;
                let offset_y = (cell_height - gap - h) / 2.0;

                let is_hovered = self.hovered_cell == Some((row_idx, col_idx));

                // Get color with brightness adjustment for hover
                let mut color = self.value_to_color(value);
                if is_hovered {
                    color = vec4(
                        (color.x + 0.15).min(1.0),
                        (color.y + 0.15).min(1.0),
                        (color.z + 0.15).min(1.0),
                        color.w,
                    );
                }

                self.draw_cell_rect(
                    cx,
                    x + offset_x, y + offset_y,
                    w, h,
                    color,
                    cell_progress,
                    is_hovered,
                );
            }
        }

        // Draw grid lines if enabled
        if self.show_grid && progress > 0.5 {
            let grid_progress = ((progress - 0.5) * 2.0).min(1.0);
            self.draw_grid_lines(cx, chart_x, chart_y, chart_width, chart_height, rows, cols, cell_width, cell_height, grid_progress);
        }
    }

    fn draw_cell_rect(
        &mut self,
        cx: &mut Cx2d,
        x: f64, y: f64,
        width: f64, height: f64,
        color: Vec4,
        progress: f64,
        is_hovered: bool,
    ) {
        // Create slight gradient for depth
        let light_color = vec4(
            (color.x + 0.08).min(1.0),
            (color.y + 0.08).min(1.0),
            (color.z + 0.08).min(1.0),
            color.w,
        );
        let dark_color = vec4(
            color.x * 0.92,
            color.y * 0.92,
            color.z * 0.92,
            color.w,
        );

        self.draw_cell.set_radial_gradient(light_color, dark_color);

        let p1 = dvec2(x, y);
        let p2 = dvec2(x + width, y);
        let p3 = dvec2(x + width, y + height);
        let p4 = dvec2(x, y + height);

        self.draw_cell.draw_triangle(cx, p1, p2, p3);
        self.draw_cell.draw_triangle(cx, p1, p3, p4);

        // Draw border for hovered cells
        if is_hovered {
            self.draw_line.color = vec4(1.0, 1.0, 1.0, 0.8);
            self.draw_line.draw_line(cx, p1, p2, 2.0);
            self.draw_line.draw_line(cx, p2, p3, 2.0);
            self.draw_line.draw_line(cx, p3, p4, 2.0);
            self.draw_line.draw_line(cx, p4, p1, 2.0);
        }

        // Draw rounded corners if enabled
        if self.rounded_cells && progress > 0.8 {
            let corner_progress = ((progress - 0.8) / 0.2).min(1.0);
            let radius = 2.5 * corner_progress;
            self.draw_point.color = color;
            self.draw_point.draw_point(cx, dvec2(x, y), radius);
            self.draw_point.draw_point(cx, dvec2(x + width, y), radius);
            self.draw_point.draw_point(cx, dvec2(x + width, y + height), radius);
            self.draw_point.draw_point(cx, dvec2(x, y + height), radius);
        }

        // Inner highlight
        if progress > 0.6 {
            let highlight_alpha = ((progress - 0.6) / 0.4).min(1.0) as f32 * 0.25;
            self.draw_line.color = vec4(1.0, 1.0, 1.0, highlight_alpha);
            self.draw_line.draw_line(
                cx,
                dvec2(x + 1.5, y + 1.5),
                dvec2(x + width - 1.5, y + 1.5),
                1.0,
            );
        }
    }

    fn draw_grid_lines(
        &mut self,
        cx: &mut Cx2d,
        chart_x: f64, chart_y: f64,
        chart_width: f64, chart_height: f64,
        rows: usize, cols: usize,
        cell_width: f64, cell_height: f64,
        progress: f64,
    ) {
        self.draw_line.color = vec4(0.3, 0.3, 0.35, 0.4 * progress as f32);

        // Vertical lines
        for i in 0..=cols {
            let x = chart_x + i as f64 * cell_width;
            let animated_height = chart_height * progress;
            self.draw_line.draw_line(
                cx,
                dvec2(x, chart_y),
                dvec2(x, chart_y + animated_height),
                1.0,
            );
        }

        // Horizontal lines
        for i in 0..=rows {
            let y = chart_y + i as f64 * cell_height;
            let animated_width = chart_width * progress;
            self.draw_line.draw_line(
                cx,
                dvec2(chart_x, y),
                dvec2(chart_x + animated_width, y),
                1.0,
            );
        }
    }

    pub fn set_data(&mut self, data: Vec<Vec<f64>>, row_labels: Vec<String>, col_labels: Vec<String>) {
        self.data = data;
        self.row_labels = row_labels;
        self.col_labels = col_labels;
        self.initialized = false;
    }

    pub fn set_color_scale(&mut self, scale: HeatmapColorScale) {
        self.color_scale = scale;
    }
}
