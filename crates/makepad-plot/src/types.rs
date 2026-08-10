// Shared plotting types for makepad-plot (Makepad 2.0 / Splash)

use makepad_widgets::*;

script_mod! {
    mod.plot.LineStyle = #(LineStyle::script_api(vm))
    mod.plot.MarkerStyle = #(MarkerStyle::script_api(vm))
    mod.plot.StepStyle = #(StepStyle::script_api(vm))
    mod.plot.ScaleType = #(ScaleType::script_api(vm))
    mod.plot.LegendPosition = #(LegendPosition::script_api(vm))
}

/// Line style enumeration
#[derive(Clone, Copy, Debug, Default, PartialEq, Script, ScriptHook)]
pub enum LineStyle {
    #[default]
    Solid,
    Dashed,
    Dotted,
    DashDot,
}

/// Marker style enumeration
#[derive(Clone, Copy, Debug, Default, PartialEq, Script, ScriptHook)]
pub enum MarkerStyle {
    None,
    #[default]
    Circle,
    Square,
    TriangleUp,
    TriangleDown,
    Diamond,
    Cross,
    Plus,
    Star,
}

/// Step plot style - where to place the step
#[derive(Clone, Copy, Debug, Default, PartialEq, Script, ScriptHook)]
pub enum StepStyle {
    #[default]
    None,   // Normal line (no step)
    Pre,    // Step before the point (y value changes at x)
    Post,   // Step after the point (y value changes at next x)
    Mid,    // Step in the middle between points
}

/// Axis scale type
#[derive(Clone, Copy, Debug, Default, PartialEq, Script, ScriptHook)]
pub enum ScaleType {
    #[default]
    Linear,
    Log,      // Logarithmic (base 10)
    SymLog,   // Symmetric log (handles negative values)
    Time,     // Time axis (values are Unix timestamps in seconds)
}

impl ScaleType {
    /// Transform a value according to the scale type
    pub fn transform(&self, value: f64) -> f64 {
        match self {
            ScaleType::Linear | ScaleType::Time => value,
            ScaleType::Log => {
                if value > 0.0 {
                    value.log10()
                } else {
                    f64::NEG_INFINITY
                }
            }
            ScaleType::SymLog => {
                // Symmetric log: sign(x) * log10(1 + |x|)
                let sign = if value >= 0.0 { 1.0 } else { -1.0 };
                sign * (1.0 + value.abs()).log10()
            }
        }
    }

    /// Inverse transform a value
    pub fn inverse(&self, value: f64) -> f64 {
        match self {
            ScaleType::Linear | ScaleType::Time => value,
            ScaleType::Log => 10.0_f64.powf(value),
            ScaleType::SymLog => {
                let sign = if value >= 0.0 { 1.0 } else { -1.0 };
                sign * (10.0_f64.powf(value.abs()) - 1.0)
            }
        }
    }

    /// Generate nice tick values for this scale type
    pub fn generate_ticks(&self, min: f64, max: f64, count: usize) -> Vec<f64> {
        match self {
            ScaleType::Linear => {
                let step = (max - min) / count as f64;
                (0..=count).map(|i| min + i as f64 * step).collect()
            }
            ScaleType::Time => {
                // Time intervals in seconds
                let intervals = [
                    1.0,           // 1 second
                    5.0,           // 5 seconds
                    10.0,          // 10 seconds
                    30.0,          // 30 seconds
                    60.0,          // 1 minute
                    300.0,         // 5 minutes
                    600.0,         // 10 minutes
                    1800.0,        // 30 minutes
                    3600.0,        // 1 hour
                    7200.0,        // 2 hours
                    21600.0,       // 6 hours
                    43200.0,       // 12 hours
                    86400.0,       // 1 day
                    172800.0,      // 2 days
                    604800.0,      // 1 week
                    2592000.0,     // 30 days
                    7776000.0,     // 90 days
                    31536000.0,    // 1 year
                ];

                let range = max - min;
                let target_interval = range / count as f64;

                // Find best interval
                let interval = intervals
                    .iter()
                    .copied()
                    .find(|&i| i >= target_interval)
                    .unwrap_or(intervals[intervals.len() - 1]);

                // Generate ticks aligned to interval
                let first_tick = (min / interval).ceil() * interval;
                let mut ticks = Vec::new();
                let mut tick = first_tick;
                while tick <= max {
                    ticks.push(tick);
                    tick += interval;
                }
                ticks
            }
            ScaleType::Log => {
                if min <= 0.0 || max <= 0.0 {
                    return vec![];
                }
                let log_min = min.log10().floor() as i32;
                let log_max = max.log10().ceil() as i32;
                (log_min..=log_max)
                    .map(|exp| 10.0_f64.powi(exp))
                    .filter(|&v| v >= min && v <= max)
                    .collect()
            }
            ScaleType::SymLog => {
                // For symlog, generate ticks including negative, zero, and positive
                let mut ticks = Vec::new();

                // Add negative ticks
                if min < 0.0 {
                    let neg_max = min.abs();
                    let log_max = neg_max.log10().ceil() as i32;
                    for exp in (0..=log_max).rev() {
                        let val = -10.0_f64.powi(exp);
                        if val >= min {
                            ticks.push(val);
                        }
                    }
                }

                // Add zero if in range
                if min <= 0.0 && max >= 0.0 {
                    ticks.push(0.0);
                }

                // Add positive ticks
                if max > 0.0 {
                    let log_max = max.log10().ceil() as i32;
                    for exp in 0..=log_max {
                        let val = 10.0_f64.powi(exp);
                        if val <= max && val >= min {
                            ticks.push(val);
                        }
                    }
                }

                ticks
            }
        }
    }

