use core::fmt;

#[derive(Debug)]
pub enum Error {
    MismatchedGeometry {
        expected: &'static str,
        found: &'static str,
    },
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::MismatchedGeometry { expected, found } => {
                write!(f, "Expected a {expected}, but found a {found}")
            }
        }
    }
}

// #[cfg(test)]
// mod test {
//     use crate::{Geometry, PointZ};
//     use alloc::string::ToString;
//     use core::convert::TryFrom;

//     #[test]
//     fn error_output() {
//         let point = PointZ::new(1.0, 2.0, 3.0);
//         let point_geometry = Geometry::from(point);

//         // let rect = Cube::new(PointZ::new(1.0, 2.0), PointZ::new(3.0, 4.0));
//         // let rect_geometry = Geometry::from(rect);

//         PointZ::try_from(point_geometry).expect("failed to unwrap inner enum Point");

//         let failure = PointZ::try_from(rect_geometry).unwrap_err();
//         assert_eq!(
//             failure.to_string(),
//             "Expected a geo_types::geometry::point::Point, but found a geo_types::geometry::rect::Rect"
//         );
//     }
// }
