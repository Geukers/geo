use crate::{CoordFloat};

use geojson::{Feature, FeatureCollection};

use geojson::{LineStringType, PointType, PolygonType};
use std::convert::From;

// #[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<&crate::PointZ<T>> for geojson::Value
where
    T: CoordFloat,
{
    fn from(point: &crate::PointZ<T>) -> Self {
        let coords = create_point_type(point);

        geojson::Value::Point(coords)
    }
}

// #[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<&crate::MultiPointZ<T>> for geojson::Value
where
    T: CoordFloat,
{
    fn from(multi_point: &crate::MultiPointZ<T>) -> Self {
        let coords = multi_point
            .0
            .iter()
            .map(|point| create_point_type(point))
            .collect();

        geojson::Value::MultiPoint(coords)
    }
}

// #[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<&crate::LineStringZ<T>> for geojson::Value
where
    T: CoordFloat,
{
    fn from(line_string: &crate::LineStringZ<T>) -> Self {
        let coords = create_line_string_type(line_string);

        geojson::Value::LineString(coords)
    }
}

// #[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<&crate::LineZ<T>> for geojson::Value
where
    T: CoordFloat,
{
    fn from(line: &crate::LineZ<T>) -> Self {
        let coords = create_from_line_type(line);

        geojson::Value::LineString(coords)
    }
}

// #[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<&crate::Triangle<T>> for geojson::Value
where
    T: CoordFloat,
{
    fn from(triangle: &crate::Triangle<T>) -> Self {
        let coords = create_from_triangle_type(triangle);

        geojson::Value::Polygon(coords)
    }
}

// #[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
// impl<T> From<&crate::RectZ<T>> for geojson::Value
// where
//     T: CoordFloat,
// {
//     fn from(rect: &crate::RectZ<T>) -> Self {
//         let coords = create_from_rect_type(rect);

//         geojson::Value::Polygon(coords)
//     }
// }

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<&crate::MultiLineStringZ<T>> for geojson::Value
where
    T: CoordFloat,
{
    fn from(multi_line_string: &crate::MultiLineStringZ<T>) -> Self {
        let coords = create_multi_line_string_type(multi_line_string);

        geojson::Value::MultiLineString(coords)
    }
}

// #[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<&crate::PolygonZ<T>> for geojson::Value
where
    T: CoordFloat,
{
    fn from(polygon: &crate::PolygonZ<T>) -> Self {
        let coords = create_polygon_type(polygon);

        geojson::Value::Polygon(coords)
    }
}

// #[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<&crate::MultiPolygonZ<T>> for geojson::Value
where
    T: CoordFloat,
{
    fn from(multi_polygon: &crate::MultiPolygonZ<T>) -> Self {
        let coords = create_multi_polygon_type(multi_polygon);

        geojson::Value::MultiPolygon(coords)
    }
}

// #[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<&crate::GeometryCollection<T>> for geojson::Value
where
    T: CoordFloat,
{
    fn from(geometry_collection: &crate::GeometryCollection<T>) -> Self {
        let values = geometry_collection
            .0
            .iter()
            .map(|geojson| geojson::Geometry::new(geojson::Value::from(geojson)))
            .collect();

        geojson::Value::GeometryCollection(values)
    }
}

// #[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> From<&crate::GeometryCollection<T>> for FeatureCollection
where
    T: CoordFloat,
{
    fn from(geometry_collection: &crate::GeometryCollection<T>) -> Self {
        let values: Vec<Feature> = geometry_collection
            .0
            .iter()
            .map(|geojson| geojson::Geometry::new(geojson::Value::from(geojson)).into())
            .collect();

        FeatureCollection {
            bbox: None,
            features: values,
            foreign_members: None,
        }
    }
}

