//! Legend Renderer
//!
//! GPU-accelerated legend rendering for multi-series charts.

use makepad_widgets::*;
use super::draw_primitives::{DrawChartLine, DrawPoint};
use super::axis_renderer::DrawAxisText;

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawChartLine;
    use super::draw_primitives::DrawPoint;
    use super::axis_renderer::DrawAxisText;
}

/// Legend item data
#[derive(Clone, Debug)]
pub struct LegendItem {
    /// Label text
    pub label: String,
    /// Color for this item
    pub color: Vec4,
    /// Optional: marker shape (line, circle, square)
    pub marker: LegendMarker,
}

impl LegendItem {
    /// Create a new legend item with a label and color
    pub fn new(label: impl Into<String>, color: Vec4) -> Self {
        Self {
            label: label.into(),
            color,
            marker: LegendMarker::Circle,
        }
    }

    /// Set the marker type
    pub fn with_marker(mut self, marker: LegendMarker) -> Self {
        self.marker = marker;
        self
    }
}

/// Legend marker types
#[derive(Clone, Copy, Debug, Default)]
pub enum LegendMarker {
    /// Circle marker (for scatter, line charts)
    #[default]
    Circle,
    /// Square/rectangle marker (for bar charts)
    Square,
    /// Short line marker (for line charts)
    Line,
}

/// Legend position relative to chart
#[derive(Clone, Copy, Debug, Default)]
pub enum LegendPosition {
    /// Top of chart, centered
    #[default]
    Top,
    /// Bottom of chart, centered
    Bottom,
    /// Right side of chart
    Right,
    /// Left side of chart
    Left,
    /// Top-right corner
    TopRight,
    /// Top-left corner
    TopLeft,
    /// Bottom-right corner
    BottomRight,
    /// Bottom-left corner
    BottomLeft,
}

/// Legend layout direction
#[derive(Clone, Copy, Debug, Default)]
pub enum LegendLayout {
    /// Items arranged horizontally
    #[default]
    Horizontal,
    /// Items arranged vertically
    Vertical,
}

/// Legend renderer configuration
#[derive(Clone, Debug)]
pub struct LegendConfig {
    /// Position of the legend
    pub position: LegendPosition,
    /// Layout direction
    pub layout: LegendLayout,
    /// Marker size
    pub marker_size: f64,
    /// Spacing between marker and label
    pub marker_label_gap: f64,
    /// Spacing between items
    pub item_spacing: f64,
    /// Font size for labels
    pub font_size: f64,
    /// Label color
    pub label_color: Vec4,
    /// Padding around legend
    pub padding: f64,
}

impl Default for LegendConfig {
    fn default() -> Self {
        Self {
            position: LegendPosition::TopRight,
            layout: LegendLayout::Vertical,
            marker_size: 10.0,
            marker_label_gap: 8.0,
            item_spacing: 16.0,
            font_size: 10.0,
            label_color: vec4(0.4, 0.4, 0.4, 1.0),
            padding: 10.0,
        }
    }
}

impl LegendConfig {
    /// Create a horizontal legend for top/bottom positioning
    pub fn horizontal() -> Self {
        Self {
            position: LegendPosition::Top,
            layout: LegendLayout::Horizontal,
            item_spacing: 24.0,
            ..Self::default()
        }
    }

    /// Create a vertical legend for right/left positioning
    pub fn vertical() -> Self {
        Self {
            position: LegendPosition::TopRight,
            layout: LegendLayout::Vertical,
            item_spacing: 16.0,
            ..Self::default()
        }
    }

    /// Set position
    pub fn with_position(mut self, position: LegendPosition) -> Self {
        self.position = position;
        self
    }
}

/// Computed legend layout
pub struct LegendLayout_Computed {
    /// Total width of legend
    pub width: f64,
    /// Total height of legend
    pub height: f64,
    /// Position of each item (marker center)
    pub item_positions: Vec<DVec2>,
}

/// Render a legend
pub fn render_legend(
    cx: &mut Cx2d,
    draw_line: &mut DrawChartLine,
    draw_point: &mut DrawPoint,
    draw_text: &mut DrawAxisText,
    items: &[LegendItem],
    chart_rect: Rect,
    config: &LegendConfig,
) {
    if items.is_empty() {
        return;
    }

    // Compute legend dimensions and position
    let layout = compute_legend_layout(items, config);
    let legend_pos = compute_legend_position(&layout, chart_rect, config);

    // Draw each item
    match config.layout {
        LegendLayout::Horizontal => {
            let mut x = legend_pos.x;
            let y = legend_pos.y;

            for item in items {
                // Draw marker
                draw_marker(cx, draw_line, draw_point, item, dvec2(x, y), config);

                // Draw label
                draw_text.color = config.label_color;
                let label_x = x + config.marker_size / 2.0 + config.marker_label_gap;
                draw_text.draw_abs(cx, dvec2(label_x, y - config.font_size / 2.0), &item.label);

                // Move to next item
                x += config.marker_size + config.marker_label_gap + estimate_text_width(&item.label, config.font_size) + config.item_spacing;
            }
        }
        LegendLayout::Vertical => {
            let x = legend_pos.x;
            let mut y = legend_pos.y;

            for item in items {
                // Draw marker
                draw_marker(cx, draw_line, draw_point, item, dvec2(x, y), config);

                // Draw label
                draw_text.color = config.label_color;
                let label_x = x + config.marker_size / 2.0 + config.marker_label_gap;
                draw_text.draw_abs(cx, dvec2(label_x, y - config.font_size / 2.0), &item.label);

                // Move to next item
                y += config.item_spacing;
            }
        }
    }
}

