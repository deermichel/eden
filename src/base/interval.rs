use num_traits::Float;

/// Interval defined by start and end value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Interval<T: Float = f32> {
    /// Lower bound.
    start: T,

    /// Upper bound.
    end: T,
}

impl<T: Float> Interval<T> {
    /// Creates interval with start and end value.
    pub fn new(start: T, end: T) -> Self {
        Interval { start, end }
    }

    /// Whether interval contains value (start exclusive, end exclusive).
    pub fn contains(&self, x: T) -> bool {
        self.start < x && x < self.end
    }
}

/// Unit tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains() {
        let a = Interval::new(2.0, 5.0);
        assert_eq!(a.contains(0.0), false);
        assert_eq!(a.contains(2.0), false);
        assert_eq!(a.contains(2.1), true);
        assert_eq!(a.contains(4.9), true);
        assert_eq!(a.contains(5.0), false);
        assert_eq!(a.contains(7.0), false);
    }

    #[test]
    fn infinity() {
        let a = Interval::new(f64::NEG_INFINITY, f64::INFINITY);
        assert_eq!(a.contains(f64::NEG_INFINITY), false);
        assert_eq!(a.contains(f64::MIN), true);
        assert_eq!(a.contains(f64::MAX), true);
        assert_eq!(a.contains(f64::INFINITY), false);

        let b = Interval::new(f32::INFINITY, f32::NEG_INFINITY);
        assert_eq!(b.contains(f32::NEG_INFINITY), false);
        assert_eq!(b.contains(f32::MIN), false);
        assert_eq!(b.contains(f32::MAX), false);
        assert_eq!(b.contains(f32::INFINITY), false);
    }
}
