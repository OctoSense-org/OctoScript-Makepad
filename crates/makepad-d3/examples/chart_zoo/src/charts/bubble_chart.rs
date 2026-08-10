//! Bubble chart - sized circles by data value
//!
//! Displays three dimensions of data: x, y position and size (radius).
//! Often used for comparing multiple variables simultaneously.

use makepad_widgets::*;
use super::draw_primitives::{DrawPoint, DrawChartLine};
use super::animation::{ChartAnimator, get_color};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawPoint;
    use super::draw_primitives::DrawChartLine;

    pub BubbleChart = {{BubbleChart}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Clone, Debug)]
pub struct BubbleData {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub label: String,
    pub category: String,
    pub color: Option<Vec4>,
}

impl BubbleData {
    pub fn new(x: f64, y: f64, size: f64) -> Self {
        Self {
            x,
            y,
            size,
            label: String::new(),
            category: String::new(),
            color: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
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
pub struct BubbleChart {
    #[redraw]
    #[live]
    draw_point: DrawPoint,

    #[redraw]
    #[live]
    draw_line: DrawChartLine,

    #[walk]
    walk: Walk,

    #[rust]
    bubbles: Vec<BubbleData>,

    #[rust]
    min_radius: f64,

    #[rust]
    max_radius: f64,

    #[rust]
    show_grid: bool,

    #[rust]
    categories: Vec<String>,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,
}

impl BubbleChart {
    pub fn set_data(&mut self, bubbles: Vec<BubbleData>) {
        // Extract unique categories
        let mut categories: Vec<String> = bubbles
            .iter()
            .filter(|b| !b.category.is_empty())
            .map(|b| b.category.clone())
            .collect();
        categories.sort();
        categories.dedup();
        self.categories = categories;

        self.bubbles = bubbles;
        self.initialized = false;
    }

    pub fn set_radius_range(&mut self, min: f64, max: f64) {
        self.min_radius = min;
        self.max_radius = max;
    }

    pub fn set_show_grid(&mut self, show: bool) {
        self.show_grid = show;
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        if self.bubbles.is_empty() {
            return;
        }

        let padding = 60.0;
        let chart_x = rect.pos.x + padding;
        let chart_y = rect.pos.y + padding;
        let chart_w = rect.size.x - padding * 2.0;
        let chart_h = rect.size.y - padding * 2.0;

        // Find data ranges
        let x_min = self.bubbles.iter().map(|b| b.x).fold(f64::INFINITY, f64::min);
        let x_max = self.bubbles.iter().map(|b| b.x).fold(f64::NEG_INFINITY, f64::max);
        let y_min = self.bubbles.iter().map(|b| b.y).fold(f64::INFINITY, f64::min);
        let y_max = self.bubbles.iter().map(|b| b.y).fold(f64::NEG_INFINITY, f64::max);
        let size_min = self.bubbles.iter().map(|b| b.size).fold(f64::INFINITY, f64::min);
        let size_max = self.bubbles.iter().map(|b| b.size).fold(f64::NEG_INFINITY, f64::max);

        let x_range = (x_max - x_min).max(1.0);
        let y_range = (y_max - y_min).max(1.0);
        let size_range = (size_max - size_min).max(1.0);

        // Draw grid
        if self.show_grid {
            let grid_color = vec4(0.25, 0.25, 0.28, 1.0);
            let grid_count = 5;

            for i in 0..=grid_count {
                let t = i as f64 / grid_count as f64;

                // Horizontal grid lines
                let gy = chart_y + t * chart_h;
                self.draw_line.color = grid_color;
                self.draw_line.draw_line(cx, dvec2(chart_x, gy), dvec2(chart_x + chart_w, gy), 0.5);

                // Vertical grid lines
                let gx = chart_x + t * chart_w;
                self.draw_line.color = grid_color;
                self.draw_line.draw_line(cx, dvec2(gx, chart_y), dvec2(gx, chart_y + chart_h), 0.5);
            }
        }

        // Draw axes
        let axis_color = vec4(0.5, 0.5, 0.5, 1.0);
        self.draw_line.color = axis_color;
        // X axis
        self.draw_line.draw_line(
            cx,
            dvec2(chart_x, chart_y + chart_h),
            dvec2(chart_x + chart_w, chart_y + chart_h),
            1.5,
        );
        // Y axis
        self.draw_line.draw_line(
            cx,
            dvec2(chart_x, chart_y),
            dvec2(chart_x, chart_y + chart_h),
            1.5,
        );

        // Color palette for categories
        let colors = [
            vec4(0.4, 0.76, 0.65, 0.7),  // Teal
            vec4(0.99, 0.55, 0.38, 0.7), // Coral
            vec4(0.55, 0.63, 0.80, 0.7), // Lavender
            vec4(0.91, 0.84, 0.42, 0.7), // Gold
            vec4(0.65, 0.85, 0.33, 0.7), // Lime
            vec4(0.90, 0.45, 0.77, 0.7), // Pink
        ];

        // Sort bubbles by size (largest first) for proper overlap
        let mut sorted_indices: Vec<usize> = (0..self.bubbles.len()).collect();
        sorted_indices.sort_by(|&a, &b| {
            self.bubbles[b].size.partial_cmp(&self.bubbles[a].size).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Get animation progress
        let progress = self.animator.get_progress();

        // Draw bubbles
        for (draw_idx, &idx) in sorted_indices.iter().enumerate() {
            let bubble = &self.bubbles[idx];
            let nx = (bubble.x - x_min) / x_range;
            let ny = 1.0 - (bubble.y - y_min) / y_range;
            let ns = (bubble.size - size_min) / size_range;

            let px = chart_x + nx * chart_w;
            let py = chart_y + ny * chart_h;
            let radius = self.min_radius + ns * (self.max_radius - self.min_radius);

            let color = bubble.color.unwrap_or_else(|| {
                if !bubble.category.is_empty() {
                    let cat_idx = self.categories.iter().position(|c| c == &bubble.category).unwrap_or(0);
                    colors[cat_idx % colors.len()]
                } else {
                    get_color(idx)
                }
            });

            // Animate appearance
            let point_progress = (progress as f32 * self.bubbles.len() as f32 - draw_idx as f32).clamp(0.0, 1.0);
            let animated_radius = radius * point_progress as f64;

            self.draw_point.color = color;
            self.draw_point.draw_point(cx, dvec2(px, py), animated_radius * 2.0);
        }
    }
}

impl BubbleChart {
    fn initialize_demo_data(&mut self) {
        self.bubbles = vec![
            BubbleData::new(20.0, 30.0, 15.0).with_category("Tech"),
            BubbleData::new(40.0, 50.0, 25.0).with_category("Tech"),
            BubbleData::new(60.0, 40.0, 20.0).with_category("Finance"),
            BubbleData::new(80.0, 70.0, 35.0).with_category("Finance"),
            BubbleData::new(30.0, 80.0, 18.0).with_category("Health"),
            BubbleData::new(70.0, 20.0, 28.0).with_category("Health"),
            BubbleData::new(50.0, 60.0, 22.0).with_category("Tech"),
            BubbleData::new(25.0, 55.0, 12.0).with_category("Finance"),
            BubbleData::new(85.0, 45.0, 30.0).with_category("Health"),
            BubbleData::new(45.0, 35.0, 16.0).with_category("Tech"),
        ];
        self.min_radius = 10.0;
        self.max_radius = 40.0;
        self.show_grid = true;
    }
}

impl Widget for BubbleChart {
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
