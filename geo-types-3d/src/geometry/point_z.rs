use crate::{pointZ, CoordFloat, CoordNum, CoordZ};

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A single point in 3D space.
///
/// # Semantics
///
/// The _interior_ of the point is itself (a singleton set),
/// and its _boundary_ is empty. A point is _valid_ if and
/// only if the `CoordZ` is valid.
///
/// # Creating a PointZ
///
/// There are many ways to construct a point.
/// ```
/// use geo_types_3d::{coordZ, pointZ, PointZ};
///
/// let p1 = PointZ::new(0., 1., 2.);
///
/// let p2 = pointZ! { x: 1000.0, y: 2000.0, z: 3000.0 };
///
/// let p3: PointZ = (0., 1., 2.).into();
///
/// let c = coordZ! { x: 10., y: 20., z: 30. };
/// let p4: PointZ = c.into();
/// ```
///
/// See the `From` impl section for a complete list of conversions.
///
/// ## Coordinate order for geographic points
///
/// For geographic points, typically `x` corresponds to longitude, `y` to latitude, and `z` to altitude.
///
/// Geographic methods in the [`geo`](https://crates.io/crates/geo) crate expect this common
/// lon/lat/alt order, but different conventions exist in other coordinate systems,
/// notably EPSG:4326, which uses lat/lon ordering.
/// ```
/// use geo_types_3d::{coordZ, pointZ, PointZ};
///
/// let lon = 179.9;
/// let lat = 45.0;
/// let alt = 10.0;
/// let geographic_point = PointZ::new(lon, lat, alt);
/// ```
///
#[derive(Eq, PartialEq, Clone, Copy, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PointZ<T: CoordNum = f64>(pub CoordZ<T>);

impl<T: CoordNum> From<CoordZ<T>> for PointZ<T> {
    fn from(x: CoordZ<T>) -> Self {
        PointZ(x)
    }
}

impl<T: CoordNum> From<(T, T, T)> for PointZ<T> {
    fn from(coords: (T, T, T)) -> Self {
        PointZ::new(coords.0, coords.1, coords.2)
    }
}

impl<T: CoordNum> From<[T; 3]> for PointZ<T> {
    fn from(coords: [T; 3]) -> Self {
        PointZ::new(coords[0], coords[1], coords[2])
    }
}

impl<T: CoordNum> From<PointZ<T>> for (T, T, T) {
    fn from(point: PointZ<T>) -> Self {
        point.0.into()
    }
}

impl<T: CoordNum> From<PointZ<T>> for [T; 3] {
    fn from(point: PointZ<T>) -> Self {
        point.0.into()
    }
}

impl<T: CoordNum> PointZ<T> {
    /// Creates a new point.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let p = PointZ::new(1.234, 2.345, 3.456);
    ///
    /// assert_eq!(p.x(), 1.234);
    /// assert_eq!(p.y(), 2.345);
    /// assert_eq!(p.z(), 3.456);
    /// ```
    pub fn new(x: T, y: T, z: T) -> Self {
        pointZ! { x: x, y: y , z: z}
    }

    /// Returns the x/horizontal component of the point.
    ///
    /// Typically, `x` is the horizontal position, or longitude for geographic coordinates,
    /// but its interpretation can vary across coordinate systems.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let p = PointZ::new(1.234, 2.345, 3.456);
    ///
    /// assert_eq!(p.x(), 1.234);
    /// ```
    pub fn x(self) -> T {
        self.0.x
    }

    /// Sets the x/horizontal component of the point.
    ///
    /// Typically, `x` is the horizontal position, or longitude for geographic coordinates,
    /// but its interpretation can vary across coordinate systems.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let mut p = PointZ::new(1.234, 2.345, 3.456);
    /// p.set_x(9.876);
    ///
    /// assert_eq!(p.x(), 9.876);
    /// ```
    pub fn set_x(&mut self, x: T) -> &mut Self {
        self.0.x = x;
        self
    }

    /// Returns a mutable reference to the x/horizontal component of the point
    ///
    /// Typically, `x` is the horizontal position, or longitude for geographic coordinates,
    /// but its interpretation can vary across coordinate systems.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use geo_types_3d::PointZ;
    /// let mut p = PointZ::new(1.234, 2.345, 3.456);
    /// let mut p_x = p.x_mut();
    /// *p_x += 1.0;
    /// assert_relative_eq!(p.x(), 2.234);
    /// ```
    pub fn x_mut(&mut self) -> &mut T {
        &mut self.0.x
    }

