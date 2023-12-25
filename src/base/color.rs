use crate::base::vector::Vector;
use num_traits::Float;

/// Abstract color with N components.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color<T: Float, const N: usize> {
    /// Color vector.
    values: Vector<T, N>,
}

impl<T: Float> Color<T, 3> {
    /// Creates RGB color from values.
    pub fn new(r: T, g: T, b: T) -> Self {
        Color {
            values: Vector::new(r, g, b),
        }
    }

    /// Black.
    pub fn black() -> Self {
        Color::new(T::zero(), T::zero(), T::zero())
    }

    /// White.
    pub fn white() -> Self {
        Color::new(T::one(), T::one(), T::one())
    }
}

/// Helper macro for binary operator overloading.
macro_rules! impl_binary_op {
    // $op_trait is something like 'Add'.
    // $op_fn is something like 'add'.
    // $op_symbol is something like '+'.
    ($op_trait:ident $op_fn:ident $op_symbol:tt) => {
        impl<T: Float, const N: usize> std::ops::$op_trait for Color<T, N> {
            type Output = Color<T, N>;

            // Color <op> Color -> Color.
            #[doc = concat!("Color ", stringify!($op_symbol), " Color -> Color.")]
            fn $op_fn(self, rhs: Self) -> Self::Output {
                Color { values: self.values $op_symbol rhs.values }
            }
        }

        impl<T: Float, const N: usize> std::ops::$op_trait<T> for Color<T, N> {
            type Output = Color<T, N>;

            // Color <op> Scalar -> Color.
            #[doc = concat!("Color ", stringify!($op_symbol), " Scalar -> Color.")]
            fn $op_fn(self, rhs: T) -> Self::Output {
                Color { values: self.values $op_symbol rhs }
            }
        }

        impl<const N: usize> std::ops::$op_trait<Color<f32, N>> for f32 {
            type Output = Color<f32, N>;

            // Scalar <op> Color -> Color.
            #[doc = concat!("Scalar ", stringify!($op_symbol), " Color -> Color.")]
            fn $op_fn(self, rhs: Color<f32, N>) -> Self::Output {
                Color { values: self $op_symbol rhs.values }
            }
        }

        impl<const N: usize> std::ops::$op_trait<Color<f64, N>> for f64 {
            type Output = Color<f64, N>;

            // Scalar <op> Color -> Color.
            #[doc = concat!("Scalar ", stringify!($op_symbol), " Color -> Color.")]
            fn $op_fn(self, rhs: Color<f64, N>) -> Self::Output {
                Color { values: self $op_symbol rhs.values }
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
        impl<T: Float, const N: usize> std::ops::$op_trait for Color<T, N> {
            type Output = Color<T, N>;

            // <op> Color -> Color.
            #[doc = concat!(stringify!($op_symbol), " Color -> Color.")]
            fn $op_fn(self) -> Self::Output {
                Color { values: $op_symbol self.values }
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
        impl<T: Float, const N: usize> std::ops::$op_trait for Color<T, N> {
            // Color <op>= Color -> Color.
            #[doc = concat!("Color ", stringify!($op_symbol), "= Color -> Color.")]
            fn $op_fn(&mut self, rhs: Self) {
                self.values = self.values $op_symbol rhs.values
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

/// RGB color represented by single precision floats.
pub type Color3f = Color<f32, 3>;

/// Unit tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operators() {
        let a = Color3f::new(1.0, 2.0, 3.0);
        let b = Color3f::new(2.0, 3.0, 4.0);
        let c = Color3f::new(3.0, 5.0, 7.0);
        let d = Color3f::new(2.0, 6.0, 12.0);
        let e = Color3f::new(0.0, -1.0, -2.0);
        let f = Color3f::new(6.0, 9.0, 12.0);
        let g = Color3f::new(6.0, 3.0, 2.0);
        let h = Color3f::new(-6.0, -3.0, -2.0);

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
}
