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

/// 3-dim point represented by single precision floats.
pub type Point3f = Point<f32, 3>;