    /// Returns the y/vertical component of the point.
    ///
    /// Typically, `y` is the vertical position, or latitude for geographic coordinates,
    /// but its interpretation can vary across coordinate systems.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let p = PointZ::new(1.234, 2.345, 3.456);
    ///
    /// assert_eq!(p.y(), 2.345);
    /// ```
    pub fn y(self) -> T {
        self.0.y
    }

    /// Sets the y/vertical component of the point.
    ///
    /// Typically, `y` is the vertical position, or latitude for geographic coordinates,
    /// but its interpretation can vary across coordinate systems.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let mut p = PointZ::new(1.234, 2.345, 3.456);
    /// p.set_y(9.876);
    ///
    /// assert_eq!(p.y(), 9.876);
    /// ```
    pub fn set_y(&mut self, y: T) -> &mut Self {
        self.0.y = y;
        self
    }

    /// Returns a mutable reference to the x/horizontal component of the point
    ///
    /// Typically, `y` is the vertical position, or latitude for geographic coordinates,
    /// but its interpretation can vary across coordinate systems.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use geo_types_3d::PointZ;
    /// let mut p = PointZ::new(1.234, 2.345, 3.456);
    /// let mut p_y = p.y_mut();
    /// *p_y += 1.0;
    /// assert_relative_eq!(p.y(), 3.345);
    /// ```
    pub fn y_mut(&mut self) -> &mut T {
        &mut self.0.y
    }

    /// Returns the z/height component of the point.
    ///
    /// Typically, `z` is the height position, or altitude for geographic coordinates,
    /// but its interpretation can vary across coordinate systems.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let p = PointZ::new(1.234, 2.345, 3.456);
    ///
    /// assert_eq!(p.z(), 3.456);
    /// ```
    pub fn z(self) -> T {
        self.0.z
    }

    /// Sets the z/height component of the point.
    ///
    /// Typically, `z` is the height position, or altitude for geographic coordinates,
    /// but its interpretation can vary across coordinate systems.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let mut p = PointZ::new(1.234, 2.345, 3.456);
    /// p.set_z(9.876);
    ///
    /// assert_eq!(p.z(), 9.876);
    /// ```
    pub fn set_z(&mut self, z: T) -> &mut Self {
        self.0.z = z;
        self
    }

    /// Returns a mutable reference to the x/horizontal component of the point
    ///
    /// Typically, `z` is the height position, or altitude for geographic coordinates,
    /// but its interpretation can vary across coordinate systems.
    ///
    /// # Examples
    ///
    /// ```
    /// use approx::assert_relative_eq;
    /// use geo_types_3d::PointZ;
    /// let mut p = PointZ::new(1.234, 2.345, 3.456);
    /// let mut p_z = p.z_mut();
    /// *p_z += 1.0;
    /// assert_relative_eq!(p.z(), 4.456);
    /// ```
    pub fn z_mut(&mut self) -> &mut T {
        &mut self.0.z
    }

    /// Returns a tuple that contains the x/horizontal & y/vertical & z/height component of the point.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let mut p = PointZ::new(1.234, 2.345, 3.456);
    /// let (x, y, z) = p.x_y_z();
    ///
    /// assert_eq!(z, 3.456);
    /// assert_eq!(x, 1.234);
    /// assert_eq!(y, 2.345);
    /// ```
    pub fn x_y_z(self) -> (T, T, T) {
        (self.0.x, self.0.y, self.0.z)
    }

    /// Returns the longitude/horizontal component of the point.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let p = PointZ::new(1.234, 2.345, 3.456);
    ///
    /// assert_eq!(p.x(), 1.234);
    /// ```
    #[deprecated = "use `PointZ::x` instead, it's less ambiguous"]
    pub fn lng(self) -> T {
        self.x()
    }

    /// Sets the longitude/horizontal component of the point.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let mut p = PointZ::new(1.234, 2.345, 3.456);
    /// #[allow(deprecated)]
    /// p.set_lng(9.876);
    ///
    /// assert_eq!(p.x(), 9.876);
    /// ```
    #[deprecated = "use `PointZ::set_x` instead, it's less ambiguous"]
    pub fn set_lng(&mut self, lng: T) -> &mut Self {
        self.set_x(lng)
    }

    /// Returns the latitude/vertical component of the point.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let p = PointZ::new(1.234, 2.345, 3.456);
    ///
    /// assert_eq!(p.y(), 2.345);
    /// ```
    #[deprecated = "use `PointZ::y` instead, it's less ambiguous"]
    pub fn lat(self) -> T {
        self.y()
    }

