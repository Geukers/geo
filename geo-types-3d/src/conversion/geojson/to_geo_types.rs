use crate::CoordFloat;
use geojson::{Value, Error, Result};
use geojson::{Feature, FeatureCollection, GeoJson, LineStringType, PointType, PolygonType};
use std::convert::{TryFrom, TryInto};

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<&Value> for crate::PointZ<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        match value {
            Value::Point(point_type) => Ok(create_geo_point(point_type)),
            other => Err(mismatch_geom_err("Point", other)),
        }
    }
}
try_from_owned_value!(crate::PointZ<T>);

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<&Value> for crate::MultiPointZ<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        match value {
            Value::MultiPoint(multi_point_type) => Ok(crate::MultiPointZ(
                multi_point_type
                    .iter()
                    .map(|point_type| create_geo_point(point_type))
                    .collect(),
            )),
            other => Err(mismatch_geom_err("MultiPoint", other)),
        }
    }
}
try_from_owned_value!(crate::MultiPointZ<T>);

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<&Value> for crate::LineStringZ<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        match value {
            Value::LineString(multi_point_type) => {
                Ok(create_geo_line_string(multi_point_type))
            }
            other => Err(mismatch_geom_err("LineString", other)),
        }
    }
}
try_from_owned_value!(crate::LineStringZ<T>);

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<&Value> for crate::MultiLineStringZ<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        match value {
            Value::MultiLineString(multi_line_string_type) => {
                Ok(create_geo_multi_line_string(multi_line_string_type))
            }
            other => Err(mismatch_geom_err("MultiLineStringZ", other)),
        }
    }
}
try_from_owned_value!(crate::MultiLineStringZ<T>);

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<&Value> for crate::PolygonZ<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        match value {
            Value::Polygon(polygon_type) => Ok(create_geo_polygon(polygon_type)),
            other => Err(mismatch_geom_err("Polygon", other)),
        }
    }
}
try_from_owned_value!(crate::PolygonZ<T>);

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<&Value> for crate::MultiPolygonZ<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(value: &Value) -> Result<crate::MultiPolygonZ<T>> {
        match value {
            Value::MultiPolygon(multi_polygon_type) => {
                Ok(create_geo_multi_polygon(multi_polygon_type))
            }
            other => Err(mismatch_geom_err("MultiPolygon", other)),
        }
    }
}
try_from_owned_value!(crate::MultiPolygonZ<T>);

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<&Value> for crate::GeometryCollection<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        match value {
            Value::GeometryCollection(geometries) => {
                let geojson_geometries = geometries
                    .iter()
                    .map(|geometry| (&geometry.value).try_into().unwrap())
                    .collect();

                Ok(crate::GeometryCollection(geojson_geometries))
            }
            other => Err(mismatch_geom_err("GeometryCollection", other)),
        }
    }
}
try_from_owned_value!(crate::GeometryCollection<T>);

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<&Value> for crate::Geometry<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self> {
        match value {
            Value::Point(ref point_type) => {
                Ok(crate::Geometry::PointZ(create_geo_point(point_type)))
            }
            Value::MultiPoint(ref multi_point_type) => {
                Ok(crate::Geometry::MultiPointZ(crate::MultiPointZ(
                    multi_point_type
                        .iter()
                        .map(|point_type| create_geo_point(point_type))
                        .collect(),
                )))
            }
            Value::LineString(ref line_string_type) => Ok(
                crate::Geometry::LineStringZ(create_geo_line_string(line_string_type)),
            ),
            Value::MultiLineString(ref multi_line_string_type) => {
                Ok(crate::Geometry::MultiLineStringZ(
                    create_geo_multi_line_string(multi_line_string_type),
                ))
            }
            Value::Polygon(ref polygon_type) => Ok(crate::Geometry::PolygonZ(
                create_geo_polygon(polygon_type),
            )),
            Value::MultiPolygon(ref multi_polygon_type) => Ok(
                crate::Geometry::MultiPolygonZ(create_geo_multi_polygon(multi_polygon_type)),
            ),
            Value::GeometryCollection(ref gc_type) => {
                let gc = crate::Geometry::GeometryCollection(crate::GeometryCollection(
                    gc_type
                        .iter()
                        .cloned()
                        .map(|geom| geom.try_into())
                        .collect::<Result<Vec<crate::Geometry<T>>>>()?,
                ));
                Ok(gc)
            }
        }
    }
}
try_from_owned_value!(crate::Geometry<T>);