    /// Format a tick label for this scale type
    pub fn format_tick(&self, value: f64) -> String {
        match self {
            ScaleType::Linear => format!("{:.1}", value),
            ScaleType::Time => {
                // Format Unix timestamp as human-readable date
                let secs = value as i64;
                let days_since_epoch = secs / 86400;

                // Simple year/month/day calculation
                let mut year: i64 = 1970;
                let mut days_left = days_since_epoch;

                // Advance years
                while days_left >= 365 {
                    let days_in_year: i64 = if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 { 366 } else { 365 };
                    if days_left < days_in_year {
                        break;
                    }
                    days_left -= days_in_year;
                    year += 1;
                }

                // Days in each month
                let is_leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
                let days_in_months: [i64; 12] = [31, if is_leap { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

                let mut month: i64 = 1;
                for &days in &days_in_months {
                    if days_left < days {
                        break;
                    }
                    days_left -= days;
                    month += 1;
                }

                let day = days_left + 1;

                // Show as M/D format
                format!("{}/{}", month, day)
            }
            ScaleType::Log => {
                if value > 0.0 {
                    let exp = value.log10().round() as i32;
                    if (10.0_f64.powi(exp) - value).abs() < 1e-10 {
                        format!("10^{}", exp)
                    } else {
                        format!("{:.1}", value)
                    }
                } else {
                    format!("{:.1}", value)
                }
            }
            ScaleType::SymLog => {
                if value == 0.0 {
                    "0".to_string()
                } else if value.abs() >= 1.0 {
                    let exp = value.abs().log10().round() as i32;
                    if (10.0_f64.powi(exp) - value.abs()).abs() < 1e-10 {
                        if value < 0.0 {
                            format!("-10^{}", exp)
                        } else {
                            format!("10^{}", exp)
                        }
                    } else {
                        format!("{:.1}", value)
                    }
                } else {
                    format!("{:.2}", value)
                }
            }
        }
    }
}

/// Legend position options
#[derive(Clone, Copy, Debug, Default, PartialEq, Script, ScriptHook)]
pub enum LegendPosition {
    #[default]
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    None, // Hidden
}

/// Vertical line annotation
#[derive(Clone)]
pub struct VLine {
    pub x: f64,
    pub color: Vec4,
    pub line_width: f64,
    pub line_style: LineStyle,
}

/// Horizontal line annotation
#[derive(Clone)]
pub struct HLine {
    pub y: f64,
    pub color: Vec4,
    pub line_width: f64,
    pub line_style: LineStyle,
}

/// Vertical span (shaded region)
#[derive(Clone)]
pub struct VSpan {
    pub x1: f64,
    pub x2: f64,
    pub color: Vec4,
}

/// Horizontal span (shaded region)
#[derive(Clone)]
pub struct HSpan {
    pub y1: f64,
    pub y2: f64,
    pub color: Vec4,
}

/// Data series for plotting
#[derive(Clone, Debug, Default)]
pub struct Series {
    pub label: String,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub color: Option<Vec4>,
    pub line_style: LineStyle,
    pub marker_style: MarkerStyle,
    pub step_style: StepStyle,
    pub line_width: Option<f64>,
    pub marker_size: Option<f64>,
    // Error bar data
    pub xerr_minus: Option<Vec<f64>>,
    pub xerr_plus: Option<Vec<f64>>,
    pub yerr_minus: Option<Vec<f64>>,
    pub yerr_plus: Option<Vec<f64>>,
}

impl Series {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            marker_style: MarkerStyle::None,
            ..Default::default()
        }
    }

