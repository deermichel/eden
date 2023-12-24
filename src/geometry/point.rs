use crate::geometry::vector::Vector;
use num_traits::Float;

/// Abstract point in N-dimensional space.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point<T: Float, const N: usize> {
    /// Position vector.
    position: Vector<T, N>,
}

impl<T: Float> Point<T, 3> {
    /// Creates 3-dim point from coordinates.
    pub fn new(x: T, y: T, z: T) -> Self {
        Point { position: Vector::new(x, y, z) }
    }
}

impl<T: Float, const N: usize> std::ops::Add<Vector<T, N>> for Point<T, N> {
    type Output = Point<T, N>;

    /// Point + Vector -> Point.
    fn add(self, rhs: Vector<T, N>) -> Self::Output {
        Point { position: self.position + rhs }
    }
}

impl<T: Float, const N: usize> std::ops::Sub for Point<T, N> {
    type Output = Vector<T, N>;

    /// Point - Point -> Vector.
    fn sub(self, rhs: Point<T, N>) -> Self::Output {
        self.position - rhs.position
    }
}

impl<T: Float, const N: usize> std::ops::Sub<Vector<T, N>> for Point<T, N> {
    type Output = Point<T, N>;

    /// Point - Vector -> Point.
    fn sub(self, rhs: Vector<T, N>) -> Self::Output {
        Point { position: self.position - rhs }
    }
}

/// 3-dim point represented by single precision floats.
pub type Point3f = Point<f32, 3>;

/// Unit tests.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::vector::Vector3f;

    #[test]
    fn operators() {
        let a = Point3f::new(1.0, 2.0, 3.0);
        let b = Point3f::new(4.0, 2.0, 1.0);
        let ab = b - a;
        assert_eq!(ab, Vector3f::new(3.0, 0.0, -2.0));
        assert_eq!(a + ab, b);
        assert_eq!(b - ab, a);
    }
}
