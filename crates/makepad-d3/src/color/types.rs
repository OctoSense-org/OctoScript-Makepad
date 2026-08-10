//! Color types and utilities for visualization
//!
//! Provides color representations and conversions used by color scales.

use serde::{Deserialize, Serialize};

/// RGBA color with f32 components (0.0 to 1.0)
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Rgba {
    /// Red component (0.0 to 1.0)
    pub r: f32,
    /// Green component (0.0 to 1.0)
    pub g: f32,
    /// Blue component (0.0 to 1.0)
    pub b: f32,
    /// Alpha component (0.0 to 1.0)
    pub a: f32,
}

impl Rgba {
    /// Create a new RGBA color
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Create an opaque RGB color
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Create from hex value (0xRRGGBB or 0xRRGGBBAA)
    pub fn from_hex(hex: u32) -> Self {
        if hex > 0xFFFFFF {
            // Has alpha
            Self {
                r: ((hex >> 24) & 0xFF) as f32 / 255.0,
                g: ((hex >> 16) & 0xFF) as f32 / 255.0,
                b: ((hex >> 8) & 0xFF) as f32 / 255.0,
                a: (hex & 0xFF) as f32 / 255.0,
            }
        } else {
            Self {
                r: ((hex >> 16) & 0xFF) as f32 / 255.0,
                g: ((hex >> 8) & 0xFF) as f32 / 255.0,
                b: (hex & 0xFF) as f32 / 255.0,
                a: 1.0,
            }
        }
    }

    /// Create from RGB bytes (0-255)
    pub fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        }
    }

    /// Create from RGBA bytes (0-255)
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    /// Convert to hex value (0xRRGGBB)
    pub fn to_hex(&self) -> u32 {
        let r = (self.r.clamp(0.0, 1.0) * 255.0).round() as u32;
        let g = (self.g.clamp(0.0, 1.0) * 255.0).round() as u32;
        let b = (self.b.clamp(0.0, 1.0) * 255.0).round() as u32;
        (r << 16) | (g << 8) | b
    }

    /// Convert to (r, g, b, a) tuple
    pub fn to_tuple(&self) -> (f32, f32, f32, f32) {
        (self.r, self.g, self.b, self.a)
    }

    /// Convert to [r, g, b, a] array
    pub fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    /// Linear interpolation between two colors
    pub fn lerp(&self, other: &Rgba, t: f32) -> Rgba {
        let t = t.clamp(0.0, 1.0);
        Rgba {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }

    /// Multiply color by a factor (for brightness adjustment)
    pub fn multiply(&self, factor: f32) -> Rgba {
        Rgba {
            r: (self.r * factor).clamp(0.0, 1.0),
            g: (self.g * factor).clamp(0.0, 1.0),
            b: (self.b * factor).clamp(0.0, 1.0),
            a: self.a,
        }
    }

    /// Set the alpha value
    pub fn with_alpha(mut self, alpha: f32) -> Self {
        self.a = alpha.clamp(0.0, 1.0);
        self
    }

    /// Common color constants
    pub const BLACK: Rgba = Rgba::rgb(0.0, 0.0, 0.0);
    /// White color
    pub const WHITE: Rgba = Rgba::rgb(1.0, 1.0, 1.0);
    /// Red color
    pub const RED: Rgba = Rgba::rgb(1.0, 0.0, 0.0);
    /// Green color
    pub const GREEN: Rgba = Rgba::rgb(0.0, 1.0, 0.0);
    /// Blue color
    pub const BLUE: Rgba = Rgba::rgb(0.0, 0.0, 1.0);
    /// Transparent color
    pub const TRANSPARENT: Rgba = Rgba::new(0.0, 0.0, 0.0, 0.0);
}

impl From<(f32, f32, f32)> for Rgba {
    fn from((r, g, b): (f32, f32, f32)) -> Self {
        Self::rgb(r, g, b)
    }
}

impl From<(f32, f32, f32, f32)> for Rgba {
    fn from((r, g, b, a): (f32, f32, f32, f32)) -> Self {
        Self::new(r, g, b, a)
    }
}

impl From<[f32; 3]> for Rgba {
    fn from([r, g, b]: [f32; 3]) -> Self {
        Self::rgb(r, g, b)
    }
}

impl From<[f32; 4]> for Rgba {
    fn from([r, g, b, a]: [f32; 4]) -> Self {
        Self::new(r, g, b, a)
    }
}

