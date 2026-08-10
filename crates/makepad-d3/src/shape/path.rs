//! Path segment types for shape generation
//!
//! Provides common path primitives used by all shape generators.

/// A 2D point/vector
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
}

impl Point {
    /// Create a new point
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Create a point at the origin
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Distance to another point
    pub fn distance(&self, other: &Point) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Linear interpolation between two points
    pub fn lerp(&self, other: &Point, t: f64) -> Point {
        Point {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }

    /// Add two points
    pub fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    /// Subtract two points
    pub fn sub(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    /// Scale a point
    pub fn scale(&self, factor: f64) -> Point {
        Point {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

impl From<(f64, f64)> for Point {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

impl From<Point> for (f64, f64) {
    fn from(p: Point) -> Self {
        (p.x, p.y)
    }
}

/// Path segment for shape drawing
#[derive(Clone, Debug, PartialEq)]
pub enum PathSegment {
    /// Move to a point (pen up, then position)
    MoveTo(Point),
    /// Draw a line to a point
    LineTo(Point),
    /// Draw a quadratic Bezier curve
    QuadTo {
        /// Control point
        cp: Point,
        /// End point
        end: Point,
    },
    /// Draw a cubic Bezier curve
    CurveTo {
        /// First control point
        cp1: Point,
        /// Second control point
        cp2: Point,
        /// End point
        end: Point,
    },
    /// Draw an arc
    ArcTo {
        /// Arc center
        center: Point,
        /// Arc radius
        radius: f64,
        /// Start angle in radians
        start_angle: f64,
        /// End angle in radians
        end_angle: f64,
        /// Whether the arc is counterclockwise
        counterclockwise: bool,
    },
    /// Close the current path
    ClosePath,
}

impl PathSegment {
    /// Create a move-to segment
    pub fn move_to(x: f64, y: f64) -> Self {
        Self::MoveTo(Point::new(x, y))
    }

    /// Create a line-to segment
    pub fn line_to(x: f64, y: f64) -> Self {
        Self::LineTo(Point::new(x, y))
    }

    /// Create a quadratic curve segment
    pub fn quad_to(cpx: f64, cpy: f64, x: f64, y: f64) -> Self {
        Self::QuadTo {
            cp: Point::new(cpx, cpy),
            end: Point::new(x, y),
        }
    }

    /// Create a cubic curve segment
    pub fn curve_to(cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) -> Self {
        Self::CurveTo {
            cp1: Point::new(cp1x, cp1y),
            cp2: Point::new(cp2x, cp2y),
            end: Point::new(x, y),
        }
    }

    /// Create an arc segment
    pub fn arc_to(
        cx: f64,
        cy: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        counterclockwise: bool,
    ) -> Self {
        Self::ArcTo {
            center: Point::new(cx, cy),
            radius,
            start_angle,
            end_angle,
            counterclockwise,
        }
    }

    /// Get the end point of this segment (if applicable)
    pub fn end_point(&self) -> Option<Point> {
        match self {
            Self::MoveTo(p) | Self::LineTo(p) => Some(*p),
            Self::QuadTo { end, .. } | Self::CurveTo { end, .. } => Some(*end),
            Self::ArcTo {
                center,
                radius,
                end_angle,
                ..
            } => Some(Point::new(
                center.x + radius * end_angle.cos(),
                center.y + radius * end_angle.sin(),
            )),
            Self::ClosePath => None,
        }
    }
}

/// A complete path consisting of multiple segments
#[derive(Clone, Debug, Default)]
pub struct Path {
    /// The segments making up this path
    pub segments: Vec<PathSegment>,
}

impl Path {
    /// Create a new empty path
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    /// Create a path with given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            segments: Vec::with_capacity(capacity),
        }
    }

    /// Add a segment to the path
    pub fn push(&mut self, segment: PathSegment) {
        self.segments.push(segment);
    }

    /// Extend with multiple segments
    pub fn extend(&mut self, segments: impl IntoIterator<Item = PathSegment>) {
        self.segments.extend(segments);
    }

    /// Move to a point
    pub fn move_to(&mut self, x: f64, y: f64) -> &mut Self {
        self.segments.push(PathSegment::move_to(x, y));
        self
    }

    /// Draw a line to a point
    pub fn line_to(&mut self, x: f64, y: f64) -> &mut Self {
        self.segments.push(PathSegment::line_to(x, y));
        self
    }

    /// Draw a quadratic curve
    pub fn quad_to(&mut self, cpx: f64, cpy: f64, x: f64, y: f64) -> &mut Self {
        self.segments.push(PathSegment::quad_to(cpx, cpy, x, y));
        self
    }

    /// Draw a cubic curve
    pub fn curve_to(
        &mut self,
        cp1x: f64,
        cp1y: f64,
        cp2x: f64,
        cp2y: f64,
        x: f64,
        y: f64,
    ) -> &mut Self {
        self.segments
            .push(PathSegment::curve_to(cp1x, cp1y, cp2x, cp2y, x, y));
        self
    }

    /// Close the path
    pub fn close(&mut self) -> &mut Self {
        self.segments.push(PathSegment::ClosePath);
        self
    }

    /// Check if the path is empty
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Get the number of segments
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    /// Get an iterator over segments
    pub fn iter(&self) -> impl Iterator<Item = &PathSegment> {
        self.segments.iter()
    }

    /// Convert to a vector of segments
    pub fn into_segments(self) -> Vec<PathSegment> {
        self.segments
    }
}

impl FromIterator<PathSegment> for Path {
    fn from_iter<T: IntoIterator<Item = PathSegment>>(iter: T) -> Self {
        Self {
            segments: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for Path {
    type Item = PathSegment;
    type IntoIter = std::vec::IntoIter<PathSegment>;

    fn into_iter(self) -> Self::IntoIter {
        self.segments.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_new() {
        let p = Point::new(3.0, 4.0);
        assert_eq!(p.x, 3.0);
        assert_eq!(p.y, 4.0);
    }

    #[test]
    fn test_point_distance() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        assert!((p1.distance(&p2) - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_point_lerp() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(10.0, 10.0);
        let mid = p1.lerp(&p2, 0.5);
        assert_eq!(mid, Point::new(5.0, 5.0));
    }

    #[test]
    fn test_path_builder() {
        let mut path = Path::new();
        path.move_to(0.0, 0.0)
            .line_to(100.0, 0.0)
            .line_to(100.0, 100.0)
            .close();

        assert_eq!(path.len(), 4);
    }

    #[test]
    fn test_path_segment_end_point() {
        let seg = PathSegment::line_to(50.0, 50.0);
        assert_eq!(seg.end_point(), Some(Point::new(50.0, 50.0)));

        let seg = PathSegment::ClosePath;
        assert_eq!(seg.end_point(), None);
    }
}
