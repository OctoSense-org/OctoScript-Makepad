//! Scale utility functions

/// Calculate a "nice" step size for tick generation
///
/// Returns a step size that produces clean tick values (1, 2, 5, 10, 20, 50, etc.)
///
/// # Example
/// ```
/// use makepad_d3::scale::nice_step;
///
/// let step = nice_step(100.0, 10);
/// assert_eq!(step, 10.0);
///
/// let step = nice_step(73.0, 5);
/// assert_eq!(step, 20.0);
/// ```
pub fn nice_step(span: f64, target_count: usize) -> f64 {
    if span.abs() < f64::EPSILON || target_count == 0 {
        return 1.0;
    }

    let raw_step = span / target_count as f64;
    let magnitude = 10.0_f64.powf(raw_step.abs().log10().floor());
    let normalized = raw_step / magnitude;

    let nice_normalized = if normalized <= 1.0 {
        1.0
    } else if normalized <= 2.0 {
        2.0
    } else if normalized <= 5.0 {
        5.0
    } else {
        10.0
    };

    nice_normalized * magnitude
}

/// Calculate "nice" bounds for a domain
///
/// Extends the domain to round values that are easy to read.
///
/// # Example
/// ```
/// use makepad_d3::scale::nice_bounds;
///
/// let (min, max) = nice_bounds(3.2, 97.8);
/// assert_eq!(min, 0.0);
/// assert_eq!(max, 100.0);
/// ```
pub fn nice_bounds(min: f64, max: f64) -> (f64, f64) {
    if (max - min).abs() < f64::EPSILON {
        return (min - 1.0, max + 1.0);
    }

    let span = max - min;
    let step = nice_step(span, 10);

    let nice_min = (min / step).floor() * step;
    let nice_max = (max / step).ceil() * step;

    (nice_min, nice_max)
}

/// Format a number for display
///
/// Automatically selects appropriate precision based on value.
pub fn format_number(value: f64) -> String {
    if !value.is_finite() {
        return value.to_string();
    }

    let abs = value.abs();

    if abs == 0.0 {
        return "0".to_string();
    }

    // Very large or very small numbers use scientific notation
    if abs >= 1e9 || abs < 1e-4 {
        return format!("{:.2e}", value);
    }

    // Determine decimal places based on magnitude
    let decimals = if abs >= 1000.0 {
        0
    } else if abs >= 100.0 {
        1
    } else if abs >= 10.0 {
        1
    } else if abs >= 1.0 {
        2
    } else if abs >= 0.1 {
        2
    } else {
        3
    };

    // Format and trim trailing zeros
    let formatted = format!("{:.1$}", value, decimals);
    trim_trailing_zeros(&formatted)
}

/// Trim trailing zeros from a formatted number
fn trim_trailing_zeros(s: &str) -> String {
    if !s.contains('.') {
        return s.to_string();
    }

    let trimmed = s.trim_end_matches('0').trim_end_matches('.');
    trimmed.to_string()
}

/// Linear interpolation between two values
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Inverse linear interpolation
pub fn unlerp(a: f64, b: f64, x: f64) -> f64 {
    if (b - a).abs() < f64::EPSILON {
        0.5
    } else {
        (x - a) / (b - a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nice_step() {
        assert_eq!(nice_step(100.0, 10), 10.0);
        assert_eq!(nice_step(100.0, 5), 20.0);
        assert_eq!(nice_step(1000.0, 10), 100.0);
        assert_eq!(nice_step(7.5, 10), 1.0);
    }

    #[test]
    fn test_nice_step_edge_cases() {
        assert_eq!(nice_step(0.0, 10), 1.0);
        assert_eq!(nice_step(100.0, 0), 1.0);
    }

    #[test]
    fn test_nice_bounds() {
        let (min, max) = nice_bounds(3.2, 97.8);
        assert_eq!(min, 0.0);
        assert_eq!(max, 100.0);
    }

    #[test]
    fn test_nice_bounds_small_range() {
        let (min, max) = nice_bounds(5.0, 5.0);
        assert_eq!(min, 4.0);
        assert_eq!(max, 6.0);
    }

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(0.0), "0");
        assert_eq!(format_number(100.0), "100");
        assert_eq!(format_number(1234.0), "1234");
        assert_eq!(format_number(12.5), "12.5");
        assert_eq!(format_number(0.123), "0.12");
    }

    #[test]
    fn test_format_number_scientific() {
        let formatted = format_number(1e10);
        assert!(formatted.contains("e"));

        let formatted = format_number(1e-5);
        assert!(formatted.contains("e"));
    }

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0.0, 100.0, 0.5), 50.0);
        assert_eq!(lerp(0.0, 100.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 100.0, 1.0), 100.0);
    }

    #[test]
    fn test_unlerp() {
        assert_eq!(unlerp(0.0, 100.0, 50.0), 0.5);
        assert_eq!(unlerp(0.0, 100.0, 0.0), 0.0);
        assert_eq!(unlerp(0.0, 100.0, 100.0), 1.0);
    }

    #[test]
    fn test_unlerp_same_values() {
        assert_eq!(unlerp(50.0, 50.0, 50.0), 0.5);
    }
}