impl From<u32> for Rgba {
    fn from(hex: u32) -> Self {
        Self::from_hex(hex)
    }
}

/// HSL color representation
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Hsl {
    /// Hue (0.0 to 360.0)
    pub h: f32,
    /// Saturation (0.0 to 1.0)
    pub s: f32,
    /// Lightness (0.0 to 1.0)
    pub l: f32,
}

impl Hsl {
    /// Create a new HSL color
    pub fn new(h: f32, s: f32, l: f32) -> Self {
        Self {
            h: h % 360.0,
            s: s.clamp(0.0, 1.0),
            l: l.clamp(0.0, 1.0),
        }
    }

    /// Convert to RGBA
    pub fn to_rgba(&self) -> Rgba {
        if self.s == 0.0 {
            return Rgba::rgb(self.l, self.l, self.l);
        }

        let h = self.h / 360.0;
        let q = if self.l < 0.5 {
            self.l * (1.0 + self.s)
        } else {
            self.l + self.s - self.l * self.s
        };
        let p = 2.0 * self.l - q;

        let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
        let g = hue_to_rgb(p, q, h);
        let b = hue_to_rgb(p, q, h - 1.0 / 3.0);

        Rgba::rgb(r, g, b)
    }

    /// Create from RGBA
    pub fn from_rgba(rgba: &Rgba) -> Self {
        let r = rgba.r;
        let g = rgba.g;
        let b = rgba.b;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let l = (max + min) / 2.0;

        if max == min {
            return Hsl::new(0.0, 0.0, l);
        }

        let d = max - min;
        let s = if l > 0.5 {
            d / (2.0 - max - min)
        } else {
            d / (max + min)
        };

        let h = if max == r {
            ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0
        } else if max == g {
            ((b - r) / d + 2.0) / 6.0
        } else {
            ((r - g) / d + 4.0) / 6.0
        };

        Hsl::new(h * 360.0, s, l)
    }

    /// Rotate hue by given degrees
    pub fn rotate(&self, degrees: f32) -> Self {
        Hsl::new(self.h + degrees, self.s, self.l)
    }

    /// Adjust saturation
    pub fn saturate(&self, amount: f32) -> Self {
        Hsl::new(self.h, (self.s + amount).clamp(0.0, 1.0), self.l)
    }

    /// Adjust lightness
    pub fn lighten(&self, amount: f32) -> Self {
        Hsl::new(self.h, self.s, (self.l + amount).clamp(0.0, 1.0))
    }
}

/// Helper for HSL to RGB conversion
fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }

    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgba_from_hex() {
        let c = Rgba::from_hex(0xFF0000);
        assert!((c.r - 1.0).abs() < 0.01);
        assert!((c.g - 0.0).abs() < 0.01);
        assert!((c.b - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_rgba_to_hex() {
        let c = Rgba::rgb(1.0, 0.0, 0.0);
        assert_eq!(c.to_hex(), 0xFF0000);
    }

    #[test]
    fn test_rgba_lerp() {
        let black = Rgba::BLACK;
        let white = Rgba::WHITE;
        let mid = black.lerp(&white, 0.5);

        assert!((mid.r - 0.5).abs() < 0.01);
        assert!((mid.g - 0.5).abs() < 0.01);
        assert!((mid.b - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_hsl_to_rgba() {
        // Red
        let hsl = Hsl::new(0.0, 1.0, 0.5);
        let rgba = hsl.to_rgba();
        assert!((rgba.r - 1.0).abs() < 0.01);
        assert!((rgba.g - 0.0).abs() < 0.01);
        assert!((rgba.b - 0.0).abs() < 0.01);

        // Green
        let hsl = Hsl::new(120.0, 1.0, 0.5);
        let rgba = hsl.to_rgba();
        assert!((rgba.r - 0.0).abs() < 0.01);
        assert!((rgba.g - 1.0).abs() < 0.01);
        assert!((rgba.b - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_rgba_to_hsl_roundtrip() {
        let original = Rgba::from_hex(0x4285F4);
        let hsl = Hsl::from_rgba(&original);
        let back = hsl.to_rgba();

        assert!((original.r - back.r).abs() < 0.02);
        assert!((original.g - back.g).abs() < 0.02);
        assert!((original.b - back.b).abs() < 0.02);
    }
}