// #[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<'a, T> From<&'a crate::Geometry<T>> for geojson::Value
where
    T: CoordFloat,
{
    /// Convert from `crate::Geometry` enums
    fn from(geojson: &'a crate::Geometry<T>) -> Self {
        match *geojson {
            crate::Geometry::Point(ref point) => geojson::Value::from(point),
            crate::Geometry::MultiPointZ(ref multi_point) => geojson::Value::from(multi_point),
            crate::Geometry::LineString(ref line_string) => geojson::Value::from(line_string),
            crate::Geometry::Line(ref line) => geojson::Value::from(line),
            // crate::Geometry::Triangle(_) => geojson::Value::Polygon(vec![]),
            crate::Geometry::Rect(ref rect) => geojson::Value::from(rect),
            crate::Geometry::GeometryCollection(ref gc) => geojson::Value::from(gc),
            crate::Geometry::MultiLineString(ref multi_line_string) => {
                geojson::Value::from(multi_line_string)
            }
            crate::Geometry::Polygon(ref polygon) => geojson::Value::from(polygon),
            crate::Geometry::MultiPolygon(ref multi_polygon) => {
                geojson::Value::from(multi_polygon)
            }
            _ => panic!("Not valid geojson {:?}", geojson), // TODO: handle this
        }
    }
}

fn create_point_type<T>(point: &crate::PointZ<T>) -> PointType
where
    T: CoordFloat,
{
    let x: f64 = point.x().to_f64().unwrap();
    let y: f64 = point.y().to_f64().unwrap();

    vec![x, y]
}

fn create_line_string_type<T>(line_string: &crate::LineStringZ<T>) -> LineStringType
where
    T: CoordFloat,
{
    line_string
        .points()
        .map(|point| create_point_type(&point))
        .collect()
}

fn create_from_line_type<T>(line_string: &crate::LineZ<T>) -> LineStringType
where
    T: CoordFloat,
{
    vec![
        create_point_type(&line_string.start_point()),
        create_point_type(&line_string.end_point()),
    ]
}

fn create_from_triangle_type<T>(triangle: &crate::Triangle<T>) -> PolygonType
where
    T: CoordFloat,
{
    create_polygon_type(&triangle.to_polygon())
}

// fn create_from_rect_type<T>(rect: &crate::Rect<T>) -> PolygonType
// where
//     T: CoordFloat,
// {
//     create_polygon_type(&rect.to_polygon())
// }

fn create_multi_line_string_type<T>(
    multi_line_string: &crate::MultiLineStringZ<T>,
) -> Vec<LineStringType>
where
    T: CoordFloat,
{
    multi_line_string
        .0
        .iter()
        .map(|line_string| create_line_string_type(line_string))
        .collect()
}

fn create_polygon_type<T>(polygon: &crate::PolygonZ<T>) -> PolygonType
where
    T: CoordFloat,
{
    let mut coords = vec![polygon
        .exterior()
        .points()
        .map(|point| create_point_type(&point))
        .collect()];

    coords.extend(
        polygon
            .interiors()
            .iter()
            .map(|line_string| create_line_string_type(line_string)),
    );

    coords
}

fn create_multi_polygon_type<T>(multi_polygon: &crate::MultiPolygonZ<T>) -> Vec<PolygonType>
where
    T: CoordFloat,
{
    multi_polygon
        .0
        .iter()
        .map(|polygon| create_polygon_type(polygon))
        .collect()
}

#[cfg(test)]
mod tests {
    use geojson::{Geometry, Value};

    use crate::{
        CoordZ, GeometryCollection, LineZ, LineStringZ, MultiLineStringZ, MultiPointZ, MultiPolygonZ,
        PointZ, PolygonZ, Triangle,
    };

    #[test]
    fn geo_point_conversion_test() {
        // Test with f32 coordinates
        let geo_point = PointZ::new(40.02f32, 116.34f32, 0.0f32);
        let geojson_point = Value::from(&geo_point);

        if let Value::Point(c) = geojson_point {
            assert_almost_eq!(geo_point.x(), c[0] as f32, 1e-6);
            assert_almost_eq!(geo_point.y(), c[1] as f32, 1e-6);
            assert_almost_eq!(geo_point.z(), c[2] as f32, 1e-6);
        } else {
            panic!("Not valid geojson {:?}", geojson_point);
        }

        // Test with f64 coordinates.
        let geo_point = PointZ::new(40.02f64, 116.34f64, 0.0f64);
        let geojson_point = Value::from(&geo_point);

        if let Value::Point(c) = geojson_point {
            assert_almost_eq!(geo_point.x(), c[0], 1e-6);
            assert_almost_eq!(geo_point.y(), c[1], 1e-6);
            assert_almost_eq!(geo_point.z(), c[2], 1e-6);
        } else {
            panic!("Not valid geojson {:?}", geojson_point);
        }
    }

