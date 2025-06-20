/// Creates a [`pointZ`] from the given coordinates.
///
/// ```txt
/// pointZ! { x: <number>, y: <number>, z: <number> }
/// pointZ!(<coordinate>)
/// ```
///
/// # Examples
///
/// Creating a [`pointZ`], supplying x/y values:
///
/// ```
/// use geo_types_3d::{pointZ, coordZ};
///
/// let p = pointZ! { x: 181.2, y: 51.79, z: 0.0 };
///
/// assert_eq!(p.x(), 181.2);
/// assert_eq!(p.y(), 51.79);
/// assert_eq!(p.z(), 0.0);
///
/// let p = pointZ!(coordZ! { x: 181.2, y: 51.79, z: 0.0 });
///
/// assert_eq!(p.x(), 181.2);
/// assert_eq!(p.y(), 51.79);
/// assert_eq!(p.z(), 0.0);
/// ```
///
/// [`pointZ`]: ./struct.pointZ.html
#[macro_export]
macro_rules! pointZ {
    ( x: $x:expr, y: $y:expr, z: $z:expr $(,)? ) => {
        $crate::PointZ::from($crate::coordZ! { x: $x, y: $y, z: $z })
    };
    ( $coordZ:expr $(,)? ) => {
        $crate::PointZ::from($coordZ)
    };
}

#[macro_export]
macro_rules! coordZ {
    (x: $x:expr, y: $y:expr, z: $z:expr $(,)? ) => {
        $crate::CoordZ { x: $x, y: $y, z: $z }
    };
}

/// Creates a [`LineStringZ`] containing the given coordinates.
///
/// ```txt
/// line_string_z![coordZ OR (x: <number>, y: <number>, z: <number>), …]
/// ```
///
/// # Examples
///
/// Creating a [`LineStringZ`], supplying x/y/z values:
///
/// ```
/// use geo_types_3d::line_string_z;
///
/// let ls = line_string_z![
///     (x: -21.95156, y: 64.1446, z: 0.0),
///     (x: -21.951, y: 64.14479, z: 0.0),
///     (x: -21.95044, y: 64.14527, z: 0.0),
///     (x: -21.951445, y: 64.145508, z: 0.0),
/// ];
///
/// assert_eq!(ls[1], geo_types_3d::coordZ! {
///     x: -21.951,
///     y: 64.14479,
///     z: 0.0,
/// });
/// ```
///
/// Creating a [`LineStringZ`], supplying [`coordZ`]s:
///
/// ```
/// use geo_types_3d::line_string_z;
///
/// let coord1 = geo_types_3d::coordZ! {
///     x: -21.95156,
///     y: 64.1446,
///     z: 0.0,
/// };
/// let coord2 = geo_types_3d::coordZ! {
///     x: -21.951,
///     y: 64.14479,
///     z: 0.0,
/// };
/// let coord3 = geo_types_3d::coordZ! {
///     x: -21.95044,
///     y: 64.14527,
///     z: 0.0,
/// };
/// let coord4 = geo_types_3d::coordZ! {
///     x: -21.951445,
///     y: 64.145508,
///     z: 0.0,
/// };
///
/// let ls = line_string_z![coord1, coord2, coord3, coord4];
///
/// assert_eq!(
///     ls[1],
///     geo_types_3d::coordZ! {
///         x: -21.951,
///         y: 64.14479,
///         z: 0.0,
///     }
/// );
/// ```
///
/// [`coordZ`]: ./struct.coordZ.html
/// [`LineString`]: ./line_string/struct.LineString.html
#[macro_export]
macro_rules! line_string_z {
    () => { $crate::LineStringZ::empty() };
    (
        $(( $($tag:tt : $val:expr),* $(,)? )),*
        $(,)?
    ) => {
        line_string_z![
            $(
                $crate::coordZ! { $( $tag: $val , )* },
            )*
        ]
    };
    (
        $($coordZ:expr),*
        $(,)?
    ) => {
        $crate::LineStringZ::new(
            $crate::_alloc::vec![
                $($coordZ),*
            ]
        )
    };
}