    /// Sets the latitude/vertical component of the point.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let mut p = PointZ::new(1.234, 2.345, 3.456);
    /// #[allow(deprecated)]
    /// p.set_lat(9.876);
    ///
    /// assert_eq!(p.y(), 9.876);
    /// ```
    #[deprecated = "use `PointZ::set_y` instead, it's less ambiguous"]
    pub fn set_lat(&mut self, lat: T) -> &mut Self {
        self.set_y(lat)
    }

    /// Sets the altitude/height component of the point.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let mut p = PointZ::new(1.234, 2.345, 3.456);
    /// #[allow(deprecated)]
    /// p.set_alt(9.876);
    ///
    /// assert_eq!(p.z(), 9.876);
    /// ```
    /// 
    /// 
    #[deprecated = "use `PointZ::set_z` instead, it's less ambiguous"]
    pub fn set_alt(&mut self, alt: T) -> &mut Self {
        self.set_z(alt)
    }

    /// Returns the altitude/height component of the point.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let p = PointZ::new(1.234, 2.345, 3.456);
    ///
    /// assert_eq!(p.z(), 3.456);
    /// ```
    #[deprecated = "use `PointZ::z` instead, it's less ambiguous"]
    pub fn alt(self) -> T {
        self.z()
    }

}

impl<T: CoordNum> PointZ<T> {
    /// Returns the dot product of the two points:
    /// `dot = x1 * x2 + y1 * y2 + z1 * z2`
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::{pointZ, PointZ};
    ///
    /// let point = pointZ! { x: 1.5, y: 0.5, z: 2.0 };
    /// let dot = point.dot(pointZ! { x: 2.0, y: 4.5, z: 1.0 });
    ///
    /// assert_eq!(dot, 7.25);
    /// ```
    pub fn dot(self, other: Self) -> T {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    /// Returns the cross product of 3 points. A positive value implies
    /// `self` → `point_b` → `point_c` is counter-clockwise, negative implies
    /// clockwise.
    ///
    /// # Note on Robustness
    ///
    /// This function is **not** robust against floating-point errors.
    /// The [`geo`](https://docs.rs/geo) crate
    /// offers robust predicates for standard numeric types using the
    /// [`Kernel`](https://docs.rs/geo/algorithm/kernels/trait.Kernel.html)
    /// trait, and these should be preferred if possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::pointZ;
    ///
    /// let point_a = pointZ! { x: 1., y: 2., z: 3.};
    /// let point_b = pointZ! { x: 3., y: 5., z: 7. };
    /// let point_c = pointZ! { x: 7., y: 12., z: 15. };
    ///
    /// let cross = point_a.cross_prod(point_b, point_c);
    ///
    /// assert_eq!(cross, 2.0);
    /// ```
    pub fn cross_prod(self, point_b: Self, point_c: Self) -> T {
        (point_b.x() - self.x()) * (point_c.y() - self.y())
            - (point_b.y() - self.y()) * (point_c.x() - self.x())
    }
}

impl<T: CoordFloat> PointZ<T> {
    /// Converts the (x,y,z) components of PointZ to degrees
    ///
    /// # Example
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let p = PointZ::new(1.234, 2.345, 3.456);
    /// let (x, y, z): (f32, f32, f32) = p.to_degrees().x_y_z();
    /// assert_eq!(x.round(), 71.0);
    /// assert_eq!(y.round(), 134.0);
    /// assert_eq!(z.round(), 198.0);
    /// ```
    pub fn to_degrees(self) -> Self {
        let (x, y, z) = self.x_y_z();
        let x = x.to_degrees();
        let y = y.to_degrees();
        let z = z.to_degrees();
        PointZ::new(x, y, z)
    }

    /// Converts the (x,y,z) components of PointZ to radians
    ///
    /// # Example
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let p = PointZ::new(180.0, 341.5, 115.0);
    /// let (x, y, z): (f32, f32, f32) = p.to_radians().x_y_z();
    /// assert_eq!(x.round(), 3.0);
    /// assert_eq!(y.round(), 6.0);
    /// assert_eq!(z.round(), 2.0);
    /// ```
    pub fn to_radians(self) -> Self {
        let (x, y, z) = self.x_y_z();
        let x = x.to_radians();
        let y = y.to_radians();
        let z = z.to_radians();
        PointZ::new(x, y, z)
    }
}

impl<T> Neg for PointZ<T>
where
    T: CoordNum + Neg<Output = T>,
{
    type Output = Self;

    /// Returns a point with the x, y and z components negated.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let p = -PointZ::new(-1.25, 2.5, 3.5);
    ///
    /// assert_eq!(p.x(), 1.25);
    /// assert_eq!(p.y(), -2.5);
    /// assert_eq!(p.z(), -3.5);
    /// ```
    fn neg(self) -> Self::Output {
        PointZ::from(-self.0)
    }
}

impl<T: CoordNum> Add for PointZ<T> {
    type Output = Self;

