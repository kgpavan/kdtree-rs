//! Defines different distance metrics, in simplest case it defines the
//! euclidean distance which is no more than the square root of the sum of the
//! squares of the distances in each dimension.

/// Returns the squared euclidean distance between two points, when you only
/// need to compare distances, rhather than having the exact distance between
/// the points this metric is benefitial because it avoids the expensive square
/// root computation.
///
/// Examples:
/// ---------
/// ```
/// use kdtree::distance::squared_euclidean;
///
/// assert!(0.0 == squared_euclidean(&[0.0, 0.0], &[0.0, 0.0]));
/// assert!(2.0 == squared_euclidean(&[0.0, 0.0], &[1.0, 1.0]));
/// assert!(1.0 == squared_euclidean(&[0.0, 0.0], &[1.0, 0.0]));
/// ```
///
/// Panics:
/// -------
/// Only in debug mode, the lenght of the slices at input will be compared, if
/// they do not match, there will be a panic:
///
/// ```rust,should_panic
/// # use kdtree::distance::squared_euclidean;
/// // this is broken
/// let _ = squared_euclidean(&[0.0, 0.0], &[1.0, 0.0, 0.0]);
/// ```
pub fn squared_euclidean(a: &[f64], b: &[f64]) -> f64 {
    debug_assert!(a.len() == b.len());
    a.iter().zip(b.iter())
            .map(|(x, y)| (x - y) * (x - y))
            .fold(0f64, ::std::ops::Add::add)
}
