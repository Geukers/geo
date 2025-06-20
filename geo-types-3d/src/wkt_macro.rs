/// Creates a [`crate::geometry`] from a
/// [WKT](https://en.wikipedia.org/wiki/Well-known_text_representation_of_geometry) literal.
///
/// This is evaluated at compile time, so you don't need to worry about runtime errors from invalid
/// WKT syntax.
///
/// Note that `POINT EMPTY` is not accepted because it is not representable as a `geo_types::Point`.
///
/// ```
/// use geo_types::wkt;
/// let point = wkt! { POINT(1.0 2.0) };
/// assert_eq!(point.x(), 1.0);
/// assert_eq!(point.y(), 2.0);
///
/// let geometry_collection = wkt! {
///     GEOMETRYCOLLECTION(
///         POINT(1.0 2.0),
///         LINESTRING EMPTY,
///         POLYGON((0.0 0.0,1.0 0.0,1.0 1.0,0.0 0.0))
///     )
/// };
/// assert_eq!(geometry_collection.len(), 3);
/// ```
#[macro_export]
macro_rules! wkt {
    // Hide distracting implementation details from the generated rustdoc.
    ($($wkt:tt)+) => {
        {
            $crate::wkt_internal!($($wkt)+)
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! wkt_internal {
    // POINT
    (POINT Z EMPTY) => {
        compile_error!("EMPTY points are not supported in geo-types")
    };
    (POINT Z ($x: literal $y: literal $z: literal)) => {
        $crate::pointZ!(x: $x, y: $y, z: $z)
    };
    (POINT Z $($tail: tt)*) => {
        compile_error!("Invalid POINT wkt");
    };

    // LINESTRING
    (LINESTRING Z EMPTY) => {
        $crate::LineStringZ::empty()
    };
    (LINESTRING Z( $( $x: literal $y: literal $z: literal ),* $(,)? )) => {
        $crate::line_string_z![ $( $crate::coordZ!(x: $x, y: $y, z: $z) ),* ]
    };
    (LINESTRING Z $($tail: tt)*) => {
        compile_error!("Invalid LINESTRING Z wkt");
    };

    // POLYGON
    (POLYGON Z EMPTY) => {
        $crate::PolygonZ::empty()
    };
    (POLYGON Z( $exterior_tt: tt $(,)? )) => {
        $crate::PolygonZ::new($crate::wkt!(LINESTRING Z $exterior_tt), $crate::_alloc::vec![])
    };
    (POLYGON Z( $exterior_tt: tt, $($interiors_tt: tt),+ $(,)? )) => {
        $crate::PolygonZ::new(
            $crate::wkt!(LINESTRING Z $exterior_tt),
            $crate::_alloc::vec![ $( $crate::wkt!(LINESTRING Z $interiors_tt) ),* ]
        )
    };
    (POLYGON Z()) => {
        compile_error!("use `EMPTY` instead of () for an empty collection")
    };
    (POLYGON Z$($tail: tt)*) => {
        compile_error!("Invalid POLYGON wkt");
    };

    // MULTIPOINT
    (MULTIPOINT Z EMPTY) => {
        $crate::MultiPointZ::empty()
    };
    (MULTIPOINT Z()) => {
        compile_error!("use `EMPTY` instead of () for an empty collection")
    };
    (MULTIPOINT Z( $( ( $x: literal $y: literal $z: literal ) ),* )) => {
        $crate::MultiPointZ(
            $crate::_alloc::vec![ $( $crate::pointZ!(x: $x, y: $y, z: $z) ),* ]
        )
    };
    (MULTIPOINT Z$($tail: tt)*) => {
        compile_error!("Invalid MULTIPOINT Z wkt");
    };

    // MULTILINESTRING
    (MULTILINESTRING Z EMPTY) => {
        $crate::MultiLineStringZ::empty()
    };
    (MULTILINESTRING Z($( $line_string_tt: tt ),* $(,)?)) => {
        $crate::MultiLineStringZ($crate::_alloc::vec![
           $( $crate::wkt!(LINESTRING Z $line_string_tt) ),*
        ])
    };
    (MULTILINESTRING Z$($tail: tt)*) => {
        compile_error!("Invalid MULTILINESTRING Z wkt");
    };

    // MULTIPOLYGON
    (MULTIPOLYGON Z EMPTY) => {
        $crate::MultiPolygonZ::empty()
    };
    (MULTIPOLYGON Z()) => {
        compile_error!("use `EMPTY` instead of () for an empty collection")
    };
    (MULTIPOLYGON Z( $( $polygon_tt: tt ),* $(,)? )) => {
        $crate::MultiPolygonZ($crate::_alloc::vec![
           $( $crate::wkt!(POLYGON Z $polygon_tt) ),*
        ])
    };
    (MULTIPOLYGON Z$($tail: tt)*) => {
        compile_error!("Invalid MULTIPOLYGON Z wkt");
    };

    // GEOMETRYCOLLECTION
    (GEOMETRYCOLLECTION EMPTY) => {
        $crate::GeometryCollection::empty()
    };
    (GEOMETRYCOLLECTION ()) => {
        compile_error!("use `EMPTY` instead of () for an empty collection")
    };
    (GEOMETRYCOLLECTION ( $( $el_type:tt $el_tt: tt ),* $(,)? )) => {
        $crate::GeometryCollection($crate::_alloc::vec![
           $( $crate::Geometry::from($crate::wkt!($el_type $el_tt)) ),*
        ])
    };
    (GEOMETRYCOLLECTION $($tail: tt)*) => {
        compile_error!("Invalid GEOMETRYCOLLECTION wkt");
    };
    ($name: ident ($($tail: tt)*)) => {
        compile_error!("Unknown type. Must be one of POINT Z, LINESTRING Z, POLYGON Z, MULTIPOINT Z, MULTILINESTRING Z, MULTIPOLYGON Z, or GEOMETRYCOLLECTION Z");
    };
}

#[cfg(test)]
mod test {
    use crate::geometry::*;
    use alloc::vec;

    #[test]
    fn point() {
        let point = wkt! { POINT Z (1.0 2.0 3.0) };
        assert_eq!(point.x(), 1.0);
        assert_eq!(point.y(), 2.0);
        assert_eq!(point.z(), 3.0);

        let point = wkt! { POINT Z (1.0 2.0 3.0) };
        assert_eq!(point.x(), 1.0);
        assert_eq!(point.y(), 2.0);
        assert_eq!(point.z(), 3.0);

        // This (rightfully) fails to compile because geo-types doesn't support "empty" points
        // wkt! { POINT EMPTY }
    }

    #[test]
    fn empty_line_string() {
        let line_string: LineStringZ<f64> = wkt! { LINESTRING Z EMPTY };
        assert_eq!(line_string.0.len(), 0);

        // This (rightfully) fails to compile because its invalid wkt
        // wkt! { LINESTRING() }
    }

    #[test]
    fn line_string() {
        let line_string = wkt! { LINESTRING Z (1.0 2.0 3.0,3.0 4.0 5.0) };
        assert_eq!(line_string.0.len(), 2);
        assert_eq!(line_string[0], coordZ! { x: 1.0, y: 2.0, z: 3.0 });
    }

    #[test]
    fn empty_polygon() {
        let polygon: PolygonZ = wkt! { POLYGON Z EMPTY };
        assert_eq!(polygon.exterior().0.len(), 0);
        assert_eq!(polygon.interiors().len(), 0);

        // This (rightfully) fails to compile because its invalid wkt
        // wkt! { POLYGON() }
    }

    #[test]
    fn polygon() {
        let polygon = wkt! { POLYGON Z((1.0 2.0 3.0)) };
        assert_eq!(polygon.exterior().0.len(), 1);
        assert_eq!(polygon.exterior().0[0], coordZ! { x: 1.0, y: 2.0, z: 3.0 });

        let polygon = wkt! { POLYGON Z((1.0 2.0 3.0, 3.0 4.0 5.0)) };
        // Note: an extra coord is added to close the linestring
        assert_eq!(polygon.exterior().0.len(), 3);
        assert_eq!(polygon.exterior().0[0], coordZ! { x: 1.0, y: 2.0, z: 3.0 });
        assert_eq!(polygon.exterior().0[1], coordZ! { x: 3.0, y: 4.0, z: 5.0 });
        assert_eq!(polygon.exterior().0[2], coordZ! { x: 1.0, y: 2.0, z: 3.0 });

        let polygon = wkt! { POLYGON Z ((1.0 2.0 3.0), (1.1 2.1 3.1)) };
        assert_eq!(polygon.exterior().0.len(), 1);
        assert_eq!(polygon.interiors().len(), 1);

        assert_eq!(polygon.exterior().0[0], coordZ! { x: 1.0, y: 2.0, z: 3.0 });
        assert_eq!(polygon.interiors()[0].0[0], coordZ! { x: 1.1, y: 2.1, z: 3.1 });

        let polygon = wkt! { POLYGON Z((1.0 2.0 3.0,3.0 4.0 5.0), (1.1 2.1 3.1,3.1 4.1 5.1), (1.2 2.2 3.2,3.2 4.2 5.2)) };
        assert_eq!(polygon.exterior().0.len(), 3);
        assert_eq!(polygon.interiors().len(), 2);
        assert_eq!(polygon.interiors()[1][1], coordZ! { x: 3.2, y: 4.2, z: 5.2 });
    }

    #[test]
    fn empty_multi_point() {
        let multipoint: MultiPointZ = wkt! { MULTIPOINT Z EMPTY };
        assert!(multipoint.0.is_empty());
        // This (rightfully) fails to compile because its invalid wkt
        // wkt! { MULTIPOINT() }
    }

    #[test]
    fn multi_point() {
        let multi_point = wkt! { MULTIPOINT Z ((1.0 2.0 3.0)) };
        assert_eq!(multi_point.0, vec![pointZ! { x: 1.0, y: 2.0, z: 3.0}]);

        let multi_point = wkt! { MULTIPOINT Z ((1.0 2.0 3.0), (3.0 4.0 5.0)) };
        assert_eq!(
            multi_point.0,
            vec![pointZ! { x: 1.0, y: 2.0, z: 3.0}, pointZ! { x: 3.0, y: 4.0, z: 5.0}]
        );
    }

    #[test]
    fn empty_multi_line_string() {
        let multi_line_string: MultiLineStringZ = wkt! { MULTILINESTRING Z EMPTY };
        assert_eq!(multi_line_string.0, vec![]);
        // This (rightfully) fails to compile because its invalid wkt
        // wkt! { MULTILINESTRING() }
    }
    #[test]
    fn multi_line_string() {
        let multi_line_string = wkt! { MULTILINESTRING Z ((1.0 2.0 3.0, 3.0 4.0 5.0)) };
        assert_eq!(multi_line_string.0.len(), 1);
        assert_eq!(multi_line_string.0[0].0[1], coordZ! { x: 3.0, y: 4.0, z: 5.0 });
        let multi_line_string = wkt! { MULTILINESTRING Z ((1.0 2.0 3.0,3.0 4.0 5.0),(5.0 6.0 7.0,7.0 8.0 9.0)) };
        assert_eq!(multi_line_string.0.len(), 2);
        assert_eq!(multi_line_string.0[1].0[1], coordZ! { x: 7.0, y: 8.0, z: 9.0 });

        let multi_line_string = wkt! { MULTILINESTRING Z ((1.0 2.0 3.0, 3.0 4.0 5.0), EMPTY) };
        assert_eq!(multi_line_string.0.len(), 2);
        assert_eq!(multi_line_string.0[1].0.len(), 0);
    }

    #[test]
    fn empty_multi_polygon() {
        let multi_polygon: MultiPolygonZ = wkt! { MULTIPOLYGON Z EMPTY };
        assert!(multi_polygon.0.is_empty());

        // This (rightfully) fails to compile because its invalid wkt
        // wkt! { MULTIPOLYGON() }
    }

    #[test]
    fn multi_line_polygon() {
        let multi_polygon = wkt! { MULTIPOLYGON Z (((1.0 2.0 3.0, 3.0 4.0 5.0, 5.0 6.0 7.0, 1.0 2.0 3.0))) };
        assert_eq!(multi_polygon.0.len(), 1);
        assert_eq!(multi_polygon.0[0].exterior().0[0], coordZ! { x: 1.0, y: 2.0, z: 3.0});

        let multi_polygon = wkt! { MULTIPOLYGON Z (((1.0 2.0 3.0,3.0 4.0 5.0), (1.1 2.1 3.1,3.1 4.1 5.1), (1.2 2.2 3.2,3.2 4.2 5.2)),((1.0 2.0 3.0))) };
        assert_eq!(multi_polygon.0.len(), 2);
        assert_eq!(
            multi_polygon.0[0].interiors()[1].0[0],
            coordZ! { x: 1.2, y: 2.2, z: 3.2}
        );

        let multi_polygon = wkt! { MULTIPOLYGON Z (((1.0 2.0 3.0,3.0 4.0 5.0), (1.1 2.1 3.1,3.1 4.1 5.1), (1.2 2.2 3.2,3.2 4.2 5.2)), EMPTY) };
        assert_eq!(multi_polygon.0.len(), 2);
        assert_eq!(
            multi_polygon.0[0].interiors()[1].0[0],
            coordZ! { x: 1.2, y: 2.2, z: 3.2}
        );
        assert!(multi_polygon.0[1].exterior().0.is_empty());
    }

    // #[test]
    // fn empty_geometry_collection() {
    //     let geometry_collection: GeometryCollection = wkt! { GEOMETRYCOLLECTION EMPTY };
    //     assert!(geometry_collection.is_empty());

    //     // This (rightfully) fails to compile because its invalid wkt
    //     // wkt! { MULTIPOLYGON() }
    // }

    // #[test]
    // fn geometry_collection() {
    //     let geometry_collection = wkt! {
    //         GEOMETRYCOLLECTION (
    //             POINT Z (40.0 10.0),
    //             LINESTRING Z (10.0 10.0, 20.0 20.0, 10.0 40.0),
    //             POLYGON Z ((40.0 40.0, 20.0 45.0, 45.0 30.0, 40.0 40.0))
    //         )
    //     };
    //     assert_eq!(geometry_collection.len(), 3);

    //     let line_string = match &geometry_collection[1] {
    //         Geometry::LineString(line_string) => line_string,
    //         _ => panic!(
    //             "unexpected geometry: {geometry:?}",
    //             geometry = geometry_collection[1]
    //         ),
    //     };
    //     assert_eq!(line_string.0[1], coordZ! {x: 20.0, y: 20.0, z: 0.0 });
    // }

    #[test]
    fn other_numeric_types() {
        let point: PointZ<i32> = wkt!(POINT Z(1 2 3));
        assert_eq!(point.x(), 1i32);
        assert_eq!(point.y(), 2i32);
        assert_eq!(point.z(), 3i32);

        let point: PointZ<u64> = wkt!(POINT Z(1 2 3));
        assert_eq!(point.x(), 1u64);
        assert_eq!(point.y(), 2u64);
        assert_eq!(point.z(), 3u64);

        let point: PointZ<f32> = wkt!(POINT Z(1.0 2.0 3.0));
        assert_eq!(point.x(), 1.0f32);
        assert_eq!(point.y(), 2.0f32);
        assert_eq!(point.z(), 3.0f32);
    }
}
