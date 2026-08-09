//! Animation system for chart widgets
//!
//! Provides time-based animation with various easing functions.

/// Easing function types
#[derive(Clone, Copy, Debug, Default)]
pub enum EasingType {
    #[default]
    Linear,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuart,
    EaseOutQuart,
    EaseInOutQuart,
    EaseInSine,
    EaseOutSine,
    EaseInOutSine,
    EaseInExpo,
    EaseOutExpo,
    EaseInOutExpo,
    EaseInCirc,
    EaseOutCirc,
    EaseInOutCirc,
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,
    EaseInElastic,
    EaseOutElastic,
    EaseInOutElastic,
    EaseInBounce,
    EaseOutBounce,
    EaseInOutBounce,
}

impl EasingType {
    /// Apply easing function to progress value (0.0 to 1.0)
    pub fn apply(&self, t: f64) -> f64 {
        match self {
            EasingType::Linear => t,
            EasingType::EaseInQuad => t * t,
            EasingType::EaseOutQuad => 1.0 - (1.0 - t) * (1.0 - t),
            EasingType::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            EasingType::EaseInCubic => t * t * t,
            EasingType::EaseOutCubic => 1.0 - (1.0 - t).powi(3),
            EasingType::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            EasingType::EaseInQuart => t * t * t * t,
            EasingType::EaseOutQuart => 1.0 - (1.0 - t).powi(4),
            EasingType::EaseInOutQuart => {
                if t < 0.5 {
                    8.0 * t * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
                }
            }
            EasingType::EaseInSine => 1.0 - (t * std::f64::consts::FRAC_PI_2).cos(),
            EasingType::EaseOutSine => (t * std::f64::consts::FRAC_PI_2).sin(),
            EasingType::EaseInOutSine => -(((t * std::f64::consts::PI).cos() - 1.0) / 2.0),
            EasingType::EaseInExpo => {
                if t == 0.0 { 0.0 } else { 2.0_f64.powf(10.0 * t - 10.0) }
            }
            EasingType::EaseOutExpo => {
                if t == 1.0 { 1.0 } else { 1.0 - 2.0_f64.powf(-10.0 * t) }
            }
            EasingType::EaseInOutExpo => {
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else if t < 0.5 {
                    2.0_f64.powf(20.0 * t - 10.0) / 2.0
                } else {
                    (2.0 - 2.0_f64.powf(-20.0 * t + 10.0)) / 2.0
                }
            }
            EasingType::EaseInCirc => 1.0 - (1.0 - t * t).sqrt(),
            EasingType::EaseOutCirc => (1.0 - (t - 1.0).powi(2)).sqrt(),
            EasingType::EaseInOutCirc => {
                if t < 0.5 {
                    (1.0 - (1.0 - (2.0 * t).powi(2)).sqrt()) / 2.0
                } else {
                    ((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0) / 2.0
                }
            }
            EasingType::EaseInBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                c3 * t * t * t - c1 * t * t
            }
            EasingType::EaseOutBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
            }
            EasingType::EaseInOutBack => {
                let c1 = 1.70158;
                let c2 = c1 * 1.525;
                if t < 0.5 {
                    ((2.0 * t).powi(2) * ((c2 + 1.0) * 2.0 * t - c2)) / 2.0
                } else {
                    ((2.0 * t - 2.0).powi(2) * ((c2 + 1.0) * (t * 2.0 - 2.0) + c2) + 2.0) / 2.0
                }
            }
            EasingType::EaseInElastic => {
                let c4 = (2.0 * std::f64::consts::PI) / 3.0;
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    -2.0_f64.powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * c4).sin()
                }
            }
            EasingType::EaseOutElastic => {
                let c4 = (2.0 * std::f64::consts::PI) / 3.0;
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    2.0_f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
                }
            }
            EasingType::EaseInOutElastic => {
                let c5 = (2.0 * std::f64::consts::PI) / 4.5;
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else if t < 0.5 {
                    -(2.0_f64.powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * c5).sin()) / 2.0
                } else {
                    (2.0_f64.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * c5).sin()) / 2.0 + 1.0
                }
            }
            EasingType::EaseInBounce => 1.0 - Self::bounce_out(1.0 - t),
            EasingType::EaseOutBounce => Self::bounce_out(t),
            EasingType::EaseInOutBounce => {
                if t < 0.5 {
                    (1.0 - Self::bounce_out(1.0 - 2.0 * t)) / 2.0
                } else {
                    (1.0 + Self::bounce_out(2.0 * t - 1.0)) / 2.0
                }
            }
        }
    }

    fn bounce_out(t: f64) -> f64 {
        let n1 = 7.5625;
        let d1 = 2.75;

        if t < 1.0 / d1 {
            n1 * t * t
        } else if t < 2.0 / d1 {
            let t = t - 1.5 / d1;
            n1 * t * t + 0.75
        } else if t < 2.5 / d1 {
            let t = t - 2.25 / d1;
            n1 * t * t + 0.9375
        } else {
            let t = t - 2.625 / d1;
            n1 * t * t + 0.984375
        }
    }
}

