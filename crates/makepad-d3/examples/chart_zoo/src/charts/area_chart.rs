//! Area Chart Widget
//!
//! GPU-accelerated area chart with axis support and gradient fills.

use makepad_widgets::*;
use makepad_d3::prelude::*;
use super::draw_primitives::{DrawAreaFill, DrawChartLine};
use super::axis_renderer::{DrawAxisText, AxisRendererConfig, render_axis};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawAreaFill;
    use super::draw_primitives::DrawChartLine;
    use super::axis_renderer::DrawAxisText;

    pub AreaChartWidget = {{AreaChartWidget}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct AreaChartWidget {
    #[redraw]
    #[live]
    draw_fill: DrawAreaFill,

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
    top_color: Vec4,

    #[rust]
    bottom_color: Vec4,

    #[rust]
    line_color: Vec4,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,
}

impl Widget for AreaChartWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if let Event::NextFrame(_) = event {
            if !self.initialized {
                self.initialize_demo_data();
                self.redraw(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.initialized {
            self.initialize_demo_data();
        }

        let rect = cx.walk_turtle_with_area(&mut self.area, walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.data.is_empty() {
            self.draw_chart(cx, rect);
        }

        DrawStep::done()
    }
}

impl AreaChartWidget {
    fn initialize_demo_data(&mut self) {
        if self.initialized {
            return;
        }

        self.data = vec![30.0, 50.0, 45.0, 60.0, 35.0, 55.0, 70.0, 45.0, 60.0, 40.0];

        // Gradient colors for fill (top -> bottom)
        self.top_color = vec4(0.35, 0.60, 0.90, 0.7);    // Blue semi-transparent
        self.bottom_color = vec4(0.35, 0.60, 0.90, 0.1); // Blue very transparent
        self.line_color = vec4(0.25, 0.50, 0.85, 1.0);   // Solid blue line

        self.initialized = true;
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        // Larger padding for axes
        let padding_left = 50.0;
        let padding_bottom = 40.0;
        let padding_top = 20.0;
        let padding_right = 20.0;

        let chart_x = rect.pos.x + padding_left;
        let chart_y = rect.pos.y + padding_top;
        let chart_width = rect.size.x - padding_left - padding_right;
        let chart_height = rect.size.y - padding_top - padding_bottom;

        if chart_width <= 0.0 || chart_height <= 0.0 {
            return;
        }

        // Create scales
        let max_val = self.data.iter().cloned().fold(0.0f64, f64::max);
        let min_val = 0.0;

        let y_scale = LinearScale::new()
            .with_domain(min_val, max_val * 1.1)
            .with_range(chart_height as f64, 0.0);

        let x_scale = LinearScale::new()
            .with_domain(0.0, (self.data.len() - 1) as f64)
            .with_range(0.0, chart_width as f64);

        // Draw axes first (behind area)
        self.draw_axes(cx, chart_x, chart_y, chart_width, chart_height, &x_scale, &y_scale);

        // Draw area and line
        self.draw_area_and_line(cx, chart_x, chart_y, chart_height, &x_scale, &y_scale);
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
        // Create axis configurations with grid
        let x_config = AxisConfig::bottom()
            .with_tick_size(6.0)
            .with_tick_padding(8.0)
            .with_grid(chart_height);

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

        // Render axes
        render_axis(cx, &mut self.draw_line, &mut self.draw_axis_text, &y_layout, offset, &axis_config);
        render_axis(cx, &mut self.draw_line, &mut self.draw_axis_text, &x_layout, offset, &axis_config);
    }

    fn draw_area_and_line(
        &mut self,
        cx: &mut Cx2d,
        chart_x: f64,
        chart_y: f64,
        chart_height: f64,
        x_scale: &LinearScale,
        y_scale: &LinearScale,
    ) {
        let bottom_y = chart_y + chart_height;

        // Calculate points
        let points: Vec<DVec2> = self.data
            .iter()
            .enumerate()
            .map(|(i, &v)| dvec2(
                chart_x + x_scale.scale(i as f64),
                chart_y + y_scale.scale(v),
            ))
            .collect();

        // Draw filled area as trapezoids with gradient
        self.draw_fill.top_color = self.top_color;
        self.draw_fill.bottom_color = self.bottom_color;

        for i in 0..points.len().saturating_sub(1) {
            let x1 = points[i].x;
            let y1 = points[i].y;
            let x2 = points[i + 1].x;
            let y2 = points[i + 1].y;

            // Draw as a filled rectangle from the line to the bottom
            let top_y = y1.min(y2);
            let rect_height = bottom_y - top_y;
            let rect_width = x2 - x1;

            self.draw_fill.draw_area(
                cx,
                Rect {
                    pos: dvec2(x1, top_y),
                    size: dvec2(rect_width.max(1.0), rect_height),
                },
            );
        }

        // Draw line on top
        self.draw_line.color = self.line_color;
        let line_width = 2.5;

        for i in 0..points.len().saturating_sub(1) {
            let p1 = points[i];
            let p2 = points[i + 1];
            self.draw_line.draw_line(cx, p1, p2, line_width);
        }
    }

    pub fn set_data(&mut self, data: Vec<f64>) {
        self.data = data;
        self.initialized = true;
    }
}