macro_rules! impl_try_from_geom_value {
    ($($kind:ident),*) => {
        $(
            // #[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
            impl<T> TryFrom<&geojson::Geometry> for $crate::$kind<T>
            where
                T: CoordFloat,
            {
                type Error = Error;

                fn try_from(geometry: &geojson::Geometry) -> Result<Self> {
                    Self::try_from(&geometry.value)
                }
            }

            // #[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
            impl<T> TryFrom<geojson::Geometry> for $crate::$kind<T>
            where
                T: CoordFloat,
            {
                type Error = Error;

                fn try_from(geometry: geojson::Geometry) -> Result<Self> {
                    Self::try_from(&geometry)
                }
            }

            // #[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
            impl<T> TryFrom<geojson::Feature> for $crate::$kind<T>
            where
                T: CoordFloat,
            {
                type Error = Error;

                fn try_from(val: Feature) -> Result<Self> {
                    match val.geometry {
                        None => Err(Error::FeatureHasNoGeometry(val)),
                        Some(geom) => geom.try_into(),
                    }
                }
            }
        )*
    }
}

impl_try_from_geom_value![
    PointZ,
    LineStringZ,
    PolygonZ,
    MultiPointZ,
    MultiLineStringZ,
    MultiPolygonZ,
    Geometry,
    GeometryCollection
];

impl<T: CoordFloat> TryFrom<&GeoJson> for crate::GeometryCollection<T> {
    type Error = Error;