    /// Add a point to the given point.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let p = PointZ::new(1.25, 2.5, 3.5) + PointZ::new(1.5, 2.5, 3.5);
    ///
    /// assert_eq!(p.x(), 2.75);
    /// assert_eq!(p.y(), 5.0);
    /// assert_eq!(p.z(), 7.0);
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        PointZ::from(self.0 + rhs.0)
    }
}

impl<T: CoordNum> AddAssign for PointZ<T> {
    /// Add a point to the given point and assign it to the original point.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let mut p = PointZ::new(1.25, 2.5, 3.5);
    /// p += PointZ::new(1.5, 2.5, 3.5);
    ///
    /// assert_eq!(p.x(), 2.75);
    /// assert_eq!(p.y(), 5.0);
    /// assert_eq!(p.z(), 7.0);
    /// ```
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0 + rhs.0;
    }
}

impl<T: CoordNum> Sub for PointZ<T> {
    type Output = Self;

    /// Subtract a point from the given point.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let p = PointZ::new(1.25, 3.0, 4.0) - PointZ::new(1.5, 2.5, 1.0);
    ///
    /// assert_eq!(p.x(), -0.25);
    /// assert_eq!(p.y(), 0.5);
    /// assert_eq!(p.z(), 3.0);
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        PointZ::from(self.0 - rhs.0)
    }
}

impl<T: CoordNum> SubAssign for PointZ<T> {
    /// Subtract a point from the given point and assign it to the original point.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let mut p = PointZ::new(1.25, 2.5, 3.0);
    /// p -= PointZ::new(1.5, 2.5, 1.0);
    ///
    /// assert_eq!(p.x(), -0.25);
    /// assert_eq!(p.y(), 0.0);
    /// assert_eq!(p.z(), 2.0);
    /// ```
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0 - rhs.0;
    }
}

impl<T: CoordNum> Mul<T> for PointZ<T> {
    type Output = Self;

    /// Scaler multiplication of a point
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let p = PointZ::new(2.0, 3.0, 4.0) * 2.0;
    ///
    /// assert_eq!(p.x(), 4.0);
    /// assert_eq!(p.y(), 6.0);
    /// assert_eq!(p.z(), 8.0);
    /// ```
    fn mul(self, rhs: T) -> Self::Output {
        PointZ::from(self.0 * rhs)
    }
}

impl<T: CoordNum> MulAssign<T> for PointZ<T> {
    /// Scaler multiplication of a point in place
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let mut p = PointZ::new(2.0, 3.0, 4.0);
    /// p *= 2.0;
    ///
    /// assert_eq!(p.x(), 4.0);
    /// assert_eq!(p.y(), 6.0);
    /// assert_eq!(p.z(), 8.0);
    /// ```
    fn mul_assign(&mut self, rhs: T) {
        self.0 = self.0 * rhs
    }
}

impl<T: CoordNum> Div<T> for PointZ<T> {
    type Output = Self;

    /// Scaler division of a point
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let p = PointZ::new(2.0, 3.0, 4.0) / 2.0;
    ///
    /// assert_eq!(p.x(), 1.0);
    /// assert_eq!(p.y(), 1.5);
    /// assert_eq!(p.z(), 2.0);
    /// ```
    fn div(self, rhs: T) -> Self::Output {
        PointZ::from(self.0 / rhs)
    }
}

impl<T: CoordNum> DivAssign<T> for PointZ<T> {
    /// Scaler division of a point in place
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types_3d::PointZ;
    ///
    /// let mut p = PointZ::new(2.0, 3.0, 4.0);
    /// p /= 2.0;
    ///
    /// assert_eq!(p.x(), 1.0);
    /// assert_eq!(p.y(), 1.5);
    /// assert_eq!(p.z(), 2.0);
    /// ```
    fn div_assign(&mut self, rhs: T) {
        self.0 = self.0 / rhs
    }
}

#[cfg(any(feature = "approx", test))]
mod approx_integration {
    use super::*;
    use approx::{AbsDiffEq, RelativeEq, UlpsEq};