    #[test]
    fn geo_multi_point_conversion_test() {
        let p1 = PointZ::new(40.02f64, 116.34f64, 0.0f64);
        let p2 = PointZ::new(13.02f64, 24.34f64, 0.0f64);

        let geo_multi_point = MultiPointZ(vec![p1, p2]);
        let geojson_multi_point = Value::from(&geo_multi_point);

        if let Value::MultiPoint(c) = geojson_multi_point {
            assert_almost_eq!(p1.x(), c[0][0], 1e-6);
            assert_almost_eq!(p1.y(), c[0][1], 1e-6);
            assert_almost_eq!(p2.x(), c[1][0], 1e-6);
            assert_almost_eq!(p2.y(), c[1][1], 1e-6);
        } else {
            panic!("Not valid geojson {:?}", geojson_multi_point);
        }
    }

    #[test]
    fn geo_line_string_conversion_test() {
        let p1 = PointZ::new(40.02f64, 116.34f64, 0.0f64);
        let p2 = PointZ::new(13.02f64, 24.34f64, 0.0f64);

        let geo_line_string = LineStringZ::from(vec![p1, p2]);
        let geojson_line_point = Value::from(&geo_line_string);

        if let Value::LineString(c) = geojson_line_point {
            assert_almost_eq!(p1.x(), c[0][0], 1e-6);
            assert_almost_eq!(p1.y(), c[0][1], 1e-6);
            assert_almost_eq!(p1.z(), c[0][2], 1e-6);
            assert_almost_eq!(p2.x(), c[1][0], 1e-6);
            assert_almost_eq!(p2.y(), c[1][1], 1e-6);
            assert_almost_eq!(p2.z(), c[1][2], 1e-6);
        } else {
            panic!("Not valid geojson {:?}", geojson_line_point);
        }
    }

    #[test]
    fn geo_line_conversion_test() {
        let p1 = PointZ::new(40.02f64, 116.34f64, 0.0f64);
        let p2 = PointZ::new(13.02f64, 24.34f64, 0.0f64);

        let geo_line = LineZ::new(p1, p2);
        let geojson_line_point = Value::from(&geo_line);

        if let Value::LineString(c) = geojson_line_point {
            assert_almost_eq!(p1.x(), c[0][0], 1e-6);
            assert_almost_eq!(p1.y(), c[0][1], 1e-6);
            assert_almost_eq!(p1.z(), c[0][1], 1e-6);
            assert_almost_eq!(p2.x(), c[1][0], 1e-6);
            assert_almost_eq!(p2.y(), c[1][1], 1e-6);
            assert_almost_eq!(p2.z(), c[1][2], 1e-6);
        } else {
            panic!("Not valid geojson {:?}", geojson_line_point);
        }
    }

    #[test]
    fn geo_triangle_conversion_test() {
        let c1: CoordZ<f64> = CoordZ { x: 0., y: 0., z: 0. };
        let c2: CoordZ<f64> = CoordZ { x: 10., y: 20., z: 0. };
        let c3: CoordZ<f64> = CoordZ { x: 20., y: -10., z: 0. };

        let triangle = Triangle(c1, c2, c3);

        let geojson_polygon = Value::from(&triangle);

        // Geo-types Polygon construction introduces an extra vertex: let's check it!
        if let Value::Polygon(c) = geojson_polygon {
            assert_almost_eq!(c1.x, c[0][0][0], 1e-6);
            assert_almost_eq!(c1.y, c[0][0][1], 1e-6);
            assert_almost_eq!(c1.z, c[0][0][2], 1e-6);
            assert_almost_eq!(c2.x, c[0][1][0], 1e-6);
            assert_almost_eq!(c2.y, c[0][1][1], 1e-6);
            assert_almost_eq!(c2.z, c[0][1][2], 1e-6);
            assert_almost_eq!(c3.x, c[0][2][0], 1e-6);
            assert_almost_eq!(c3.y, c[0][2][1], 1e-6);
            assert_almost_eq!(c3.z, c[0][2][2], 1e-6);
            assert_almost_eq!(c1.x, c[0][3][0], 1e-6);
            assert_almost_eq!(c1.y, c[0][3][1], 1e-6);
            assert_almost_eq!(c1.z, c[0][3][2], 1e-6);
        } else {
            panic!("Not valid geojson {:?}", geojson_polygon);
        }
    }

