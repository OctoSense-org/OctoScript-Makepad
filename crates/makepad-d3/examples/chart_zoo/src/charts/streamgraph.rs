//! Streamgraph - stacked area chart with centered baseline
//!
//! A variation of stacked area chart where the baseline is shifted to
//! center the stream, creating a flowing river-like visualization.

use makepad_widgets::*;
use super::draw_primitives::{DrawTriangle, DrawChartLine};
use super::animation::ChartAnimator;

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawTriangle;
    use super::draw_primitives::DrawChartLine;

    pub Streamgraph = {{Streamgraph}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Clone, Debug)]
pub struct StreamSeries {
    pub name: String,
    pub values: Vec<f64>,
    pub color: Option<Vec4>,
}

impl StreamSeries {
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
pub struct Streamgraph {
    #[redraw]
    #[live]
    draw_triangle: DrawTriangle,

    #[redraw]
    #[live]
    draw_line: DrawChartLine,

    #[walk]
    walk: Walk,

    #[rust]
    series: Vec<StreamSeries>,

    #[rust]
    labels: Vec<String>,

    #[rust]
    smooth: bool,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,
}

impl Streamgraph {
    pub fn set_data(&mut self, series: Vec<StreamSeries>, labels: Vec<String>) {
        self.series = series;
        self.labels = labels;
        self.initialized = false;
    }

    pub fn set_smooth(&mut self, smooth: bool) {
        self.smooth = smooth;
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        if self.series.is_empty() {
            return;
        }

        let padding = 40.0;
        let chart_x = rect.pos.x + padding;
        let chart_y = rect.pos.y + padding;
        let chart_w = rect.size.x - padding * 2.0;
        let chart_h = rect.size.y - padding * 2.0;

        // Calculate stacked values and find max
        let n_points = self.series.iter().map(|s| s.values.len()).max().unwrap_or(0);
        if n_points == 0 {
            return;
        }

        // Calculate totals for each x position
        let mut totals: Vec<f64> = vec![0.0; n_points];
        for series in &self.series {
            for (i, &val) in series.values.iter().enumerate() {
                if i < totals.len() {
                    totals[i] += val;
                }
            }
        }

        let max_total = totals.iter().cloned().fold(0.0_f64, f64::max);
        if max_total == 0.0 {
            return;
        }

        // Color palette
        let colors = [
            vec4(0.4, 0.76, 0.65, 0.85),  // Teal
            vec4(0.99, 0.55, 0.38, 0.85), // Coral
            vec4(0.55, 0.63, 0.80, 0.85), // Lavender
            vec4(0.91, 0.84, 0.42, 0.85), // Gold
            vec4(0.65, 0.85, 0.33, 0.85), // Lime
            vec4(0.90, 0.45, 0.77, 0.85), // Pink
            vec4(0.45, 0.85, 0.90, 0.85), // Cyan
            vec4(0.85, 0.65, 0.45, 0.85), // Orange
        ];

        // Calculate baseline offsets for centering (wiggle algorithm simplified)
        let mut baselines: Vec<f64> = vec![0.0; n_points];
        for i in 0..n_points {
            baselines[i] = (max_total - totals[i]) / 2.0;
        }

        // Get animation progress
        let progress = self.animator.get_progress();

        // Collect series data to avoid borrow conflict
        let series_data: Vec<_> = self.series.iter().enumerate().map(|(idx, s)| {
            (idx, s.values.clone(), s.color)
        }).collect();

        // Draw streams from bottom to top
        let mut cumulative: Vec<f64> = baselines.clone();

        for (series_idx, values, custom_color) in series_data {
            let color = custom_color.unwrap_or(colors[series_idx % colors.len()]);

            // Build points for this stream layer
            let mut bottom_points: Vec<DVec2> = Vec::new();
            let mut top_points: Vec<DVec2> = Vec::new();

            for i in 0..n_points {
                let x = chart_x + (i as f64 / (n_points - 1).max(1) as f64) * chart_w;
                let val = if i < values.len() { values[i] } else { 0.0 };

                // Apply animation - grow from center
                let animated_val = val * progress as f64;
                let animated_cum = baselines[i] + (cumulative[i] - baselines[i]) * progress as f64;

                let bottom_y = chart_y + chart_h - (animated_cum / max_total) * chart_h;
                let top_y = chart_y + chart_h - ((animated_cum + animated_val) / max_total) * chart_h;

                bottom_points.push(DVec2 { x, y: bottom_y });
                top_points.push(DVec2 { x, y: top_y });

                cumulative[i] += val;
            }

            // Draw filled area
            self.draw_stream_area(cx, &bottom_points, &top_points, color);
        }

        // Draw x-axis labels
        if !self.labels.is_empty() {
            let label_color = vec4(0.5, 0.5, 0.5, 1.0);
            let label_step = (n_points / 6).max(1);

            for (i, _label) in self.labels.iter().enumerate() {
                if i % label_step == 0 {
                    let x = chart_x + (i as f64 / (n_points - 1).max(1) as f64) * chart_w;
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
    }

    fn draw_stream_area(&mut self, cx: &mut Cx2d, bottom: &[DVec2], top: &[DVec2], color: Vec4) {
        if bottom.len() < 2 || top.len() < 2 {
            return;
        }

        self.draw_triangle.color = color;

        // Draw as triangles connecting bottom and top edges
        for i in 0..bottom.len() - 1 {
            let b1 = bottom[i];
            let b2 = bottom[i + 1];
            let t1 = top[i];
            let t2 = top[i + 1];

            // Two triangles per segment
            self.draw_triangle.draw_triangle(cx, b1, b2, t1);
            self.draw_triangle.draw_triangle(cx, t1, b2, t2);
        }
    }
}

impl Streamgraph {
    fn initialize_demo_data(&mut self) {
        // Multi-series time data
        self.series = vec![
            StreamSeries::new("Series A", vec![10.0, 15.0, 20.0, 25.0, 22.0, 18.0, 20.0, 25.0, 30.0, 28.0]),
            StreamSeries::new("Series B", vec![8.0, 12.0, 18.0, 15.0, 20.0, 25.0, 22.0, 18.0, 15.0, 20.0]),
            StreamSeries::new("Series C", vec![15.0, 10.0, 8.0, 12.0, 15.0, 18.0, 22.0, 28.0, 25.0, 22.0]),
            StreamSeries::new("Series D", vec![5.0, 8.0, 12.0, 10.0, 8.0, 12.0, 15.0, 18.0, 20.0, 18.0]),
        ];
        self.labels = (1..=10).map(|i| format!("T{}", i)).collect();
    }
}

impl Widget for Streamgraph {
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
