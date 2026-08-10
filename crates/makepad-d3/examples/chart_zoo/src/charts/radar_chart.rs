//! Radar/Spider Chart Widget
//!
//! Multi-axis radial chart for comparing multiple variables across categories.
//! GPU-accelerated with proper animation support.

use makepad_widgets::*;
use std::f64::consts::PI;
use super::draw_primitives::{DrawTriangle, DrawPoint, DrawChartLine};
use super::animation::{ChartAnimator, EasingType};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawTriangle;
    use super::draw_primitives::DrawPoint;
    use super::draw_primitives::DrawChartLine;

    pub RadarChartWidget = {{RadarChartWidget}} {
        width: Fill,
        height: Fill,
    }
}

/// Dataset for radar chart
#[derive(Clone, Debug)]
pub struct RadarDataset {
    pub values: Vec<f64>,
    pub color: Vec4,
    pub fill_alpha: f32,
}

/// Data structure for radar charts
#[derive(Clone, Debug, Default)]
pub struct RadarData {
    pub labels: Vec<String>,
    pub datasets: Vec<RadarDataset>,
}

impl RadarData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_labels<S: Into<String>>(mut self, labels: Vec<S>) -> Self {
        self.labels = labels.into_iter().map(|s| s.into()).collect();
        self
    }

    pub fn add_dataset(mut self, values: Vec<f64>, color: Vec4, fill_alpha: f32) -> Self {
        self.datasets.push(RadarDataset { values, color, fill_alpha });
        self
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct RadarChartWidget {
    #[live]
    #[deref]
    view: View,

    #[redraw]
    #[live]
    draw_line: DrawChartLine,

    #[redraw]
    #[live]
    draw_point: DrawPoint,

    #[redraw]
    #[live]
    draw_grid: DrawChartLine,

    #[redraw]
    #[live]
    draw_fill: DrawTriangle,

    #[walk]
    walk: Walk,

    #[rust]
    radar_data: RadarData,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    center: DVec2,

    #[rust]
    radius: f64,

    #[rust(0.0)]
    padding: f64,

    #[rust]
    grid_levels: usize,

    #[rust(true)]
    show_grid: bool,

    #[rust(true)]
    show_points: bool,

    #[rust(true)]
    show_fill: bool,

    #[rust(5.0)]
    point_radius: f64,

    /// Enable gradient fill (radial from center to edges)
    #[rust(true)]
    gradient_enabled: bool,

    #[rust]
    area: Area,
}

impl Widget for RadarChartWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

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
            self.update_layout(rect);

            if !self.initialized {
                self.initialize_data();
                self.start_animation(cx);
                self.initialized = true;
            }

            if self.show_grid {
                self.draw_grid_lines(cx);
            }
            self.draw_datasets(cx);
        }

        DrawStep::done()
    }
}

impl RadarChartWidget {
    fn initialize_data(&mut self) {
        // Default sample data if not set
        if self.radar_data.datasets.is_empty() {
            self.radar_data = RadarData::new()
                .with_labels(vec!["Speed", "Power", "Accuracy", "Endurance", "Agility", "Defense"])
                .add_dataset(
                    vec![0.85, 0.70, 0.90, 0.65, 0.80, 0.75],
                    vec4(0.26, 0.52, 0.96, 1.0),
                    0.3,
                )
                .add_dataset(
                    vec![0.70, 0.85, 0.65, 0.90, 0.60, 0.80],
                    vec4(0.92, 0.26, 0.21, 1.0),
                    0.3,
                );
        }
    }

    pub fn set_data(&mut self, data: RadarData) {
        self.radar_data = data;
        self.initialized = false;
    }

    pub fn set_fill(&mut self, show_fill: bool) {
        self.show_fill = show_fill;
    }

    pub fn set_gradient(&mut self, enabled: bool) {
        self.gradient_enabled = enabled;
    }

    pub fn set_padding(&mut self, padding: f64) {
        self.padding = padding;
        self.radius = 0.0;
    }

