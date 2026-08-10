//! Pie Chart Variants
//!
//! Different pie chart configurations for the detail page.

use makepad_widgets::*;
use super::draw_primitives::DrawArc;

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawArc;

    pub DonutWidget = {{DonutWidget}} {
        width: Fill,
        height: Fill,
    }

    pub ThinRingWidget = {{ThinRingWidget}} {
        width: Fill,
        height: Fill,
    }

    pub ExplodedPieWidget = {{ExplodedPieWidget}} {
        width: Fill,
        height: Fill,
    }

    pub SemiCircleWidget = {{SemiCircleWidget}} {
        width: Fill,
        height: Fill,
    }

    pub GradientPieWidget = {{GradientPieWidget}} {
        width: Fill,
        height: Fill,
    }

    pub ManySegmentsWidget = {{ManySegmentsWidget}} {
        width: Fill,
        height: Fill,
    }
}

// Common pie drawing logic
fn draw_pie_slices(
    draw_arc: &mut DrawArc,
    cx: &mut Cx2d,
    center: DVec2,
    outer_radius: f64,
    inner_radius: f64,
    values: &[f64],
    colors: &[[f32; 4]],
    explode_offset: f64,
    start_angle: f64,
    end_angle: f64,
) {
    let total: f64 = values.iter().sum();
    if total <= 0.0 || outer_radius <= 0.0 {
        return;
    }

    let angle_range = end_angle - start_angle;
    let mut current_angle = start_angle;

    for (i, &value) in values.iter().enumerate() {
        let sweep = (value / total) * angle_range;
        let color = colors[i % colors.len()];

        // Calculate explode offset direction (middle of slice)
        let mid_angle = current_angle + sweep / 2.0;
        let offset_x = if explode_offset > 0.0 { explode_offset * mid_angle.cos() } else { 0.0 };
        let offset_y = if explode_offset > 0.0 { explode_offset * mid_angle.sin() } else { 0.0 };

        let slice_center = dvec2(center.x + offset_x, center.y + offset_y);

        draw_arc.set_arc(current_angle, sweep, inner_radius, outer_radius);
        draw_arc.color = vec4(color[0], color[1], color[2], color[3]);
        draw_arc.disable_gradient();
        draw_arc.draw_arc(cx, slice_center, outer_radius + explode_offset);

        current_angle += sweep;
    }
}

// ============ Donut Widget (50% inner radius) ============
#[derive(Live, LiveHook, Widget)]
pub struct DonutWidget {
    #[redraw] #[live] draw_arc: DrawArc,
    #[walk] walk: Walk,
    #[rust] area: Area,
}

impl Widget for DonutWidget {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            let center = dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + rect.size.y / 2.0);
            let radius = (rect.size.x.min(rect.size.y) / 2.0 - 20.0) as f64;
            let values = vec![35.0, 25.0, 20.0, 12.0, 8.0];
            let colors = [
                [0.26, 0.52, 0.96, 1.0],
                [0.92, 0.26, 0.21, 1.0],
                [0.20, 0.66, 0.33, 1.0],
                [1.0, 0.76, 0.03, 1.0],
                [0.61, 0.15, 0.69, 1.0],
            ];
            draw_pie_slices(&mut self.draw_arc, cx, center, radius, radius * 0.5, &values, &colors, 0.0, -std::f64::consts::FRAC_PI_2, std::f64::consts::FRAC_PI_2 * 3.0);
        }
        DrawStep::done()
    }
}

// ============ Thin Ring Widget (80% inner radius) ============
#[derive(Live, LiveHook, Widget)]
pub struct ThinRingWidget {
    #[redraw] #[live] draw_arc: DrawArc,
    #[walk] walk: Walk,
    #[rust] area: Area,
}

impl Widget for ThinRingWidget {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            let center = dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + rect.size.y / 2.0);
            let radius = (rect.size.x.min(rect.size.y) / 2.0 - 20.0) as f64;
            let values = vec![30.0, 25.0, 20.0, 15.0, 10.0];
            let colors = [
                [0.10, 0.74, 0.61, 1.0],
                [0.95, 0.61, 0.07, 1.0],
                [0.20, 0.60, 0.86, 1.0],
                [0.91, 0.30, 0.24, 1.0],
                [0.56, 0.27, 0.68, 1.0],
            ];
            draw_pie_slices(&mut self.draw_arc, cx, center, radius, radius * 0.8, &values, &colors, 0.0, -std::f64::consts::FRAC_PI_2, std::f64::consts::FRAC_PI_2 * 3.0);
        }
        DrawStep::done()
    }
}

// ============ Exploded Pie Widget ============
#[derive(Live, LiveHook, Widget)]
pub struct ExplodedPieWidget {
    #[redraw] #[live] draw_arc: DrawArc,
    #[walk] walk: Walk,
    #[rust] area: Area,
}