/// Creates a [`PolygonZ`] containing the given coordinates.
///
/// ```txt
/// polygon_z![coordZ OR (x: <number>, y: <number>, z: <number>), …]
///
/// // or
///
/// polygon_z!(
///     exterior: [coordZ OR (x: <number>, y: <number>, z: <number>), …],
///     interiors: [
///         [coordZ OR (x: <number>, y: <number>, z: <number>), …],
///         …
///     ],
/// )
/// ```
///
/// # Examples
///
/// Creating a [`PolygonZ`] without interior rings, supplying x/y/z values:
///
/// ```
/// use geo_types_3d::polygon_z;
///
/// let poly = polygon_z![
///     (x: -111., y: 45., z: 0.0),
///     (x: -111., y: 41., z: 0.0),
///     (x: -104., y: 41., z: 0.0),
///     (x: -104., y: 45., z: 0.0),
/// ];
///
/// assert_eq!(
///     poly.exterior()[1],
///     geo_types_3d::coordZ! { x: -111., y: 41., z: 0.0 },
/// );
/// ```
///
/// Creating a [`PolygonZ`], supplying x/y/z values:
///
/// ```
/// use geo_types_3d::polygon_z;
///
/// let poly = polygon_z!(
///     exterior: [
///         (x: -111., y: 45., z: 0.0),
///         (x: -111., y: 41., z: 0.0),
///         (x: -104., y: 41., z: 0.0),
///         (x: -104., y: 45., z: 0.0),
///     ],
///     interiors: [
///         [
///             (x: -110., y: 44., z: 0.0),
///             (x: -110., y: 42., z: 0.0),
///             (x: -105., y: 42., z: 0.0),
///             (x: -105., y: 44., z: 0.0),
///         ],
///     ],
/// );
///
/// assert_eq!(
///     poly.exterior()[1],
///     geo_types_3d::coordZ! { x: -111., y: 41., z: 0.0 },
/// );
/// ```
///
/// [`coordZ`]: ./struct.coordZ.html
/// [`PolygonZ`]: ./struct.PolygonZ.html
#[macro_export]
macro_rules! polygon_z {
    () => { $crate::PolygonZ::empty() };
    (
        exterior: [
            $(( $($exterior_tag:tt : $exterior_val:expr),* $(,)? )),*
            $(,)?
        ],
        interiors: [
            $([
                $(( $($interior_tag:tt : $interior_val:expr),* $(,)? )),*
                $(,)?
            ]),*
            $(,)?
        ]
        $(,)?
    ) => {
        polygon_z!(
            exterior: [
                $(
                    $crate::coordZ! { $( $exterior_tag: $exterior_val , )* },
                )*
            ],
            interiors: [
                $([
                    $($crate::coordZ! { $( $interior_tag: $interior_val , )* }),*
                ]),*
            ],
        )
    };
    (
        exterior: [
            $($exterior_coord:expr),*
            $(,)?
        ],
        interiors: [
            $([
                $($interior_coord:expr),*
                $(,)?
            ]),*
            $(,)?
        ]
        $(,)?
    ) => {
        $crate::PolygonZ::new(
            $crate::line_string_z![
                $($exterior_coord), *
            ],
            $crate::_alloc::vec![
                $(
                    $crate::line_string_z![$($interior_coord),*]
                ), *
            ]
        )
    };
    (
        $(( $($tag:tt : $val:expr),* $(,)? )),*
        $(,)?
    ) => {
        polygon_z![
            $($crate::coordZ! { $( $tag: $val , )* }),*
        ]
    };
    (
        $($coordZ:expr),*
        $(,)?
    ) => {
        $crate::PolygonZ::new(
            $crate::line_string_z![$($coordZ,)*],
            $crate::_alloc::vec![],
        )
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_point() {
        let p = pointZ! { x: 1.2, y: 3.4, z: 5.6 };
        assert_eq!(p.x(), 1.2);
        assert_eq!(p.y(), 3.4);
        assert_eq!(p.z(), 5.6);

        let p = pointZ! {
            x: 1.2,
            y: 3.4,
            z: 5.6,
        };
        assert_eq!(p.x(), 1.2);
        assert_eq!(p.y(), 3.4);
        assert_eq!(p.z(), 5.6);

        let p = pointZ!(coordZ! { x: 1.2, y: 3.4, z: 5.6 });
        assert_eq!(p.x(), 1.2);
        assert_eq!(p.y(), 3.4);
        assert_eq!(p.z(), 5.6);

        let p = pointZ!(coordZ! { x: 1.2, y: 3.4, z: 5.6 },);
        assert_eq!(p.x(), 1.2);
        assert_eq!(p.y(), 3.4);
        assert_eq!(p.z(), 5.6);
    }

    #[test]
    fn test_line() {
        let ls = line_string_z![(x: -1.2f32, y: 3.4f32, z: 0.0)];
        assert_eq!(ls[0], coordZ! { x: -1.2, y: 3.4, z: 0.0 });

        let ls = line_string_z![
            (x: -1.2f32, y: 3.4f32, z: 0.0),
        ];
        assert_eq!(ls[0], coordZ! { x: -1.2, y: 3.4, z: 0.0 });

        let ls = line_string_z![
            (x: -1.2f32,
            y: 3.4f32,
            z: 0.0,
        )];
        assert_eq!(ls[0], coordZ! { x: -1.2, y: 3.4, z: 0.0 });

        let ls = line_string_z![
            (x: -1.2f32, y: 3.4f32, z: 0.0),
            (x: -5.6, y: 7.8, z: 0.0),
        ];
        assert_eq!(ls[0], coordZ! { x: -1.2, y: 3.4, z: 0.0 });
        assert_eq!(ls[1], coordZ! { x: -5.6, y: 7.8, z: 0.0 });
    }

    #[test]
    fn test_polygon() {
        let p = polygon_z!(
            exterior: [(x: 1, y: 2, z: 0)],
            interiors: [[(x: 3, y: 4, z: 0)]]
        );
        assert_eq!(p.exterior()[0], coordZ! { x: 1, y: 2, z: 0 });
        assert_eq!(p.interiors()[0][0], coordZ! { x: 3, y: 4, z: 0 });

        let p = polygon_z!(
            exterior: [(x: 1, y: 2, z: 0)],
            interiors: [[(x: 3, y: 4, z: 0)]],
        );
        assert_eq!(p.exterior()[0], coordZ! { x: 1, y: 2, z: 0 });
        assert_eq!(p.interiors()[0][0], coordZ! { x: 3, y: 4, z: 0 });

        let p = polygon_z!(
            exterior: [(x: 1, y: 2, z: 0, )],
            interiors: [[(x: 3, y: 4, z: 0, )]],
        );
        assert_eq!(p.exterior()[0], coordZ! { x: 1, y: 2, z: 0 });
        assert_eq!(p.interiors()[0][0], coordZ! { x: 3, y: 4, z: 0 });

        let p = polygon_z!(
            exterior: [(x: 1, y: 2, z: 0, ), ],
            interiors: [[(x: 3, y: 4, z: 0, ), ]],
        );
        assert_eq!(p.exterior()[0], coordZ! { x: 1, y: 2, z: 0 });
        assert_eq!(p.interiors()[0][0], coordZ! { x: 3, y: 4, z: 0 });

        let p = polygon_z!(
            exterior: [(x: 1, y: 2, z: 0, ), ],
            interiors: [[(x: 3, y: 4, z: 0, ), ], ],
        );
        assert_eq!(p.exterior()[0], coordZ! { x: 1, y: 2, z: 0 });
        assert_eq!(p.interiors()[0][0], coordZ! { x: 3, y: 4, z: 0 });
    }
}