    impl<T> RelativeEq for PointZ<T>
    where
        T: CoordNum + RelativeEq<Epsilon = T>,
    {
        #[inline]
        fn default_max_relative() -> Self::Epsilon {
            T::default_max_relative()
        }

        /// Equality assertion within a relative limit.
        ///
        /// # Examples
        ///
        /// ```
        /// use geo_types_3d::PointZ;
        ///
        /// let a = PointZ::new(2.0, 3.0);
        /// let b = PointZ::new(2.0, 3.01);
        ///
        /// approx::assert_relative_eq!(a, b, max_relative=0.1)
        /// ```
        #[inline]
        fn relative_eq(
            &self,
            other: &Self,
            epsilon: Self::Epsilon,
            max_relative: Self::Epsilon,
        ) -> bool {
            self.0.relative_eq(&other.0, epsilon, max_relative)
        }
    }

    impl<T> AbsDiffEq for PointZ<T>
    where
        T: CoordNum + AbsDiffEq<Epsilon = T>,
    {
        type Epsilon = T::Epsilon;

        #[inline]
        fn default_epsilon() -> Self::Epsilon {
            T::default_epsilon()
        }

        /// Equality assertion with an absolute limit.
        ///
        /// # Examples
        ///
        /// ```
        /// use geo_types_3d::PointZ;
        ///
        /// let a = PointZ::new(2.0, 3.0);
        /// let b = PointZ::new(2.0, 3.0000001);
        ///
        /// approx::assert_relative_eq!(a, b, epsilon=0.1)
        /// ```
        #[inline]
        fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
            self.0.abs_diff_eq(&other.0, epsilon)
        }
    }

    impl<T> UlpsEq for PointZ<T>
    where
        T: CoordNum + UlpsEq<Epsilon = T>,
    {
        fn default_max_ulps() -> u32 {
            T::default_max_ulps()
        }

        fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
            self.0.ulps_eq(&other.0, epsilon, max_ulps)
        }
    }
}

#[cfg(feature = "rstar_0_8")]
// These are required for rstar RTree
impl<T> ::rstar_0_8::PointZ for PointZ<T>
where
    T: ::num_traits::Float + ::rstar_0_8::RTreeNum,
{
    type Scalar = T;

    const DIMENSIONS: usize = 3;

    fn generate(generator: impl Fn(usize) -> Self::Scalar) -> Self {
        PointZ::new(generator(0), generator(1), generator(2))
    }

    fn nth(&self, index: usize) -> Self::Scalar {
        match index {
            0 => self.0.x,
            1 => self.0.y,
            2 => self.0.z,
            _ => unreachable!(),
        }
    }
    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        match index {
            0 => &mut self.0.x,
            1 => &mut self.0.y,
            2 => &mut self.0.z,
            _ => unreachable!(),
        }
    }
}

#[cfg(feature = "rstar_0_9")]
impl<T> ::rstar_0_9::PointZ for PointZ<T>
where
    T: ::num_traits::Float + ::rstar_0_9::RTreeNum,
{
    type Scalar = T;

    const DIMENSIONS: usize = 3;

    fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self {
        PointZ::new(generator(0), generator(1), generator(2))
    }

    fn nth(&self, index: usize) -> Self::Scalar {
        match index {
            0 => self.0.x,
            1 => self.0.y,
            2 => self.0.z,
            _ => unreachable!(),
        }
    }
    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        match index {
            0 => &mut self.0.x,
            1 => &mut self.0.y,
            2 => &mut self.0.z,
            _ => unreachable!(),
        }
    }
}

#[cfg(feature = "rstar_0_10")]
impl<T> ::rstar_0_10::PointZ for PointZ<T>
where
    T: ::num_traits::Float + ::rstar_0_10::RTreeNum,
{
    type Scalar = T;

    const DIMENSIONS: usize = 3;

    fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self {
        PointZ::new(generator(0), generator(1), generator(2))
    }

    fn nth(&self, index: usize) -> Self::Scalar {
        match index {
            0 => self.0.x,
            1 => self.0.y,
            2 => self.0.z,
            _ => unreachable!(),
        }
    }
    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        match index {
            0 => &mut self.0.x,
            1 => &mut self.0.y,
            2 => &mut self.0.z,
            _ => unreachable!(),
        }
    }
}

