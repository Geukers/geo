pub(crate) mod coord_z;
pub(crate) mod geometry_collection;
pub(crate) mod line_z;
pub(crate) mod line_string_z;
pub(crate) mod multi_line_string_z;
pub(crate) mod multi_point_z;
pub(crate) mod multi_polygon_z;
pub(crate) mod point_z;
pub(crate) mod polygon;
pub(crate) mod triangle;

// re-export all the geometry variants:
#[allow(deprecated)]
pub use coord_z::{CoordZ, CoordinateZ};
use geo_types::Rect;
pub use geometry_collection::GeometryCollection;
pub use line_z::LineZ;
pub use line_string_z::LineStringZ;
pub use multi_line_string_z::MultiLineStringZ;
pub use multi_point_z::MultiPointZ;
pub use multi_polygon_z::MultiPolygonZ;
pub use point_z::PointZ;
pub use polygon::PolygonZ;


pub use geo_types::Point;
pub use geo_types::Line;
use geo_types::LineString;
use geo_types::MultiLineString;
use geo_types::MultiPoint;
use geo_types::MultiPolygon;
use geo_types::Polygon;
// pub use cube::Cube;
pub use triangle::Triangle;

use crate::{CoordNum, Error};

use core::any::type_name;
use core::convert::TryFrom;

/// An enum representing any possible geometry type.
///
/// All geometry variants ([`Point`], [`LineString`], etc.) can be converted to a `Geometry` using
/// [`Into::into`]. Conversely, [`TryFrom::try_from`] can be used to convert a [`Geometry`]
/// _back_ to one of it's specific enum members.
///
/// # Example
///
/// ```
/// use std::convert::TryFrom;
/// use geo_types::{Point, point, Geometry, GeometryCollection};
/// let p = point!(x: 1.0, y: 1.0);
/// let pe: Geometry = p.into();
/// let pn = Point::try_from(pe).unwrap();
/// ```
///
#[derive(Eq, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Geometry<T: CoordNum = f64> {
    Point(Point<T>),
    PointZ(PointZ<T>),
    Line(Line<T>),
    LineZ(LineZ<T>),
    LineString(LineString<T>),
    LineStringZ(LineStringZ<T>),
    Polygon(Polygon<T>),
    PolygonZ(PolygonZ<T>),
    MultiPoint(MultiPoint<T>),
    MultiPointZ(MultiPointZ<T>),
    MultiLineString(MultiLineString<T>),
    MultiLineStringZ(MultiLineStringZ<T>),
    MultiPolygon(MultiPolygon<T>),
    MultiPolygonZ(MultiPolygonZ<T>),
    GeometryCollection(GeometryCollection<T>),
    Rect(Rect<T>),
    // Triangle(Triangle<T>),
}

impl<T: CoordNum> From<PointZ<T>> for Geometry<T> {
    fn from(x: PointZ<T>) -> Self {
        Self::PointZ(x)
    }
}
impl<T: CoordNum> From<Line<T>> for Geometry<T> {
    fn from(x: Line<T>) -> Self {
        Self::Line(x)
    }
}
impl<T: CoordNum> From<LineString<T>> for Geometry<T> {
    fn from(x: LineString<T>) -> Self {
        Self::LineString(x)
    }
}
impl<T: CoordNum> From<Polygon<T>> for Geometry<T> {
    fn from(x: Polygon<T>) -> Self {
        Self::Polygon(x)
    }
}
impl<T: CoordNum> From<MultiPoint<T>> for Geometry<T> {
    fn from(x: MultiPoint<T>) -> Self {
        Self::MultiPoint(x)
    }
}
impl<T: CoordNum> From<MultiLineString<T>> for Geometry<T> {
    fn from(x: MultiLineString<T>) -> Self {
        Self::MultiLineString(x)
    }
}
impl<T: CoordNum> From<MultiPolygon<T>> for Geometry<T> {
    fn from(x: MultiPolygon<T>) -> Self {
        Self::MultiPolygon(x)
    }
}

