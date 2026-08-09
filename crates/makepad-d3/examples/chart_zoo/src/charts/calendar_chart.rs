//! Calendar heatmap chart - GitHub-style contribution calendar
//!
//! Displays daily data values in a calendar grid, with color intensity
//! representing the value. Great for showing activity patterns over time.

use makepad_widgets::*;
use super::draw_primitives::DrawBar;
use super::animation::ChartAnimator;

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawBar;

    pub CalendarChart = {{CalendarChart}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Clone, Debug)]
pub struct CalendarEntry {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub value: f64,
}

impl CalendarEntry {
    pub fn new(year: i32, month: u32, day: u32, value: f64) -> Self {
        Self { year, month, day, value }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct CalendarChart {
    #[redraw]
    #[live]
    draw_cell: DrawBar,

    #[walk]
    walk: Walk,

    #[rust]
    entries: Vec<CalendarEntry>,

    #[rust]
    cell_size: f64,

    #[rust]
    cell_gap: f64,

    #[rust]
    start_year: i32,

    #[rust]
    end_year: i32,

    #[rust]
    color_empty: Vec4,

    #[rust]
    color_low: Vec4,

    #[rust]
    color_high: Vec4,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,
}

impl CalendarChart {
    pub fn set_data(&mut self, entries: Vec<CalendarEntry>) {
        if !entries.is_empty() {
            self.start_year = entries.iter().map(|e| e.year).min().unwrap_or(2024);
            self.end_year = entries.iter().map(|e| e.year).max().unwrap_or(2024);
        }
        self.entries = entries;
        self.initialized = false;
    }

    pub fn set_year_range(&mut self, start: i32, end: i32) {
        self.start_year = start;
        self.end_year = end;
    }

    pub fn set_cell_size(&mut self, size: f64) {
        self.cell_size = size;
    }

    pub fn set_colors(&mut self, empty: Vec4, low: Vec4, high: Vec4) {
        self.color_empty = empty;
        self.color_low = low;
        self.color_high = high;
    }

    fn get_value(&self, year: i32, month: u32, day: u32) -> Option<f64> {
        self.entries
            .iter()
            .find(|e| e.year == year && e.month == month && e.day == day)
            .map(|e| e.value)
    }

    fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    }

    fn day_of_week(year: i32, month: u32, day: u32) -> u32 {
        // Zeller's congruence (simplified)
        let m = if month < 3 { month as i32 + 12 } else { month as i32 };
        let y = if month < 3 { year - 1 } else { year };

        let k = y % 100;
        let j = y / 100;

        let h = (day as i32 + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
        ((h + 6) % 7) as u32 // 0 = Sunday, 6 = Saturday
    }

    fn draw_chart(&mut self, cx: &mut Cx2d, rect: Rect) {
        let padding = 40.0;
        let label_width = 30.0;
        let month_label_height = 20.0;

        // Find value range for color mapping
        let max_value = self.entries
            .iter()
            .map(|e| e.value)
            .fold(0.0_f64, f64::max);

        let start_x = rect.pos.x + padding + label_width;
        let start_y = rect.pos.y + padding + month_label_height;

        // Get animation progress
        let progress = self.animator.get_progress();

        // Draw day labels on left
        let label_color = vec4(0.5, 0.5, 0.5, 1.0);
        for i in 0..7 {
            let y = start_y + (i as f64) * (self.cell_size + self.cell_gap);
            self.draw_cell.color = label_color;
            self.draw_cell.draw_bar(cx, Rect {
                pos: dvec2(rect.pos.x + padding, y),
                size: dvec2(3.0, 3.0),
            });
        }

        // Draw calendar grid for each year
        let mut year_offset = 0.0;
        let mut cell_index = 0;

        for year in self.start_year..=self.end_year {
            let year_start_x = start_x + year_offset;
            let mut week = 0;

            for month in 1..=12u32 {
                let days = Self::days_in_month(year, month);

                for day in 1..=days {
                    let dow = Self::day_of_week(year, month, day);

                    let value = self.get_value(year, month, day);
                    let color = if let Some(v) = value {
                        let t = if max_value > 0.0 { v / max_value } else { 0.0 };
                        self.interpolate_color(t)
                    } else {
                        self.color_empty
                    };

                    let x = year_start_x + (week as f64) * (self.cell_size + self.cell_gap);
                    let y = start_y + (dow as f64) * (self.cell_size + self.cell_gap);

                    // Animate cell appearance
                    let cell_progress = (progress as f32 * 2.0 - cell_index as f32 * 0.002).clamp(0.0, 1.0);
                    let animated_size = self.cell_size * cell_progress as f64;

                    if animated_size > 0.5 {
                        self.draw_cell.color = color;
                        self.draw_cell.draw_bar(cx, Rect {
                            pos: dvec2(x, y),
                            size: dvec2(animated_size, animated_size),
                        });
                    }

                    cell_index += 1;

                    // Move to next week on Saturday
                    if dow == 6 {
                        week += 1;
                    }
                }
            }

            // Calculate year width for next year offset
            let jan1_dow = Self::day_of_week(year, 1, 1);
            let dec31_dow = Self::day_of_week(year, 12, 31);
            let total_weeks = (365u32 + jan1_dow + (6u32.saturating_sub(dec31_dow))) / 7 + 1;
            year_offset += (total_weeks as f64) * (self.cell_size + self.cell_gap) + 20.0;
        }

        // Draw legend
        let legend_x = rect.pos.x + rect.size.x - 150.0;
        let legend_y = rect.pos.y + rect.size.y - 30.0;
        let legend_items = 5;

        for i in 0..legend_items {
            let t = i as f64 / (legend_items - 1) as f64;
            let color = self.interpolate_color(t);
            let x = legend_x + (i as f64) * (self.cell_size + 2.0);
            self.draw_cell.color = color;
            self.draw_cell.draw_bar(cx, Rect {
                pos: dvec2(x, legend_y),
                size: dvec2(self.cell_size, self.cell_size),
            });
        }
    }

    fn interpolate_color(&self, t: f64) -> Vec4 {
        if t <= 0.0 {
            return self.color_empty;
        }

        // Multi-stop gradient: empty -> low -> high
        vec4(
            (self.color_low.x + t as f32 * (self.color_high.x - self.color_low.x)),
            (self.color_low.y + t as f32 * (self.color_high.y - self.color_low.y)),
            (self.color_low.z + t as f32 * (self.color_high.z - self.color_low.z)),
            (self.color_low.w + t as f32 * (self.color_high.w - self.color_low.w)),
        )
    }
}

impl CalendarChart {
    fn initialize_demo_data(&mut self) {
        // Generate sample activity data for 2024
        let mut entries = Vec::new();
        for month in 1..=12u32 {
            let days = Self::days_in_month(2024, month);
            for day in 1..=days {
                // Create varying activity levels
                let value = ((month as f64 * 7.0 + day as f64 * 3.0) % 10.0) *
                           (if day % 7 < 2 { 0.3 } else { 1.0 }); // Less on weekends
                if value > 2.0 {
                    entries.push(CalendarEntry::new(2024, month, day, value));
                }
            }
        }
        self.entries = entries;
        self.start_year = 2024;
        self.end_year = 2024;
        self.cell_size = 12.0;
        self.cell_gap = 2.0;
        self.color_empty = vec4(0.15, 0.15, 0.18, 1.0);
        self.color_low = vec4(0.1, 0.4, 0.2, 1.0);
        self.color_high = vec4(0.2, 0.9, 0.4, 1.0);
    }
}

impl Widget for CalendarChart {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);

        if !self.initialized {
            self.initialize_demo_data();
            self.animator = ChartAnimator::new(1.5 * 1000.0);
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