    pub fn with_data(mut self, x: Vec<f64>, y: Vec<f64>) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_line_style(mut self, style: LineStyle) -> Self {
        self.line_style = style;
        self
    }

    pub fn with_marker(mut self, style: MarkerStyle) -> Self {
        self.marker_style = style;
        self
    }

    pub fn with_step(mut self, style: StepStyle) -> Self {
        self.step_style = style;
        self
    }

    pub fn with_line_width(mut self, width: f64) -> Self {
        self.line_width = Some(width);
        self
    }

    pub fn with_marker_size(mut self, size: f64) -> Self {
        self.marker_size = Some(size);
        self
    }

    /// Add symmetric y error bars
    pub fn with_yerr(mut self, yerr: Vec<f64>) -> Self {
        self.yerr_minus = Some(yerr.clone());
        self.yerr_plus = Some(yerr);
        self
    }

    /// Add asymmetric y error bars
    pub fn with_yerr_asymmetric(mut self, yerr_minus: Vec<f64>, yerr_plus: Vec<f64>) -> Self {
        self.yerr_minus = Some(yerr_minus);
        self.yerr_plus = Some(yerr_plus);
        self
    }

    /// Add symmetric x error bars
    pub fn with_xerr(mut self, xerr: Vec<f64>) -> Self {
        self.xerr_minus = Some(xerr.clone());
        self.xerr_plus = Some(xerr);
        self
    }

    /// Add asymmetric x error bars
    pub fn with_xerr_asymmetric(mut self, xerr_minus: Vec<f64>, xerr_plus: Vec<f64>) -> Self {
        self.xerr_minus = Some(xerr_minus);
        self.xerr_plus = Some(xerr_plus);
        self
    }
}

/// Plot area boundaries (in absolute pixels)
#[derive(Clone, Copy, Debug, Default)]
pub struct PlotArea {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl PlotArea {
    pub fn new(left: f64, top: f64, right: f64, bottom: f64) -> Self {
        Self { left, top, right, bottom }
    }

    pub fn width(&self) -> f64 {
        self.right - self.left
    }