impl<T: CoordNum> From<LineZ<T>> for Geometry<T> {
    fn from(x: LineZ<T>) -> Self {
        Self::LineZ(x)
    }
}
impl<T: CoordNum> From<LineStringZ<T>> for Geometry<T> {
    fn from(x: LineStringZ<T>) -> Self {
        Self::LineStringZ(x)
    }
}
impl<T: CoordNum> From<PolygonZ<T>> for Geometry<T> {
    fn from(x: PolygonZ<T>) -> Self {
        Self::PolygonZ(x)
    }
}
impl<T: CoordNum> From<MultiPointZ<T>> for Geometry<T> {
    fn from(x: MultiPointZ<T>) -> Self {
        Self::MultiPointZ(x)
    }
}
impl<T: CoordNum> From<MultiLineStringZ<T>> for Geometry<T> {
    fn from(x: MultiLineStringZ<T>) -> Self {
        Self::MultiLineStringZ(x)
    }
}
impl<T: CoordNum> From<MultiPolygonZ<T>> for Geometry<T> {
    fn from(x: MultiPolygonZ<T>) -> Self {
        Self::MultiPolygonZ(x)
    }
}
impl<T: CoordNum> From<GeometryCollection<T>> for Geometry<T> {
    fn from(x: GeometryCollection<T>) -> Self {
        Self::GeometryCollection(x)
    }
}

// Disabled until we remove the deprecated GeometryCollection::from(single_geom) impl.
// impl<T: CoordNum> From<Cube<T>> for Geometry<T> {
//     fn from(x: Cube<T>) -> Self {
//         Self::Cube(x)
//     }
// }

// impl<T: CoordNum> From<Triangle<T>> for Geometry<T> {
//     fn from(x: Triangle<T>) -> Self {
//         Self::Triangle(x)
//     }
// }

impl<T: CoordNum> Geometry<T> {
    /// If this Geometry is a Point, then return that, else None.
    ///
    /// # Examples
    ///
    /// ```
    /// use geo_types::*;
    /// use std::convert::TryInto;
    ///
    /// let g = Geometry::Point(Point::new(0., 0.));
    /// let p2: Point<f32> = g.try_into().unwrap();
    /// assert_eq!(p2, Point::new(0., 0.,));
    /// ```
    #[deprecated(
        note = "Will be removed in an upcoming version. Switch to std::convert::TryInto<Point>"
    )]
    pub fn into_point(self) -> Option<PointZ<T>> {
        if let Geometry::PointZ(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this Geometry is a LineString, then return that LineString, else None.
    #[deprecated(
        note = "Will be removed in an upcoming version. Switch to std::convert::TryInto<LineString>"
    )]
    pub fn into_line_string(self) -> Option<LineStringZ<T>> {
        if let Geometry::LineStringZ(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this Geometry is a Line, then return that Line, else None.
    #[deprecated(
        note = "Will be removed in an upcoming version. Switch to std::convert::TryInto<Line>"
    )]
    pub fn into_line(self) -> Option<Line<T>> {
        if let Geometry::Line(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this Geometry is a Polygon, then return that, else None.
    #[deprecated(
        note = "Will be removed in an upcoming version. Switch to std::convert::TryInto<Polygon>"
    )]
    pub fn into_polygon(self) -> Option<PolygonZ<T>> {
        if let Geometry::PolygonZ(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this Geometry is a MultiPoint, then return that, else None.
    #[deprecated(
        note = "Will be removed in an upcoming version. Switch to std::convert::TryInto<MultiPoint>"
    )]
    pub fn into_multi_point(self) -> Option<MultiPointZ<T>> {
        if let Geometry::MultiPointZ(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this Geometry is a MultiLineString, then return that, else None.
    #[deprecated(
        note = "Will be removed in an upcoming version. Switch to std::convert::TryInto<MultiLineString>"
    )]
    pub fn into_multi_line_string(self) -> Option<MultiLineStringZ<T>> {
        if let Geometry::MultiLineStringZ(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// If this Geometry is a MultiPolygon, then return that, else None.
    #[deprecated(
        note = "Will be removed in an upcoming version. Switch to std::convert::TryInto<MultiPolygon>"
    )]
    pub fn into_multi_polygon(self) -> Option<MultiPolygonZ<T>> {
        if let Geometry::MultiPolygonZ(x) = self {
            Some(x)
        } else {
            None
        }
    }
}

