//! Hexbin chart - hexagonal binning visualization
//!
//! Aggregates points into hexagonal bins, showing density through
//! color or size. Great for visualizing large point datasets.

use makepad_widgets::*;
use super::draw_primitives::{DrawTriangle, DrawChartLine, DrawPoint};
use super::animation::ChartAnimator;

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawTriangle;
    use super::draw_primitives::DrawChartLine;
    use super::draw_primitives::DrawPoint;

    pub HexbinChart = {{HexbinChart}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Clone, Debug)]
pub struct HexbinPoint {
    pub x: f64,
    pub y: f64,
}

impl HexbinPoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug)]
struct HexBin {
    center: DVec2,
    count: usize,
    ring: i32,  // Distance from center in cube coordinates
}

#[derive(Live, LiveHook, Widget)]
pub struct HexbinChart {
    #[redraw]
    #[live]
    draw_triangle: DrawTriangle,

    #[redraw]
    #[live]
    draw_line: DrawChartLine,

    #[redraw]
    #[live]
    draw_point: DrawPoint,

    #[walk]
    walk: Walk,

    #[rust]
    points: Vec<HexbinPoint>,

    #[rust]
    hex_radius: f64,

    #[rust]
    color_low: Vec4,

    #[rust]
    color_high: Vec4,

    #[rust]
    show_points: bool,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,
}

impl HexbinChart {
    pub fn set_data(&mut self, points: Vec<HexbinPoint>) {
        self.points = points;
        self.initialized = false;
    }

    pub fn set_hex_radius(&mut self, radius: f64) {
        self.hex_radius = radius.max(5.0);
    }

    pub fn set_colors(&mut self, low: Vec4, high: Vec4) {
        self.color_low = low;
        self.color_high = high;
    }

    pub fn set_show_points(&mut self, show: bool) {
        self.show_points = show;
    }

    fn calculate_bins(&self, chart_x: f64, chart_y: f64, chart_w: f64, chart_h: f64, rings: i32) -> (Vec<HexBin>, i32) {
        // Center of the chart - this will be the center of the large hexagon
        let center_x = chart_x + chart_w / 2.0;
        let center_y = chart_y + chart_h / 2.0;

        // Initialize bin counts with ring distance
        let mut bin_data: std::collections::HashMap<(i32, i32, i32), (usize, i32)> = std::collections::HashMap::new();

        // Generate hexagonal grid using cube coordinates
        // A hexagon of radius N has all hexes where |q| + |r| + |s| <= N and q + r + s = 0
        for q in -rings..=rings {
            for r in (-rings).max(-q - rings)..=rings.min(-q + rings) {
                let s = -q - r;
                // Ring distance is max of absolute values
                let ring = q.abs().max(r.abs()).max(s.abs());
                bin_data.insert((q, r, s), (0, ring));
            }
        }

        // Find data range if we have points
        let chart_size = chart_w.min(chart_h);
        if !self.points.is_empty() {
            let x_min = self.points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
            let x_max = self.points.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
            let y_min = self.points.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
            let y_max = self.points.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);

            let x_range = (x_max - x_min).max(1.0);
            let y_range = (y_max - y_min).max(1.0);

            // Assign points to bins using cube coordinates
            for point in &self.points {
                let px = ((point.x - x_min) / x_range) * chart_size - chart_size / 2.0;
                let py = ((point.y - y_min) / y_range) * chart_size - chart_size / 2.0;

                // Convert pixel to cube coordinates (pointy-topped)
                let q = (px * 3.0_f64.sqrt() / 3.0 - py / 3.0) / self.hex_radius;
                let r = (py * 2.0 / 3.0) / self.hex_radius;

                // Round to nearest hex
                let (q, r, s) = Self::cube_round(q, r);

                if let Some((count, _)) = bin_data.get_mut(&(q, r, s)) {
                    *count += 1;
                }
            }
        }

