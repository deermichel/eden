use num_traits::Float;
use rand::{distributions::uniform::SampleUniform, Rng};

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

    /// Reflects vector at the plane represented by the normal.
    pub fn reflect(self, normal: Self) -> Self {
        let two = T::one() + T::one();
        self - normal * two * self.dot(&normal)
    }

    /// Refracts (normalized!) vector at the plane represented by the normal with given refraction ratio.
    /// Returns none in case of total internal reflection.
    pub fn refract(self, normal: Self, etai_over_etat: T) -> Option<Self> {
        let cos_i = (-normal.dot(&self)).min(T::one());
        let sin2_i = T::one() - cos_i * cos_i;
        let sin2_t = (etai_over_etat * etai_over_etat) * sin2_i;

        // Total internal reflection.
        if sin2_t > T::one() {
            return None;
        }

        let cos_t = (T::one() - sin2_t).sqrt();
        let t = self * etai_over_etat + normal * (etai_over_etat * cos_i - cos_t);
        Some(t)
    }

    /// Whether vector is close to zero in all components.
    pub fn near_zero(&self) -> bool {
        self.components.iter().all(|x| x.abs() < T::epsilon())
    }
}

impl<T: Float + SampleUniform, const N: usize> Vector<T, N> {
    /// Generates random vector of unit length.
    pub fn random_unit_vector(rng: &mut impl Rng) -> Self {
        let mut result = Vector::default();
        for i in 0..N {
            result.components[i] = rng.gen_range(-T::one()..T::one());
        }
        result.normalize()
    }
}

impl<T: Float> Vector<T, 3> {
    /// Creates 3-dim vector from components.
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector {
            components: [x, y, z],
        }
    }

    /// X component.
    pub fn x(&self) -> T {
        self.components[0]
    }

    /// Y component.
    pub fn y(&self) -> T {
        self.components[1]
    }

    /// Z component.
    pub fn z(&self) -> T {
        self.components[2]
    }

    /// Cross product with other 3-dim vector.
    pub fn cross(&self, v: &Self) -> Self {
        let a = self.components;
        let b = v.components;
        Vector {
            components: [
                a[1] * b[2] - a[2] * b[1],
                a[2] * b[0] - a[0] * b[2],
                a[0] * b[1] - a[1] * b[0],
            ],
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
    use rand::{rngs::StdRng, SeedableRng};

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

        let d = Vector3f::new(3.0, -3.0, 1.0);
        let e = Vector3f::new(4.0, 9.0, 2.0);
        let f = Vector3f::new(-15.0, -2.0, 39.0);
        assert_eq!(d.cross(&e), f);
        assert_eq!(e.cross(&d), -f);
        assert_eq!(d.cross(&d), Vector3f::default());
    }

    #[test]
    fn reflect() {
        let a = Vector3f::default();
        let b = Vector3f::new(2.0, -2.0, 2.0);
        let c = Vector3f::new(2.0, 2.0, 2.0);
        let n = Vector3f::new(0.0, 1.0, 0.0);
        assert_eq!(a.reflect(n), a);
        assert_eq!(b.reflect(n), c);
        assert_eq!(c.reflect(n), b);
    }

    #[test]
    fn refract() {
        let a = Vector3f::new(f32::sqrt(3.0), -1.0, 0.0).normalize();
        let b = Vector3f::new(1.0, -1.0, 0.0).normalize();
        let n = Vector3f::new(0.0, 1.0, 0.0);
        assert_eq!(a.refract(n, 1.0), Some(a));
        assert_eq!(a.refract(n, f32::sqrt(2.0 / 3.0)), Some(b));
        assert!((b.refract(n, f32::sqrt(3.0 / 2.0)).unwrap() - a).near_zero());
        assert_eq!(a.refract(n, 2.0), None);
    }

    #[test]
    fn near_zero() {
        let a = Vector3f::default();
        assert_eq!(a.near_zero(), true);
        let b = Vector3f::new(f32::EPSILON, 0.0, 0.0);
        assert_eq!(b.near_zero(), false);
        let c = Vector3f::new(f32::EPSILON / 2.0, f32::EPSILON / 2.0, f32::EPSILON / 2.0);
        assert_eq!(c.near_zero(), true);
    }

    #[test]
    fn random() {
        let mut rng = StdRng::seed_from_u64(42);
        let a = Vector3f::random_unit_vector(&mut rng);
        let b = Vector::<f64, 4>::random_unit_vector(&mut rng);
        assert_eq!(a.length(), 1.0);
        assert_eq!(b.length(), 1.0);
    }
}
