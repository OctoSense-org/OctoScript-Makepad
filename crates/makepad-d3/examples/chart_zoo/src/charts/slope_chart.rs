//! Slope chart - before/after comparison visualization
//!
//! Shows change between two data points across categories,
//! with lines connecting the before and after values.

use makepad_widgets::*;
use super::draw_primitives::{DrawPoint, DrawChartLine, DrawBar};
use super::animation::ChartAnimator;

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawPoint;
    use super::draw_primitives::DrawChartLine;
    use super::draw_primitives::DrawBar;

    pub SlopeChart = {{SlopeChart}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Clone, Debug)]
pub struct SlopeData {
    pub label: String,
    pub before: f64,
    pub after: f64,
    pub color: Option<Vec4>,
}

impl SlopeData {
    pub fn new(label: impl Into<String>, before: f64, after: f64) -> Self {
        Self {
            label: label.into(),
            before,
            after,
            color: None,
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct SlopeChart {
    #[redraw]
    #[live]
    draw_point: DrawPoint,

    #[redraw]
    #[live]
    draw_line: DrawChartLine,

    #[redraw]
    #[live]
    draw_bar: DrawBar,

    #[walk]
    walk: Walk,

    #[rust]
    data: Vec<SlopeData>,

    #[rust]
    before_label: String,

    #[rust]
    after_label: String,

    #[rust]
    point_radius: f64,

    #[rust]
    line_width: f64,

    #[rust]
    highlight_increase: bool,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,
}

impl SlopeChart {
    pub fn set_data(&mut self, data: Vec<SlopeData>) {
        self.data = data;
        self.initialized = false;
    }

    pub fn set_labels(&mut self, before: impl Into<String>, after: impl Into<String>) {
        self.before_label = before.into();
        self.after_label = after.into();
    }

    pub fn set_point_radius(&mut self, radius: f64) {
        self.point_radius = radius;
    }

    pub fn set_line_width(&mut self, width: f64) {
        self.line_width = width;
    }

    pub fn set_highlight_increase(&mut self, highlight: bool) {
        self.highlight_increase = highlight;
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        if self.data.is_empty() {
            return;
        }

        let padding = 60.0;
        let label_margin = 80.0;
        let chart_x = rect.pos.x + padding + label_margin;
        let chart_y = rect.pos.y + padding + 30.0;
        let chart_w = rect.size.x - padding * 2.0 - label_margin * 2.0;
        let chart_h = rect.size.y - padding * 2.0 - 60.0;

        // Find value range
        let all_values: Vec<f64> = self.data.iter()
            .flat_map(|d| vec![d.before, d.after])
            .collect();
        let min_val = all_values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_val = all_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let range = (max_val - min_val).max(1.0);

        // Add some padding to the range
        let padded_min = min_val - range * 0.1;
        let padded_max = max_val + range * 0.1;
        let padded_range = padded_max - padded_min;

        // X positions for before and after columns
        let x_before = chart_x;
        let x_after = chart_x + chart_w;

        // Get animation progress
        let progress = self.animator.get_progress();

        // Draw column headers
        let header_color = vec4(0.7, 0.7, 0.7, 1.0);
        self.draw_bar.color = header_color;
        self.draw_bar.draw_bar(cx, Rect {
            pos: dvec2(x_before - 30.0, rect.pos.y + padding),
            size: dvec2(60.0, 8.0),
        });
        self.draw_bar.draw_bar(cx, Rect {
            pos: dvec2(x_after - 30.0, rect.pos.y + padding),
            size: dvec2(60.0, 8.0),
        });

        // Draw vertical axis lines
        let axis_color = vec4(0.3, 0.3, 0.3, 1.0);
        self.draw_line.color = axis_color;
        self.draw_line.draw_line(
            cx,
            dvec2(x_before, chart_y),
            dvec2(x_before, chart_y + chart_h),
            1.5,
        );
        self.draw_line.draw_line(
            cx,
            dvec2(x_after, chart_y),
            dvec2(x_after, chart_y + chart_h),
            1.5,
        );

        // Color palette
        let default_colors = [
            vec4(0.4, 0.76, 0.65, 1.0),
            vec4(0.99, 0.55, 0.38, 1.0),
            vec4(0.55, 0.63, 0.80, 1.0),
            vec4(0.91, 0.84, 0.42, 1.0),
            vec4(0.65, 0.85, 0.33, 1.0),
            vec4(0.90, 0.45, 0.77, 1.0),
        ];

        let increase_color = vec4(0.2, 0.8, 0.4, 1.0);
        let decrease_color = vec4(0.8, 0.3, 0.3, 1.0);

        // Draw slopes
        for (i, item) in self.data.iter().enumerate() {
            let y_before = chart_y + chart_h - ((item.before - padded_min) / padded_range) * chart_h;
            let y_after = chart_y + chart_h - ((item.after - padded_min) / padded_range) * chart_h;

            let color = item.color.unwrap_or_else(|| {
                if self.highlight_increase {
                    if item.after > item.before {
                        increase_color
                    } else if item.after < item.before {
                        decrease_color
                    } else {
                        vec4(0.5, 0.5, 0.5, 1.0)
                    }
                } else {
                    default_colors[i % default_colors.len()]
                }
            });

            // Animate line drawing
            let item_progress = (progress as f32 * (self.data.len() as f32 + 2.0) - i as f32).clamp(0.0, 1.0);

            // Interpolate line end position based on progress
            let x_current = x_before + (x_after - x_before) * item_progress as f64;
            let y_current = y_before + (y_after - y_before) * item_progress as f64;

            // Draw connecting line
            self.draw_line.color = color;
            self.draw_line.draw_line(
                cx,
                dvec2(x_before, y_before),
                dvec2(x_current, y_current),
                self.line_width,
            );

            // Draw points
            self.draw_point.color = color;
            self.draw_point.draw_point(cx, dvec2(x_before, y_before), self.point_radius * 2.0);

            if item_progress > 0.9 {
                self.draw_point.draw_point(cx, dvec2(x_after, y_after), self.point_radius * 2.0 * item_progress as f64);
            }

            // Draw label markers
            let label_color = vec4(0.6, 0.6, 0.6, 1.0);
            self.draw_bar.color = label_color;
            self.draw_bar.draw_bar(cx, Rect {
                pos: dvec2(rect.pos.x + padding, y_before - 3.0),
                size: dvec2(50.0, 6.0),
            });
        }

        // Draw Y-axis scale markers
        let tick_color = vec4(0.4, 0.4, 0.4, 1.0);
        let num_ticks = 5;
        for i in 0..=num_ticks {
            let t = i as f64 / num_ticks as f64;
            let y = chart_y + chart_h - t * chart_h;

            // Left axis tick
            self.draw_line.color = tick_color;
            self.draw_line.draw_line(
                cx,
                dvec2(x_before - 5.0, y),
                dvec2(x_before, y),
                1.0,
            );

            // Right axis tick
            self.draw_line.draw_line(
                cx,
                dvec2(x_after, y),
                dvec2(x_after + 5.0, y),
                1.0,
            );
        }
    }
}

impl SlopeChart {
    fn initialize_demo_data(&mut self) {
        self.data = vec![
            SlopeData::new("Product A", 45.0, 62.0),
            SlopeData::new("Product B", 78.0, 55.0),
            SlopeData::new("Product C", 32.0, 48.0),
            SlopeData::new("Product D", 65.0, 70.0),
            SlopeData::new("Product E", 50.0, 42.0),
        ];
        self.before_label = "2023".to_string();
        self.after_label = "2024".to_string();
        self.point_radius = 6.0;
        self.line_width = 2.0;
        self.highlight_increase = true;
    }
}

impl Widget for SlopeChart {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);

        if !self.initialized {
            self.initialize_demo_data();
            self.animator = ChartAnimator::new(1.0 * 1000.0);
            self.animator.start(cx.cx.seconds_since_app_start());
            self.initialized = true;
        }

        self.draw_chart(cx, rect);
        DrawStep::done()
    }

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
}