    fn update_layout(&mut self, rect: Rect) {
        let size = rect.size.x.min(rect.size.y) - self.padding * 2.0;
        self.radius = size / 2.0 - 40.0; // Leave room for labels
        self.center = dvec2(
            rect.pos.x + rect.size.x / 2.0,
            rect.pos.y + rect.size.y / 2.0,
        );
        if self.grid_levels == 0 {
            self.grid_levels = 5;
        }
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(800.0) // 800ms animation
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    pub fn replay_animation(&mut self, cx: &mut Cx) {
        self.initialized = false;
        self.animator.reset();
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(800.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        self.initialized = true;
        cx.new_next_frame();
        self.redraw(cx);
    }

    pub fn is_animating(&self) -> bool {
        self.animator.is_running()
    }

    fn get_num_axes(&self) -> usize {
        self.radar_data.labels.len().max(
            self.radar_data.datasets.first()
                .map(|d| d.values.len())
                .unwrap_or(0)
        )
    }

    fn get_angle(&self, index: usize, total: usize) -> f64 {
        -PI / 2.0 + (index as f64 / total as f64) * 2.0 * PI
    }

    fn get_point(&self, angle: f64, value: f64) -> DVec2 {
        let dist = value * self.radius;
        dvec2(
            self.center.x + dist * angle.cos(),
            self.center.y + dist * angle.sin(),
        )
    }

    fn draw_grid_lines(&mut self, cx: &mut Cx2d) {
        let num_axes = self.get_num_axes();
        if num_axes < 3 {
            return;
        }

        self.draw_grid.color = vec4(0.8, 0.8, 0.8, 0.5);

        // Draw concentric polygons
        for level in 1..=self.grid_levels {
            let ratio = level as f64 / self.grid_levels as f64;

            for i in 0..num_axes {
                let angle1 = self.get_angle(i, num_axes);
                let angle2 = self.get_angle((i + 1) % num_axes, num_axes);

                let p1 = dvec2(
                    self.center.x + self.radius * ratio * angle1.cos(),
                    self.center.y + self.radius * ratio * angle1.sin(),
                );
                let p2 = dvec2(
                    self.center.x + self.radius * ratio * angle2.cos(),
                    self.center.y + self.radius * ratio * angle2.sin(),
                );

                self.draw_grid.draw_line(cx, p1, p2, 1.0);
            }
        }

        // Draw axis lines from center
        self.draw_grid.color = vec4(0.7, 0.7, 0.7, 0.8);
        for i in 0..num_axes {
            let angle = self.get_angle(i, num_axes);
            let outer = dvec2(
                self.center.x + self.radius * angle.cos(),
                self.center.y + self.radius * angle.sin(),
            );
            self.draw_grid.draw_line(cx, self.center, outer, 1.0);
        }
    }

    fn draw_datasets(&mut self, cx: &mut Cx2d) {
        let progress = self.animator.get_progress();
        let num_axes = self.get_num_axes();

        if num_axes < 3 {
            return;
        }

        // Clone datasets to avoid borrow issues
        let datasets: Vec<_> = self.radar_data.datasets.iter().cloned().collect();

        for (dataset_idx, dataset) in datasets.iter().enumerate() {
            let color = dataset.color;

            // Collect points with animation
            let points: Vec<DVec2> = (0..num_axes).map(|i| {
                let value = dataset.values.get(i).copied().unwrap_or(0.0);
                let animated_value = value * progress;
                let angle = self.get_angle(i, num_axes);
                self.get_point(angle, animated_value)
            }).collect();

            // Draw fill first (behind lines)
            if self.show_fill {
                let fill_color = vec4(
                    color.x,
                    color.y,
                    color.z,
                    dataset.fill_alpha
                );
                self.draw_fill.color = fill_color;

                // Apply gradient if enabled
                if self.gradient_enabled {
                    let center_color = vec4(
                        (color.x + (1.0 - color.x) * 0.5).min(1.0),
                        (color.y + (1.0 - color.y) * 0.5).min(1.0),
                        (color.z + (1.0 - color.z) * 0.5).min(1.0),
                        dataset.fill_alpha,
                    );
                    let outer_color = vec4(
                        color.x * 0.8,
                        color.y * 0.8,
                        color.z * 0.8,
                        dataset.fill_alpha * 0.6,
                    );
                    self.draw_fill.set_radial_gradient(center_color, outer_color);
                } else {
                    self.draw_fill.disable_gradient();
                }

                // Draw triangles from center to each edge
                for i in 0..points.len() {
                    let p1 = points[i];
                    let p2 = points[(i + 1) % points.len()];
                    self.draw_fill.draw_triangle(cx, self.center, p1, p2);
                }
            }

            // Draw polygon lines
            self.draw_line.color = color;
            for i in 0..points.len() {
                let p1 = points[i];
                let p2 = points[(i + 1) % points.len()];
                self.draw_line.draw_line(cx, p1, p2, 2.0);
            }

            // Draw points
            if self.show_points {
                self.draw_point.color = color;
                for point in &points {
                    self.draw_point.draw_point(cx, *point, self.point_radius * 2.0);
                }
            }
        }
    }
}
