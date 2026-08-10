//! Scatter Chart Variants
//!
//! Different scatter plot configurations for the detail page.

use makepad_widgets::*;
use super::draw_primitives::DrawArc;

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawArc;

    pub BubbleChartWidget = {{BubbleChartWidget}} {
        width: Fill,
        height: Fill,
    }

    pub MultiDatasetScatterWidget = {{MultiDatasetScatterWidget}} {
        width: Fill,
        height: Fill,
    }

    pub LargePointsWidget = {{LargePointsWidget}} {
        width: Fill,
        height: Fill,
    }

    pub SmallPointsWidget = {{SmallPointsWidget}} {
        width: Fill,
        height: Fill,
    }

    pub ColorGradientScatterWidget = {{ColorGradientScatterWidget}} {
        width: Fill,
        height: Fill,
    }

    pub DenseScatterWidget = {{DenseScatterWidget}} {
        width: Fill,
        height: Fill,
    }
}

// Simple pseudo-random for consistent data
fn pseudo_random(seed: u32) -> f64 {
    let x = seed.wrapping_mul(1103515245).wrapping_add(12345);
    ((x >> 16) & 0x7FFF) as f64 / 32767.0
}

// ============ Bubble Chart Widget (x, y, size) ============
#[derive(Live, LiveHook, Widget)]
pub struct BubbleChartWidget {
    #[redraw] #[live] draw_arc: DrawArc,
    #[walk] walk: Walk,
    #[rust] area: Area,
}

impl Widget for BubbleChartWidget {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            let padding = 40.0;
            let chart_x = rect.pos.x + padding;
            let chart_y = rect.pos.y + padding;
            let chart_w = rect.size.x - padding * 2.0;
            let chart_h = rect.size.y - padding * 2.0;

            // Bubble data: (x, y, size, color)
            let bubbles = [
                (0.2, 0.3, 25.0, [0.26, 0.52, 0.96, 0.7]),
                (0.4, 0.6, 40.0, [0.92, 0.26, 0.21, 0.7]),
                (0.6, 0.4, 35.0, [0.20, 0.66, 0.33, 0.7]),
                (0.8, 0.7, 20.0, [1.0, 0.76, 0.03, 0.7]),
                (0.3, 0.8, 30.0, [0.61, 0.15, 0.69, 0.7]),
                (0.7, 0.2, 45.0, [0.10, 0.74, 0.61, 0.7]),
                (0.5, 0.5, 50.0, [0.95, 0.61, 0.07, 0.7]),
                (0.15, 0.55, 22.0, [0.20, 0.60, 0.86, 0.7]),
            ];

            for (x, y, size, color) in bubbles {
                let px = chart_x + x * chart_w;
                let py = chart_y + (1.0 - y) * chart_h;

                self.draw_arc.set_arc(0.0, std::f64::consts::PI * 2.0, 0.0, size);
                self.draw_arc.color = vec4(color[0], color[1], color[2], color[3]);
                self.draw_arc.disable_gradient();
                self.draw_arc.draw_arc(cx, dvec2(px, py), size);
            }
        }
        DrawStep::done()
    }
}

// ============ Multi-Dataset Scatter Widget ============
#[derive(Live, LiveHook, Widget)]
pub struct MultiDatasetScatterWidget {
    #[redraw] #[live] draw_arc: DrawArc,
    #[walk] walk: Walk,
    #[rust] area: Area,
}

impl Widget for MultiDatasetScatterWidget {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            let padding = 40.0;
            let chart_x = rect.pos.x + padding;
            let chart_y = rect.pos.y + padding;
            let chart_w = rect.size.x - padding * 2.0;
            let chart_h = rect.size.y - padding * 2.0;

            let datasets = [
                ([0.26, 0.52, 0.96, 1.0], 100, 0.2, 0.3),  // Blue cluster
                ([0.92, 0.26, 0.21, 1.0], 200, 0.7, 0.6),  // Red cluster
                ([0.20, 0.66, 0.33, 1.0], 300, 0.5, 0.8),  // Green cluster
            ];

            for (color, seed_base, cx_offset, cy_offset) in datasets {
                for i in 0..15 {
                    let seed = seed_base + i;
                    let x = cx_offset + (pseudo_random(seed) - 0.5) * 0.3;
                    let y = cy_offset + (pseudo_random(seed + 100) - 0.5) * 0.3;

                    let px = chart_x + x.clamp(0.0, 1.0) * chart_w;
                    let py = chart_y + (1.0 - y.clamp(0.0, 1.0)) * chart_h;

                    self.draw_arc.set_arc(0.0, std::f64::consts::PI * 2.0, 0.0, 6.0);
                    self.draw_arc.color = vec4(color[0], color[1], color[2], color[3]);
                    self.draw_arc.disable_gradient();
                    self.draw_arc.draw_arc(cx, dvec2(px, py), 6.0);
                }
            }
        }
        DrawStep::done()
    }
}

// ============ Large Points Widget ============
#[derive(Live, LiveHook, Widget)]
pub struct LargePointsWidget {
    #[redraw] #[live] draw_arc: DrawArc,
    #[walk] walk: Walk,
    #[rust] area: Area,
}

