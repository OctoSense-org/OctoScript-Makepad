//! Line Chart Widget
//!
//! GPU-accelerated line chart with axis support.

use makepad_widgets::*;
use makepad_d3::prelude::*;
use super::draw_primitives::{DrawChartLine, DrawPoint};
use super::axis_renderer::{DrawAxisText, AxisRendererConfig, render_axis};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawChartLine;
    use super::draw_primitives::DrawPoint;
    use super::axis_renderer::DrawAxisText;

    pub LineChartWidget = {{LineChartWidget}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct LineChartWidget {
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
    data: Vec<f64>,

    #[rust]
    line_color: Vec4,

    #[rust]
    point_color: Vec4,

    #[rust]
    point_center_color: Vec4,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,
}

impl Widget for LineChartWidget {
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

impl LineChartWidget {
    fn initialize_demo_data(&mut self) {
        if self.initialized {
            return;
        }

        self.data = vec![25.0, 45.0, 35.0, 65.0, 55.0, 75.0, 50.0, 80.0, 60.0, 70.0];
        self.line_color = vec4(0.35, 0.55, 0.90, 1.0);  // Soft blue
        self.point_color = vec4(0.25, 0.45, 0.80, 1.0); // Darker blue edge
        self.point_center_color = vec4(0.55, 0.75, 1.0, 1.0); // Lighter blue center

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

        let max_val = self.data.iter().cloned().fold(0.0f64, f64::max);
        let min_val = self.data.iter().cloned().fold(f64::INFINITY, f64::min);

        let y_scale = LinearScale::new()
            .with_domain(min_val * 0.9, max_val * 1.1)
            .with_range(chart_height as f64, 0.0);

        let x_scale = LinearScale::new()
            .with_domain(0.0, (self.data.len() - 1) as f64)
            .with_range(0.0, chart_width as f64);

        // Draw axes first
        self.draw_axes(cx, chart_x, chart_y, chart_width, chart_height, &x_scale, &y_scale);

        // Draw line and points
        self.draw_line_and_points(cx, chart_x, chart_y, &x_scale, &y_scale);
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

    fn draw_line_and_points(
        &mut self,
        cx: &mut Cx2d,
        chart_x: f64,
        chart_y: f64,
        x_scale: &LinearScale,
        y_scale: &LinearScale,
    ) {
        // Calculate points
        let points: Vec<DVec2> = self.data
            .iter()
            .enumerate()
            .map(|(i, &v)| dvec2(
                chart_x + x_scale.scale(i as f64),
                chart_y + y_scale.scale(v),
            ))
            .collect();

        // Draw connecting lines
        let line_width = 2.5;
        self.draw_line.color = self.line_color;

        for i in 0..points.len().saturating_sub(1) {
            let p1 = points[i];
            let p2 = points[i + 1];
            self.draw_line.draw_line(cx, p1, p2, line_width);
        }

        // Draw points on top with gradient
        let point_size = 10.0;
        self.draw_point.set_radial_gradient(self.point_center_color, self.point_color);

        for point in &points {
            self.draw_point.draw_point(cx, *point, point_size);
        }
    }

    pub fn set_data(&mut self, data: Vec<f64>) {
        self.data = data;
        self.initialized = true;
    }
}
