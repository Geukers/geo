use crate::{coordZ, CoordNum, PointZ};

/// A lightweight struct used to store coordinates on the 2-dimensional
/// Cartesian plane.
///
/// Unlike `Point` (which in the future may contain additional information such
/// as an envelope, a precision model, and spatial reference system
/// information), a `Coord` only contains ordinate values and accessor
/// methods.
///
/// This type implements the [vector space] operations:
/// [`Add`], [`Sub`], [`Neg`], [`Zero`],
/// [`Mul<T>`][`Mul`], and [`Div<T>`][`Div`] traits.
///
/// # Semantics
///
/// This type does not represent any geospatial primitive,
/// but is used in their definitions. The only requirement
/// is that the coordinates it contains are valid numbers
/// (for eg. not `f64::NAN`).
///
/// [vector space]: //en.wikipedia.org/wiki/Vector_space
#[derive(Eq, PartialEq, Clone, Copy, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CoordZ<T: CoordNum = f64> {
    /// Typically, `x` is the horizontal position, or longitude for geographic coordinates,
    /// but its interpretation can vary across coordinate systems.
    pub x: T,
    /// Typically, `y` is the vertical position, or latitude for geographic coordinates,
    /// but its interpretation can vary across coordinate systems.
    pub y: T,
    /// Typically, `z` is the elevation position, or altitude for geographic coordinates,
    /// but its interpretation can vary across coordinate systems.
    pub z: T,
}

#[deprecated(note = "Renamed to `geo_types::Coord` (or `geo::Coord`)")]
pub type Coordinate<T = f64> = CoordZ<T>;

impl<T: CoordNum> From<(T, T, T)> for CoordZ<T> {
    #[inline]
    fn from(coords: (T, T, T)) -> Self {
        coordZ! {
            x: coords.0,
            y: coords.1,
            z: coords.2,
        }
    }
}