impl Widget for ExplodedPieWidget {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            let center = dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + rect.size.y / 2.0);
            let radius = (rect.size.x.min(rect.size.y) / 2.0 - 30.0) as f64;
            let values = vec![40.0, 30.0, 20.0, 10.0];
            let colors = [
                [0.26, 0.52, 0.96, 1.0],
                [0.92, 0.26, 0.21, 1.0],
                [0.20, 0.66, 0.33, 1.0],
                [1.0, 0.76, 0.03, 1.0],
            ];
            draw_pie_slices(&mut self.draw_arc, cx, center, radius, 0.0, &values, &colors, 8.0, -std::f64::consts::FRAC_PI_2, std::f64::consts::FRAC_PI_2 * 3.0);
        }
        DrawStep::done()
    }
}

// ============ Semi-Circle Widget ============
#[derive(Live, LiveHook, Widget)]
pub struct SemiCircleWidget {
    #[redraw] #[live] draw_arc: DrawArc,
    #[walk] walk: Walk,
    #[rust] area: Area,
}

impl Widget for SemiCircleWidget {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            let center = dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + rect.size.y * 0.65);
            let radius = (rect.size.x.min(rect.size.y) / 2.0 - 20.0) as f64;
            let values = vec![35.0, 25.0, 20.0, 12.0, 8.0];
            let colors = [
                [0.95, 0.26, 0.21, 1.0],
                [0.26, 0.52, 0.96, 1.0],
                [0.30, 0.69, 0.31, 1.0],
                [1.0, 0.60, 0.00, 1.0],
                [0.61, 0.15, 0.69, 1.0],
            ];
            // Half circle: from -PI to 0
            draw_pie_slices(&mut self.draw_arc, cx, center, radius, 0.0, &values, &colors, 0.0, -std::f64::consts::PI, 0.0);
        }
        DrawStep::done()
    }
}

// ============ Gradient Pie Widget ============
#[derive(Live, LiveHook, Widget)]
pub struct GradientPieWidget {
    #[redraw] #[live] draw_arc: DrawArc,
    #[walk] walk: Walk,
    #[rust] area: Area,
}

impl Widget for GradientPieWidget {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            let center = dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + rect.size.y / 2.0);
            let radius = (rect.size.x.min(rect.size.y) / 2.0 - 20.0) as f64;
            let values = vec![30.0, 25.0, 20.0, 15.0, 10.0];

            // Draw with gradient colors (light to dark of same hue)
            let total: f64 = values.iter().sum();
            let mut current_angle = -std::f64::consts::FRAC_PI_2;
            let colors_inner = [
                [0.45, 0.70, 0.98, 1.0],
                [0.98, 0.50, 0.45, 1.0],
                [0.45, 0.85, 0.55, 1.0],
                [1.0, 0.90, 0.45, 1.0],
                [0.80, 0.45, 0.88, 1.0],
            ];
            let colors_outer = [
                [0.15, 0.40, 0.85, 1.0],
                [0.85, 0.15, 0.10, 1.0],
                [0.10, 0.55, 0.20, 1.0],
                [0.95, 0.65, 0.00, 1.0],
                [0.50, 0.05, 0.58, 1.0],
            ];

            for (i, &value) in values.iter().enumerate() {
                let sweep = (value / total) * 2.0 * std::f64::consts::PI;
                let inner = colors_inner[i % colors_inner.len()];
                let outer = colors_outer[i % colors_outer.len()];

                self.draw_arc.set_arc(current_angle, sweep, 0.0, radius);
                self.draw_arc.set_radial_gradient(
                    vec4(inner[0], inner[1], inner[2], inner[3]),
                    vec4(outer[0], outer[1], outer[2], outer[3]),
                );
                self.draw_arc.draw_arc(cx, center, radius);

                current_angle += sweep;
            }
        }
        DrawStep::done()
    }
}

// ============ Many Segments Widget ============
#[derive(Live, LiveHook, Widget)]
pub struct ManySegmentsWidget {
    #[redraw] #[live] draw_arc: DrawArc,
    #[walk] walk: Walk,
    #[rust] area: Area,
}

impl Widget for ManySegmentsWidget {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            let center = dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + rect.size.y / 2.0);
            let radius = (rect.size.x.min(rect.size.y) / 2.0 - 20.0) as f64;
            let values: Vec<f64> = (0..12).map(|i| 10.0 + (i as f64 * 1.5)).collect();
            let colors = [
                [0.26, 0.52, 0.96, 1.0],
                [0.92, 0.26, 0.21, 1.0],
                [0.20, 0.66, 0.33, 1.0],
                [1.0, 0.76, 0.03, 1.0],
                [0.61, 0.15, 0.69, 1.0],
                [0.10, 0.74, 0.61, 1.0],
                [0.95, 0.61, 0.07, 1.0],
                [0.20, 0.60, 0.86, 1.0],
                [0.91, 0.30, 0.24, 1.0],
                [0.56, 0.27, 0.68, 1.0],
                [0.30, 0.69, 0.31, 1.0],
                [0.85, 0.65, 0.13, 1.0],
            ];
            draw_pie_slices(&mut self.draw_arc, cx, center, radius, 0.0, &values, &colors, 0.0, -std::f64::consts::FRAC_PI_2, std::f64::consts::FRAC_PI_2 * 3.0);
        }
        DrawStep::done()
    }
}
