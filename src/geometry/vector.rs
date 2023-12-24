use num_traits::Float;

/// Abstract vector in N-dimensional space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector<T: Float, const N: usize> {
    /// Vector components.
    components: [T; N],
}

impl<T: Float, const N: usize> Vector<T, N> {
    /// Dot product with other vector.
    pub fn dot(&self, v: &Self) -> T {
        self.components
            .iter()
            .zip(v.components)
            .fold(T::zero(), |acc, (&a, b)| acc + (a * b))
    }

    /// Length of the vector.
    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }

    /// Squared length of the vector.
    pub fn length_squared(&self) -> T {
        self.dot(self)
    }

    /// Normalizes the vector to unit length.
    pub fn normalize(self) -> Self {
        self / self.length()
    }
}

impl<T: Float> Vector<T, 3> {
    /// Creates 3-dim vector from components.
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector {
            components: [x, y, z],
        }
    }
}

impl<T: Float, const N: usize> Default for Vector<T, N> {
    /// Default vector with all zeros.
    fn default() -> Self {
        Vector {
            components: [T::zero(); N],
        }
    }
}

/// Helper macro for binary operator overloading.
macro_rules! impl_binary_op {
    // $op_trait is something like 'Add'.
    // $op_fn is something like 'add'.
    // $op_symbol is something like '+'.
    ($op_trait:ident $op_fn:ident $op_symbol:tt) => {
        impl<T: Float, const N: usize> std::ops::$op_trait for Vector<T, N> {
            type Output = Vector<T, N>;

            // Vector <op> Vector -> Vector.
            #[doc = concat!("Vector ", stringify!($op_symbol), " Vector -> Vector.")]
            fn $op_fn(self, rhs: Self) -> Self::Output {
                let mut result = Vector::default();
                for i in 0..N {
                    result.components[i] = self.components[i] $op_symbol rhs.components[i];
                }
                result
            }
        }

        impl<T: Float, const N: usize> std::ops::$op_trait<T> for Vector<T, N> {
            type Output = Vector<T, N>;

            // Vector <op> Scalar -> Vector.
            #[doc = concat!("Vector ", stringify!($op_symbol), " Scalar -> Vector.")]
            fn $op_fn(self, rhs: T) -> Self::Output {
                self $op_symbol Vector { components: [rhs; N] }
            }
        }

        impl<const N: usize> std::ops::$op_trait<Vector<f32, N>> for f32 {
            type Output = Vector<f32, N>;

            // Scalar <op> Vector -> Vector.
            #[doc = concat!("Scalar ", stringify!($op_symbol), " Vector -> Vector.")]
            fn $op_fn(self, rhs: Vector<f32, N>) -> Self::Output {
                Vector { components: [self; N] } $op_symbol rhs
            }
        }

        impl<const N: usize> std::ops::$op_trait<Vector<f64, N>> for f64 {
            type Output = Vector<f64, N>;

            // Scalar <op> Vector -> Vector.
            #[doc = concat!("Scalar ", stringify!($op_symbol), " Vector -> Vector.")]
            fn $op_fn(self, rhs: Vector<f64, N>) -> Self::Output {
                Vector { components: [self; N] } $op_symbol rhs
            }
        }
    }
}

/// Helper macro for unary operator overloading.
macro_rules! impl_unary_op {
    // $op_trait is something like 'Neg'.
    // $op_fn is something like 'neg'.
    // $op_symbol is something like '-'.
    ($op_trait:ident $op_fn:ident $op_symbol:tt) => {
        impl<T: Float, const N: usize> std::ops::$op_trait for Vector<T, N> {
            type Output = Vector<T, N>;

            // <op> Vector -> Vector.
            #[doc = concat!(stringify!($op_symbol), " Vector -> Vector.")]
            fn $op_fn(self) -> Self::Output {
                let mut result = Vector::default();
                for i in 0..N {
                    result.components[i] = $op_symbol self.components[i];
                }
                result
            }
        }
    }
}

/// Helper macro for assign operator overloading.
macro_rules! impl_assign_op {
    // $op_trait is something like 'AddAssign'.
    // $op_fn is something like 'add_assign'.
    // $op_symbol is something like '+'.
    ($op_trait:ident $op_fn:ident $op_symbol:tt) => {
        impl<T: Float, const N: usize> std::ops::$op_trait for Vector<T, N> {
            // Vector <op>= Vector -> Vector.
            #[doc = concat!("Vector ", stringify!($op_symbol), "= Vector -> Vector.")]
            fn $op_fn(&mut self, rhs: Self) {
                for i in 0..N {
                    self.components[i] = self.components[i] $op_symbol rhs.components[i];
                }
            }
        }
    }
}

impl_binary_op!(Add add +);
impl_assign_op!(AddAssign add_assign +);

impl_binary_op!(Sub sub -);
impl_unary_op!(Neg neg -);
impl_assign_op!(SubAssign sub_assign -);

impl_binary_op!(Mul mul *);
impl_assign_op!(MulAssign mul_assign *);

impl_binary_op!(Div div /);
impl_assign_op!(DivAssign div_assign /);

/// 3-dim vector represented by single precision floats.
pub type Vector3f = Vector<f32, 3>;

/// Unit tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(Vector3f::default(), Vector3f::new(0.0, 0.0, 0.0));
        assert_eq!(
            Vector::<f64, 2>::default(),
            Vector {
                components: [0.0, 0.0]
            }
        );
    }

    #[test]
    fn operators() {
        let a = Vector3f::new(1.0, 2.0, 3.0);
        let b = Vector3f::new(2.0, 3.0, 4.0);
        let c = Vector3f::new(3.0, 5.0, 7.0);
        let d = Vector3f::new(2.0, 6.0, 12.0);
        let e = Vector3f::new(0.0, -1.0, -2.0);
        let f = Vector3f::new(6.0, 9.0, 12.0);
        let g = Vector3f::new(6.0, 3.0, 2.0);
        let h = Vector3f::new(-6.0, -3.0, -2.0);

        assert_eq!(a + b, c);
        assert_eq!(c - b, a);
        assert_eq!(a * b, d);
        assert_eq!(d / b, a);

        assert_eq!(a + 1.0, b);
        assert_eq!(1.0 + a, b);
        assert_eq!(b - 1.0, a);
        assert_eq!(1.0 - a, e);
        assert_eq!(b * 3.0, f);
        assert_eq!(3.0 * b, f);
        assert_eq!(f / 3.0, b);
        assert_eq!(6.0 / g, a);

        assert_eq!(-g, h);

        let mut v = b;
        v += a;
        assert_eq!(v, c);
        v -= b;
        assert_eq!(v, a);
        v *= b;
        assert_eq!(v, d);
        v /= b;
        assert_eq!(v, a);
    }

    #[test]
    fn geometry() {
        let a = Vector3f::new(3.0, 4.0, 5.0);
        assert_eq!(a.length_squared(), 50.0);
        assert_eq!(a.length(), 50.0.sqrt());

        let b = a.normalize();
        assert_eq!(b.length(), 1.0);
        
        let c = Vector3f::new(2.0, 1.0, 0.0);
        assert_eq!(a.dot(&c), 10.0);
        assert_eq!(c.dot(&a), 10.0);
    }
}
