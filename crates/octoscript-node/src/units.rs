//! Unit calculations shared by native L0 capability adapters.

/// Convert the supported unit pairs in either direction. Coefficients belong
/// to the runtime, never to model-authored card state. `gal` is a US gallon.
pub fn convert(amount: f64, from: &str, to: &str) -> Option<f64> {
    if !amount.is_finite() {
        return None;
    }
    let pairs = [
        ("km", "mi", 1000.0 / 1609.344, 0.0),
        ("m", "ft", 1.0 / 0.3048, 0.0),
        ("cm", "in", 1.0 / 2.54, 0.0),
        ("kg", "lb", 1.0 / 0.453_592_37, 0.0),
        ("g", "oz", 1.0 / 28.349_523_125, 0.0),
        ("l", "gal", 1.0 / 3.785_411_784, 0.0),
        ("km/h", "mph", 1000.0 / 1609.344, 0.0),
        ("c", "f", 1.8, 32.0),
    ];
    for (a, b, factor, offset) in pairs {
        let result = if from == a && to == b {
            amount * factor + offset
        } else if from == b && to == a {
            (amount - offset) / factor
        } else {
            continue;
        };
        return result.is_finite().then_some(result);
    }
    None
}

/// A temperature difference has a scale conversion but no absolute offset.
pub fn temperature(celsius: f64, unit: &str, difference: bool) -> Option<f64> {
    let result = match unit {
        "c" => celsius,
        "f" if difference => celsius * 1.8,
        "f" => celsius * 1.8 + 32.0,
        _ => return None,
    };
    (celsius.is_finite() && result.is_finite()).then_some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn near(actual: Option<f64>, expected: f64) {
        assert!((actual.unwrap() - expected).abs() < 1e-9);
    }

    #[test]
    fn conversion_pairs_and_reverse_preserve_physical_values() {
        for (from, to, amount, expected) in [
            ("km", "mi", 1.609344, 1.0),
            ("m", "ft", 0.3048, 1.0),
            ("cm", "in", 2.54, 1.0),
            ("kg", "lb", 0.45359237, 1.0),
            ("g", "oz", 28.349523125, 1.0),
            ("l", "gal", 3.785411784, 1.0),
            ("km/h", "mph", 1.609344, 1.0),
            ("c", "f", 20.0, 68.0),
            ("c", "f", -40.0, -40.0),
        ] {
            near(convert(amount, from, to), expected);
            near(convert(expected, to, from), amount);
        }
        near(convert(0.0, "c", "f"), 32.0);
    }

    #[test]
    fn unsupported_and_nonfinite_conversions_have_no_value() {
        for (amount, from, to) in [
            (1.0, "usd", "eur"), (1.0, "kg", "mi"),
            (f64::NAN, "km", "mi"), (f64::INFINITY, "c", "f"),
            (f64::MAX, "m", "ft"),
        ] {
            assert_eq!(convert(amount, from, to), None);
        }
    }

    #[test]
    fn temperature_differences_do_not_gain_a_fahrenheit_offset() {
        near(temperature(20.0, "f", false), 68.0);
        near(temperature(2.0, "f", true), 3.6);
        near(temperature(-2.0, "f", true), -3.6);
        near(temperature(0.0, "f", true), 0.0);
        near(temperature(2.0, "c", true), 2.0);
        assert_eq!(temperature(f64::NAN, "f", true), None);
        assert_eq!(temperature(2.0, "unknown", false), None);
    }
}
