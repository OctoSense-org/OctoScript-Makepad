//! Candlestick Chart Widget
//!
//! GPU-accelerated OHLC financial chart with smooth animations.

use makepad_widgets::*;
use makepad_d3::prelude::*;
use super::draw_primitives::{DrawTriangle, DrawChartLine};
use super::animation::{ChartAnimator, EasingType};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawTriangle;
    use super::draw_primitives::DrawChartLine;

    pub CandlestickWidget = {{CandlestickWidget}} {
        width: Fill,
        height: Fill,
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct CandlestickWidget {
    #[redraw]
    #[live]
    draw_candle: DrawTriangle,

    #[redraw]
    #[live]
    draw_line: DrawChartLine,

    #[walk]
    walk: Walk,

    #[rust]
    data: Vec<CandleData>,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,
}

#[derive(Clone)]
struct CandleData {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

impl Widget for CandlestickWidget {
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
            if !self.initialized {
                self.initialize_demo_data();
                self.start_animation(cx);
                self.initialized = true;
            }

            self.draw_candlesticks(cx, rect);
        }

        DrawStep::done()
    }
}

impl CandlestickWidget {
    fn initialize_demo_data(&mut self) {
        let mut price = 100.0;
        let mut data = Vec::new();

        let dates = [
            "Jan 2", "Jan 3", "Jan 4", "Jan 5", "Jan 8",
            "Jan 9", "Jan 10", "Jan 11", "Jan 12", "Jan 15",
            "Jan 16", "Jan 17", "Jan 18", "Jan 19", "Jan 22",
            "Jan 23", "Jan 24", "Jan 25", "Jan 26", "Jan 29",
        ];

        for (i, date) in dates.iter().enumerate() {
            let seed = (i * 7 + 3) as f64;
            let change = ((seed * 1.7).sin() * 5.0) + ((seed * 0.3).cos() * 3.0);

            let open = price;
            let close = price + change;
            let high = open.max(close) + (seed * 0.5).abs() % 3.0;
            let low = open.min(close) - (seed * 0.3).abs() % 3.0;

            data.push(CandleData {
                date: date.to_string(),
                open, high, low, close,
            });

            price = close;
        }

        self.data = data;
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1400.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        cx.new_next_frame();
    }

    fn draw_candlesticks(&mut self, cx: &mut Cx2d, rect: Rect) {
        let progress = self.animator.get_progress();

        let padding = 50.0;
        let chart_x = rect.pos.x + padding;
        let chart_y = rect.pos.y + 20.0;
        let chart_width = rect.size.x - padding - 20.0;
        let chart_height = rect.size.y - padding - 20.0;

        if chart_width <= 0.0 || chart_height <= 0.0 || self.data.is_empty() {
            return;
        }

        // Find price range
        let min_price = self.data.iter().map(|d| d.low).fold(f64::INFINITY, f64::min);
        let max_price = self.data.iter().map(|d| d.high).fold(f64::NEG_INFINITY, f64::max);

        let price_padding = (max_price - min_price) * 0.1;
        let y_scale = LinearScale::new()
            .with_domain(min_price - price_padding, max_price + price_padding)
            .with_range(chart_height as f64, 0.0);

        let n = self.data.len();
        let candle_width = (chart_width as f64 / n as f64) * 0.7;
        let spacing = chart_width as f64 / n as f64;

        // Colors
        let bullish_color = vec4(0.20, 0.66, 0.33, 1.0);
        let bearish_color = vec4(0.92, 0.26, 0.21, 1.0);
        let wick_color = vec4(0.4, 0.4, 0.4, 1.0);

        // Clone data
        let data: Vec<_> = self.data.clone();

        for (i, candle) in data.iter().enumerate() {
            let candle_progress = ((progress - i as f64 * 0.03) / 0.4).clamp(0.0, 1.0);
            if candle_progress <= 0.0 {
                continue;
            }

            let center_x = chart_x as f64 + spacing * (i as f64 + 0.5);
            let is_bullish = candle.close >= candle.open;

            // Calculate positions
            let high_y = chart_y as f64 + y_scale.scale(candle.high);
            let low_y = chart_y as f64 + y_scale.scale(candle.low);
            let body_top = if is_bullish { candle.close } else { candle.open };
            let body_bottom = if is_bullish { candle.open } else { candle.close };
            let body_y = chart_y as f64 + y_scale.scale(body_top);
            let body_height = y_scale.scale(body_bottom) - y_scale.scale(body_top);

            // Animate from center
            let mid_y = (high_y + low_y) / 2.0;
            let anim_high_y = mid_y + (high_y - mid_y) * candle_progress;
            let anim_low_y = mid_y + (low_y - mid_y) * candle_progress;
            let anim_body_y = mid_y + (body_y - mid_y) * candle_progress;
            let anim_body_height = body_height * candle_progress;

            // Draw wick
            self.draw_line.color = wick_color;
            self.draw_line.draw_line(
                cx,
                dvec2(center_x, anim_high_y),
                dvec2(center_x, anim_low_y),
                2.0,
            );

            // Draw body
            let color = if is_bullish { bullish_color } else { bearish_color };
            self.draw_candle.color = color;
            self.draw_candle.disable_gradient();

            let p1 = dvec2(center_x - candle_width / 2.0, anim_body_y);
            let p2 = dvec2(center_x + candle_width / 2.0, anim_body_y);
            let p3 = dvec2(center_x + candle_width / 2.0, anim_body_y + anim_body_height.max(2.0));
            let p4 = dvec2(center_x - candle_width / 2.0, anim_body_y + anim_body_height.max(2.0));

            self.draw_candle.draw_triangle(cx, p1, p2, p3);
            self.draw_candle.draw_triangle(cx, p1, p3, p4);

            // Draw border
            if candle_progress > 0.6 {
                self.draw_line.color = vec4(0.15, 0.15, 0.18, 0.6);
                self.draw_line.draw_line(cx, p1, p2, 1.0);
                self.draw_line.draw_line(cx, p2, p3, 1.0);
                self.draw_line.draw_line(cx, p3, p4, 1.0);
                self.draw_line.draw_line(cx, p4, p1, 1.0);
            }
        }
    }

    pub fn set_data(&mut self, data: Vec<(String, f64, f64, f64, f64)>) {
        self.data = data
            .into_iter()
            .map(|(date, open, high, low, close)| CandleData { date, open, high, low, close })
            .collect();
        self.initialized = false;
    }
}