    pub fn height(&self) -> f64 {
        self.bottom - self.top
    }
}

/// Represents a filled region between two y values (for fill_between)
#[derive(Clone)]
pub struct FillRegion {
    pub x: Vec<f64>,
    pub y1: Vec<f64>,
    pub y2: Vec<f64>,
    pub color: Vec4,
}

/// Text annotation on the plot
#[derive(Clone)]
pub struct TextAnnotation {
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub color: Vec4,
    pub font_size: f64,
    pub is_math: bool, // Rendered as plain text in the Splash port
}

/// Arrow annotation pointing from one location to another
#[derive(Clone)]
pub struct ArrowAnnotation {
    pub start_x: f64,
    pub start_y: f64,
    pub end_x: f64,
    pub end_y: f64,
    pub color: Vec4,
    pub line_width: f64,
    pub head_size: f64,
    pub text: Option<String>, // Optional label near the arrow start
}

impl ArrowAnnotation {
    pub fn new(start_x: f64, start_y: f64, end_x: f64, end_y: f64) -> Self {
        Self {
            start_x,
            start_y,
            end_x,
            end_y,
            color: vec4(0.2, 0.2, 0.2, 1.0),
            line_width: 1.5,
            head_size: 8.0,
            text: None,
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn with_line_width(mut self, width: f64) -> Self {
        self.line_width = width;
        self
    }

    pub fn with_head_size(mut self, size: f64) -> Self {
        self.head_size = size;
        self
    }
}

/// Default matplotlib-style color cycle (tab10)
pub const COLOR_CYCLE: [Vec4; 10] = [
    Vec4 { x: 0.121, y: 0.466, z: 0.705, w: 1.0 }, // tab:blue
    Vec4 { x: 1.000, y: 0.498, z: 0.054, w: 1.0 }, // tab:orange
    Vec4 { x: 0.172, y: 0.627, z: 0.172, w: 1.0 }, // tab:green
    Vec4 { x: 0.839, y: 0.152, z: 0.156, w: 1.0 }, // tab:red
    Vec4 { x: 0.580, y: 0.403, z: 0.741, w: 1.0 }, // tab:purple
    Vec4 { x: 0.549, y: 0.337, z: 0.294, w: 1.0 }, // tab:brown
    Vec4 { x: 0.890, y: 0.466, z: 0.760, w: 1.0 }, // tab:pink
    Vec4 { x: 0.498, y: 0.498, z: 0.498, w: 1.0 }, // tab:gray
    Vec4 { x: 0.737, y: 0.741, z: 0.133, w: 1.0 }, // tab:olive
    Vec4 { x: 0.090, y: 0.745, z: 0.811, w: 1.0 }, // tab:cyan
];

/// Get color from the default cycle by index
pub fn cycle_color(i: usize) -> Vec4 {
    COLOR_CYCLE[i % COLOR_CYCLE.len()]
}

// =============================================================================
// Colormaps
// =============================================================================

#[derive(Clone, Debug, PartialEq)]
pub enum Colormap {
    // Perceptually uniform sequential
    Viridis,
    Plasma,
    Inferno,
    Magma,
    Cividis,  // Colorblind-friendly
    // Diverging
    Coolwarm,
    RdBu,     // Red-Blue diverging
    Spectral,
    // Sequential
    Blues,
    Greens,
    Oranges,
    Reds,
    Greys,
    // Classic
    Jet,      // Rainbow (legacy)
    Hot,      // Black-Red-Yellow-White
    // Special
    Turbo,    // Improved rainbow
    Custom(Vec<(f64, Vec4)>),  // User-defined color stops
}

impl Default for Colormap {
    fn default() -> Self {
        Colormap::Viridis
    }
}

impl Colormap {
    /// Look up a named colormap (used by Splash-side `colormap: "Viridis"` props)
    pub fn from_name(name: &str) -> Self {
        match name.to_ascii_lowercase().as_str() {
            "viridis" => Colormap::Viridis,
            "plasma" => Colormap::Plasma,
            "inferno" => Colormap::Inferno,
            "magma" => Colormap::Magma,
            "cividis" => Colormap::Cividis,
            "coolwarm" => Colormap::Coolwarm,
            "rdbu" => Colormap::RdBu,
            "spectral" => Colormap::Spectral,
            "blues" => Colormap::Blues,
            "greens" => Colormap::Greens,
            "oranges" => Colormap::Oranges,
            "reds" => Colormap::Reds,
            "greys" | "grays" => Colormap::Greys,
            "jet" => Colormap::Jet,
            "hot" => Colormap::Hot,
            "turbo" => Colormap::Turbo,
            _ => Colormap::Viridis,
        }
    }

    /// Sample a color from the colormap at position t (0.0 to 1.0)
    pub fn sample(&self, t: f64) -> Vec4 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Colormap::Viridis => {
                // Perceptually uniform purple-green-yellow
                let r = 0.267 + t * 0.329 - t * t * 0.5 + t * t * t * 0.9;
                let g = 0.004 + t * 0.873;
                let b = 0.329 + t * 0.5 - t * t * 0.6;
                vec4(r.clamp(0.0, 1.0) as f32, g.clamp(0.0, 1.0) as f32, b.clamp(0.0, 1.0) as f32, 1.0)
            }
            Colormap::Plasma => {
                // Purple-red-yellow
                let r = 0.05 + t * 0.9 + t * t * 0.05;
                let g = t * t * 0.9;
                let b = 0.53 + (1.0 - t) * 0.47 - t * t * 0.6;
                vec4(r.clamp(0.0, 1.0) as f32, g.clamp(0.0, 1.0) as f32, b.clamp(0.0, 1.0) as f32, 1.0)
            }
            Colormap::Inferno => {
                // Black-purple-red-yellow
                let r = t * t * 1.2;
                let g = t * t * t * 1.5;
                let b = (1.0 - t) * t * 2.0 + 0.1 * t;
                vec4(r.clamp(0.0, 1.0) as f32, g.clamp(0.0, 1.0) as f32, b.clamp(0.0, 1.0) as f32, 1.0)
            }
            Colormap::Magma => {
                // Black-purple-pink-white
                let r = t * t * 0.8 + t * 0.2;
                let g = t * t * t * 1.2;
                let b = t * 0.5 + (1.0 - t) * t * 1.0;
                vec4(r.clamp(0.0, 1.0) as f32, g.clamp(0.0, 1.0) as f32, b.clamp(0.0, 1.0) as f32, 1.0)
            }
            Colormap::Cividis => {
                // Colorblind-friendly blue-yellow
                let r = -0.01 + t * 1.0 + t * t * 0.01;
                let g = 0.14 + t * 0.72;
                let b = 0.35 + t * 0.1 - t * t * 0.35;
                vec4(r.clamp(0.0, 1.0) as f32, g.clamp(0.0, 1.0) as f32, b.clamp(0.0, 1.0) as f32, 1.0)
            }
            Colormap::Coolwarm => {
                // Blue-white-red diverging
                let r = if t < 0.5 { 0.2 + t * 1.6 } else { 1.0 };
                let g = if t < 0.5 { 0.2 + t * 1.0 } else { 1.0 - (t - 0.5) * 1.6 };
                let b = if t < 0.5 { 1.0 } else { 1.0 - (t - 0.5) * 1.6 };
                vec4(r as f32, g as f32, b as f32, 1.0)
            }
            Colormap::RdBu => {
                // Red-white-blue diverging (red=high, blue=low)
                let r = if t < 0.5 { 0.1 + t * 1.8 } else { 1.0 - (t - 0.5) * 1.4 };
                let g = if t < 0.5 { t * 1.8 } else { 0.9 - (t - 0.5) * 1.6 };
                let b = if t < 0.5 { 1.0 - t * 0.2 } else { 0.9 - (t - 0.5) * 1.0 };
                vec4(r.clamp(0.0, 1.0) as f32, g.clamp(0.0, 1.0) as f32, b.clamp(0.0, 1.0) as f32, 1.0)
            }
            Colormap::Spectral => {
                // Red-orange-yellow-green-blue (diverging rainbow)
                let (r, g, b) = if t < 0.25 {
                    let s = t / 0.25;
                    (0.62 + s * 0.38, 0.0 + s * 0.5, 0.26 * (1.0 - s))
                } else if t < 0.5 {
                    let s = (t - 0.25) / 0.25;
                    (1.0, 0.5 + s * 0.5, 0.0)
                } else if t < 0.75 {
                    let s = (t - 0.5) / 0.25;
                    (1.0 - s * 0.5, 1.0 - s * 0.2, s * 0.4)
                } else {
                    let s = (t - 0.75) / 0.25;
                    (0.5 - s * 0.3, 0.8 - s * 0.4, 0.4 + s * 0.6)
                };
                vec4(r as f32, g as f32, b as f32, 1.0)
            }
            Colormap::Blues => {
                let r = 1.0 - t * 0.8;
                let g = 1.0 - t * 0.5;
                let b = 1.0;
                vec4(r as f32, g as f32, b as f32, 1.0)
            }
            Colormap::Greens => {
                let r = 1.0 - t * 0.75;
                let g = 1.0 - t * 0.15;
                let b = 1.0 - t * 0.7;
                vec4(r as f32, g as f32, b as f32, 1.0)
            }
            Colormap::Oranges => {
                let r = 1.0;
                let g = 1.0 - t * 0.6;
                let b = 1.0 - t * 0.85;
                vec4(r as f32, g as f32, b as f32, 1.0)
            }
            Colormap::Reds => {
                let r = 1.0;
                let g = 1.0 - t * 0.85;
                let b = 1.0 - t * 0.85;
                vec4(r as f32, g as f32, b as f32, 1.0)
            }
            Colormap::Greys => {
                let v = 1.0 - t * 0.9;
                vec4(v as f32, v as f32, v as f32, 1.0)
            }
            Colormap::Jet => {
                // Classic rainbow: blue-cyan-green-yellow-red
                let (r, g, b) = if t < 0.125 {
                    (0.0, 0.0, 0.5 + t * 4.0)
                } else if t < 0.375 {
                    let s = (t - 0.125) / 0.25;
                    (0.0, s, 1.0)
                } else if t < 0.625 {
                    let s = (t - 0.375) / 0.25;
                    (s, 1.0, 1.0 - s)
                } else if t < 0.875 {
                    let s = (t - 0.625) / 0.25;
                    (1.0, 1.0 - s, 0.0)
                } else {
                    let s = (t - 0.875) / 0.125;
                    (1.0 - s * 0.5, 0.0, 0.0)
                };
                vec4(r as f32, g as f32, b as f32, 1.0)
            }
            Colormap::Hot => {
                // Black-red-yellow-white
                let (r, g, b) = if t < 0.33 {
                    (t * 3.0, 0.0, 0.0)
                } else if t < 0.67 {
                    let s = (t - 0.33) / 0.34;
                    (1.0, s, 0.0)
                } else {
                    let s = (t - 0.67) / 0.33;
                    (1.0, 1.0, s)
                };
                vec4(r as f32, g as f32, b as f32, 1.0)
            }
            Colormap::Turbo => {
                // Improved rainbow with better perceptual uniformity
                let r = 0.13572 + t * (4.6153 + t * (-42.66 + t * (132.13 + t * (-152.95 + t * 56.31))));
                let g = 0.09140 + t * (2.1745 + t * (4.8321 + t * (-36.60 + t * (43.05 + t * (-13.22)))));
                let b = 0.10667 + t * (12.755 + t * (-60.58 + t * (109.33 + t * (-87.15 + t * 25.25))));
                vec4(r.clamp(0.0, 1.0) as f32, g.clamp(0.0, 1.0) as f32, b.clamp(0.0, 1.0) as f32, 1.0)
            }
            Colormap::Custom(stops) => {
                if stops.is_empty() {
                    return vec4(0.5, 0.5, 0.5, 1.0);
                }
                if stops.len() == 1 {
                    return stops[0].1;
                }
                // Find surrounding stops and interpolate
                for i in 0..stops.len() - 1 {
                    if t <= stops[i + 1].0 {
                        let t0 = stops[i].0;
                        let t1 = stops[i + 1].0;
                        let c0 = stops[i].1;
                        let c1 = stops[i + 1].1;
                        let s = if t1 > t0 { (t - t0) / (t1 - t0) } else { 0.0 };
                        return vec4(
                            c0.x + (c1.x - c0.x) * s as f32,
                            c0.y + (c1.y - c0.y) * s as f32,
                            c0.z + (c1.z - c0.z) * s as f32,
                            c0.w + (c1.w - c0.w) * s as f32,
                        );
                    }
                }
                stops.last().unwrap().1
            }
        }
    }