    // #[test]
    // fn geo_rect_conversion_test() {
    //     // Same rect as crate::geojson::Rect::to_polygon doctest
    //     let c1: CoordZ<f64> = CoordZ { x: 0., y: 0., z: 0. };
    //     let c2: CoordZ<f64> = CoordZ { x: 1., y: 2., z: 0. };

    //     let rect = RectZ::new(c1, c2);

    //     let geojson_polygon = Value::from(&rect);

    //     // Geo-types Polygon construction introduces an extra vertex: let's check it!
    //     if let Value::Polygon(c) = geojson_polygon {
    //         // checks are in the same order as the crate::geojson::Rect.to_polygon doctest
    //         assert_almost_eq!(c2.x, c[0][0][0], 1e-6);
    //         assert_almost_eq!(c1.y, c[0][0][1], 1e-6);
    //         assert_almost_eq!(c2.x, c[0][1][0], 1e-6);
    //         assert_almost_eq!(c2.y, c[0][1][1], 1e-6);
    //         assert_almost_eq!(c1.x, c[0][2][0], 1e-6);
    //         assert_almost_eq!(c2.y, c[0][2][1], 1e-6);
    //         assert_almost_eq!(c1.x, c[0][3][0], 1e-6);
    //         assert_almost_eq!(c1.y, c[0][3][1], 1e-6);
    //         assert_almost_eq!(c2.x, c[0][4][0], 1e-6);
    //         assert_almost_eq!(c1.y, c[0][4][1], 1e-6);
    //     } else {
    //         panic!("Not valid geojson {:?}", geojson_polygon);
    //     }
    // }

    #[test]
    fn geo_multi_line_string_conversion_test() {
        let p1 = PointZ::new(40.02f64, 116.34f64, 0.0f64);
        let p2 = PointZ::new(13.02f64, 24.34f64, 0.0f64);
        let p3 = PointZ::new(46.84f64, 160.95f64, 0.0f64);
        let p4 = PointZ::new(42.02f64, 96.34f64, 0.0f64);

        let geo_line_string1 = LineStringZ::from(vec![p1, p2]);
        let geo_line_string2 = LineStringZ::from(vec![p3, p4]);

        let geo_multi_line_string = MultiLineStringZ(vec![geo_line_string1, geo_line_string2]);
        let geojson_multi_line_point = Value::from(&geo_multi_line_string);

        if let Value::MultiLineString(c) = geojson_multi_line_point {
            assert_almost_eq!(p1.x(), c[0][0][0], 1e-6);
            assert_almost_eq!(p1.y(), c[0][0][1], 1e-6);
            assert_almost_eq!(p2.x(), c[0][1][0], 1e-6);
            assert_almost_eq!(p2.y(), c[0][1][1], 1e-6);
            assert_almost_eq!(p3.x(), c[1][0][0], 1e-6);
            assert_almost_eq!(p3.y(), c[1][0][1], 1e-6);
            assert_almost_eq!(p4.x(), c[1][1][0], 1e-6);
            assert_almost_eq!(p4.y(), c[1][1][1], 1e-6);
        } else {
            panic!("Not valid geojson {:?}", geojson_multi_line_point);
        }
    }

