//! Contour Chart Widget (D3 smooth contours inspired)
//!
//! Displays density contours or topographic-style visualizations.
//! GPU-accelerated with smooth color interpolation and animation.
//! Inspired by D3's smooth contours example with Maungawhau volcano data.

use makepad_widgets::*;
use super::draw_primitives::DrawChartLine;
use super::animation::{ChartAnimator, EasingType};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawChartLine;

    // Filled contour region using GPU shader
    pub DrawContourFill = {{DrawContourFill}} {
        fn pixel(self) -> vec4 {
            // Smooth gradient based on normalized value
            let alpha = self.fill_alpha;
            return vec4(self.color.rgb * alpha, alpha);
        }
    }

    pub ContourChartWidget = {{ContourChartWidget}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawContourFill {
    #[deref] pub draw_super: DrawQuad,
    #[live] pub color: Vec4,
    #[live(0.8)] pub fill_alpha: f32,
}

/// Contour data for visualization
#[derive(Clone, Debug)]
pub struct ContourData {
    /// Grid of values (row-major)
    pub values: Vec<Vec<f64>>,
    /// Contour levels to draw
    pub levels: Vec<f64>,
    /// Min/max range
    pub min_val: f64,
    pub max_val: f64,
}

impl Default for ContourData {
    fn default() -> Self {
        Self {
            values: Vec::new(),
            levels: Vec::new(),
            min_val: 0.0,
            max_val: 1.0,
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ContourChartWidget {
    #[redraw]
    #[live]
    draw_fill: DrawContourFill,

    #[redraw]
    #[live]
    draw_line: DrawChartLine,

    #[walk]
    walk: Walk,

    #[rust]
    contour_data: ContourData,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,

    #[rust]
    chart_rect: Rect,

    #[rust]
    grid_size: usize,

    #[rust(true)]
    show_lines: bool,

    #[rust(true)]
    show_fill: bool,

    #[rust(true)]
    smooth: bool,  // D3-style smooth contours

    #[rust]
    use_grayscale: bool,  // Grayscale color scheme like D3 example

    #[rust]
    use_magma: bool,  // Magma color scheme like GeoTIFF example
}

impl Widget for ContourChartWidget {
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

            self.draw_contours(cx);
        }

        DrawStep::done()
    }
}

impl ContourChartWidget {
    fn initialize_data(&mut self) {
        // Set default grid size - higher resolution for smoother contours
        if self.grid_size == 0 {
            self.grid_size = 61;  // Similar to Maungawhau volcano data (87x61)
        }

        // Generate volcano-like topographic data (Maungawhau style)
        let rows = self.grid_size;
        let cols = (rows as f64 * 1.4) as usize;  // Aspect ratio similar to volcano
        let mut values = vec![vec![0.0; cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                let x = j as f64 / cols as f64;
                let y = i as f64 / rows as f64;

                // Base elevation - gentle slope
                let base = 90.0 + 20.0 * (1.0 - y);

                // Main volcanic peak (off-center)
                let px = 0.55;
                let py = 0.45;
                let dx = x - px;
                let dy = y - py;
                let dist = (dx * dx + dy * dy).sqrt();
                let peak = 100.0 * (-dist * dist * 8.0).exp();

                // Secondary ridge
                let rx = 0.35;
                let ry = 0.6;
                let rdx = x - rx;
                let rdy = y - ry;
                let ridge_dist = (rdx * rdx + rdy * rdy).sqrt();
                let ridge = 40.0 * (-ridge_dist * ridge_dist * 12.0).exp();

                // Crater depression at the top
                let crater_dist = ((x - 0.55).powi(2) + (y - 0.42).powi(2)).sqrt();
                let crater = if crater_dist < 0.08 {
                    -15.0 * (1.0 - crater_dist / 0.08)
                } else {
                    0.0
                };

                // Small variations for natural look
                let noise = ((x * 20.0).sin() * (y * 15.0).cos()) * 3.0
                    + ((x * 35.0).cos() * (y * 25.0).sin()) * 2.0;

                values[i][j] = base + peak + ridge + crater + noise;
            }
        }

        // Find min/max
        let mut min_val = f64::MAX;
        let mut max_val = f64::MIN;
        for row in &values {
            for &v in row {
                min_val = min_val.min(v);
                max_val = max_val.max(v);
            }
        }

        // Generate more contour levels for smoother appearance (D3 uses ~20)
        let num_levels = 20;
        let levels: Vec<f64> = (0..=num_levels)
            .map(|i| min_val + (max_val - min_val) * (i as f64 / num_levels as f64))
            .collect();

        self.contour_data = ContourData {
            values,
            levels,
            min_val,
            max_val,
        };

        // Use grayscale like D3 example
        self.use_grayscale = true;
        self.smooth = true;
    }

    pub fn set_data(&mut self, data: ContourData) {
        self.contour_data = data;
        self.initialized = false;
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1200.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    pub fn replay_animation(&mut self, cx: &mut Cx) {
        self.animator.reset();
        self.start_animation(cx);
        self.redraw(cx);
    }

    fn get_value(&self, i: usize, j: usize) -> f64 {
        if i < self.contour_data.values.len() && j < self.contour_data.values[0].len() {
            self.contour_data.values[i][j]
        } else {
            0.0
        }
    }

    fn value_to_color(&self, value: f64) -> Vec4 {
        let t = (value - self.contour_data.min_val)
            / (self.contour_data.max_val - self.contour_data.min_val);
        let t = t.clamp(0.0, 1.0) as f32;

        if self.use_magma {
            // Magma colormap: black -> purple -> orange -> yellow
            if t < 0.25 {
                let s = t / 0.25;
                vec4(s * 0.27, s * 0.0, s * 0.33, 1.0)
            } else if t < 0.5 {
                let s = (t - 0.25) / 0.25;
                vec4(0.27 + s * 0.45, 0.0 + s * 0.15, 0.33 + s * 0.15, 1.0)
            } else if t < 0.75 {
                let s = (t - 0.5) / 0.25;
                vec4(0.72 + s * 0.2, 0.15 + s * 0.35, 0.48 - s * 0.25, 1.0)
            } else {
                let s = (t - 0.75) / 0.25;
                vec4(0.92 + s * 0.08, 0.5 + s * 0.4, 0.23 + s * 0.5, 1.0)
            }
        } else if self.use_grayscale {
            // D3 interpolateGreys - dark at low values, light at high values
            // Inverted so high elevation (peaks) are dark
            let gray = 1.0 - t * 0.85;  // Range from ~0.15 to 1.0
            vec4(gray, gray, gray, 1.0)
        } else {
            // Viridis-like color scheme (original)
            if t < 0.25 {
                let s = t / 0.25;
                vec4(
                    0.267 + s * 0.0,
                    0.004 + s * 0.2,
                    0.329 + s * 0.15,
                    1.0,
                )
            } else if t < 0.5 {
                let s = (t - 0.25) / 0.25;
                vec4(
                    0.267 - s * 0.05,
                    0.204 + s * 0.25,
                    0.479 - s * 0.05,
                    1.0,
                )
            } else if t < 0.75 {
                let s = (t - 0.5) / 0.25;
                vec4(
                    0.217 + s * 0.15,
                    0.454 + s * 0.2,
                    0.429 - s * 0.15,
                    1.0,
                )
            } else {
                let s = (t - 0.75) / 0.25;
                vec4(
                    0.367 + s * 0.55,
                    0.654 + s * 0.15,
                    0.279 - s * 0.1,
                    1.0,
                )
            }
        }
    }

    fn draw_contours(&mut self, cx: &mut Cx2d) {
        let progress = self.animator.get_progress();
        let rect = self.chart_rect;
        let padding = 20.0;

        let draw_width = rect.size.x - padding * 2.0;
        let draw_height = rect.size.y - padding * 2.0;

        let rows = self.contour_data.values.len();
        if rows == 0 { return; }
        let cols = self.contour_data.values[0].len();
        if cols == 0 { return; }

        let cell_width = draw_width / (cols - 1) as f64;
        let cell_height = draw_height / (rows - 1) as f64;

        // Draw filled regions first (behind lines)
        if self.show_fill {
            for i in 0..rows {
                for j in 0..cols {
                    let value = self.get_value(i, j);
                    let animated_value = self.contour_data.min_val
                        + (value - self.contour_data.min_val) * progress;

                    let color = self.value_to_color(animated_value);
                    self.draw_fill.color = color;
                    self.draw_fill.fill_alpha = 0.9;

                    let x = rect.pos.x + padding + j as f64 * cell_width;
                    let y = rect.pos.y + padding + i as f64 * cell_height;

                    // Draw cell as small rectangle
                    let half_w = cell_width * 0.5;
                    let half_h = cell_height * 0.5;

                    self.draw_fill.draw_abs(
                        cx,
                        Rect {
                            pos: dvec2(x - half_w.max(0.0), y - half_h.max(0.0)),
                            size: dvec2(cell_width.max(1.0), cell_height.max(1.0)),
                        },
                    );
                }
            }
        }

        // Draw contour lines using marching squares-like approach
        if self.show_lines {
            for level_idx in 0..self.contour_data.levels.len() {
                let level = self.contour_data.levels[level_idx];
                let animated_level = self.contour_data.min_val
                    + (level - self.contour_data.min_val) * progress;

                // Line color (darker version of fill)
                let color = self.value_to_color(animated_level);
                self.draw_line.color = vec4(
                    color.x * 0.7,
                    color.y * 0.7,
                    color.z * 0.7,
                    0.8,
                );

                // Find and draw contour segments
                for i in 0..rows - 1 {
                    for j in 0..cols - 1 {
                        self.draw_contour_cell(
                            cx,
                            i, j,
                            animated_level,
                            rect.pos.x + padding,
                            rect.pos.y + padding,
                            cell_width,
                            cell_height,
                        );
                    }
                }
            }
        }
    }

    fn draw_contour_cell(
        &mut self,
        cx: &mut Cx2d,
        i: usize,
        j: usize,
        level: f64,
        offset_x: f64,
        offset_y: f64,
        cell_width: f64,
        cell_height: f64,
    ) {
        // Get corner values
        let v00 = self.get_value(i, j);
        let v01 = self.get_value(i, j + 1);
        let v10 = self.get_value(i + 1, j);
        let v11 = self.get_value(i + 1, j + 1);

        // Corner positions
        let x0 = offset_x + j as f64 * cell_width;
        let x1 = offset_x + (j + 1) as f64 * cell_width;
        let y0 = offset_y + i as f64 * cell_height;
        let y1 = offset_y + (i + 1) as f64 * cell_height;

        // Marching squares case index
        let mut case = 0;
        if v00 >= level { case |= 1; }
        if v01 >= level { case |= 2; }
        if v11 >= level { case |= 4; }
        if v10 >= level { case |= 8; }

        // Interpolation helper - linear interpolation for smooth contours
        let interp = |va: f64, vb: f64, pa: f64, pb: f64| -> f64 {
            if (vb - va).abs() < 1e-10 {
                (pa + pb) / 2.0
            } else {
                let t = (level - va) / (vb - va);
                // Clamp to valid range for smoother results
                let t = t.clamp(0.0, 1.0);
                pa + t * (pb - pa)
            }
        };

        // Edge midpoints (interpolated)
        let top = dvec2(interp(v00, v01, x0, x1), y0);
        let bottom = dvec2(interp(v10, v11, x0, x1), y1);
        let left = dvec2(x0, interp(v00, v10, y0, y1));
        let right = dvec2(x1, interp(v01, v11, y0, y1));

        // Line width - thinner for smoother appearance
        let line_width = if self.smooth { 0.8 } else { 1.5 };

        // Draw lines based on case
        match case {
            1 | 14 => self.draw_line.draw_line(cx, top, left, line_width),
            2 | 13 => self.draw_line.draw_line(cx, top, right, line_width),
            3 | 12 => self.draw_line.draw_line(cx, left, right, line_width),
            4 | 11 => self.draw_line.draw_line(cx, right, bottom, line_width),
            5 => {
                // Saddle point - use center value to determine correct topology
                let center = (v00 + v01 + v10 + v11) / 4.0;
                if center >= level {
                    self.draw_line.draw_line(cx, top, left, line_width);
                    self.draw_line.draw_line(cx, right, bottom, line_width);
                } else {
                    self.draw_line.draw_line(cx, top, right, line_width);
                    self.draw_line.draw_line(cx, left, bottom, line_width);
                }
            }
            6 | 9 => self.draw_line.draw_line(cx, top, bottom, line_width),
            7 | 8 => self.draw_line.draw_line(cx, left, bottom, line_width),
            10 => {
                // Saddle point - use center value to determine correct topology
                let center = (v00 + v01 + v10 + v11) / 4.0;
                if center >= level {
                    self.draw_line.draw_line(cx, top, right, line_width);
                    self.draw_line.draw_line(cx, left, bottom, line_width);
                } else {
                    self.draw_line.draw_line(cx, top, left, line_width);
                    self.draw_line.draw_line(cx, right, bottom, line_width);
                }
            }
            _ => {} // 0 and 15: no contour
        }
    }

    pub fn set_smooth(&mut self, smooth: bool) {
        self.smooth = smooth;
    }

    pub fn set_grayscale(&mut self, grayscale: bool) {
        self.use_grayscale = grayscale;
    }

    /// Initialize with original viridis-style data (Gaussian peaks pattern)
    pub fn initialize_original_style(&mut self) {
        let size = 30;
        let mut values = vec![vec![0.0; size]; size];

        for i in 0..size {
            for j in 0..size {
                let x = (i as f64 / size as f64 - 0.5) * 4.0;
                let y = (j as f64 / size as f64 - 0.5) * 4.0;

                // Create interesting contour pattern (Gaussian peaks)
                let peak1 = (-((x - 0.8).powi(2) + (y - 0.8).powi(2)) / 0.5).exp();
                let peak2 = (-((x + 0.8).powi(2) + (y + 0.5).powi(2)) / 0.8).exp() * 0.8;
                let peak3 = (-((x - 0.3).powi(2) + (y + 0.9).powi(2)) / 0.3).exp() * 0.6;
                let valley = -(-((x).powi(2) + (y).powi(2)) / 2.0).exp() * 0.3;

                // Ripple effect
                let dist = (x * x + y * y).sqrt();
                let ripple = (dist * 3.0).sin() * 0.15 * (-dist * 0.5).exp();

                values[i][j] = peak1 + peak2 + peak3 + valley + ripple;
            }
        }

        // Find min/max
        let mut min_val = f64::MAX;
        let mut max_val = f64::MIN;
        for row in &values {
            for &v in row {
                min_val = min_val.min(v);
                max_val = max_val.max(v);
            }
        }

        // Generate contour levels
        let num_levels = 12;
        let levels: Vec<f64> = (0..=num_levels)
            .map(|i| min_val + (max_val - min_val) * (i as f64 / num_levels as f64))
            .collect();

        self.contour_data = ContourData {
            values,
            levels,
            min_val,
            max_val,
        };

        // Use viridis colors (not grayscale)
        self.use_grayscale = false;
        self.smooth = false;
        self.initialized = true;
    }

    /// Initialize with GeoTIFF-style data (global temperature pattern with Magma colors)
    pub fn initialize_geotiff_style(&mut self) {
        // Simulate global surface temperature data
        let rows = 50;
        let cols = 100;  // 2:1 aspect ratio like equirectangular projection
        let mut values = vec![vec![0.0; cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                let lat = (i as f64 / rows as f64) * 180.0 - 90.0;  // -90 to 90
                let lon = (j as f64 / cols as f64) * 360.0 - 180.0; // -180 to 180

                // Base temperature: warmer at equator, cooler at poles
                let lat_effect = (lat.to_radians().cos()) * 30.0;

                // Land mass effects (simplified continents)
                let continent1 = if lon > -30.0 && lon < 60.0 && lat > -35.0 && lat < 70.0 {
                    // Africa/Europe
                    5.0 * (-((lon - 20.0).powi(2) / 1000.0 + (lat - 10.0).powi(2) / 800.0)).exp()
                } else { 0.0 };

                let continent2 = if lon > 60.0 && lon < 150.0 && lat > 0.0 && lat < 70.0 {
                    // Asia
                    4.0 * (-((lon - 100.0).powi(2) / 1500.0 + (lat - 40.0).powi(2) / 600.0)).exp()
                } else { 0.0 };

                let continent3 = if lon > -130.0 && lon < -60.0 && lat > 10.0 && lat < 70.0 {
                    // North America
                    3.0 * (-((lon + 100.0).powi(2) / 1200.0 + (lat - 40.0).powi(2) / 700.0)).exp()
                } else { 0.0 };

                // Ocean currents (Gulf Stream, etc.)
                let ocean_current = 2.0 * ((lon * 0.02 + lat * 0.03).sin());

                // Base temperature around 15°C
                values[i][j] = 15.0 + lat_effect + continent1 + continent2 + continent3 + ocean_current;
            }
        }

        // Find min/max
        let mut min_val = f64::MAX;
        let mut max_val = f64::MIN;
        for row in &values {
            for &v in row {
                min_val = min_val.min(v);
                max_val = max_val.max(v);
            }
        }

        // Generate contour levels
        let num_levels = 15;
        let levels: Vec<f64> = (0..=num_levels)
            .map(|i| min_val + (max_val - min_val) * (i as f64 / num_levels as f64))
            .collect();

        self.contour_data = ContourData {
            values,
            levels,
            min_val,
            max_val,
        };

        // Use Magma-like color scheme
        self.use_grayscale = false;
        self.use_magma = true;
        self.smooth = true;
        self.initialized = true;
    }

    pub fn set_use_magma(&mut self, use_magma: bool) {
        self.use_magma = use_magma;
    }
}

impl ContourChartWidgetRef {
    pub fn initialize_original_style(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.initialize_original_style();
            // Start animation
            let time = cx.seconds_since_app_start();
            inner.animator = ChartAnimator::new(1200.0)
                .with_easing(EasingType::EaseOutCubic);
            inner.animator.start(time);
            inner.redraw(cx);
            cx.new_next_frame();
        }
    }

    pub fn initialize_geotiff_style(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.initialize_geotiff_style();
            // Start animation
            let time = cx.seconds_since_app_start();
            inner.animator = ChartAnimator::new(1200.0)
                .with_easing(EasingType::EaseOutCubic);
            inner.animator.start(time);
            inner.redraw(cx);
            cx.new_next_frame();
        }
    }
}