    /// Get a list of all named colormaps
    pub fn all_named() -> Vec<Colormap> {
        vec![
            Colormap::Viridis,
            Colormap::Plasma,
            Colormap::Inferno,
            Colormap::Magma,
            Colormap::Cividis,
            Colormap::Coolwarm,
            Colormap::RdBu,
            Colormap::Spectral,
            Colormap::Blues,
            Colormap::Greens,
            Colormap::Oranges,
            Colormap::Reds,
            Colormap::Greys,
            Colormap::Jet,
            Colormap::Hot,
            Colormap::Turbo,
        ]
    }

    /// Get the name of this colormap
    pub fn name(&self) -> &'static str {
        match self {
            Colormap::Viridis => "Viridis",
            Colormap::Plasma => "Plasma",
            Colormap::Inferno => "Inferno",
            Colormap::Magma => "Magma",
            Colormap::Cividis => "Cividis",
            Colormap::Coolwarm => "Coolwarm",
            Colormap::RdBu => "RdBu",
            Colormap::Spectral => "Spectral",
            Colormap::Blues => "Blues",
            Colormap::Greens => "Greens",
            Colormap::Oranges => "Oranges",
            Colormap::Reds => "Reds",
            Colormap::Greys => "Greys",
            Colormap::Jet => "Jet",
            Colormap::Hot => "Hot",
            Colormap::Turbo => "Turbo",
            Colormap::Custom(_) => "Custom",
        }
    }