macro_rules! try_from_geometry_impl {
    ($($type: ident),+) => {
        $(
        /// Convert a Geometry enum into its inner type.
        ///
        /// Fails if the enum case does not match the type you are trying to convert it to.
        impl <T: CoordNum> TryFrom<Geometry<T>> for $type<T> {
            type Error = Error;

            fn try_from(geom: Geometry<T>) -> Result<Self, Self::Error> {
                match geom {
                    Geometry::$type(g) => Ok(g),
                    other => Err(Error::MismatchedGeometry {
                        expected: type_name::<$type<T>>(),
                        found: inner_type_name(other)
                    })
                }
            }
        }
        )+
    }
}

try_from_geometry_impl!(
    PointZ,
    Line,
    LineStringZ,
    PolygonZ,
    MultiPointZ,
    MultiLineStringZ,
    MultiPolygonZ
    // Disabled until we remove the deprecated GeometryCollection::from(single_geom) impl.
    // GeometryCollection,
    // Cube,
    // Triangle
);

fn inner_type_name<T>(geometry: Geometry<T>) -> &'static str
where
    T: CoordNum,
{
    match geometry {
        Geometry::Point(_) => type_name::<Point<T>>(),
        Geometry::PointZ(_) => type_name::<PointZ<T>>(),
        Geometry::Line(_) => type_name::<Line<T>>(),
        Geometry::LineString(_) => type_name::<LineStringZ<T>>(),
        Geometry::Polygon(_) => type_name::<PolygonZ<T>>(),
        Geometry::MultiPoint(_) => type_name::<MultiPointZ<T>>(),
        Geometry::MultiLineString(_) => type_name::<MultiLineStringZ<T>>(),
        Geometry::MultiPolygon(_) => type_name::<MultiPolygonZ<T>>(),
        Geometry::GeometryCollection(_) => type_name::<GeometryCollection<T>>(),
        Geometry::LineZ(_) => type_name::<LineZ<T>>(),
        Geometry::LineStringZ(_) => type_name::<LineStringZ<T>>(),
        Geometry::PolygonZ(_) => type_name::<PolygonZ<T>>(),
        Geometry::MultiPointZ(_) => type_name::<MultiPointZ<T>>(),
        Geometry::MultiLineStringZ(_) => type_name::<MultiLineStringZ<T>>(),
        Geometry::MultiPolygonZ(_) => type_name::<MultiPolygonZ<T>>(),
        Geometry::Rect(_) => type_name::<Rect<T>>(),
    }
}

// #[cfg(any(feature = "approx", test))]
// mod approx_integration {
//     use super::*;
//     use approx::{AbsDiffEq, RelativeEq, UlpsEq};

//     impl<T> RelativeEq for Geometry<T>
//     where
//         T: CoordNum + RelativeEq<Epsilon = T>,
//     {
//         #[inline]
//         fn default_max_relative() -> Self::Epsilon {
//             T::default_max_relative()
//         }