/// Draw a legend marker
fn draw_marker(
    cx: &mut Cx2d,
    draw_line: &mut DrawChartLine,
    draw_point: &mut DrawPoint,
    item: &LegendItem,
    pos: DVec2,
    config: &LegendConfig,
) {
    match item.marker {
        LegendMarker::Circle => {
            draw_point.color = item.color;
            draw_point.draw_point(cx, pos, config.marker_size);
        }
        LegendMarker::Square => {
            // Draw a small filled square using lines
            let half = config.marker_size / 2.0;
            draw_line.color = item.color;
            // Fill with multiple horizontal lines
            for i in 0..=(config.marker_size as i32) {
                let y = pos.y - half + i as f64;
                draw_line.draw_line(cx, dvec2(pos.x - half, y), dvec2(pos.x + half, y), 1.0);
            }
        }
        LegendMarker::Line => {
            draw_line.color = item.color;
            let half = config.marker_size / 2.0 + 2.0;
            draw_line.draw_line(cx, dvec2(pos.x - half, pos.y), dvec2(pos.x + half, pos.y), 2.5);
            // Small circle at center
            draw_point.color = item.color;
            draw_point.draw_point(cx, pos, 4.0);
        }
    }
}

/// Compute legend layout dimensions
fn compute_legend_layout(items: &[LegendItem], config: &LegendConfig) -> LegendLayout_Computed {
    let mut total_width = 0.0;
    let mut total_height = 0.0;
    let mut item_positions = Vec::new();

    match config.layout {
        LegendLayout::Horizontal => {
            let mut x = config.padding;
            let y = config.padding + config.marker_size / 2.0;

            for item in items {
                item_positions.push(dvec2(x + config.marker_size / 2.0, y));
                let item_width = config.marker_size + config.marker_label_gap + estimate_text_width(&item.label, config.font_size);
                x += item_width + config.item_spacing;
            }

            total_width = x - config.item_spacing + config.padding;
            total_height = config.padding * 2.0 + config.marker_size;
        }
        LegendLayout::Vertical => {
            let x = config.padding + config.marker_size / 2.0;
            let mut y = config.padding + config.marker_size / 2.0;

            let mut max_item_width = 0.0;
            for item in items {
                item_positions.push(dvec2(x, y));
                let item_width = config.marker_size + config.marker_label_gap + estimate_text_width(&item.label, config.font_size);
                max_item_width = max_item_width.max(item_width);
                y += config.item_spacing;
            }

            total_width = config.padding * 2.0 + max_item_width;
            total_height = y - config.item_spacing + config.padding;
        }
    }

    LegendLayout_Computed {
        width: total_width,
        height: total_height,
        item_positions,
    }
}

/// Compute legend position based on chart rect and config
fn compute_legend_position(layout: &LegendLayout_Computed, chart_rect: Rect, config: &LegendConfig) -> DVec2 {
    match config.position {
        LegendPosition::Top => {
            dvec2(
                chart_rect.pos.x + (chart_rect.size.x - layout.width) / 2.0 + config.padding,
                chart_rect.pos.y + config.padding,
            )
        }
        LegendPosition::Bottom => {
            dvec2(
                chart_rect.pos.x + (chart_rect.size.x - layout.width) / 2.0 + config.padding,
                chart_rect.pos.y + chart_rect.size.y - layout.height + config.padding,
            )
        }
        LegendPosition::Right => {
            dvec2(
                chart_rect.pos.x + chart_rect.size.x - layout.width + config.padding,
                chart_rect.pos.y + (chart_rect.size.y - layout.height) / 2.0 + config.padding,
            )
        }
        LegendPosition::Left => {
            dvec2(
                chart_rect.pos.x + config.padding,
                chart_rect.pos.y + (chart_rect.size.y - layout.height) / 2.0 + config.padding,
            )
        }
        LegendPosition::TopRight => {
            dvec2(
                chart_rect.pos.x + chart_rect.size.x - layout.width + config.padding,
                chart_rect.pos.y + config.padding,
            )
        }
        LegendPosition::TopLeft => {
            dvec2(
                chart_rect.pos.x + config.padding,
                chart_rect.pos.y + config.padding,
            )
        }
        LegendPosition::BottomRight => {
            dvec2(
                chart_rect.pos.x + chart_rect.size.x - layout.width + config.padding,
                chart_rect.pos.y + chart_rect.size.y - layout.height + config.padding,
            )
        }
        LegendPosition::BottomLeft => {
            dvec2(
                chart_rect.pos.x + config.padding,
                chart_rect.pos.y + chart_rect.size.y - layout.height + config.padding,
            )
        }
    }
}

/// Estimate text width (simplified)
fn estimate_text_width(text: &str, font_size: f64) -> f64 {
    // Rough estimate: ~0.6 * font_size per character
    text.len() as f64 * font_size * 0.6
}