    /// Create a custom colormap from color stops
    /// Stops should be (position, color) pairs where position is 0.0 to 1.0
    pub fn custom(stops: Vec<(f64, Vec4)>) -> Self {
        Colormap::Custom(stops)
    }
}

// =============================================================================
// Normalization for Colormaps
// =============================================================================

/// Linear normalization: maps [vmin, vmax] to [0, 1]
#[derive(Clone, Debug)]
pub struct Normalize {
    pub vmin: f64,
    pub vmax: f64,
    pub clip: bool,
}

impl Normalize {
    pub fn new(vmin: f64, vmax: f64) -> Self {
        Self { vmin, vmax, clip: true }
    }

    pub fn with_clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// Normalize a value to [0, 1]
    pub fn normalize(&self, value: f64) -> f64 {
        if self.vmax == self.vmin {
            return 0.5;
        }
        let t = (value - self.vmin) / (self.vmax - self.vmin);
        if self.clip { t.clamp(0.0, 1.0) } else { t }
    }

    /// Inverse: convert [0, 1] back to original scale
    pub fn inverse(&self, t: f64) -> f64 {
        self.vmin + t * (self.vmax - self.vmin)
    }
}

impl Default for Normalize {
    fn default() -> Self {
        Self { vmin: 0.0, vmax: 1.0, clip: true }
    }
}