//         /// Equality assertion within a relative limit.
//         ///
//         /// # Examples
//         ///
//         /// ```
//         /// use geo_types::{Geometry, polygon};
//         ///
//         /// let a: Geometry<f32> = polygon![(x: 0., y: 0.), (x: 5., y: 0.), (x: 7., y: 9.), (x: 0., y: 0.)].into();
//         /// let b: Geometry<f32> = polygon![(x: 0., y: 0.), (x: 5., y: 0.), (x: 7.01, y: 9.), (x: 0., y: 0.)].into();
//         ///
//         /// approx::assert_relative_eq!(a, b, max_relative=0.1);
//         /// approx::assert_relative_ne!(a, b, max_relative=0.001);
//         /// ```
//         ///
//         fn relative_eq(
//             &self,
//             other: &Self,
//             epsilon: Self::Epsilon,
//             max_relative: Self::Epsilon,
//         ) -> bool {
//             match (self, other) {
//                 (Geometry::PointZ(g1), Geometry::PointZ(g2)) => {
//                     g1.relative_eq(g2, epsilon, max_relative)
//                 }
//                 (Geometry::LineZ(g1), Geometry::LineZ(g2)) => {
//                     g1.relative_eq(g2, epsilon, max_relative)
//                 }
//                 (Geometry::LineStringZ(g1), Geometry::LineStringZ(g2)) => {
//                     g1.relative_eq(g2, epsilon, max_relative)
//                 }
//                 (Geometry::PolygonZ(g1), Geometry::PolygonZ(g2)) => {
//                     g1.relative_eq(g2, epsilon, max_relative)
//                 }
//                 (Geometry::MultiPointZ(g1), Geometry::MultiPointZ(g2)) => {
//                     g1.relative_eq(g2, epsilon, max_relative)
//                 }
//                 (Geometry::MultiLineStringZ(g1), Geometry::MultiLineStringZ(g2)) => {
//                     g1.relative_eq(g2, epsilon, max_relative)
//                 }
//                 (Geometry::MultiPolygonZ(g1), Geometry::MultiPolygonZ(g2)) => {
//                     g1.relative_eq(g2, epsilon, max_relative)
//                 }
//                 (Geometry::GeometryCollection(g1), Geometry::GeometryCollection(g2)) => {
//                     g1.relative_eq(g2, epsilon, max_relative)
//                 }
//                 // (Geometry::Cube(g1), Geometry::Cube(g2)) => {
//                 //     g1.relative_eq(g2, epsilon, max_relative)
//                 // }
//                 // (Geometry::Triangle(g1), Geometry::Triangle(g2)) => {
//                 //     g1.relative_eq(g2, epsilon, max_relative)
//                 // }
//                 (_, _) => false,
//             }
//         }
//     }

//     impl<T> AbsDiffEq for Geometry<T>
//     where
//         T: CoordNum + AbsDiffEq<Epsilon = T>,
//     {
//         type Epsilon = T;

//         #[inline]
//         fn default_epsilon() -> Self::Epsilon {
//             T::default_epsilon()
//         }

//         /// Equality assertion with an absolute limit.
//         ///
//         /// # Examples
//         ///
//         /// ```
//         /// use geo_types::{Geometry, polygon};
//         ///
//         /// let a: Geometry<f32> = polygon![(x: 0., y: 0.), (x: 5., y: 0.), (x: 7., y: 9.), (x: 0., y: 0.)].into();
//         /// let b: Geometry<f32> = polygon![(x: 0., y: 0.), (x: 5., y: 0.), (x: 7.01, y: 9.), (x: 0., y: 0.)].into();
//         ///
//         /// approx::assert_abs_diff_eq!(a, b, epsilon=0.1);
//         /// approx::assert_abs_diff_ne!(a, b, epsilon=0.001);
//         /// ```
//         fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
//             true
//             // match (self, other) {
//             //     (Geometry::Point(g1), Geometry::Point(g2)) => g1.abs_diff_eq(g2, epsilon),
//             //     (Geometry::Line(g1), Geometry::Line(g2)) => g1.abs_diff_eq(g2, epsilon),
//             //     (Geometry::LineString(g1), Geometry::LineString(g2)) => {
//             //         g1.abs_diff_eq(g2, epsilon)
//             //     }
//             //     (Geometry::Polygon(g1), Geometry::Polygon(g2)) => g1.abs_diff_eq(g2, epsilon),
//             //     (Geometry::MultiPolygon(g1), Geometry::MultiPolygon(g2)) => g1.abs_diff_eq(g2, epsilon),
//             //     (Geometry::MultiPoint(g1), Geometry::MultiPoint(g2)) => {
//             //         g1.abs_diff_eq(g2, epsilon)
//             //     }
//             //     (Geometry::MultiLineString(g1), Geometry::MultiLineString(g2)) => {
//             //         g1.abs_diff_eq(g2, epsilon)
//             //     }
//             //     (Geometry::PointZ(g1), Geometry::PointZ(g2)) => g1.abs_diff_eq(g2, epsilon),
//             //     (Geometry::LineZ(g1), Geometry::LineZ(g2)) => g1.abs_diff_eq(g2, epsilon),
//             //     (Geometry::LineStringZ(g1), Geometry::LineStringZ(g2)) => g1.abs_diff_eq(g2, epsilon),
//             //     (Geometry::PolygonZ(g1), Geometry::PolygonZ(g2)) => g1.abs_diff_eq(g2, epsilon),
//             //     (Geometry::MultiPointZ(g1), Geometry::MultiPointZ(g2)) => g1.abs_diff_eq(g2, epsilon),
//             //     (Geometry::MultiLineStringZ(g1), Geometry::MultiLineStringZ(g2)) => {
//             //         g1.abs_diff_eq(g2, epsilon)
//             //     }
//             //     (Geometry::MultiPolygonZ(g1), Geometry::MultiPolygonZ(g2)) => {
//             //         g1.abs_diff_eq(g2, epsilon)
//             //     }
//             //     (Geometry::GeometryCollection(g1), Geometry::GeometryCollection(g2)) => {
//             //         g1.abs_diff_eq(g2, epsilon)
//             //     }
//             //     (Geometry::Rect(g1), Geometry::Rect(g2)) => g1.abs_diff_eq(g2, epsilon),
//             //     // (Geometry::Triangle(g1), Geometry::Triangle(g2)) => g1.abs_diff_eq(g2, epsilon),
//             //     // (_, _) => false,
//             // }
//         }
//     }

