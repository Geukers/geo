use core::fmt::{Debug, Formatter};

use crate::geometry::*;
use crate::CoordNum;

impl<T: CoordNum> Debug for CoordZ<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "COORD Z({x:?} {y:?} {z:?})", x = self.x, y = self.y, z = self.z)
    }
}

impl<T: CoordNum> Debug for PointZ<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "POINT Z({x:?} {y:?} {z:?})", x = self.x(), y = self.y(), z = self.z())
    }
}

impl<T: CoordNum> Debug for LineZ<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "LINE Z")?;
        write_coord_seq(f, [self.start, self.end].iter())
    }
}

impl<T: CoordNum> Debug for LineStringZ<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "LINESTRING Z")?;
        if self.0.is_empty() {
            write!(f, " ")?;
        }
        write_coord_seq(f, self.0.iter())?;
        Ok(())
    }
}

impl<T: CoordNum> Debug for PolygonZ<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "POLYGON Z")?;
        if self.exterior().0.is_empty() && self.interiors().is_empty() {
            write!(f, " ")?;
        }
        write_polygon_inner(f, self)
    }
}

impl<T: CoordNum> Debug for MultiPointZ<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "MULTIPOINT Z")?;
        if self.0.is_empty() {
            write!(f, " ")?;
        }
        write_coord_seq(f, self.0.iter().map(|p| &p.0))
    }
}

impl<T: CoordNum> Debug for MultiLineStringZ<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "MULTILINESTRING Z")?;
        let mut line_strings = self.0.iter();
        let Some(first) = line_strings.next() else {
            return write!(f, " EMPTY");
        };
        write!(f, "(")?;
        write_coord_seq(f, first.0.iter())?;
        for line_string in line_strings {
            write!(f, ",")?;
            write_coord_seq(f, line_string.0.iter())?;
        }
        write!(f, ")")
    }
}
impl<T: CoordNum> Debug for MultiPolygonZ<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "MULTIPOLYGON Z")?;
        let mut polygons = self.0.iter();
        let Some(first) = polygons.next() else {
            return write!(f, " EMPTY");
        };
        write!(f, "(")?;
        write_polygon_inner(f, first)?;
        for polygon in polygons {
            write!(f, ",")?;
            write_polygon_inner(f, polygon)?;
        }
        write!(f, ")")
    }
}

// impl<T: CoordNum> Debug for Cube<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
//         write!(f, "RECT")?;
//         write_coord_seq(f, [self.min(), self.max()].iter())
//     }
// }

impl<T: CoordNum> Debug for Triangle<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "TRIANGLE")?;
        write_coord_seq(f, [self.0, self.1, self.2].iter())
    }
}

impl<T: CoordNum> Debug for Geometry<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Geometry::PointZ(inner) => inner.fmt(f),
            Geometry::Line(inner) => inner.fmt(f),
            Geometry::LineString(inner) => inner.fmt(f),
            Geometry::Polygon(inner) => inner.fmt(f),
            Geometry::MultiPoint(inner) => inner.fmt(f),
            Geometry::MultiLineString(inner) => inner.fmt(f),
            Geometry::MultiPolygon(inner) => inner.fmt(f),
            Geometry::GeometryCollection(inner) => inner.fmt(f),
            Geometry::Point(point) => point.fmt(f),
            Geometry::LineZ(line_z) => line_z.fmt(f),
            Geometry::LineStringZ(line_string_z) => line_string_z.fmt(f),
            Geometry::PolygonZ(polygon_z) => polygon_z.fmt(f),
            Geometry::MultiPointZ(multi_point_z) => multi_point_z.fmt(f),
            Geometry::MultiLineStringZ(multi_line_string_z) => multi_line_string_z.fmt(f),
            Geometry::MultiPolygonZ(multi_polygon_z) => multi_polygon_z.fmt(f),
            Geometry::Rect(rect) => rect.fmt(f),
        }
    }
}

impl<T: CoordNum> Debug for GeometryCollection<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "GEOMETRYCOLLECTION")?;
        let mut geometries = self.0.iter();
        let Some(first) = geometries.next() else {
            return write!(f, " EMPTY");
        };
        write!(f, "({first:?}")?;
        for geometry in geometries {
            write!(f, ",{geometry:?}")?;
        }
        write!(f, ")")
    }
}

fn write_coord_seq<'a, T: CoordNum + 'a>(
    f: &mut Formatter<'_>,
    mut coords: impl Iterator<Item = &'a CoordZ<T>>,
) -> core::fmt::Result {
    let Some(coord) = coords.next() else {
        write!(f, "EMPTY")?;
        return Ok(());
    };
    write!(f, "({x:?} {y:?} {z:?}", x = coord.x, y = coord.y, z = coord.z)?;
    for coord in coords {
        write!(f, ",{x:?} {y:?} {z:?}", x = coord.x, y = coord.y, z = coord.z)?;
    }
    write!(f, ")")
}

