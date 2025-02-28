/// Kernels to compute various predicates
pub mod kernels;
pub use kernels::{HasKernel, Kernel, Orientation};

/// Calculate the area of the surface of a `Geometry`.
pub mod area;
pub use area::Area;

/// Calculate the bearing to another `Point`, in degrees.
pub mod bearing;
pub use bearing::Bearing;

/// Boolean Ops such as union, xor, difference;
pub mod bool_ops;
pub use bool_ops::{BooleanOps, OpType};

/// Calculate the bounding rectangle of a `Geometry`.
pub mod bounding_rect;
pub use bounding_rect::BoundingRect;

/// Calculate the centroid of a `Geometry`.
pub mod centroid;
pub use centroid::Centroid;

/// Smoothen `LineString`, `Polygon`, `MultiLineString` and `MultiPolygon` using Chaikins algorithm.
pub mod chaikin_smoothing;
pub use chaikin_smoothing::ChaikinSmoothing;

/// Calculate the signed approximate geodesic area of a `Geometry`.
pub mod chamberlain_duquette_area;
pub use chamberlain_duquette_area::ChamberlainDuquetteArea;

/// Calculate the closest `Point` between a `Geometry` and an input `Point`.
pub mod closest_point;
pub use closest_point::ClosestPoint;

/// Calculate the concave hull of a `Geometry`.
pub mod concave_hull;
pub use concave_hull::ConcaveHull;

/// Determine whether `Geometry` `A` completely encloses `Geometry` `B`.
pub mod contains;
pub use contains::Contains;

/// Convert the type of a geometry’s coordinate value.
pub mod convert;
pub use convert::{Convert, TryConvert};

/// Calculate the convex hull of a `Geometry`.
pub mod convex_hull;
pub use convex_hull::ConvexHull;

/// Determine whether a `Coord` lies inside, outside, or on the boundary of a geometry.
pub mod coordinate_position;
pub use coordinate_position::CoordinatePosition;

/// Iterate over geometry coordinates.
pub mod coords_iter;
pub use coords_iter::CoordsIter;

/// Densify linear geometry components
pub mod densify;
pub use densify::Densify;

/// Dimensionality of a geometry and its boundary, based on OGC-SFA.
pub mod dimensions;
pub use dimensions::HasDimensions;

/// Calculate the minimum Euclidean distance between two `Geometries`.
pub mod euclidean_distance;
pub use euclidean_distance::EuclideanDistance;

/// Calculate the length of a planar line between two `Geometries`.
pub mod euclidean_length;
pub use euclidean_length::EuclideanLength;

/// Calculate the extreme coordinates and indices of a geometry.
pub mod extremes;
pub use extremes::Extremes;

/// Calculate the Frechet distance between two `LineStrings`.
pub mod frechet_distance;
pub use frechet_distance::FrechetDistance;

/// Calculate the Geodesic distance between two `Point`s.
pub mod geodesic_distance;
pub use geodesic_distance::GeodesicDistance;

/// Calculate a new `Point` lying on a Geodesic arc between two `Point`s.
pub mod geodesic_intermediate;
pub use geodesic_intermediate::GeodesicIntermediate;

/// Calculate the Geodesic length of a line.
pub mod geodesic_length;
pub use geodesic_length::GeodesicLength;

/// Calculate a destination `Point`, given a distance and a bearing.
pub mod haversine_destination;
pub use haversine_destination::HaversineDestination;

/// Calculate the Haversine distance between two `Geometries`.
pub mod haversine_distance;
pub use haversine_distance::HaversineDistance;

/// Calculate a new `Point` lying on a Great Circle arc between two `Point`s.
pub mod haversine_intermediate;
pub use haversine_intermediate::HaversineIntermediate;

/// Calculate the Haversine length of a Line.
pub mod haversine_length;
pub use haversine_length::HaversineLength;

/// Calculate a representative `Point` inside a `Geometry`
pub mod interior_point;
pub use interior_point::InteriorPoint;

/// Determine whether `Geometry` `A` intersects `Geometry` `B`.
pub mod intersects;
pub use intersects::Intersects;

/// Determines whether a `LineString` is convex.
pub mod is_convex;
pub use is_convex::IsConvex;

/// Calculate concave hull using k-nearest algorithm
pub mod k_nearest_concave_hull;
pub use k_nearest_concave_hull::KNearestConcaveHull;

/// Interpolate a point along a `Line` or `LineString`.
pub mod line_interpolate_point;
pub use line_interpolate_point::LineInterpolatePoint;

/// Computes the intersection of two Lines.
pub mod line_intersection;
pub use line_intersection::LineIntersection;

/// Locate a point along a `Line` or `LineString`.
pub mod line_locate_point;
pub use line_locate_point::LineLocatePoint;

/// Iterate over the lines in a geometry.
pub mod lines_iter;
pub use lines_iter::LinesIter;

/// Apply a function to all `Coord`s of a `Geometry`.
pub mod map_coords;
pub use map_coords::{MapCoords, MapCoordsInPlace};
#[allow(deprecated)]
pub use map_coords::{MapCoordsInplace, TryMapCoords, TryMapCoordsInplace};

/// Orient a `Polygon`'s exterior and interior rings.
pub mod orient;
pub use orient::Orient;

/// Coordinate projections and transformations using the current stable version of [PROJ](http://proj.org).
#[cfg(feature = "use-proj")]
pub mod proj;

/// Relate two geometries based on DE-9IM
pub mod relate;
pub use relate::Relate;

/// Remove (consecutive) repeated points
pub mod remove_repeated_points;
pub use remove_repeated_points::RemoveRepeatedPoints;

/// Rotate a `Geometry` by an angle given in degrees.
pub mod rotate;
pub use rotate::Rotate;
#[doc(hidden)]
#[allow(deprecated)]
pub use rotate::RotatePoint;

/// Scale a `Geometry` up or down by a factor
pub mod scale;
pub use scale::Scale;

/// Skew a `Geometry` by shearing it at angles along the x and y dimensions
pub mod skew;
pub use skew::Skew;

/// Composable affine operations such as rotate, scale, skew, and translate
pub mod affine_ops;
pub use affine_ops::{AffineOps, AffineTransform};

/// Simplify `Geometries` using the Ramer-Douglas-Peucker algorithm.
pub mod simplify;
pub use simplify::{Simplify, SimplifyIdx};

/// Simplify `Geometries` using the Visvalingam-Whyatt algorithm. Includes a topology-preserving variant.
pub mod simplify_vw;
pub use simplify_vw::{SimplifyVw, SimplifyVwIdx, SimplifyVwPreserve};

/// Transform a geometry using PROJ.
#[cfg(feature = "use-proj")]
pub mod transform;
#[cfg(feature = "use-proj")]
pub use transform::Transform;

/// Translate a `Geometry` along the given offsets.
pub mod translate;
pub use translate::Translate;

/// Calculate the Vincenty distance between two `Point`s.
pub mod vincenty_distance;
pub use vincenty_distance::VincentyDistance;

/// Calculate the Vincenty length of a `LineString`.
pub mod vincenty_length;
pub use vincenty_length::VincentyLength;

/// Calculate and work with the winding order of `Linestring`s.
pub mod winding_order;
pub use winding_order::Winding;

/// Determine whether `Geometry` `A` is completely within by `Geometry` `B`.
pub mod within;
pub use within::Within;

/// Planar sweep algorithm and related utils
pub mod sweep;

/// Detect outliers in a group of points using [LOF](https://en.wikipedia.org/wiki/Local_outlier_factor)
pub mod outlier_detection;
pub use outlier_detection::OutlierDetection;
