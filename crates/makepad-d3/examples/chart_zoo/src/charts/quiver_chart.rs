//! Quiver Chart Widget (Vector Field)
//!
//! Displays vector fields using arrows to show direction and magnitude.
//! GPU-accelerated with smooth animation.

use makepad_widgets::*;
use std::f64::consts::PI;
use super::draw_primitives::{DrawChartLine, DrawTriangle};
use super::animation::{ChartAnimator, EasingType};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawChartLine;
    use super::draw_primitives::DrawTriangle;

    pub QuiverChartWidget = {{QuiverChartWidget}} {
        width: Fill,
        height: Fill,
    }
}

/// Vector at a point
#[derive(Clone, Debug)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
}

impl Vector2D {
    pub fn new(x: f64, y: f64, dx: f64, dy: f64) -> Self {
        Self { x, y, dx, dy }
    }

    pub fn magnitude(&self) -> f64 {
        (self.dx * self.dx + self.dy * self.dy).sqrt()
    }

    pub fn angle(&self) -> f64 {
        self.dy.atan2(self.dx)
    }
}

/// Vector field data
#[derive(Clone, Debug, Default)]
pub struct VectorFieldData {
    pub vectors: Vec<Vector2D>,
    pub max_magnitude: f64,
}

impl VectorFieldData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_vector(&mut self, v: Vector2D) {
        let mag = v.magnitude();
        if mag > self.max_magnitude {
            self.max_magnitude = mag;
        }
        self.vectors.push(v);
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct QuiverChartWidget {
    #[redraw]
    #[live]
    draw_line: DrawChartLine,

    #[redraw]
    #[live]
    draw_arrow: DrawTriangle,

    #[walk]
    walk: Walk,

    #[rust]
    field_data: VectorFieldData,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,

    #[rust]
    chart_rect: Rect,

    #[rust]
    grid_resolution: usize,

    #[rust(15.0)]
    arrow_scale: f64,

    #[rust(true)]
    color_by_magnitude: bool,

    #[rust(true)]
    show_streamlines: bool,
}

impl Widget for QuiverChartWidget {
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
                self.initialize_data();
                self.start_animation(cx);
                self.initialized = true;
            }

            self.draw_field(cx);
        }

        DrawStep::done()
    }
}

impl QuiverChartWidget {
    fn initialize_data(&mut self) {
        if self.grid_resolution == 0 {
            self.grid_resolution = 12;
        }
        let n = self.grid_resolution;
        let mut field = VectorFieldData::new();

        // Create interesting vector field pattern
        // This creates a vortex/dipole pattern
        for i in 0..n {
            for j in 0..n {
                let x = (i as f64 / (n - 1) as f64) * 2.0 - 1.0;
                let y = (j as f64 / (n - 1) as f64) * 2.0 - 1.0;

                // Vortex field with two centers
                let (dx1, dy1) = self.vortex_field(x - 0.3, y - 0.2, 0.5);
                let (dx2, dy2) = self.vortex_field(x + 0.3, y + 0.2, -0.4);

                // Source/sink at center
                let r = (x * x + y * y).sqrt().max(0.1);
                let source_strength = 0.2 * (-r * 2.0).exp();
                let dx3 = x / r * source_strength;
                let dy3 = y / r * source_strength;

                // Combine fields
                let dx = dx1 + dx2 + dx3;
                let dy = dy1 + dy2 + dy3;

                field.add_vector(Vector2D::new(
                    i as f64 / (n - 1) as f64,
                    j as f64 / (n - 1) as f64,
                    dx,
                    dy,
                ));
            }
        }

        self.field_data = field;
    }

    fn vortex_field(&self, x: f64, y: f64, strength: f64) -> (f64, f64) {
        let r = (x * x + y * y).sqrt().max(0.1);
        let decay = (-r * 2.0).exp();
        let dx = -y / r * strength * decay;
        let dy = x / r * strength * decay;
        (dx, dy)
    }