//     impl<T> UlpsEq for Geometry<T>
//     where
//         T: CoordNum + UlpsEq<Epsilon = T>,
//     {
//         fn default_max_ulps() -> u32 {
//             T::default_max_ulps()
//         }

//         /// Approximate equality assertion for floating point geometries based on the number of
//         /// representable floats that fit between the two numbers being compared.
//         ///
//         /// "relative_eq" might be more intuitive, but it does floating point math in its error
//         /// calculation, introducing its **own** error into the error calculation.
//         ///
//         /// Working with `ulps` avoids this problem. `max_ulps` means "how many floating points
//         /// are representable that fit between these two numbers", which lets us tune how "sloppy"
//         /// we're willing to be while avoiding any danger of floating point rounding in the
//         /// comparison itself.
//         ///
//         /// # Examples
//         ///
//         /// ```
//         /// use geo_types::{Geometry, Point};
//         ///
//         /// let a: Geometry = Point::new(1.0, 1.0).into();
//         /// let b: Geometry = Point::new(1.0 + 4.0 * f64::EPSILON, 1.0 + 4.0 * f64::EPSILON).into();
//         ///
//         /// approx::assert_ulps_eq!(a, b);
//         /// approx::assert_ulps_ne!(a, b, max_ulps=3);
//         /// approx::assert_ulps_eq!(a, b, max_ulps=5);
//         /// ```
//         ///
//         /// # References
//         ///
//         /// <https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/>
//         fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
//             match (self, other) {
//                 (Geometry::PointZ(g1), Geometry::PointZ(g2)) => g1.ulps_eq(g2, epsilon, max_ulps),
//                 (Geometry::Line(g1), Geometry::Line(g2)) => g1.ulps_eq(g2, epsilon, max_ulps),
//                 (Geometry::LineString(g1), Geometry::LineString(g2)) => {
//                     g1.ulps_eq(g2, epsilon, max_ulps)
//                 }
//                 (Geometry::Polygon(g1), Geometry::Polygon(g2)) => g1.ulps_eq(g2, epsilon, max_ulps),
//                 (Geometry::MultiPoint(g1), Geometry::MultiPoint(g2)) => {
//                     g1.ulps_eq(g2, epsilon, max_ulps)
//                 }
//                 (Geometry::MultiLineString(g1), Geometry::MultiLineString(g2)) => {
//                     g1.ulps_eq(g2, epsilon, max_ulps)
//                 }
//                 (Geometry::MultiPolygon(g1), Geometry::MultiPolygon(g2)) => {
//                     g1.ulps_eq(g2, epsilon, max_ulps)
//                 }
//                 (Geometry::GeometryCollection(g1), Geometry::GeometryCollection(g2)) => {
//                     g1.ulps_eq(g2, epsilon, max_ulps)
//                 }
//                 // (Geometry::Cube(g1), Geometry::Cube(g2)) => g1.ulps_eq(g2, epsilon, max_ulps),
//                 // (Geometry::Triangle(g1), Geometry::Triangle(g2)) => {
//                 //     g1.ulps_eq(g2, epsilon, max_ulps)
//                 // }
//                 // mismatched geometry types
//                 _ => false,
//             }
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     mod approx_integration {
//         use crate::{Geometry, PointZ};

//         #[test]
//         fn test_abs_diff() {
//             let g = Geometry::from(PointZ::new(1.0, 1.0, 1.0));
//             let abs_diff_eq_point =
//                 Geometry::from(PointZ::new(1.0 + f64::EPSILON, 1.0 + f64::EPSILON, 1.0 + f64::EPSILON));
//             assert_ne!(g, abs_diff_eq_point);
//             assert_abs_diff_eq!(g, abs_diff_eq_point);

//             let a_little_farther = Geometry::from(PointZ::new(1.001, 1.001, 1.001));
//             assert_ne!(g, a_little_farther);
//             assert_abs_diff_ne!(g, a_little_farther);
//             assert_abs_diff_eq!(g, a_little_farther, epsilon = 1e-3);
//             assert_abs_diff_ne!(g, a_little_farther, epsilon = 5e-4);
//         }

//         #[test]
//         fn test_relative() {
//             let g = Geometry::from(PointZ::new(2.0, 2.0, 2.0));

//             let relative_eq_point = Geometry::from(PointZ::new(
//                 2.0 + 2.0 * f64::EPSILON,
//                 2.0 + 2.0 * f64::EPSILON,
//                 2.0 + 2.0 * f64::EPSILON,
//             ));
//             assert_ne!(g, relative_eq_point);
//             assert_relative_eq!(g, relative_eq_point);

//             let a_little_farther = Geometry::from(PointZ::new(2.001, 2.001, 2.001));
//             assert_ne!(g, a_little_farther);
//             assert_relative_ne!(g, a_little_farther);
//             assert_relative_eq!(g, a_little_farther, epsilon = 1e-3);
//             assert_relative_ne!(g, a_little_farther, epsilon = 5e-4);
//             assert_relative_eq!(g, a_little_farther, max_relative = 5e-4);

//             // point * 2
//             let far = Geometry::from(PointZ::new(4.0, 4.0, 4.0));
//             assert_relative_eq!(g, far, max_relative = 1.0 / 2.0);
//             assert_relative_ne!(g, far, max_relative = 0.49);
//         }

//         #[test]
//         fn test_ulps() {
//             let g = Geometry::from(PointZ::new(1.0, 1.0, 1.0));

//             let ulps_eq_point = Geometry::from(PointZ::new(1.0 + f64::EPSILON, 1.0 + f64::EPSILON, 1.0 + f64::EPSILON));
//             assert_ne!(g, ulps_eq_point);
//             assert_ulps_eq!(g, ulps_eq_point);
//         }

//         #[test]
//         fn test_ulps_vs_relative() {
//             // "relative_eq" measures the difference between two floating point outputs, but to do
//             // so involves doing its own floating point math, which introduces some of its own
//             // error in the error calculation.
//             //
//             // Working with `ulps` avoids this problem. `max_ulps` means "how many floating points
//             // are representable that fit between these two numbers", which lets us tune how "sloppy"
//             // we're willing to be while avoiding any danger of floating point rounding in the
//             // comparison itself.
//             let a = 1000.000000000001;
//             let b = 1000.0000000000008;

//             let p1 = PointZ::new(a, a, a);
//             let p2 = PointZ::new(b, b, b);

//             assert_ne!(p1, p2);
//             assert_relative_ne!(p1, p2);
//             assert_ulps_eq!(p1, p2);
//         }
//     }
// }
