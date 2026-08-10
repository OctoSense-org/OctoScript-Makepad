//! Pie Chart Widget
//!
//! Renders pie charts using makepad-d3 PieLayout and custom arc shaders.

use makepad_widgets::*;
use makepad_d3::prelude::*;
use std::f64::consts::PI;
use super::draw_primitives::{DrawArc, DrawChartLine, DrawPoint};
use super::axis_renderer::DrawAxisText;
use super::legend_renderer::{render_legend, LegendItem, LegendConfig, LegendMarker, LegendPosition};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawArc;
    use super::draw_primitives::DrawChartLine;
    use super::draw_primitives::DrawPoint;
    use super::axis_renderer::DrawAxisText;

    pub PieChartWidget = {{PieChartWidget}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct PieChartWidget {
    #[redraw]
    #[live]
    draw_arc: DrawArc,

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
    labels: Vec<String>,

    #[rust]
    colors: Vec<(Vec4, Vec4)>, // (inner_color, outer_color) for gradient

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,
}

impl Widget for PieChartWidget {
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

impl PieChartWidget {
    fn initialize_demo_data(&mut self) {
        if self.initialized {
            return;
        }

        self.data = vec![30.0, 20.0, 25.0, 15.0, 10.0];
        self.labels = vec![
            "Desktop".to_string(),
            "Mobile".to_string(),
            "Tablet".to_string(),
            "Web".to_string(),
            "Other".to_string(),
        ];

        // Modern gradient color palette (inner, outer)
        self.colors = vec![
            (vec4(0.35, 0.60, 0.90, 1.0), vec4(0.25, 0.50, 0.80, 1.0)),  // Blue
            (vec4(0.90, 0.50, 0.40, 1.0), vec4(0.80, 0.40, 0.30, 1.0)),  // Coral
            (vec4(0.45, 0.80, 0.55, 1.0), vec4(0.35, 0.70, 0.45, 1.0)),  // Green
            (vec4(0.95, 0.75, 0.35, 1.0), vec4(0.85, 0.65, 0.25, 1.0)),  // Orange
            (vec4(0.70, 0.50, 0.80, 1.0), vec4(0.60, 0.40, 0.70, 1.0)),  // Purple
        ];

        self.initialized = true;
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        // Reserve space for legend on the right
        let legend_width = 100.0;
        let chart_rect = Rect {
            pos: rect.pos,
            size: dvec2(rect.size.x - legend_width, rect.size.y),
        };

        let center_x = chart_rect.pos.x + chart_rect.size.x / 2.0;
        let center_y = chart_rect.pos.y + chart_rect.size.y / 2.0;
        let radius = (chart_rect.size.x.min(chart_rect.size.y) / 2.0 - 16.0) as f64;

        if radius <= 0.0 {
            return;
        }

        // Use makepad-d3 PieLayout
        let pie = PieLayout::new()
            .start_angle(-PI / 2.0) // Start at 12 o'clock
            .end_angle(3.0 * PI / 2.0)
            .pad_angle(0.02);

        let slices = pie.compute(&self.data);
        let center = dvec2(center_x as f64, center_y as f64);

        // Draw each slice using the arc shader
        for (i, slice) in slices.iter().enumerate() {
            let color_idx = i % self.colors.len();
            let (inner_color, outer_color) = self.colors[color_idx];

            let sweep = slice.end_angle - slice.start_angle;
            self.draw_arc.set_arc(slice.start_angle, sweep, 0.0, radius);
            self.draw_arc.set_radial_gradient(inner_color, outer_color);
            self.draw_arc.draw_arc(cx, center, radius);
        }

        // Draw legend
        let labels = self.labels.clone();
        let colors = self.colors.clone();
        let legend_items: Vec<LegendItem> = labels.iter().enumerate()
            .map(|(i, label)| {
                let color_idx = i % colors.len();
                let (_, outer_color) = colors[color_idx];
                LegendItem::new(label, outer_color).with_marker(LegendMarker::Square)
            })
            .collect();

        let legend_config = LegendConfig::vertical()
            .with_position(LegendPosition::Right);

        render_legend(
            cx,
            &mut self.draw_line,
            &mut self.draw_point,
            &mut self.draw_axis_text,
            &legend_items,
            rect,
            &legend_config,
        );
    }

    pub fn set_data(&mut self, data: Vec<f64>) {
        self.data = data;
        self.initialized = true;
    }
}