impl<T: CoordNum> From<[T; 3]> for CoordZ<T> {
    #[inline]
    fn from(coords: [T; 3]) -> Self {
        coordZ! {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }
}

impl<T: CoordNum> From<PointZ<T>> for CoordZ<T> {
    #[inline]
    fn from(point: PointZ<T>) -> Self {
        coordZ! {
            x: point.x(),
            y: point.y(),
            z: point.z(),
        }
    }
}

impl<T: CoordNum> From<CoordZ<T>> for (T, T, T) {
    #[inline]
    fn from(coord: CoordZ<T>) -> Self {
        (coord.x, coord.y,  coord.z)
    }
}

impl<T: CoordNum> From<CoordZ<T>> for [T; 3] {
    #[inline]
    fn from(coord: CoordZ<T>) -> Self {
        [coord.x, coord.y, coord.z]
    }
}

impl<T: CoordNum> CoordZ<T> {
    /// Returns a tuple that contains the x/horizontal & y/vertical component of the coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types::coord;
    ///
    /// let c = coord! {
    ///     x: 40.02f64,
    ///     y: 116.34,
    /// };
    /// let (x, y) = c.x_y();
    ///
    /// assert_eq!(y, 116.34);
    /// assert_eq!(x, 40.02f64);
    /// ```
    #[inline]
    pub fn x_y_z(&self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }
}

use core::ops::{Add, Div, Mul, Neg, Sub};

/// Negate a coordinate.
///
/// # Examples
///
/// ```
/// use geo_types::coord;
///
/// let p = coord! { x: 1.25, y: 2.5 };
/// let q = -p;
///
/// assert_eq!(q.x, -p.x);
/// assert_eq!(q.y, -p.y);
/// ```
impl<T> Neg for CoordZ<T>
where
    T: CoordNum + Neg<Output = T>,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        coordZ! {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

/// Add two coordinates.
///
/// # Examples
///
/// ```
/// use geo_types::coord;
///
/// let p = coord! { x: 1.25, y: 2.5 };
/// let q = coord! { x: 1.5, y: 2.5 };
/// let sum = p + q;
///
/// assert_eq!(sum.x, 2.75);
/// assert_eq!(sum.y, 5.0);
/// ```
impl<T: CoordNum> Add for CoordZ<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        coordZ! {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

/// Subtract a coordinate from another.
///
/// # Examples
///
/// ```
/// use geo_types::coord;
///
/// let p = coord! { x: 1.5, y: 2.5 };
/// let q = coord! { x: 1.25, y: 2.5 };
/// let diff = p - q;
///
/// assert_eq!(diff.x, 0.25);
/// assert_eq!(diff.y, 0.);
/// ```
impl<T: CoordNum> Sub for CoordZ<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        coordZ! {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

/// Multiply coordinate wise by a scalar.
///
/// # Examples
///
/// ```
/// use geo_types::coord;
///
/// let p = coord! { x: 1.25, y: 2.5 };
/// let q = p * 4.;
///
/// assert_eq!(q.x, 5.0);
/// assert_eq!(q.y, 10.0);
/// ```
impl<T: CoordNum> Mul<T> for CoordZ<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self {
        coordZ! {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

/// Divide coordinate wise by a scalar.
///
/// # Examples
///
/// ```
/// use geo_types::coord;
///
/// let p = coord! { x: 5., y: 10. };
/// let q = p / 4.;
///
/// assert_eq!(q.x, 1.25);
/// assert_eq!(q.y, 2.5);
/// ```
impl<T: CoordNum> Div<T> for CoordZ<T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self {
        coordZ! {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

use num_traits::Zero;
/// Create a coordinate at the origin.
///
/// # Examples
///
/// ```
/// use geo_types::Coord;
/// use num_traits::Zero;
///
/// let p: Coord = Zero::zero();
///
/// assert_eq!(p.x, 0.);
/// assert_eq!(p.y, 0.);
/// ```
impl<T: CoordNum> CoordZ<T> {
    #[inline]
    pub fn zero() -> Self {
        coordZ! {
            x: T::zero(),
            y: T::zero(),
            z: T::zero()
        }
    }
}

impl<T: CoordNum> Zero for CoordZ<T> {
    #[inline]
    fn zero() -> Self {
        Self::zero()
    }
    #[inline]
    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

#[cfg(any(feature = "approx", test))]
mod approx_integration {
    use super::*;
    use approx::{AbsDiffEq, RelativeEq, UlpsEq};

    impl<T> AbsDiffEq for CoordZ<T>
    where
        T: CoordNum + AbsDiffEq<Epsilon = T>,
    {
        type Epsilon = T::Epsilon;

        #[inline]
        fn default_epsilon() -> T::Epsilon {
            T::default_epsilon()
        }

        #[inline]
        fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
            T::abs_diff_eq(&self.x, &other.x, epsilon) && T::abs_diff_eq(&self.y, &other.y, epsilon)
        }
    }

    impl<T> RelativeEq for CoordZ<T>
    where
        T: CoordNum + RelativeEq<Epsilon = T>,
    {
        #[inline]
        fn default_max_relative() -> T::Epsilon {
            T::default_max_relative()
        }

        #[inline]
        fn relative_eq(&self, other: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
            T::relative_eq(&self.x, &other.x, epsilon, max_relative)
                && T::relative_eq(&self.y, &other.y, epsilon, max_relative)
        }
    }

    impl<T> UlpsEq for CoordZ<T>
    where
        T: CoordNum + UlpsEq<Epsilon = T>,
    {
        #[inline]
        fn default_max_ulps() -> u32 {
            T::default_max_ulps()
        }

        #[inline]
        fn ulps_eq(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
            T::ulps_eq(&self.x, &other.x, epsilon, max_ulps)
                && T::ulps_eq(&self.y, &other.y, epsilon, max_ulps)
        }
    }
}

// #[cfg(feature = "rstar_0_8")]
// impl<T> ::rstar_0_8::Point for CoordZ<T>
// where
//     T: ::num_traits::Float + ::rstar_0_8::RTreeNum,
// {
//     type Scalar = T;

//     const DIMENSIONS: usize = 2;

//     #[inline]
//     fn generate(generator: impl Fn(usize) -> Self::Scalar) -> Self {
//         coord! {
//             x: generator(0),
//             y: generator(1),
//         }
//     }

//     #[inline]
//     fn nth(&self, index: usize) -> Self::Scalar {
//         match index {
//             0 => self.x,
//             1 => self.y,
//             _ => unreachable!(),
//         }
//     }

//     #[inline]
//     fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
//         match index {
//             0 => &mut self.x,
//             1 => &mut self.y,
//             _ => unreachable!(),
//         }
//     }
// }

// #[cfg(feature = "rstar_0_9")]
// impl<T> ::rstar_0_9::Point for CoordZ<T>
// where
//     T: ::num_traits::Float + ::rstar_0_9::RTreeNum,
// {
//     type Scalar = T;

//     const DIMENSIONS: usize = 2;

//     #[inline]
//     fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self {
//         coord! {
//             x: generator(0),
//             y: generator(1),
//         }
//     }

//     #[inline]
//     fn nth(&self, index: usize) -> Self::Scalar {
//         match index {
//             0 => self.x,
//             1 => self.y,
//             _ => unreachable!(),
//         }
//     }

//     #[inline]
//     fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
//         match index {
//             0 => &mut self.x,
//             1 => &mut self.y,
//             _ => unreachable!(),
//         }
//     }
// }

// #[cfg(feature = "rstar_0_10")]
// impl<T> ::rstar_0_10::Point for CoordZ<T>
// where
//     T: ::num_traits::Float + ::rstar_0_10::RTreeNum,
// {
//     type Scalar = T;

//     const DIMENSIONS: usize = 2;

//     #[inline]
//     fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self {
//         coord! {
//             x: generator(0),
//             y: generator(1),
//         }
//     }

//     #[inline]
//     fn nth(&self, index: usize) -> Self::Scalar {
//         match index {
//             0 => self.x,
//             1 => self.y,
//             _ => unreachable!(),
//         }
//     }

//     #[inline]
//     fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
//         match index {
//             0 => &mut self.x,
//             1 => &mut self.y,
//             _ => unreachable!(),
//         }
//     }
// }

// #[cfg(feature = "rstar_0_11")]
// impl<T> ::rstar_0_11::Point for CoordZ<T>
// where
//     T: ::num_traits::Float + ::rstar_0_11::RTreeNum,
// {
//     type Scalar = T;

//     const DIMENSIONS: usize = 2;

//     #[inline]
//     fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self {
//         coord! {
//             x: generator(0),
//             y: generator(1),
//         }
//     }

//     #[inline]
//     fn nth(&self, index: usize) -> Self::Scalar {
//         match index {
//             0 => self.x,
//             1 => self.y,
//             _ => unreachable!(),
//         }
//     }

//     #[inline]
//     fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
//         match index {
//             0 => &mut self.x,
//             1 => &mut self.y,
//             _ => unreachable!(),
//         }
//     }
// }

// #[cfg(feature = "rstar_0_12")]
// impl<T> ::rstar_0_12::Point for CoordZ<T>
// where
//     T: ::num_traits::Float + ::rstar_0_12::RTreeNum,
// {
//     type Scalar = T;

//     const DIMENSIONS: usize = 2;

//     #[inline]
//     fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self {
//         coordZ! {
//             x: generator(0),
//             y: generator(1),
//             z: generator(2)
//         }
//     }

//     #[inline]
//     fn nth(&self, index: usize) -> Self::Scalar {
//         match index {
//             0 => self.x,
//             1 => self.y,
//             _ => unreachable!(),
//         }
//     }

//     #[inline]
//     fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
//         match index {
//             0 => &mut self.x,
//             1 => &mut self.y,
//             _ => unreachable!(),
//         }
//     }
// }

impl<T: CoordNum> AsRef<CoordZ<T>> for CoordZ<T> {
    fn as_ref(&self) -> &CoordZ<T> {
        self
    }
}