    pub fn set_data(&mut self, data: VectorFieldData) {
        self.field_data = data;
        self.initialized = false;
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1500.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    pub fn replay_animation(&mut self, cx: &mut Cx) {
        self.animator.reset();
        self.start_animation(cx);
        self.redraw(cx);
    }

    fn magnitude_to_color(&self, magnitude: f64) -> Vec4 {
        let t = (magnitude / self.field_data.max_magnitude).clamp(0.0, 1.0) as f32;

        // Cool (blue) to hot (red) colormap
        if t < 0.25 {
            let s = t / 0.25;
            vec4(0.0, 0.0, 0.5 + s * 0.5, 1.0)
        } else if t < 0.5 {
            let s = (t - 0.25) / 0.25;
            vec4(0.0, s, 1.0, 1.0)
        } else if t < 0.75 {
            let s = (t - 0.5) / 0.25;
            vec4(s, 1.0, 1.0 - s, 1.0)
        } else {
            let s = (t - 0.75) / 0.25;
            vec4(1.0, 1.0 - s * 0.5, 0.0, 1.0)
        }
    }

    fn draw_field(&mut self, cx: &mut Cx2d) {
        let progress = self.animator.get_progress();
        let rect = self.chart_rect;
        let padding = 30.0;

        let draw_width = rect.size.x - padding * 2.0;
        let draw_height = rect.size.y - padding * 2.0;

        // Draw grid background
        self.draw_grid(cx, rect, padding);

        // Draw each vector as an arrow
        for vector in &self.field_data.vectors.clone() {
            let x = rect.pos.x + padding + vector.x * draw_width;
            let y = rect.pos.y + padding + vector.y * draw_height;

            let magnitude = vector.magnitude();
            let animated_mag = magnitude * progress;

            // Scale arrow length
            let scale = self.arrow_scale * (draw_width / self.grid_resolution as f64) * 0.8;
            let length = (animated_mag / self.field_data.max_magnitude) * scale;

            if length < 1.0 {
                continue;
            }

            let angle = vector.angle();

            // End point
            let ex = x + angle.cos() * length;
            let ey = y + angle.sin() * length;

            // Color based on magnitude
            let color = if self.color_by_magnitude {
                self.magnitude_to_color(magnitude)
            } else {
                vec4(0.3, 0.5, 0.8, 1.0)
            };

            // Draw arrow shaft
            self.draw_line.color = color;
            self.draw_line.draw_line(cx, dvec2(x, y), dvec2(ex, ey), 1.5);

            // Draw arrowhead
            let head_size = length * 0.3;
            let head_angle = 0.4; // radians

            let h1_angle = angle + PI - head_angle;
            let h2_angle = angle + PI + head_angle;

            let h1 = dvec2(
                ex + h1_angle.cos() * head_size,
                ey + h1_angle.sin() * head_size,
            );
            let h2 = dvec2(
                ex + h2_angle.cos() * head_size,
                ey + h2_angle.sin() * head_size,
            );

            // Draw filled arrowhead triangle
            self.draw_arrow.color = color;
            self.draw_arrow.disable_gradient();
            self.draw_arrow.draw_triangle(cx, dvec2(ex, ey), h1, h2);
        }
    }

    fn draw_grid(&mut self, cx: &mut Cx2d, rect: Rect, padding: f64) {
        let grid_color = vec4(0.85, 0.85, 0.85, 0.5);
        self.draw_line.color = grid_color;

        let draw_width = rect.size.x - padding * 2.0;
        let draw_height = rect.size.y - padding * 2.0;

        // Draw 5 vertical and horizontal lines
        for i in 0..=4 {
            let t = i as f64 / 4.0;

            // Vertical
            let x = rect.pos.x + padding + t * draw_width;
            self.draw_line.draw_line(
                cx,
                dvec2(x, rect.pos.y + padding),
                dvec2(x, rect.pos.y + padding + draw_height),
                1.0,
            );

            // Horizontal
            let y = rect.pos.y + padding + t * draw_height;
            self.draw_line.draw_line(
                cx,
                dvec2(rect.pos.x + padding, y),
                dvec2(rect.pos.x + padding + draw_width, y),
                1.0,
            );
        }
    }
}