#[cfg(feature = "rstar_0_11")]
impl<T> ::rstar_0_11::PointZ for PointZ<T>
where
    T: ::num_traits::Float + ::rstar_0_11::RTreeNum,
{
    type Scalar = T;

    const DIMENSIONS: usize = 3;

    fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self {
        PointZ::new(generator(0), generator(1), generator(2))
    }

    fn nth(&self, index: usize) -> Self::Scalar {
        match index {
            0 => self.0.x,
            1 => self.0.y,
            2 => self.0.z,
            _ => unreachable!(),
        }
    }
    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        match index {
            0 => &mut self.0.x,
            1 => &mut self.0.y,
            2 => &mut self.0.z,
            _ => unreachable!(),
        }
    }
}

#[cfg(feature = "rstar_0_12")]
impl<T> ::rstar_0_12::PointZ for PointZ<T>
where
    T: ::num_traits::Float + ::rstar_0_12::RTreeNum,
{
    type Scalar = T;

    const DIMENSIONS: usize = 3;

    fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self {
        PointZ::new(generator(0), generator(1), generator(2))
    }

    fn nth(&self, index: usize) -> Self::Scalar {
        match index {
            0 => self.0.x,
            1 => self.0.y,
            2 => self.0.z,
            _ => unreachable!(),
        }
    }
    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        match index {
            0 => &mut self.0.x,
            1 => &mut self.0.y,
            2 => &mut self.0.z,
            _ => unreachable!(),
        }
    }
}

impl<T: CoordNum> AsRef<CoordZ<T>> for PointZ<T> {
    fn as_ref(&self) -> &CoordZ<T> {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use approx::{AbsDiffEq, RelativeEq};

    #[test]
    fn test_abs_diff_eq() {
        let delta = 1e-6;
        let p = PointZ::new(1.0, 1.0, 1.0);

        let p_x = PointZ::new(1.0 - delta, 1.0, 1.0);
        assert!(p.abs_diff_eq(&p_x, 1e-2));
        assert!(p.abs_diff_ne(&p_x, 1e-12));

        let p_y = PointZ::new(1.0, 1.0 + delta, 1.0);
        assert!(p.abs_diff_eq(&p_y, 1e-2));
        assert!(p.abs_diff_ne(&p_y, 1e-12));

        let p_z = PointZ::new(1.0, 1.0, 1.0 + delta);
        assert!(p.abs_diff_eq(&p_z, 1e-2));
        assert!(p.abs_diff_ne(&p_z, 1e-12));

        let p_xy = PointZ::new(1.0 + delta, 1.0 - delta, 1.0);
        assert!(p.abs_diff_eq(&p_xy, 1e-2));
        assert!(p.abs_diff_ne(&p_xy, 1e-12));

        let p_xyz = PointZ::new(1.0 + delta, 1.0 - delta, 1.0 + delta);
        assert!(p.abs_diff_eq(&p_xyz, 1e-2));
        assert!(p.abs_diff_ne(&p_xyz, 1e-12));

        let p_inf = PointZ::new(f64::INFINITY, 1., 1.0);
        assert!(p.abs_diff_ne(&p_inf, 1e-2));
    }

    #[test]
    fn test_relative_eq() {
        let delta = 1e-6;
        let p = PointZ::new(1.0, 1.0, 1.0);

        let p_x = PointZ::new(1.0 - delta, 1.0, 1.0);
        assert!(p.relative_eq(&p_x, 1e-2, 1e-2));
        assert!(p.relative_ne(&p_x, 1e-12, 1e-12));

        let p_y = PointZ::new(1.0, 1.0 + delta, 1.0);
        assert!(p.relative_eq(&p_y, 1e-2, 1e-2));
        assert!(p.relative_ne(&p_y, 1e-12, 1e-12));

        let p_z = PointZ::new(1.0, 1.0, 1.0 + delta);
        assert!(p.relative_eq(&p_z, 1e-2, 1e-2));
        assert!(p.relative_ne(&p_z, 1e-12, 1e-12));

        let p_xy = PointZ::new(1.0 + delta, 1.0 - delta, 1.0);
        assert!(p.relative_eq(&p_xy, 1e-2, 1e-2));
        assert!(p.relative_ne(&p_xy, 1e-12, 1e-12));

        let p_xyz = PointZ::new(1.0 + delta, 1.0 - delta, 1.0 + delta);
        assert!(p.relative_eq(&p_xyz, 1e-2, 1e-2));
        assert!(p.relative_ne(&p_xyz, 1e-12, 1e-12));

        let p_inf = PointZ::new(f64::INFINITY, 1., 1.0);
        assert!(p.relative_ne(&p_inf, 1e-2, 1e-2));
    }
}
