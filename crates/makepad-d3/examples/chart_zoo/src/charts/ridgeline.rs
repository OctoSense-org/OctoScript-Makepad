//! Ridgeline plot (Joy plot) - overlapping area charts
//!
//! Displays multiple distributions as overlapping area charts,
//! creating a mountain ridge-like visualization effect.

use makepad_widgets::*;
use super::draw_primitives::{DrawTriangle, DrawChartLine, DrawBar};
use super::animation::ChartAnimator;

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawTriangle;
    use super::draw_primitives::DrawChartLine;
    use super::draw_primitives::DrawBar;

    pub RidgelinePlot = {{RidgelinePlot}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Clone, Debug)]
pub struct RidgeSeries {
    pub name: String,
    pub values: Vec<f64>,
    pub color: Option<Vec4>,
}

impl RidgeSeries {
    pub fn new(name: impl Into<String>, values: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            values,
            color: None,
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct RidgelinePlot {
    #[redraw]
    #[live]
    draw_triangle: DrawTriangle,

    #[redraw]
    #[live]
    draw_line: DrawChartLine,

    #[redraw]
    #[live]
    draw_bar: DrawBar,

    #[walk]
    walk: Walk,

    #[rust]
    series: Vec<RidgeSeries>,

    #[rust]
    overlap: f64,

    #[rust]
    fill_opacity: f64,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,
}

impl RidgelinePlot {
    pub fn set_data(&mut self, series: Vec<RidgeSeries>) {
        self.series = series;
        self.initialized = false;
    }

    pub fn set_overlap(&mut self, overlap: f64) {
        self.overlap = overlap.clamp(0.0, 1.0);
    }

    pub fn set_fill_opacity(&mut self, opacity: f64) {
        self.fill_opacity = opacity.clamp(0.0, 1.0);
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        if self.series.is_empty() {
            return;
        }

        let padding = 60.0;
        let label_width = 80.0;
        let chart_x = rect.pos.x + padding + label_width;
        let chart_y = rect.pos.y + padding;
        let chart_w = rect.size.x - padding * 2.0 - label_width;
        let chart_h = rect.size.y - padding * 2.0;

        let n_series = self.series.len();
        let row_height = chart_h / n_series as f64;
        let effective_height = row_height * (1.0 + self.overlap);

        // Find global max for consistent scaling
        let global_max = self.series
            .iter()
            .flat_map(|s| s.values.iter())
            .cloned()
            .fold(0.0_f64, f64::max);

        if global_max == 0.0 {
            return;
        }

        // Color palette with varying hues
        let base_colors = [
            vec4(0.2, 0.5, 0.8, 1.0),   // Blue
            vec4(0.3, 0.6, 0.75, 1.0),  // Teal-blue
            vec4(0.4, 0.65, 0.7, 1.0),  // Cyan
            vec4(0.5, 0.7, 0.6, 1.0),   // Teal
            vec4(0.55, 0.75, 0.5, 1.0), // Green-teal
            vec4(0.6, 0.7, 0.45, 1.0),  // Yellow-green
            vec4(0.7, 0.65, 0.4, 1.0),  // Orange-yellow
            vec4(0.8, 0.55, 0.35, 1.0), // Orange
        ];

        // Get animation progress
        let progress = self.animator.get_progress();

        // Collect series data - draw from back (index 0) to front (last index)
        // so that later rows overlap earlier ones (perspective effect)
        let series_data: Vec<_> = self.series.iter().enumerate().map(|(idx, series)| {
            let base_y = chart_y + (idx as f64) * row_height;
            let n_points = series.values.len();
            let values = series.values.clone();
            let color = series.color.unwrap_or_else(|| {
                // Gradient from blue (back) to warmer colors (front)
                let t = idx as f64 / (n_series - 1).max(1) as f64;
                let ci = (t * (base_colors.len() - 1) as f64) as usize;
                base_colors[ci.min(base_colors.len() - 1)]
            });
            (idx, base_y, n_points, values, color)
        }).collect();

        // Draw series from back (top) to front (bottom) for perspective overlap
        for (idx, base_y, n_points, values, color) in series_data {
            if n_points == 0 {
                continue;
            }

            let fill_color = vec4(color.x, color.y, color.z, self.fill_opacity as f32);
            let stroke_color = color;

            // Build area points
            let mut area_points: Vec<DVec2> = Vec::new();
            let mut line_points: Vec<DVec2> = Vec::new();

            for (i, &val) in values.iter().enumerate() {
                let x = chart_x + (i as f64 / (n_points - 1).max(1) as f64) * chart_w;
                let height = (val / global_max) * effective_height * progress as f64;
                let y = base_y + row_height - height;

                area_points.push(DVec2 { x, y });
                line_points.push(DVec2 { x, y });
            }

            // Close area path
            let baseline_y = base_y + row_height;
            area_points.push(DVec2 { x: chart_x + chart_w, y: baseline_y });
            area_points.push(DVec2 { x: chart_x, y: baseline_y });

            // Draw filled area
            self.draw_area(cx, &area_points, fill_color);

            // Draw line on top
            self.draw_polyline(cx, &line_points, stroke_color, 1.5);

            // Draw series label marker
            let label_x = rect.pos.x + padding;
            let label_y = base_y + row_height / 2.0;
            self.draw_bar.color = color;
            self.draw_bar.draw_bar(cx, Rect {
                pos: dvec2(label_x, label_y - 3.0),
                size: dvec2(6.0, 6.0),
            });
        }
    }

    fn draw_area(&mut self, cx: &mut Cx2d, points: &[DVec2], color: Vec4) {
        if points.len() < 3 {
            return;
        }

        self.draw_triangle.color = color;

        // Fan triangulation from first point
        let p0 = points[0];
        for i in 1..points.len() - 1 {
            self.draw_triangle.draw_triangle(cx, p0, points[i], points[i + 1]);
        }
    }

    fn draw_polyline(&mut self, cx: &mut Cx2d, points: &[DVec2], color: Vec4, width: f64) {
        self.draw_line.color = color;
        for i in 0..points.len().saturating_sub(1) {
            self.draw_line.draw_line(cx, points[i], points[i + 1], width);
        }
    }
}

impl RidgelinePlot {
    fn initialize_demo_data(&mut self) {
        // Distribution-like data for multiple series - more series for better perspective effect
        self.series = vec![
            RidgeSeries::new("2018", vec![1.0, 3.0, 8.0, 15.0, 20.0, 22.0, 18.0, 10.0, 4.0, 1.0]),
            RidgeSeries::new("2019", vec![2.0, 6.0, 14.0, 22.0, 28.0, 25.0, 18.0, 8.0, 3.0, 1.0]),
            RidgeSeries::new("2020", vec![1.0, 4.0, 10.0, 18.0, 24.0, 30.0, 26.0, 15.0, 6.0, 2.0]),
            RidgeSeries::new("2021", vec![3.0, 8.0, 16.0, 24.0, 20.0, 16.0, 22.0, 18.0, 8.0, 2.0]),
            RidgeSeries::new("2022", vec![2.0, 5.0, 12.0, 20.0, 28.0, 32.0, 24.0, 12.0, 5.0, 1.0]),
            RidgeSeries::new("2023", vec![1.0, 4.0, 10.0, 16.0, 22.0, 26.0, 28.0, 20.0, 10.0, 3.0]),
            RidgeSeries::new("2024", vec![2.0, 6.0, 14.0, 22.0, 30.0, 28.0, 20.0, 10.0, 4.0, 1.0]),
        ];
        self.overlap = 0.6;  // Higher overlap for perspective effect
        self.fill_opacity = 0.85;
    }
}

impl Widget for RidgelinePlot {
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