        // Convert cube coordinates to pixel positions
        let mut bins = Vec::new();
        for ((q, r, _s), (count, ring)) in bin_data {
            // Cube to pixel (pointy-topped)
            let px = self.hex_radius * (3.0_f64.sqrt() * q as f64 + 3.0_f64.sqrt() / 2.0 * r as f64);
            let py = self.hex_radius * (3.0 / 2.0 * r as f64);

            let cx = center_x + px;
            let cy = center_y + py;

            bins.push(HexBin {
                center: DVec2 { x: cx, y: cy },
                count,
                ring,
            });
        }

        (bins, rings)
    }

    fn cube_round(q: f64, r: f64) -> (i32, i32, i32) {
        let s = -q - r;

        let mut rq = q.round();
        let mut rr = r.round();
        let mut rs = s.round();

        let q_diff = (rq - q).abs();
        let r_diff = (rr - r).abs();
        let s_diff = (rs - s).abs();

        if q_diff > r_diff && q_diff > s_diff {
            rq = -rr - rs;
        } else if r_diff > s_diff {
            rr = -rq - rs;
        } else {
            rs = -rq - rr;
        }

        (rq as i32, rr as i32, rs as i32)
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        let padding = 20.0;
        let chart_x = rect.pos.x + padding;
        let chart_y = rect.pos.y + padding;
        let chart_w = rect.size.x - padding * 2.0;
        let chart_h = rect.size.y - padding * 2.0;

        // Calculate number of rings to fill the available space
        let chart_size = chart_w.min(chart_h);
        let rings = ((chart_size / 2.0) / (self.hex_radius * 1.5)).floor() as i32;

        // Calculate bins with ring information
        let (bins, max_ring) = self.calculate_bins(chart_x, chart_y, chart_w, chart_h, rings);

        if bins.is_empty() {
            return;
        }

        // Get animation progress
        let progress = self.animator.get_progress();

        // Sort bins by ring so center draws first (for proper layering if needed)
        let mut sorted_bins: Vec<_> = bins.iter().enumerate().collect();
        sorted_bins.sort_by_key(|(_, b)| b.ring);

        // Draw hexagons with radial gradient (dark center, light edge)
        for (i, bin) in sorted_bins.iter() {
            // Radial gradient: t=0 at center (dark), t=1 at edge (light)
            let t = if max_ring > 0 { bin.ring as f64 / max_ring as f64 } else { 0.0 };

            // Apply smooth easing for more gradual color transition
            let t_eased = Self::smooth_step(t);
            let color = self.interpolate_radial_color(t_eased);

            // Animate hexagon appearance from center outward
            let ring_delay = bin.ring as f32 * 0.08;
            let hex_progress = ((progress as f32 * 2.0 - ring_delay) * 1.5).clamp(0.0, 1.0);
            let animated_radius = self.hex_radius * 0.94 * hex_progress as f64;

            if animated_radius > 1.0 {
                self.draw_hexagon(cx, bin.center, animated_radius, color);
            }
        }

        // Draw radial gradient legend (center to edge)
        let legend_x = rect.pos.x + rect.size.x - 140.0;
        let legend_y = rect.pos.y + 15.0;
        let legend_w = 120.0;
        let legend_h = 12.0;
        let legend_steps = 20;  // More steps for smoother gradient

        for i in 0..legend_steps {
            let t = i as f64 / (legend_steps - 1) as f64;
            let t_eased = Self::smooth_step(t);
            let color = self.interpolate_radial_color(t_eased);
            let x = legend_x + (i as f64 / legend_steps as f64) * legend_w;

            self.draw_triangle.color = color;
            let w = legend_w / legend_steps as f64 + 1.0;
            self.draw_triangle.draw_triangle(
                cx,
                dvec2(x, legend_y),
                dvec2(x + w, legend_y),
                dvec2(x + w, legend_y + legend_h),
            );
            self.draw_triangle.draw_triangle(
                cx,
                dvec2(x, legend_y),
                dvec2(x + w, legend_y + legend_h),
                dvec2(x, legend_y + legend_h),
            );
        }
    }

    // Smooth step function for gradual color transition
    fn smooth_step(t: f64) -> f64 {
        // Hermite interpolation for smoother gradient
        t * t * (3.0 - 2.0 * t)
    }

    fn interpolate_color(&self, t: f64) -> Vec4 {
        vec4(
            self.color_low.x + t as f32 * (self.color_high.x - self.color_low.x),
            self.color_low.y + t as f32 * (self.color_high.y - self.color_low.y),
            self.color_low.z + t as f32 * (self.color_high.z - self.color_low.z),
            self.color_low.w + t as f32 * (self.color_high.w - self.color_low.w),
        )
    }

    // Radial gradient: dark at center (t=0), light at edge (t=1)
    fn interpolate_radial_color(&self, t: f64) -> Vec4 {
        // Use color_high for center (dark), color_low for edge (light)
        vec4(
            self.color_high.x + t as f32 * (self.color_low.x - self.color_high.x),
            self.color_high.y + t as f32 * (self.color_low.y - self.color_high.y),
            self.color_high.z + t as f32 * (self.color_low.z - self.color_high.z),
            self.color_high.w + t as f32 * (self.color_low.w - self.color_high.w),
        )
    }

    fn draw_hexagon(&mut self, cx: &mut Cx2d, center: DVec2, radius: f64, color: Vec4) {
        // Pointy-topped hexagon (D3 style) - vertices at top and bottom
        let corners: Vec<DVec2> = (0..6)
            .map(|i| {
                // Start at 90 degrees (top) for pointy-topped orientation
                let angle = std::f64::consts::PI / 3.0 * i as f64 + std::f64::consts::PI / 2.0;
                DVec2 {
                    x: center.x + radius * angle.cos(),
                    y: center.y + radius * angle.sin(),
                }
            })
            .collect();

        self.draw_triangle.color = color;

        // Draw as triangles from center
        for i in 0..6 {
            self.draw_triangle.draw_triangle(cx, center, corners[i], corners[(i + 1) % 6]);
        }
    }
}

