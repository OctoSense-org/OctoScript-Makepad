//! Horizon chart - layered compact time series visualization
//!
//! Displays time series data in a compact form by folding bands of
//! different values on top of each other with varying color intensity.

use makepad_widgets::*;
use super::draw_primitives::{DrawTriangle, DrawChartLine, DrawBar};
use super::animation::ChartAnimator;

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawTriangle;
    use super::draw_primitives::DrawChartLine;
    use super::draw_primitives::DrawBar;

    pub HorizonChart = {{HorizonChart}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Clone, Debug)]
pub struct HorizonSeries {
    pub name: String,
    pub values: Vec<f64>,
}

impl HorizonSeries {
    pub fn new(name: impl Into<String>, values: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            values,
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct HorizonChart {
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
    series: Vec<HorizonSeries>,

    #[rust]
    bands: usize,

    #[rust]
    positive_color: Vec4,

    #[rust]
    negative_color: Vec4,

    #[rust]
    labels: Vec<String>,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,
}

impl HorizonChart {
    pub fn set_data(&mut self, series: Vec<HorizonSeries>) {
        self.series = series;
        self.initialized = false;
    }

    pub fn set_labels(&mut self, labels: Vec<String>) {
        self.labels = labels;
    }

    pub fn set_bands(&mut self, bands: usize) {
        self.bands = bands.max(1).min(6);
    }

    pub fn set_colors(&mut self, positive: Vec4, negative: Vec4) {
        self.positive_color = positive;
        self.negative_color = negative;
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        if self.series.is_empty() {
            return;
        }

        let padding = 40.0;
        let label_width = 100.0;
        let chart_x = rect.pos.x + padding + label_width;
        let chart_y = rect.pos.y + padding;
        let chart_w = rect.size.x - padding * 2.0 - label_width;
        let chart_h = rect.size.y - padding * 2.0;

        let n_series = self.series.len();
        let row_height = chart_h / n_series as f64;

        // Find global max absolute value
        let global_max = self.series
            .iter()
            .flat_map(|s| s.values.iter())
            .map(|v| v.abs())
            .fold(0.0_f64, f64::max);

        if global_max == 0.0 {
            return;
        }

        let band_height = row_height / self.bands as f64;
        let bands = self.bands;
        let positive_color = self.positive_color;
        let negative_color = self.negative_color;

        // Get animation progress
        let progress = self.animator.get_progress();

        // Clone series data to avoid borrow conflicts
        let series_data: Vec<_> = self.series.iter().enumerate().map(|(idx, s)| {
            (idx, s.values.clone())
        }).collect();

        // Draw each series
        for (series_idx, values) in series_data {
            let base_y = chart_y + (series_idx as f64) * row_height;
            let n_points = values.len();

            if n_points == 0 {
                continue;
            }

            // Draw bands for this series
            for band in 0..bands {
                let band_min = (band as f64 / bands as f64) * global_max;
                let band_max = ((band + 1) as f64 / bands as f64) * global_max;

                // Calculate opacity for this band (darker for higher bands)
                let opacity = 0.3 + (band as f64 / bands as f64) * 0.7;

                for i in 0..n_points.saturating_sub(1) {
                    let val1 = values[i] * progress as f64;
                    let val2 = values[i + 1] * progress as f64;

                    let x1 = chart_x + (i as f64 / (n_points - 1).max(1) as f64) * chart_w;
                    let x2 = chart_x + ((i + 1) as f64 / (n_points - 1).max(1) as f64) * chart_w;

                    // Process positive values
                    if val1 > band_min || val2 > band_min {
                        let h1 = ((val1.max(0.0) - band_min).min(band_max - band_min) / global_max * bands as f64) * band_height;
                        let h2 = ((val2.max(0.0) - band_min).min(band_max - band_min) / global_max * bands as f64) * band_height;

                        if h1 > 0.0 || h2 > 0.0 {
                            let color = vec4(
                                positive_color.x,
                                positive_color.y,
                                positive_color.z,
                                opacity as f32,
                            );

                            let y_base = base_y + row_height;
                            self.draw_area_segment(cx, x1, x2, y_base, h1.max(0.0), h2.max(0.0), color);
                        }
                    }

                    // Process negative values
                    if val1 < -band_min || val2 < -band_min {
                        let h1 = (((-val1).max(0.0) - band_min).min(band_max - band_min) / global_max * bands as f64) * band_height;
                        let h2 = (((-val2).max(0.0) - band_min).min(band_max - band_min) / global_max * bands as f64) * band_height;

                        if h1 > 0.0 || h2 > 0.0 {
                            let color = vec4(
                                negative_color.x,
                                negative_color.y,
                                negative_color.z,
                                opacity as f32,
                            );

                            let y_base = base_y + row_height;
                            self.draw_area_segment(cx, x1, x2, y_base, h1.max(0.0), h2.max(0.0), color);
                        }
                    }
                }
            }

            // Draw series separator line
            let sep_color = vec4(0.3, 0.3, 0.3, 1.0);
            self.draw_line.color = sep_color;
            self.draw_line.draw_line(
                cx,
                dvec2(chart_x, base_y + row_height),
                dvec2(chart_x + chart_w, base_y + row_height),
                0.5,
            );

            // Draw label marker
            let label_color = vec4(0.6, 0.6, 0.6, 1.0);
            self.draw_bar.color = label_color;
            self.draw_bar.draw_bar(cx, Rect {
                pos: dvec2(rect.pos.x + padding, base_y + row_height / 2.0 - 3.0),
                size: dvec2(60.0, 6.0),
            });
        }

        // Draw x-axis time labels
        let n_labels = self.labels.len();
        let label_step = (n_labels / 5).max(1);
        let label_color = vec4(0.5, 0.5, 0.5, 1.0);

        for i in 0..n_labels {
            if i % label_step == 0 {
                let x = chart_x + (i as f64 / (n_labels - 1).max(1) as f64) * chart_w;
                self.draw_line.color = label_color;
                self.draw_line.draw_line(
                    cx,
                    dvec2(x, chart_y + chart_h),
                    dvec2(x, chart_y + chart_h + 5.0),
                    1.0,
                );
            }
        }
    }

    fn draw_area_segment(&mut self, cx: &mut Cx2d, x1: f64, x2: f64, y_base: f64, h1: f64, h2: f64, color: Vec4) {
        let p1 = dvec2(x1, y_base);
        let p2 = dvec2(x2, y_base);
        let p3 = dvec2(x2, y_base - h2);
        let p4 = dvec2(x1, y_base - h1);

        self.draw_triangle.color = color;
        self.draw_triangle.draw_triangle(cx, p1, p2, p3);
        self.draw_triangle.draw_triangle(cx, p1, p3, p4);
    }
}

impl HorizonChart {
    fn initialize_demo_data(&mut self) {
        // Time series with positive and negative values
        self.series = vec![
            HorizonSeries::new("Stock A", vec![10.0, 15.0, 8.0, -5.0, -10.0, 5.0, 20.0, 15.0, 10.0, 5.0, -8.0, 12.0]),
            HorizonSeries::new("Stock B", vec![-5.0, 8.0, 15.0, 20.0, 12.0, -3.0, -8.0, 5.0, 18.0, 22.0, 15.0, 8.0]),
            HorizonSeries::new("Stock C", vec![8.0, -2.0, -10.0, -15.0, -8.0, 5.0, 12.0, 8.0, -5.0, 10.0, 18.0, 15.0]),
        ];
        self.labels = vec!["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]
            .into_iter().map(String::from).collect();
        self.bands = 3;
        self.positive_color = vec4(0.2, 0.6, 0.9, 1.0);
        self.negative_color = vec4(0.9, 0.3, 0.3, 1.0);
    }
}

impl Widget for HorizonChart {
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