impl Widget for LargePointsWidget {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            let padding = 40.0;
            let chart_x = rect.pos.x + padding;
            let chart_y = rect.pos.y + padding;
            let chart_w = rect.size.x - padding * 2.0;
            let chart_h = rect.size.y - padding * 2.0;

            let colors = [
                [0.26, 0.52, 0.96, 1.0],
                [0.92, 0.26, 0.21, 1.0],
                [0.20, 0.66, 0.33, 1.0],
                [1.0, 0.76, 0.03, 1.0],
                [0.61, 0.15, 0.69, 1.0],
            ];

            for i in 0..10usize {
                let x = pseudo_random((i * 7 + 11) as u32);
                let y = pseudo_random((i * 13 + 23) as u32);

                let px = chart_x + x * chart_w;
                let py = chart_y + (1.0 - y) * chart_h;

                self.draw_arc.set_arc(0.0, std::f64::consts::PI * 2.0, 0.0, 12.0);
                self.draw_arc.color = vec4(colors[i % 5][0], colors[i % 5][1], colors[i % 5][2], colors[i % 5][3]);
                self.draw_arc.disable_gradient();
                self.draw_arc.draw_arc(cx, dvec2(px, py), 12.0);
            }
        }
        DrawStep::done()
    }
}

// ============ Small Points Widget ============
#[derive(Live, LiveHook, Widget)]
pub struct SmallPointsWidget {
    #[redraw] #[live] draw_arc: DrawArc,
    #[walk] walk: Walk,
    #[rust] area: Area,
}

impl Widget for SmallPointsWidget {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            let padding = 40.0;
            let chart_x = rect.pos.x + padding;
            let chart_y = rect.pos.y + padding;
            let chart_w = rect.size.x - padding * 2.0;
            let chart_h = rect.size.y - padding * 2.0;

            let color = [0.26, 0.52, 0.96, 1.0];

            for i in 0..50 {
                let x = pseudo_random(i * 7 + 11);
                let y = pseudo_random(i * 13 + 23);

                let px = chart_x + x * chart_w;
                let py = chart_y + (1.0 - y) * chart_h;

                self.draw_arc.set_arc(0.0, std::f64::consts::PI * 2.0, 0.0, 3.0);
                self.draw_arc.color = vec4(color[0], color[1], color[2], color[3]);
                self.draw_arc.disable_gradient();
                self.draw_arc.draw_arc(cx, dvec2(px, py), 3.0);
            }
        }
        DrawStep::done()
    }
}

// ============ Color Gradient Scatter Widget ============
#[derive(Live, LiveHook, Widget)]
pub struct ColorGradientScatterWidget {
    #[redraw] #[live] draw_arc: DrawArc,
    #[walk] walk: Walk,
    #[rust] area: Area,
}

impl Widget for ColorGradientScatterWidget {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            let padding = 40.0;
            let chart_x = rect.pos.x + padding;
            let chart_y = rect.pos.y + padding;
            let chart_w = rect.size.x - padding * 2.0;
            let chart_h = rect.size.y - padding * 2.0;

            // Points with color based on position
            for i in 0..30 {
                let x = pseudo_random(i * 7 + 11);
                let y = pseudo_random(i * 13 + 23);

                let px = chart_x + x * chart_w;
                let py = chart_y + (1.0 - y) * chart_h;

                // Color gradient from blue (low) to red (high) based on y
                let r = y as f32;
                let g = 0.3;
                let b = (1.0 - y) as f32;

                self.draw_arc.set_arc(0.0, std::f64::consts::PI * 2.0, 0.0, 7.0);
                self.draw_arc.color = vec4(r, g, b, 1.0);
                self.draw_arc.disable_gradient();
                self.draw_arc.draw_arc(cx, dvec2(px, py), 7.0);
            }
        }
        DrawStep::done()
    }
}

// ============ Dense Scatter Widget ============
#[derive(Live, LiveHook, Widget)]
pub struct DenseScatterWidget {
    #[redraw] #[live] draw_arc: DrawArc,
    #[walk] walk: Walk,
    #[rust] area: Area,
}

impl Widget for DenseScatterWidget {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            let padding = 40.0;
            let chart_x = rect.pos.x + padding;
            let chart_y = rect.pos.y + padding;
            let chart_w = rect.size.x - padding * 2.0;
            let chart_h = rect.size.y - padding * 2.0;

            // Create correlation pattern (y roughly follows x)
            for i in 0..100 {
                let base_x = pseudo_random(i * 7 + 11);
                let noise = (pseudo_random(i * 17 + 31) - 0.5) * 0.3;
                let y = (base_x + noise).clamp(0.0, 1.0);

                let px = chart_x + base_x * chart_w;
                let py = chart_y + (1.0 - y) * chart_h;

                self.draw_arc.set_arc(0.0, std::f64::consts::PI * 2.0, 0.0, 3.0);
                self.draw_arc.color = vec4(0.26, 0.52, 0.96, 0.6);
                self.draw_arc.disable_gradient();
                self.draw_arc.draw_arc(cx, dvec2(px, py), 3.0);
            }
        }
        DrawStep::done()
    }
}