    /// Process top-level `GeoJSON` items, returning a crate::GeometryCollection or an Error
    fn try_from(gj: &GeoJson) -> Result<crate::GeometryCollection<T>>
    where
        T: CoordFloat,
    {
        match gj {
            GeoJson::FeatureCollection(collection) => Ok(crate::GeometryCollection(
                collection
                    .features
                    .iter()
                    // Only pass on non-empty geometries
                    .filter_map(|feature| feature.geometry.as_ref())
                    .map(|geometry| geometry.clone().try_into())
                    .collect::<Result<_>>()?,
            )),
            GeoJson::Feature(feature) => {
                if let Some(geometry) = &feature.geometry {
                    Ok(crate::GeometryCollection(vec![geometry
                        .clone()
                        .try_into()?]))
                } else {
                    Ok(crate::GeometryCollection(vec![]))
                }
            }
            GeoJson::Geometry(geometry) => Ok(crate::GeometryCollection(vec![geometry
                .clone()
                .try_into()?])),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<FeatureCollection> for crate::Geometry<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(val: FeatureCollection) -> Result<crate::Geometry<T>> {
        Ok(crate::Geometry::GeometryCollection(
            crate::GeometryCollection::try_from(&GeoJson::FeatureCollection(val))?,
        ))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo-types")))]
impl<T> TryFrom<GeoJson> for crate::Geometry<T>
where
    T: CoordFloat,
{
    type Error = Error;

    fn try_from(val: GeoJson) -> Result<crate::Geometry<T>> {
        match val {
            GeoJson::Geometry(geom) => geom.try_into(),
            GeoJson::Feature(feat) => feat.try_into(),
            GeoJson::FeatureCollection(fc) => fc.try_into(),
        }
    }
}

fn create_geo_coordinate<T>(point_type: &PointType) -> crate::CoordZ<T>
where
    T: CoordFloat,
{
    crate::CoordZ {
        x: T::from(point_type[0]).unwrap(),
        y: T::from(point_type[1]).unwrap(),
        z: T::from(point_type[2]).unwrap(),
    }
}

fn create_geo_point<T>(point_type: &PointType) -> crate::PointZ<T>
where
    T: CoordFloat,
{
    crate::PointZ::new(
        T::from(point_type[0]).unwrap(),
        T::from(point_type[1]).unwrap(),
        T::from(point_type[2]).unwrap(),
    )
}

fn create_geo_line_string<T>(line_type: &LineStringType) -> crate::LineStringZ<T>
where
    T: CoordFloat,
{
    crate::LineStringZ(
        line_type
            .iter()
            .map(|point_type| create_geo_coordinate(point_type))
            .collect(),
    )
}

fn create_geo_multi_line_string<T>(
    multi_line_type: &[LineStringType],
) -> crate::MultiLineStringZ<T>
where
    T: CoordFloat,
{
    crate::MultiLineStringZ(
        multi_line_type
            .iter()
            .map(|point_type| create_geo_line_string(point_type))
            .collect(),
    )
}

fn create_geo_polygon<T>(polygon_type: &PolygonType) -> crate::PolygonZ<T>
where
    T: CoordFloat,
{
    let exterior = polygon_type
        .first()
        .map(|e| create_geo_line_string(e))
        .unwrap_or_else(|| create_geo_line_string(&vec![]));

    let interiors = if polygon_type.len() < 2 {
        vec![]
    } else {
        polygon_type[1..]
            .iter()
            .map(|line_string_type| create_geo_line_string(line_string_type))
            .collect()
    };

    crate::PolygonZ::new(exterior, interiors)
}

fn create_geo_multi_polygon<T>(multi_polygon_type: &[PolygonType]) -> crate::MultiPolygonZ<T>
where
    T: CoordFloat,
{
    crate::MultiPolygonZ(
        multi_polygon_type
            .iter()
            .map(|polygon_type| create_geo_polygon(polygon_type))
            .collect(),
    )
}

fn mismatch_geom_err(expected_type: &'static str, found: &Value) -> Error {
    Error::InvalidGeometryConversion {
        expected_type,
        found_type: found.type_name(),
    }
}

#[cfg(test)]
mod tests {
    use geojson::{Geometry, Value};
    use serde_json::json;

    use std::convert::TryInto;

    #[test]
    fn geojson_point_conversion_test() {
        let coords = vec![100.0, 0.2];
        let geojson_point = Value::Point(coords.clone());
        let geo_point: crate::Point<f64> = geojson_point.try_into().unwrap();

        assert_almost_eq!(geo_point.x(), coords[0], 1e-6);
        assert_almost_eq!(geo_point.y(), coords[1], 1e-6);
    }

    #[test]
    fn geojson_multi_point_conversion_test() {
        let coord1 = vec![100.0, 0.2];
        let coord2 = vec![101.0, 1.0];
        let geojson_multi_point = Value::MultiPoint(vec![coord1.clone(), coord2.clone()]);
        let geo_multi_point: crate::MultiPointZ<f64> = geojson_multi_point.try_into().unwrap();

        assert_almost_eq!(geo_multi_point.0[0].x(), coord1[0], 1e-6);
        assert_almost_eq!(geo_multi_point.0[0].y(), coord1[1], 1e-6);
        assert_almost_eq!(geo_multi_point.0[1].x(), coord2[0], 1e-6);
        assert_almost_eq!(geo_multi_point.0[1].y(), coord2[1], 1e-6);
    }

    #[test]
    fn geojson_line_string_conversion_test() {
        let coord1 = vec![100.0, 0.2];
        let coord2 = vec![101.0, 1.0];
        let geojson_line_string = Value::LineString(vec![coord1.clone(), coord2.clone()]);
        let geo_line_string: crate::LineStringZ<f64> = geojson_line_string.try_into().unwrap();

        assert_almost_eq!(geo_line_string.0[0].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string.0[0].y, coord1[1], 1e-6);
        assert_almost_eq!(geo_line_string.0[1].x, coord2[0], 1e-6);
        assert_almost_eq!(geo_line_string.0[1].y, coord2[1], 1e-6);
    }

    #[test]
    fn geojson_multi_line_string_conversion_test() {
        let coord1 = vec![100.0, 0.2];
        let coord2 = vec![101.0, 1.0];
        let coord3 = vec![102.0, 0.8];
        let geojson_multi_line_string = Value::MultiLineString(vec![
            vec![coord1.clone(), coord2.clone()],
            vec![coord2.clone(), coord3.clone()],
        ]);
        let geo_multi_line_string: crate::MultiLineStringZ<f64> =
            geojson_multi_line_string.try_into().unwrap();

        let geo_line_string1 = &geo_multi_line_string.0[0];
        assert_almost_eq!(geo_line_string1.0[0].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[0].y, coord1[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].x, coord2[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].y, coord2[1], 1e-6);

        let geo_line_string2 = &geo_multi_line_string.0[1];
        assert_almost_eq!(geo_line_string2.0[0].x, coord2[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[0].y, coord2[1], 1e-6);
        assert_almost_eq!(geo_line_string2.0[1].x, coord3[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[1].y, coord3[1], 1e-6);
    }

    #[test]
    fn geojson_polygon_conversion_test() {
        let coord1 = vec![100.0, 0.0];
        let coord2 = vec![101.0, 1.0];
        let coord3 = vec![101.0, 1.0];
        let coord4 = vec![104.0, 0.2];
        let coord5 = vec![100.9, 0.2];
        let coord6 = vec![100.9, 0.7];

        let geojson_multi_line_string_type1 = vec![
            vec![
                coord1.clone(),
                coord2.clone(),
                coord3.clone(),
                coord1.clone(),
            ],
            vec![
                coord4.clone(),
                coord5.clone(),
                coord6.clone(),
                coord4.clone(),
            ],
        ];
        let geojson_polygon = Value::Polygon(geojson_multi_line_string_type1);
        let geo_polygon: crate::PolygonZ<f64> = geojson_polygon.try_into().unwrap();

        let geo_line_string1 = geo_polygon.exterior();
        assert_almost_eq!(geo_line_string1.0[0].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[0].y, coord1[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].x, coord2[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].y, coord2[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[2].x, coord3[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[2].y, coord3[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[3].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[3].y, coord1[1], 1e-6);

        let geo_line_string2 = &geo_polygon.interiors()[0];
        assert_almost_eq!(geo_line_string2.0[0].x, coord4[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[0].y, coord4[1], 1e-6);
        assert_almost_eq!(geo_line_string2.0[1].x, coord5[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[1].y, coord5[1], 1e-6);
        assert_almost_eq!(geo_line_string2.0[2].x, coord6[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[2].y, coord6[1], 1e-6);
        assert_almost_eq!(geo_line_string2.0[3].x, coord4[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[3].y, coord4[1], 1e-6);
    }

    #[test]
    fn geojson_empty_polygon_conversion_test() {
        let geojson_polygon = Value::Polygon(vec![]);
        let geo_polygon: crate::PolygonZ<f64> = geojson_polygon.try_into().unwrap();

        assert!(geo_polygon.exterior().0.is_empty());
    }

    #[test]
    fn geojson_polygon_without_interiors_conversion_test() {
        let coord1 = vec![100.0, 0.0];
        let coord2 = vec![101.0, 1.0];
        let coord3 = vec![101.0, 1.0];

        let geojson_multi_line_string_type1 = vec![vec![
            coord1.clone(),
            coord2.clone(),
            coord3.clone(),
            coord1.clone(),
        ]];
        let geojson_polygon = Value::Polygon(geojson_multi_line_string_type1);
        let geo_polygon: crate::PolygonZ<f64> = geojson_polygon.try_into().unwrap();

        let geo_line_string1 = geo_polygon.exterior();
        assert_almost_eq!(geo_line_string1.0[0].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[0].y, coord1[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].x, coord2[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].y, coord2[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[2].x, coord3[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[2].y, coord3[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[3].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[3].y, coord1[1], 1e-6);

        assert_eq!(0, geo_polygon.interiors().len());
    }

    #[test]
    fn geojson_multi_polygon_conversion_test() {
        let coord1 = vec![100.0, 0.0];
        let coord2 = vec![101.0, 1.0];
        let coord3 = vec![101.0, 1.0];
        let coord4 = vec![104.0, 0.2];
        let coord5 = vec![100.9, 0.2];
        let coord6 = vec![100.9, 0.7];

        let geojson_line_string_type1 = vec![
            coord1.clone(),
            coord2.clone(),
            coord3.clone(),
            coord1.clone(),
        ];

        let geojson_line_string_type2 = vec![
            coord4.clone(),
            coord5.clone(),
            coord6.clone(),
            coord4.clone(),
        ];
        let geojson_multi_polygon = Value::MultiPolygon(vec![
            vec![geojson_line_string_type1],
            vec![geojson_line_string_type2],
        ]);
        let geo_multi_polygon: crate::MultiPolygonZ<f64> =
            geojson_multi_polygon.try_into().unwrap();

        let geo_line_string1 = geo_multi_polygon.0[0].exterior();
        assert_almost_eq!(geo_line_string1.0[0].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[0].y, coord1[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].x, coord2[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[1].y, coord2[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[2].x, coord3[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[2].y, coord3[1], 1e-6);
        assert_almost_eq!(geo_line_string1.0[3].x, coord1[0], 1e-6);
        assert_almost_eq!(geo_line_string1.0[3].y, coord1[1], 1e-6);

        let geo_line_string2 = geo_multi_polygon.0[1].exterior();
        assert_almost_eq!(geo_line_string2.0[0].x, coord4[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[0].y, coord4[1], 1e-6);
        assert_almost_eq!(geo_line_string2.0[1].x, coord5[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[1].y, coord5[1], 1e-6);
        assert_almost_eq!(geo_line_string2.0[2].x, coord6[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[2].y, coord6[1], 1e-6);
        assert_almost_eq!(geo_line_string2.0[3].x, coord4[0], 1e-6);
        assert_almost_eq!(geo_line_string2.0[3].y, coord4[1], 1e-6);
    }

    #[test]
    fn geojson_geometry_collection_conversion_test() {
        let coord1 = vec![100.0, 0.0];
        let coord2 = vec![100.0, 1.0];
        let coord3 = vec![101.0, 1.0];
        let coord4 = vec![102.0, 0.0];
        let coord5 = vec![101.0, 0.0];

        let geojson_multi_point = Value::MultiPoint(vec![coord1.clone(), coord2.clone()]);
        let geojson_multi_line_string = Value::MultiLineString(vec![
            vec![coord1.clone(), coord2.clone()],
            vec![coord2.clone(), coord3.clone()],
        ]);
        let geojson_multi_polygon = Value::MultiPolygon(vec![
            vec![vec![
                coord3.clone(),
                coord4.clone(),
                coord5.clone(),
                coord3.clone(),
            ]],
            vec![vec![
                coord1.clone(),
                coord5.clone(),
                coord3.clone(),
                coord1.clone(),
            ]],
        ]);

        let geojson_geometry_collection = Value::GeometryCollection(vec![
            Geometry::new(geojson_multi_point),
            Geometry::new(geojson_multi_line_string),
            Geometry::new(geojson_multi_polygon),
        ]);

        let geo_geometry_collection: crate::GeometryCollection<f64> =
            geojson_geometry_collection.try_into().unwrap();

        assert_eq!(3, geo_geometry_collection.0.len());
    }

    #[test]
    fn geojson_geometry_conversion() {
        let coords = vec![100.0, 0.2];
        let geojson_geometry = Geometry::from(Value::Point(coords.clone()));
        let geo_geometry: crate::Geometry<f64> = geojson_geometry
            .try_into()
            .expect("Should be able to convert to crate::Geometry");
        let geo_point: crate::PointZ<_> =
            geo_geometry.try_into().expect("this should be a point");
        assert_almost_eq!(geo_point.x(), coords[0], 1e-6);
        assert_almost_eq!(geo_point.y(), coords[1], 1e-6);
    }

    #[test]
    fn geojson_mismatch_geometry_conversion_test() {
        let coord1 = vec![100.0, 0.2];
        let coord2 = vec![101.0, 1.0];
        let geojson_line_string = Value::LineString(vec![coord1.clone(), coord2.clone()]);
        use std::convert::TryFrom;
        let error = crate::Point::<f64>::try_from(geojson_line_string).unwrap_err();
        assert_eq!(
            "Expected type: `Point`, but found `LineString`",
            format!("{}", error)
        )
    }

    #[test]
    fn feature_collection_with_geom_collection() {
        let geojson_str = json!({
            "type": "FeatureCollection",
            "features": [
            {
                "type": "Feature",
                "geometry": {
                    "type": "GeometryCollection",
                    "geometries": [
                    {
                        "type": "Polygon",
                        "coordinates": [
                            [
                                [1.0, 1.0],
                                [2.0, 2.0],
                                [3.0, 1.0],
                                [1.0, 1.0]
                            ]
                        ]
                    }
                    ]
                },
                "properties": {}
            }
            ]
        })
        .to_string();
        let geojson: geojson::GeoJson = geojson_str.parse().unwrap();
        let mut geojson_feature_collection: geojson::FeatureCollection = geojson.try_into().unwrap();
        let feature: geojson::Feature = geojson_feature_collection.features.remove(0);

        use std::convert::TryFrom;
        let geo_geom = crate::Geometry::try_from(feature).unwrap();

        let expected =
            crate::Geometry::GeometryCollection(crate::GeometryCollection(vec![
                crate::Geometry::PolygonZ(crate::PolygonZ::new(
                    crate::LineStringZ::new(vec![
                        crate::coordZ!(x: 1.0, y: 1.0, z: 1.0),
                        crate::coordZ!(x: 2.0, y: 2.0, z: 2.0),
                        crate::coordZ!(x: 3.0, y: 1.0, z: 3.0),
                        crate::coordZ!(x: 1.0, y: 1.0, z: 1.0),
                    ]),
                    vec![],
                )),
            ]));
        assert_eq!(geo_geom, expected);
    }

    #[test]
    fn borrowed_value_conversions_test() -> geojson::Result<()> {
        let coord1 = vec![100.0, 0.2];
        let coord2 = vec![101.0, 1.0];
        let coord3 = vec![102.0, 0.8];
        let coord4 = vec![104.0, 0.2];

        let geojson_point = Value::Point(coord1.clone());
        let _: crate::Point<f64> = (&geojson_point).try_into()?;

        let geojson_multi_point = Value::MultiPoint(vec![coord1.clone(), coord2.clone()]);
        let _: crate::MultiPointZ<f64> = (&geojson_multi_point).try_into()?;

        let geojson_line_string = Value::LineString(vec![coord1.clone(), coord2.clone()]);
        let _: crate::LineStringZ<f64> = (&geojson_line_string).try_into()?;

        let geojson_multi_line_string = Value::MultiLineString(vec![
            vec![coord1.clone(), coord2.clone()],
            vec![coord2.clone(), coord3.clone()],
        ]);
        let _: crate::MultiLineStringZ<f64> = (&geojson_multi_line_string).try_into()?;

        let geojson_multi_line_string_type1 = vec![
            vec![
                coord1.clone(),
                coord2.clone(),
                coord3.clone(),
                coord1.clone(),
            ],
            vec![
                coord4.clone(),
                coord1.clone(),
                coord2.clone(),
                coord4.clone(),
            ],
        ];
        let geojson_polygon = Value::Polygon(geojson_multi_line_string_type1);
        let _: crate::PolygonZ<f64> = (&geojson_polygon).try_into()?;

        let geojson_line_string_type1 = vec![
            coord1.clone(),
            coord2.clone(),
            coord3.clone(),
            coord1.clone(),
        ];

        let geojson_line_string_type2 = vec![
            coord4.clone(),
            coord3.clone(),
            coord2.clone(),
            coord4.clone(),
        ];
        let geojson_multi_polygon = Value::MultiPolygon(vec![
            vec![geojson_line_string_type1],
            vec![geojson_line_string_type2],
        ]);
        let _: crate::MultiPolygonZ<f64> = (&geojson_multi_polygon).try_into()?;

        let geojson_geometry_collection = Value::GeometryCollection(vec![
            Geometry::new(geojson_multi_point),
            Geometry::new(geojson_multi_line_string),
            Geometry::new(geojson_multi_polygon),
        ]);

        let _: crate::GeometryCollection<f64> = (&geojson_geometry_collection).try_into()?;

        Ok(())
    }
}