impl HexbinChart {
    fn initialize_demo_data(&mut self) {
        // Generate clustered random points for density visualization
        let mut points = Vec::new();

        // Dense cluster 1 - center left
        for i in 0..50 {
            let angle = i as f64 * 0.4;
            let r = (i as f64 * 0.3) % 15.0;
            let x = 25.0 + r * angle.cos();
            let y = 50.0 + r * angle.sin();
            points.push(HexbinPoint::new(x, y));
        }

        // Dense cluster 2 - center right
        for i in 0..45 {
            let angle = i as f64 * 0.35;
            let r = (i as f64 * 0.25) % 12.0;
            let x = 75.0 + r * angle.cos();
            let y = 50.0 + r * angle.sin();
            points.push(HexbinPoint::new(x, y));
        }

        // Sparse scatter across the field
        for i in 0..60 {
            let x = (i as f64 * 17.3) % 100.0;
            let y = (i as f64 * 23.7) % 100.0;
            points.push(HexbinPoint::new(x, y));
        }

        // Small cluster top
        for i in 0..20 {
            let x = 50.0 + (i as f64 * 2.1) % 10.0 - 5.0;
            let y = 20.0 + (i as f64 * 1.7) % 8.0 - 4.0;
            points.push(HexbinPoint::new(x, y));
        }

        self.points = points;
        self.hex_radius = 14.0;  // Smaller radius = more hexes = larger overall hexagonal shape
        // Radial gradient: dark blue at center, light blue/white at edge
        self.color_high = vec4(0.05, 0.15, 0.45, 1.0);  // Deep dark blue for center
        self.color_low = vec4(0.92, 0.95, 0.98, 1.0);   // Very light blue/white for edge
        self.show_points = false;
    }
}

impl Widget for HexbinChart {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);

        if !self.initialized {
            self.initialize_demo_data();
            self.animator = ChartAnimator::new(1.2 * 1000.0);
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
