//! Scatter Chart Widget
//!
//! GPU-accelerated scatter plots with axis support.

use makepad_widgets::*;
use makepad_d3::prelude::*;
use super::draw_primitives::{DrawPoint, DrawChartLine};
use super::axis_renderer::{DrawAxisText, AxisRendererConfig, render_axis};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawPoint;
    use super::draw_primitives::DrawChartLine;
    use super::axis_renderer::DrawAxisText;

    pub ScatterChartWidget = {{ScatterChartWidget}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Clone)]
pub struct ScatterPoint {
    pub x: f64,
    pub y: f64,
    pub r: Option<f64>,
}

#[derive(Live, LiveHook, Widget)]
pub struct ScatterChartWidget {
    #[redraw]
    #[live]
    draw_point: DrawPoint,

    #[redraw]
    #[live]
    draw_line: DrawChartLine,

    #[redraw]
    #[live]
    draw_axis_text: DrawAxisText,

    #[walk]
    walk: Walk,

    #[rust]
    data: Vec<ScatterPoint>,

    #[rust]
    colors: Vec<(Vec4, Vec4)>, // (center_color, outer_color) for gradient

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,
}

impl Widget for ScatterChartWidget {
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

impl ScatterChartWidget {
    fn initialize_demo_data(&mut self) {
        if self.initialized {
            return;
        }

        self.data = vec![
            ScatterPoint { x: 10.0, y: 20.0, r: Some(12.0) },
            ScatterPoint { x: 25.0, y: 50.0, r: Some(18.0) },
            ScatterPoint { x: 40.0, y: 35.0, r: Some(10.0) },
            ScatterPoint { x: 55.0, y: 80.0, r: Some(24.0) },
            ScatterPoint { x: 70.0, y: 45.0, r: Some(14.0) },
            ScatterPoint { x: 85.0, y: 90.0, r: Some(20.0) },
            ScatterPoint { x: 30.0, y: 60.0, r: Some(16.0) },
            ScatterPoint { x: 65.0, y: 25.0, r: Some(11.0) },
        ];

        // Modern gradient colors (center, outer)
        self.colors = vec![
            (vec4(0.55, 0.75, 1.0, 0.9), vec4(0.30, 0.55, 0.90, 0.8)),   // Blue
            (vec4(1.0, 0.70, 0.60, 0.9), vec4(0.90, 0.45, 0.35, 0.8)),   // Coral
            (vec4(0.60, 0.95, 0.70, 0.9), vec4(0.35, 0.80, 0.50, 0.8)), // Green
        ];

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

        // Find data ranges
        let x_max = self.data.iter().map(|p| p.x).fold(0.0f64, f64::max);
        let y_max = self.data.iter().map(|p| p.y).fold(0.0f64, f64::max);

        let x_scale = LinearScale::new()
            .with_domain(0.0, x_max * 1.1)
            .with_range(0.0, chart_width as f64);

        let y_scale = LinearScale::new()
            .with_domain(0.0, y_max * 1.1)
            .with_range(chart_height as f64, 0.0);

        // Draw axes first
        self.draw_axes(cx, chart_x, chart_y, chart_width, chart_height, &x_scale, &y_scale);

        // Draw points
        self.draw_points(cx, chart_x, chart_y, &x_scale, &y_scale);
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

    fn draw_points(
        &mut self,
        cx: &mut Cx2d,
        chart_x: f64,
        chart_y: f64,
        x_scale: &LinearScale,
        y_scale: &LinearScale,
    ) {
        // Draw points with gradient
        for (i, point) in self.data.iter().enumerate() {
            let px = chart_x + x_scale.scale(point.x);
            let py = chart_y + y_scale.scale(point.y);
            let size = point.r.unwrap_or(10.0);

            let color_idx = i % self.colors.len();
            let (center_color, outer_color) = self.colors[color_idx];

            self.draw_point.set_radial_gradient(center_color, outer_color);
            self.draw_point.draw_point(cx, dvec2(px, py), size);
        }
    }

    pub fn set_data(&mut self, data: Vec<ScatterPoint>) {
        self.data = data;
        self.initialized = true;
    }
}