fn write_polygon_inner<T: CoordNum>(
    f: &mut Formatter<'_>,
    polygon: &PolygonZ<T>,
) -> core::fmt::Result {
    if polygon.exterior().0.is_empty() {
        let mut interiors = polygon.interiors().iter();
        let Some(interior) = interiors.next() else {
            write!(f, "EMPTY")?;
            return Ok(());
        };

        // Invalid polygon - having interiors but no exterior!
        // Still, we should try to print something meaningful.
        write!(f, "(EMPTY,")?;
        write_coord_seq(f, interior.0.iter())?;
        for interior in interiors {
            write!(f, ",")?;
            write_coord_seq(f, interior.0.iter())?;
        }
        write!(f, ")")?;
    } else {
        write!(f, "(")?;
        write_coord_seq(f, polygon.exterior().0.iter())?;
        for interior in polygon.interiors().iter() {
            write!(f, ",")?;
            write_coord_seq(f, interior.0.iter())?;
        }
        write!(f, ")")?;
    }
    Ok(())
}

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_coord() {
        let coord = CoordZ { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!("COORD Z(1.0 2.0 3.0)", format!("{coord:?}"));
    }
    #[test]
    fn int_coord() {
        let coord = CoordZ { x: 1, y: 2, z: 3 };
        assert_eq!("COORD Z(1 2 3)", format!("{coord:?}"));
    }
    #[test]
    fn float_point() {
        let point = PointZ::new(1.0, 2.0, 3.0);
        assert_eq!("POINT Z(1.0 2.0 3.0)", format!("{point:?}"));
    }
    #[test]
    fn int_point() {
        let point = PointZ::new(1, 2, 3);
        assert_eq!("POINT Z(1 2 3)", format!("{point:?}"));
    }
    #[test]
    fn line() {
        let line_string = LineZ::new((1, 2, 3), (4, 5, 6));
        assert_eq!("LINE Z(1 2 3,4 5 6)", format!("{line_string:?}"));
    }
    #[test]
    fn line_string() {
        let line_string = LineStringZ::new(vec![(1, 2, 3).into(), (4, 5, 6).into()]);
        assert_eq!("LINESTRING Z(1 2 3,4 5 6)", format!("{line_string:?}"));
    }
    #[test]
    fn line_string_with_single_element() {
        let line_string = LineStringZ::new(vec![(1, 2, 3).into()]);
        assert_eq!("LINESTRING Z(1 2 3)", format!("{line_string:?}"));
    }
    #[test]
    fn empty_line_string() {
        let line_string = LineStringZ::<i32>::empty();
        assert_eq!("LINESTRING Z EMPTY", format!("{line_string:?}"));
    }
    #[test]
    fn polygon_no_holes() {
        let polygon = wkt!(POLYGON Z((1 2 3,3 4 5,5 6 7)));
        assert_eq!("POLYGON Z((1 2 3,3 4 5,5 6 7,1 2 3))", format!("{polygon:?}"));
    }
    #[test]
    fn polygon_with_hole() {
        let polygon = wkt!(POLYGON Z(
            (1 1 1,10 1 10,10 10 10,1 10 1,1 1 1),
            (3 3 3,7 3 7,7 7 7,3 7 3,3 3 3)
        ));
        assert_eq!(
            "POLYGON Z((1 1 1,10 1 10,10 10 10,1 10 1,1 1 1),(3 3 3,7 3 7,7 7 7,3 7 3,3 3 3))",
            format!("{polygon:?}")
        );
    }
    #[test]
    fn polygon_with_multiple_holes() {
        let polygon = wkt!(POLYGON Z(
            (0 0 0,10 0 10,10 10 10,0 10 10,0 0 0),
            (2 2 2,4 2 4,4 4 4,2 4 4,2 2 2),
            (6 6 6,8 6 8,8 8 8,6 8 8,6 6 6)
        ));
        assert_eq!(
            "POLYGON Z((0 0 0,10 0 10,10 10 10,0 10 10,0 0 0),(2 2 2,4 2 4,4 4 4,2 4 4,2 2 2),(6 6 6,8 6 8,8 8 8,6 8 8,6 6 6))",
            format!("{polygon:?}")
        );
    }
    #[test]
    fn invalid_polygon_interior_but_no_exterior() {
        // Not a valid polygon, but we should still have reasonable debug output - note this is *not* valid WKT
        let interior = LineStringZ::new(vec![(1, 2, 3).into()]);
        let polygon = PolygonZ::new(LineStringZ::empty(), vec![interior]);
        assert_eq!("POLYGON Z(EMPTY,(1 2 3))", format!("{polygon:?}"));
    }
    #[test]
    fn empty_polygon() {
        let polygon: PolygonZ = wkt!(POLYGON Z EMPTY);
        assert_eq!("POLYGON Z EMPTY", format!("{polygon:?}"));
    }
    #[test]
    fn multi_point_empty() {
        let multi_point: MultiPointZ = wkt!(MULTIPOINT Z EMPTY);
        assert_eq!("MULTIPOINT Z EMPTY", format!("{multi_point:?}"));
    }
    #[test]
    fn multi_point_one_point() {
        let multi_point = wkt!(MULTIPOINT Z((1 2 3)));
        assert_eq!("MULTIPOINT Z(1 2 3)", format!("{multi_point:?}"));
    }
    #[test]
    fn multi_point_three_points() {
        let multi_point = wkt!(MULTIPOINT Z((1 2 3),(3 4 5),(5 6 7)));
        assert_eq!("MULTIPOINT Z(1 2 3,3 4 5,5 6 7)", format!("{multi_point:?}"));
    }
    #[test]
    fn multilinestring_empty() {
        let multi_line_string: MultiLineStringZ = wkt!(MULTILINESTRING Z EMPTY);
        assert_eq!("MULTILINESTRING Z EMPTY", format!("{multi_line_string:?}"));
    }

    #[test]
    fn multi_line_string_one_line() {
        let multi_line_string = wkt!(MULTILINESTRING Z((1 2 3, 3 4 5)));
        assert_eq!(
            "MULTILINESTRING Z((1 2 3,3 4 5))",
            format!("{multi_line_string:?}")
        );
    }

    #[test]
    fn multi_line_string_multiple_lines() {
        let multi_line_string = wkt!(MULTILINESTRING Z(
            (1 2 3, 3 4 5, 5 6 7),
            (7 8 9, 9 10 11, 11 12 13)
        ));
        assert_eq!(
            "MULTILINESTRING Z((1 2 3,3 4 5,5 6 7),(7 8 9,9 10 11,11 12 13))",
            format!("{multi_line_string:?}")
        );
    }

    #[test]
    fn multi_line_string_multiple_lines_with_empty() {
        let multi_line_string = wkt!(MULTILINESTRING Z(
            (1 2 3, 3 4 5, 5 6 7),
            EMPTY,
            (7 8 9, 9 10 11, 11 12 13)
        ));
        assert_eq!(
            "MULTILINESTRING Z((1 2 3,3 4 5,5 6 7),EMPTY,(7 8 9,9 10 11,11 12 13))",
            format!("{multi_line_string:?}")
        );
    }
    #[test]
    fn multi_polygon_empty() {
        let multi_polygon: MultiPolygonZ = wkt!(MULTIPOLYGON Z EMPTY);
        assert_eq!("MULTIPOLYGON Z EMPTY", format!("{multi_polygon:?}"));
    }

    #[test]
    fn multi_polygon_one_polygon() {
        let multi_polygon = wkt!(MULTIPOLYGON Z(
            ((1 2 3, 3 4 5, 5 6 7, 1 2 3))
        ));
        assert_eq!(
            "MULTIPOLYGON Z(((1 2 3,3 4 5,5 6 7,1 2 3)))",
            format!("{multi_polygon:?}")
        );
    }

    #[test]
    fn multi_polygon_multiple_polygons() {
        let multi_polygon = wkt!(MULTIPOLYGON Z(
            ((1 2 3, 3 4 5, 5 6 7, 1 2 3)),
            ((7 8 9, 9 10 11, 11 12 13, 7 8 9))
        ));
        assert_eq!(
            "MULTIPOLYGON Z(((1 2 3,3 4 5,5 6 7,1 2 3)),((7 8 9,9 10 11,11 12 13,7 8 9)))",
            format!("{multi_polygon:?}")
        );
    }

    #[test]
    fn multi_polygon_with_holes() {
        let multi_polygon = wkt!(MULTIPOLYGON Z(
            (
                (1 1 1, 10 1 10, 10 10 10, 1 10 10, 1 1 1)
            ),
            (
                (20 20 20, 30 20 30, 30 30 30, 20 30 30, 20 20 20),
                (22 22 22, 28 22 28, 28 28 28, 22 28 28, 22 22 22)
            )
        ));
        assert_eq!(
            "MULTIPOLYGON Z(((1 1 1,10 1 10,10 10 10,1 10 10,1 1 1)),((20 20 20,30 20 30,30 30 30,20 30 30,20 20 20),(22 22 22,28 22 28,28 28 28,22 28 28,22 22 22)))",
            format!("{multi_polygon:?}")
        );
    }
    #[test]
    fn multi_polygon_with_holes_and_empty_polygon() {
        let multi_polygon = wkt!(MULTIPOLYGON Z(
            (
                (1 1 1, 10 1 10, 10 10 10, 1 10 10, 1 1 1)
            ),
            EMPTY,
            (
                (20 20 20, 30 20 30, 30 30 30, 20 30 30, 20 20 20),
                (22 22 22, 28 22 28, 28 28 28, 22 28 28, 22 22 22)
            )
        ));
        assert_eq!(
            "MULTIPOLYGON Z(((1 1 1,10 1 10,10 10 10,1 10 10,1 1 1)),EMPTY,((20 20 20,30 20 30,30 30 30,20 30 30,20 20 20),(22 22 22,28 22 28,28 28 28,22 28 28,22 22 22)))",
            format!("{multi_polygon:?}")
        );
    }
    // #[test]
    // fn rect() {
    //     let rect = Cube::new((1, 2), (3, 4));
    //     assert_eq!("RECT(1 2,3 4)", format!("{rect:?}"));

    //     let rect = Cube::new((3, 4), (1, 2));
    //     // output is always (min, max)
    //     assert_eq!("RECT(1 2,3 4)", format!("{rect:?}"));
    // }
    // #[test]
    // fn triangle() {
    //     let rect = Triangle::new((1, 2, 3).into(), (4, 5, 6).into(), (7, 8, 9).into());
    //     assert_eq!("TRIANGLE(1 2,3 4,5 6)", format!("{rect:?}"));
    // }

    // #[test]
    // fn geometry() {
    //     let rect = Geometry::Triangle(Triangle::new((1, 2).into(), (3, 4).into(), (5, 6).into()));
    //     assert_eq!("TRIANGLE(1 2,3 4,5 6)", format!("{rect:?}"));
    // }

    // #[test]
    // fn geometry_collection() {
    //     let rect = Geometry::Triangle(Triangle::new((1, 2).into(), (3, 4).into(), (5, 6).into()));
    //     assert_eq!("TRIANGLE(1 2,3 4,5 6)", format!("{rect:?}"));
    // }

    #[test]
    fn empty_geometry_collection() {
        let geometry_collection: GeometryCollection = GeometryCollection::default();
        assert_eq!(
            "GEOMETRYCOLLECTION EMPTY",
            format!("{geometry_collection:?}")
        );
    }

    #[test]
    fn geometry_collection_with_mixed_geometries() {
        let geometry_collection: GeometryCollection<i32> = GeometryCollection::from(vec![
            Geometry::PointZ(PointZ::new(1, 2, 3)),
            Geometry::LineZ(LineZ::new((1, 2, 3), (4, 5, 6))),
            Geometry::PolygonZ(PolygonZ::new(
                LineStringZ::from(vec![(0, 0, 0), (1, 0, 0), (1, 1, 0), (0, 0, 0)]),
                vec![],
            )),
        ]);

        assert_eq!(
            "GEOMETRYCOLLECTION(POINT Z(1 2 3),LINE Z(1 2 3,4 5 6),POLYGON Z((0 0 0,1 0 0,1 1 0,0 0 0)))",
            format!("{geometry_collection:?}")
        );
    }

    // #[test]
    // fn nested_geometry_collection() {
    //     let inner_collection: GeometryCollection<i32> = GeometryCollection::from(vec![
    //         Geometry::Point(PointZ::new(5, 6, 7)),
    //         Geometry::LineString(LineStringZ::from(vec![(1, 2, 3), (4, 5, 6)])),
    //     ]);

    //     let outer_collection: GeometryCollection<i32> = GeometryCollection::from(vec![
    //         Geometry::Point(PointZ::new(1, 2, 3)),
    //         Geometry::GeometryCollection(inner_collection),
    //     ]);

    //     assert_eq!(
    //         "GEOMETRYCOLLECTION(POINT(1 2),GEOMETRYCOLLECTION(POINT(5 6),LINESTRING(1 2,3 4)))",
    //         format!("{outer_collection:?}")
    //     );
    // }

    // #[test]
    // fn geometry_collection_with_no_coordinates() {
    //     let geometry_collection: GeometryCollection<f64> = GeometryCollection::from(vec![
    //         Geometry::Point(PointZ::new(0.0, 0.0, 0.0)),
    //         Geometry::Polygon(PolygonZ::empty()),
    //     ]);

    //     assert_eq!(
    //         "GEOMETRYCOLLECTION(POINT(0.0 0.0),POLYGON EMPTY)",
    //         format!("{geometry_collection:?}")
    //     );
    // }
}