    #[test]
    fn geo_polygon_conversion_test() {
        let p1 = PointZ::new(100.0f64, 0.0f64, 0.0f64);
        let p2 = PointZ::new(101.0f64, 0.0f64, 0.0f64);
        let p3 = PointZ::new(101.0f64, 1.0f64, 0.0f64);
        let p4 = PointZ::new(104.0f64, 0.2f64, 0.0f64);
        let p5 = PointZ::new(100.9f64, 0.2f64, 0.0f64);
        let p6 = PointZ::new(100.9f64, 0.7f64, 0.0f64);

        let geo_line_string1 = LineStringZ::from(vec![p1, p2, p3, p1]);
        let geo_line_string2 = LineStringZ::from(vec![p4, p5, p6, p4]);

        let geo_polygon = PolygonZ::new(geo_line_string1, vec![geo_line_string2]);
        let geojson_polygon = Value::from(&geo_polygon);

        if let Value::Polygon(c) = geojson_polygon {
            assert_almost_eq!(p1.x(), c[0][0][0], 1e-6);
            assert_almost_eq!(p1.y(), c[0][0][1], 1e-6);
            assert_almost_eq!(p2.x(), c[0][1][0], 1e-6);
            assert_almost_eq!(p2.z(), c[0][1][2], 1e-6);
            assert_almost_eq!(p3.x(), c[0][2][0], 1e-6);
            assert_almost_eq!(p3.y(), c[0][2][1], 1e-6);
            assert_almost_eq!(p3.z(), c[0][2][2], 1e-6);
            assert_almost_eq!(p4.x(), c[1][0][0], 1e-6);
            assert_almost_eq!(p4.y(), c[1][0][1], 1e-6);
            assert_almost_eq!(p4.z(), c[1][0][2], 1e-6);
            assert_almost_eq!(p5.x(), c[1][1][0], 1e-6);
            assert_almost_eq!(p5.y(), c[1][1][1], 1e-6);
            assert_almost_eq!(p5.z(), c[1][1][2], 1e-6);
            assert_almost_eq!(p6.x(), c[1][2][0], 1e-6);
            assert_almost_eq!(p6.y(), c[1][2][1], 1e-6);
            assert_almost_eq!(p6.z(), c[1][2][2], 1e-6);
        } else {
            panic!("Not valid geojson {:?}", geojson_polygon);
        }
    }

    #[test]
    fn geo_multi_polygon_conversion_test() {
        let p1 = PointZ::new(102.0f64, 2.0f64, 0.0f64);
        let p2 = PointZ::new(103.0f64, 2.0f64, 0.0f64);
        let p3 = PointZ::new(103.0f64, 30.0f64, 0.0f64);
        let p4 = PointZ::new(100.0f64, 0.0f64, 0.0f64);
        let p5 = PointZ::new(101.0f64, 0.0f64, 0.0f64);
        let p6 = PointZ::new(101.0f64, 1.0f64, 0.0f64);

        let geo_line_string1 = LineStringZ::from(vec![p1, p2, p3, p1]);
        let geo_line_string2 = LineStringZ::from(vec![p4, p5, p6, p4]);

        let geo_polygon1 = PolygonZ::new(geo_line_string1, vec![]);
        let geo_polygon2 = PolygonZ::new(geo_line_string2, vec![]);
        let geo_multi_polygon = MultiPolygonZ(vec![geo_polygon1, geo_polygon2]);
        let geojson_multi_polygon = Value::from(&geo_multi_polygon);

        if let Value::MultiPolygon(c) = geojson_multi_polygon {
            assert_almost_eq!(p1.x(), c[0][0][0][0], 1e-6);
            assert_almost_eq!(p1.y(), c[0][0][0][1], 1e-6);
            assert_almost_eq!(p2.x(), c[0][0][1][0], 1e-6);
            assert_almost_eq!(p2.y(), c[0][0][1][1], 1e-6);
            assert_almost_eq!(p3.x(), c[0][0][2][0], 1e-6);
            assert_almost_eq!(p3.y(), c[0][0][2][1], 1e-6);
            assert_almost_eq!(p4.x(), c[1][0][0][0], 1e-6);
            assert_almost_eq!(p4.y(), c[1][0][0][1], 1e-6);
            assert_almost_eq!(p5.x(), c[1][0][1][0], 1e-6);
            assert_almost_eq!(p5.y(), c[1][0][1][1], 1e-6);
            assert_almost_eq!(p6.x(), c[1][0][2][0], 1e-6);
            assert_almost_eq!(p6.y(), c[1][0][2][1], 1e-6);
        } else {
            panic!("Not valid geojson {:?}", geojson_multi_polygon);
        }
    }

