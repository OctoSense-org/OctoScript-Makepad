//! Beeswarm chart - force-positioned dots on a single axis
//!
//! Shows distribution of data points along one axis, with force simulation
//! to prevent overlap while maintaining accurate position on the primary axis.

use makepad_widgets::*;
use super::draw_primitives::{DrawPoint, DrawChartLine};
use super::animation::{ChartAnimator, get_color};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawPoint;
    use super::draw_primitives::DrawChartLine;

    pub BeeswarmChart = {{BeeswarmChart}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Clone, Debug)]
pub struct BeeswarmPoint {
    pub value: f64,
    pub category: String,
    pub label: String,
    pub color: Option<Vec4>,
    // Computed positions
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
}

impl BeeswarmPoint {
    pub fn new(value: f64, label: impl Into<String>) -> Self {
        Self {
            value,
            category: String::new(),
            label: label.into(),
            color: None,
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
        }
    }

    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = category.into();
        self
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BeeswarmChart {
    #[redraw]
    #[live]
    draw_point: DrawPoint,

    #[redraw]
    #[live]
    draw_line: DrawChartLine,

    #[walk]
    walk: Walk,

    #[rust]
    points: Vec<BeeswarmPoint>,

    #[rust]
    radius: f64,

    #[rust]
    horizontal: bool,

    #[rust]
    simulation_steps: usize,

    #[rust]
    categories: Vec<String>,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,
}

impl BeeswarmChart {
    pub fn set_data(&mut self, points: Vec<BeeswarmPoint>) {
        // Extract unique categories
        let mut categories: Vec<String> = points
            .iter()
            .filter(|p| !p.category.is_empty())
            .map(|p| p.category.clone())
            .collect();
        categories.sort();
        categories.dedup();
        self.categories = categories;

        self.points = points;
        self.run_simulation();
        self.initialized = false;
    }

    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
        self.run_simulation();
    }

    pub fn set_horizontal(&mut self, horizontal: bool) {
        self.horizontal = horizontal;
        self.run_simulation();
    }

    fn run_simulation(&mut self) {
        if self.points.is_empty() {
            return;
        }

        // Find value range
        let min_val = self.points.iter().map(|p| p.value).fold(f64::INFINITY, f64::min);
        let max_val = self.points.iter().map(|p| p.value).fold(f64::NEG_INFINITY, f64::max);
        let range = (max_val - min_val).max(1.0);

        // Initialize positions based on value
        for point in &mut self.points {
            let normalized = (point.value - min_val) / range;
            if self.horizontal {
                point.x = normalized;
                point.y = 0.5;
            } else {
                point.x = 0.5;
                point.y = 1.0 - normalized;
            }
            point.vx = 0.0;
            point.vy = 0.0;
        }

        // Run force simulation
        let collision_radius = self.radius * 2.2 / 400.0; // Normalized radius
        let strength = 0.5;

        for _ in 0..self.simulation_steps {
            // Apply collision forces
            let n = self.points.len();
            for i in 0..n {
                for j in (i + 1)..n {
                    let dx = self.points[j].x - self.points[i].x;
                    let dy = self.points[j].y - self.points[i].y;
                    let dist = (dx * dx + dy * dy).sqrt().max(0.001);

                    if dist < collision_radius {
                        let force = (collision_radius - dist) / dist * strength;

                        // Only apply force perpendicular to the main axis
                        if self.horizontal {
                            self.points[i].vy -= dy * force;
                            self.points[j].vy += dy * force;
                        } else {
                            self.points[i].vx -= dx * force;
                            self.points[j].vx += dx * force;
                        }
                    }
                }
            }

            // Apply velocity and damping
            for point in &mut self.points {
                if self.horizontal {
                    point.y += point.vy;
                    point.vy *= 0.6;
                    point.y = point.y.clamp(0.1, 0.9);
                } else {
                    point.x += point.vx;
                    point.vx *= 0.6;
                    point.x = point.x.clamp(0.1, 0.9);
                }
            }
        }
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        if self.points.is_empty() {
            return;
        }

        let padding = 60.0;
        let chart_x = rect.pos.x + padding;
        let chart_y = rect.pos.y + padding;
        let chart_w = rect.size.x - padding * 2.0;
        let chart_h = rect.size.y - padding * 2.0;

        // Draw axis
        self.draw_line.color = vec4(0.4, 0.4, 0.4, 1.0);

        if self.horizontal {
            self.draw_line.draw_line(
                cx,
                dvec2(chart_x, chart_y + chart_h / 2.0),
                dvec2(chart_x + chart_w, chart_y + chart_h / 2.0),
                1.5,
            );
        } else {
            self.draw_line.draw_line(
                cx,
                dvec2(chart_x + chart_w / 2.0, chart_y),
                dvec2(chart_x + chart_w / 2.0, chart_y + chart_h),
                1.5,
            );
        }

        // Color palette for categories
        let colors = [
            vec4(0.4, 0.76, 0.65, 1.0),
            vec4(0.99, 0.55, 0.38, 1.0),
            vec4(0.55, 0.63, 0.80, 1.0),
            vec4(0.91, 0.84, 0.42, 1.0),
            vec4(0.65, 0.85, 0.33, 1.0),
            vec4(0.90, 0.45, 0.77, 1.0),
        ];

        // Get animation progress
        let progress = self.animator.get_progress();

        // Draw points
        for (i, point) in self.points.iter().enumerate() {
            let px = chart_x + point.x * chart_w;
            let py = chart_y + point.y * chart_h;

            let color = point.color.unwrap_or_else(|| {
                if !point.category.is_empty() {
                    let cat_idx = self.categories.iter().position(|c| c == &point.category).unwrap_or(0);
                    colors[cat_idx % colors.len()]
                } else {
                    get_color(i)
                }
            });

            let point_progress = (progress as f32 * self.points.len() as f32 - i as f32).clamp(0.0, 1.0);
            let animated_radius = self.radius * point_progress as f64;

            self.draw_point.color = color;
            self.draw_point.draw_point(cx, dvec2(px, py), animated_radius);
        }
    }
}

impl BeeswarmChart {
    fn initialize_demo_data(&mut self) {
        // Sample data points with categories
        self.points = vec![
            BeeswarmPoint::new(23.0, "A").with_category("Group 1"),
            BeeswarmPoint::new(28.0, "B").with_category("Group 1"),
            BeeswarmPoint::new(25.0, "C").with_category("Group 1"),
            BeeswarmPoint::new(45.0, "D").with_category("Group 2"),
            BeeswarmPoint::new(48.0, "E").with_category("Group 2"),
            BeeswarmPoint::new(42.0, "F").with_category("Group 2"),
            BeeswarmPoint::new(67.0, "G").with_category("Group 3"),
            BeeswarmPoint::new(72.0, "H").with_category("Group 3"),
            BeeswarmPoint::new(70.0, "I").with_category("Group 3"),
            BeeswarmPoint::new(35.0, "J").with_category("Group 1"),
            BeeswarmPoint::new(55.0, "K").with_category("Group 2"),
            BeeswarmPoint::new(80.0, "L").with_category("Group 3"),
            BeeswarmPoint::new(30.0, "M").with_category("Group 1"),
            BeeswarmPoint::new(50.0, "N").with_category("Group 2"),
            BeeswarmPoint::new(75.0, "O").with_category("Group 3"),
        ];
        self.radius = 8.0;
        self.horizontal = true;
        self.simulation_steps = 50;
        self.run_simulation();
    }
}

impl Widget for BeeswarmChart {
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
