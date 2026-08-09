//! Axis Renderer
//!
//! GPU-accelerated axis rendering using DrawChartLine for lines and DrawText for labels.

use makepad_widgets::*;
use makepad_d3::prelude::*;
use super::draw_primitives::DrawChartLine;

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawChartLine;

    AXIS_FONT = {
        font_family: {
            latin = font("crate://self/resources/Manrope-Regular.ttf", 0.0, 0.0),
        }
    }

    pub DrawAxisText = {{DrawAxisText}} {
        color: #666666,
        text_style: <AXIS_FONT> {
            font_size: 10.0,
        }
    }
}

/// Draw shader for axis text labels
#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawAxisText {
    #[deref]
    pub draw_text: DrawText,
}

/// Axis renderer configuration
#[derive(Clone, Debug)]
pub struct AxisRendererConfig {
    /// Color for axis domain line
    pub domain_color: Vec4,
    /// Color for tick marks
    pub tick_color: Vec4,
    /// Color for grid lines
    pub grid_color: Vec4,
    /// Color for tick labels
    pub label_color: Vec4,
    /// Width of domain line
    pub domain_width: f64,
    /// Width of tick marks
    pub tick_width: f64,
    /// Width of grid lines
    pub grid_width: f64,
    /// Whether to show grid lines
    pub show_grid: bool,
}

impl Default for AxisRendererConfig {
    fn default() -> Self {
        Self {
            domain_color: vec4(0.4, 0.4, 0.4, 1.0),
            tick_color: vec4(0.4, 0.4, 0.4, 1.0),
            grid_color: vec4(0.9, 0.9, 0.9, 0.5),
            label_color: vec4(0.4, 0.4, 0.4, 1.0),
            domain_width: 1.0,
            tick_width: 1.0,
            grid_width: 1.0,
            show_grid: true,
        }
    }
}

impl AxisRendererConfig {
    /// Create config with grid enabled
    pub fn with_grid() -> Self {
        Self {
            show_grid: true,
            ..Self::default()
        }
    }

    /// Create config without grid
    pub fn without_grid() -> Self {
        Self {
            show_grid: false,
            ..Self::default()
        }
    }

    /// Set domain line color
    pub fn domain_color(mut self, color: Vec4) -> Self {
        self.domain_color = color;
        self
    }

    /// Set grid color
    pub fn grid_color(mut self, color: Vec4) -> Self {
        self.grid_color = color;
        self
    }
}

/// Render an axis layout using GPU primitives
pub fn render_axis(
    cx: &mut Cx2d,
    draw_line: &mut DrawChartLine,
    draw_text: &mut DrawAxisText,
    layout: &AxisLayout,
    offset: DVec2,
    config: &AxisRendererConfig,
) {
    // Draw domain line
    if layout.show_domain_line {
        draw_line.color = config.domain_color;
        draw_line.draw_line(
            cx,
            dvec2(layout.domain_start.0 + offset.x, layout.domain_start.1 + offset.y),
            dvec2(layout.domain_end.0 + offset.x, layout.domain_end.1 + offset.y),
            config.domain_width,
        );
    }

    // Draw ticks and labels
    for tick in &layout.ticks {
        // Draw tick mark
        draw_line.color = config.tick_color;
        draw_line.draw_line(
            cx,
            dvec2(tick.tick_start.0 + offset.x, tick.tick_start.1 + offset.y),
            dvec2(tick.tick_end.0 + offset.x, tick.tick_end.1 + offset.y),
            config.tick_width,
        );

        // Draw grid line if enabled
        if config.show_grid {
            if let Some(grid_end) = tick.grid_end {
                draw_line.color = config.grid_color;
                draw_line.draw_line(
                    cx,
                    dvec2(tick.tick_start.0 + offset.x, tick.tick_start.1 + offset.y),
                    dvec2(grid_end.0 + offset.x, grid_end.1 + offset.y),
                    config.grid_width,
                );
            }
        }

        // Draw label
        draw_text.color = config.label_color;
        let label_pos = compute_label_position(tick, layout, offset);
        draw_text.draw_abs(cx, label_pos, &tick.label);
    }
}

/// Compute label position with text anchor adjustment
fn compute_label_position(tick: &AxisTick, layout: &AxisLayout, offset: DVec2) -> DVec2 {
    let base_pos = dvec2(
        tick.label_position.0 + offset.x,
        tick.label_position.1 + offset.y,
    );

    // Adjust position based on text anchor and orientation
    // For now, simple offset adjustments
    // TODO: Get actual text metrics for proper centering
    match layout.orientation {
        AxisOrientation::Bottom => {
            // Labels below, centered horizontally
            dvec2(base_pos.x - estimate_text_width(&tick.label) / 2.0, base_pos.y)
        }
        AxisOrientation::Top => {
            // Labels above, centered horizontally
            dvec2(base_pos.x - estimate_text_width(&tick.label) / 2.0, base_pos.y - 12.0)
        }
        AxisOrientation::Left => {
            // Labels to the left, right-aligned
            dvec2(base_pos.x - estimate_text_width(&tick.label), base_pos.y - 4.0)
        }
        AxisOrientation::Right => {
            // Labels to the right, left-aligned
            dvec2(base_pos.x, base_pos.y - 4.0)
        }
    }
}

/// Estimate text width (simplified - actual width depends on font metrics)
fn estimate_text_width(text: &str) -> f64 {
    // Rough estimate: ~6 pixels per character at font size 10
    text.len() as f64 * 6.0
}

/// Helper to create and configure axes for a chart
pub struct ChartAxes {
    pub x_axis: Axis,
    pub y_axis: Axis,
}

impl ChartAxes {
    /// Create axes for a chart with given dimensions
    pub fn new(chart_width: f64, chart_height: f64) -> Self {
        let x_config = AxisConfig::bottom()
            .with_tick_size(6.0)
            .with_tick_padding(8.0)
            .with_grid(chart_height);

        let y_config = AxisConfig::left()
            .with_tick_size(6.0)
            .with_tick_padding(8.0)
            .with_grid(chart_width);

        Self {
            x_axis: Axis::with_config(x_config),
            y_axis: Axis::with_config(y_config),
        }
    }

    /// Configure x-axis from a scale
    pub fn set_x_scale<S: Scale>(&mut self, scale: &S) {
        self.x_axis.set_scale(scale);
    }

    /// Configure y-axis from a scale
    pub fn set_y_scale<S: Scale>(&mut self, scale: &S) {
        self.y_axis.set_scale(scale);
    }

    /// Compute layouts for both axes
    pub fn compute_layouts(&self, x_position: f64, y_position: f64) -> (AxisLayout, AxisLayout) {
        let x_layout = self.x_axis.compute_layout(x_position);
        let y_layout = self.y_axis.compute_layout(y_position);
        (x_layout, y_layout)
    }
}