    #[test]
    fn geo_geometry_collection_conversion_test() {
        let p1 = PointZ::new(100.0f64, 0.0f64, 0.0f64);
        let p2 = PointZ::new(100.0f64, 1.0f64, 0.0f64);
        let p3 = PointZ::new(101.0f64, 1.0f64, 0.0f64);
        let p4 = PointZ::new(102.0f64, 0.0f64, 0.0f64);
        let p5 = PointZ::new(101.0f64, 0.0f64, 0.0f64);
        let geo_multi_point = MultiPointZ(vec![p1, p2]);
        let geo_multi_line_string = MultiLineStringZ(vec![
            LineStringZ::from(vec![p1, p2]),
            LineStringZ::from(vec![p2, p3]),
        ]);
        let geo_multi_polygon = MultiPolygonZ(vec![
            PolygonZ::new(LineStringZ::from(vec![p3, p4, p5, p3]), vec![]),
            PolygonZ::new(LineStringZ::from(vec![p1, p5, p3, p1]), vec![]),
        ]);
        let geo_geometry_collection = GeometryCollection(vec![
            crate::Geometry::MultiPointZ(geo_multi_point),
            crate::Geometry::MultiLineStringZ(geo_multi_line_string),
            crate::Geometry::MultiPolygonZ(geo_multi_polygon),
        ]);

        let geojson_geometry_collection = Value::from(&geo_geometry_collection);

        if let Value::GeometryCollection(geometries) = geojson_geometry_collection {
            let geometry_type = |geojson: &Geometry| match geojson.value {
                Value::Point(..) => "PointZ",
                Value::MultiPoint(..) => "MultiPointZ",
                Value::LineString(..) => "LineString",
                Value::MultiLineString(..) => "MultiLineString",
                Value::Polygon(..) => "Polygon",
                Value::MultiPolygon(..) => "MultiPolygon",
                Value::GeometryCollection(..) => "GeometryCollection",
            };

            assert_eq!(3, geometries.len());
            assert_eq!(geometry_type(&geometries[0]), "MultiPointZ");
            assert_eq!(geometry_type(&geometries[1]), "MultiLineString");
            assert_eq!(geometry_type(&geometries[2]), "MultiPolygon");
        } else {
            panic!("Not valid geojson {:?}", geojson_geometry_collection);
        }
    }

    #[test]
    fn test_from_geo_type_to_geojson() {
        let p1 = crate::PointZ::new(100.0f64, 0.0f64, 0.0f64);
        let actual = serde_json::Value::from(geojson::GeoJson::from(&p1));
        let expected: serde_json::Value =
            serde_json::json!({"coordinates": [100.0, 0.0, 0.0], "type": "PointZ"});
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_from_iter_geo_type_to_geojson() {
        let p1 = crate::PointZ::new(100.0f64, 0.0f64, 0.0f64);
        let p2 = crate::PointZ::new(200.0f64, 0.0f64, 0.0f64);
        let points: Vec<_> = vec![p1, p2];

        use std::iter::FromIterator;

        let actual = geojson::GeoJson::from_iter(points.iter());
        let actual2 = points.iter().collect::<geojson::GeoJson>();
        assert_eq!(actual, actual2);

        let expected: serde_json::Value = serde_json::json!({
            "type": "GeometryCollection",
            "geometries": [
                {"coordinates": [100.0, 0.0, 0.0], "type": "PointZ"},
                {"coordinates": [200.0, 0.0, 0.0], "type": "PointZ"},
            ]
        });
        assert_eq!(expected, serde_json::Value::from(actual));
    }
}