/// Animation controller for time-based animations
#[derive(Clone, Debug, Default)]
pub struct ChartAnimator {
    /// Animation duration in milliseconds
    duration_ms: f64,
    /// Start time (in seconds since app start)
    start_time: f64,
    /// Current progress (0.0 to 1.0)
    progress: f64,
    /// Easing function to use
    easing: EasingType,
    /// Whether animation is currently running
    running: bool,
    /// Optional delay before animation starts (in milliseconds)
    delay_ms: f64,
}

impl ChartAnimator {
    /// Create a new animator with the given duration in milliseconds
    pub fn new(duration_ms: f64) -> Self {
        Self {
            duration_ms,
            start_time: 0.0,
            progress: 0.0,
            easing: EasingType::EaseOutCubic,
            running: false,
            delay_ms: 0.0,
        }
    }

    /// Set the easing function
    pub fn with_easing(mut self, easing: EasingType) -> Self {
        self.easing = easing;
        self
    }

    /// Set a delay before the animation starts
    pub fn with_delay(mut self, delay_ms: f64) -> Self {
        self.delay_ms = delay_ms;
        self
    }

    /// Start the animation
    pub fn start(&mut self, current_time: f64) {
        self.start_time = current_time + self.delay_ms / 1000.0;
        self.progress = 0.0;
        self.running = true;
    }

    /// Reset the animation to initial state
    pub fn reset(&mut self) {
        self.progress = 0.0;
        self.running = false;
    }

    /// Update the animation state, returns true if still animating
    pub fn update(&mut self, current_time: f64) -> bool {
        if !self.running {
            return false;
        }

        let elapsed = current_time - self.start_time;
        if elapsed < 0.0 {
            // Still in delay period
            self.progress = 0.0;
            return true;
        }

        let duration_sec = self.duration_ms / 1000.0;
        let linear_progress = (elapsed / duration_sec).clamp(0.0, 1.0);

        self.progress = self.easing.apply(linear_progress);

        if linear_progress >= 1.0 {
            self.running = false;
            self.progress = 1.0;
        }

        true
    }

    /// Get the current eased progress (0.0 to 1.0)
    pub fn get_progress(&self) -> f64 {
        self.progress
    }

    /// Check if animation is still running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Interpolate between two values based on current progress
    pub fn interpolate(&self, start: f64, end: f64) -> f64 {
        start + (end - start) * self.progress
    }
}

/// Helper function to lighten a color
pub fn lighten(color: makepad_widgets::Vec4, amount: f32) -> makepad_widgets::Vec4 {
    makepad_widgets::vec4(
        (color.x + amount).min(1.0),
        (color.y + amount).min(1.0),
        (color.z + amount).min(1.0),
        color.w,
    )
}

/// Helper function to darken a color
pub fn darken(color: makepad_widgets::Vec4, amount: f32) -> makepad_widgets::Vec4 {
    makepad_widgets::vec4(
        (color.x - amount).max(0.0),
        (color.y - amount).max(0.0),
        (color.z - amount).max(0.0),
        color.w,
    )
}

/// Get a color from a predefined palette
pub fn get_color(index: usize) -> makepad_widgets::Vec4 {
    let colors = [
        makepad_widgets::vec4(0.26, 0.52, 0.96, 1.0),  // Blue
        makepad_widgets::vec4(0.92, 0.26, 0.21, 1.0),  // Red
        makepad_widgets::vec4(0.20, 0.66, 0.33, 1.0),  // Green
        makepad_widgets::vec4(1.0, 0.76, 0.03, 1.0),   // Yellow
        makepad_widgets::vec4(0.61, 0.15, 0.69, 1.0),  // Purple
        makepad_widgets::vec4(0.10, 0.74, 0.61, 1.0),  // Teal
        makepad_widgets::vec4(0.95, 0.61, 0.07, 1.0),  // Orange
        makepad_widgets::vec4(0.56, 0.27, 0.68, 1.0),  // Violet
    ];
    colors[index % colors.len()]
}