/// Logarithmic normalization: maps [vmin, vmax] to [0, 1] on log scale
/// Values must be positive
#[derive(Clone, Debug)]
pub struct LogNorm {
    pub vmin: f64,
    pub vmax: f64,
    pub clip: bool,
    log_vmin: f64,
    log_vmax: f64,
}

impl LogNorm {
    pub fn new(vmin: f64, vmax: f64) -> Self {
        let vmin = vmin.max(1e-10);  // Ensure positive
        let vmax = vmax.max(vmin + 1e-10);
        Self {
            vmin,
            vmax,
            clip: true,
            log_vmin: vmin.log10(),
            log_vmax: vmax.log10(),
        }
    }

    pub fn with_clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// Normalize a value to [0, 1] on log scale
    pub fn normalize(&self, value: f64) -> f64 {
        if value <= 0.0 {
            return 0.0;
        }
        let log_val = value.log10();
        let t = (log_val - self.log_vmin) / (self.log_vmax - self.log_vmin);
        if self.clip { t.clamp(0.0, 1.0) } else { t }
    }

    /// Inverse: convert [0, 1] back to original scale
    pub fn inverse(&self, t: f64) -> f64 {
        10.0_f64.powf(self.log_vmin + t * (self.log_vmax - self.log_vmin))
    }
}

impl Default for LogNorm {
    fn default() -> Self {
        Self::new(1.0, 10.0)
    }
}

/// Symmetric log normalization: handles negative values
#[derive(Clone, Debug)]
pub struct SymLogNorm {
    pub vmin: f64,
    pub vmax: f64,
    pub linthresh: f64,  // Linear threshold
    pub clip: bool,
}

impl SymLogNorm {
    pub fn new(vmin: f64, vmax: f64, linthresh: f64) -> Self {
        Self {
            vmin,
            vmax,
            linthresh: linthresh.abs().max(1e-10),
            clip: true,
        }
    }

    pub fn with_clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    fn transform(&self, value: f64) -> f64 {
        if value.abs() <= self.linthresh {
            value / self.linthresh
        } else {
            let sign = if value >= 0.0 { 1.0 } else { -1.0 };
            sign * (1.0 + (value.abs() / self.linthresh).log10())
        }
    }

    /// Normalize a value to [0, 1]
    pub fn normalize(&self, value: f64) -> f64 {
        let t_val = self.transform(value);
        let t_min = self.transform(self.vmin);
        let t_max = self.transform(self.vmax);

        if t_max == t_min {
            return 0.5;
        }

        let t = (t_val - t_min) / (t_max - t_min);
        if self.clip { t.clamp(0.0, 1.0) } else { t }
    }
}

impl Default for SymLogNorm {
    fn default() -> Self {
        Self::new(-10.0, 10.0, 1.0)
    }
}

// =============================================================================
// Deterministic pseudo-random demo data helpers (shared by chart demo data)
// =============================================================================

/// Simple xorshift PRNG for reproducible demo data
pub struct DemoRng(pub u64);

impl DemoRng {
    pub fn new(seed: u64) -> Self {
        Self(if seed == 0 { 0x9E3779B97F4A7C15 } else { seed })
    }
    pub fn next_f64(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 as f64) / (u64::MAX as f64)
    }
}
