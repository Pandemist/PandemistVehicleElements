//! # Piecewise Linear Function
//!
//! A Rust library for creating and evaluating piecewise linear functions.
//!
//! A piecewise linear function is defined by a series of connected line segments,
//! where each segment connects two consecutive points. This library provides
//! efficient storage, modification, and evaluation of such functions.
//!
//! ## Features
//!
//! - **Sorted storage**: Points are automatically kept sorted by x-coordinate
//! - **Linear interpolation**: Values between points are calculated using linear interpolation
//! - **Edge case handling**: Values outside the defined range return the nearest endpoint value
//! - **Duplicate handling**: Points with the same x-coordinate overwrite previous values
//! - **Error handling**: Comprehensive error handling for invalid inputs
//!
//! ## Quick Start
//!
//! ```rust
//! use piecewise_linear_function::PiecewiseLinearFunction;
//!
//! // Create a function from a vector of points
//! let points = vec![(0.0, 1.0), (2.0, 3.0), (4.0, 2.0)];
//! let function = PiecewiseLinearFunction::new(points);
//!
//! // Evaluate the function at different points
//! assert_eq!(function.get_value(0.0).unwrap(), 1.0);  // Exact match
//! assert_eq!(function.get_value(1.0).unwrap(), 2.0);  // Interpolated
//! assert_eq!(function.get_value(5.0).unwrap(), 2.0);  // Beyond range
//! ```
//!
//! ## Building Functions
//!
//! You can create piecewise linear functions in several ways:
//!
//! ```rust
//! use piecewise_linear_function::PiecewiseLinearFunction;
//!
//! // From a vector of points
//! let function1 = PiecewiseLinearFunction::new(vec![(0.0, 1.0), (1.0, 2.0)]);
//!
//! // Start empty and add points
//! let mut function2 = PiecewiseLinearFunction::empty();
//! function2.add_point(0.0, 1.0).unwrap();
//! function2.add_point(1.0, 2.0).unwrap();
//!
//! // From an iterator
//! let points = vec![(0.0, 1.0), (1.0, 2.0)];
//! let function3: PiecewiseLinearFunction = points.into_iter().collect();
//! ```

use std::fmt;

/// Errors that can occur when working with piecewise linear functions.
#[derive(Debug, Clone, PartialEq)]
pub enum PiecewiseError {
    /// The function has no points defined and cannot be evaluated.
    EmptyFunction,
    /// A point contains invalid coordinates (NaN or infinite values).
    InvalidPoint,
}

impl fmt::Display for PiecewiseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PiecewiseError::EmptyFunction => write!(f, "Function has no points defined"),
            PiecewiseError::InvalidPoint => write!(f, "Invalid point coordinates"),
        }
    }
}

impl std::error::Error for PiecewiseError {}

/// A piecewise linear function defined by a series of connected line segments.
///
/// The function is represented by a collection of points (x, y), where consecutive
/// points are connected by straight lines. The points are automatically sorted by
/// their x-coordinates for efficient evaluation.
///
/// # Behavior
///
/// - **Interpolation**: For x-values between defined points, the function uses linear interpolation
/// - **Extrapolation**: For x-values outside the defined range, the function returns the y-value of the nearest endpoint
/// - **Duplicate x-values**: Adding a point with an existing x-coordinate overwrites the previous y-value
///
/// # Examples
///
/// ```rust
/// use piecewise_linear_function::PiecewiseLinearFunction;
///
/// let mut function = PiecewiseLinearFunction::empty();
/// function.add_point(0.0, 0.0).unwrap();
/// function.add_point(1.0, 2.0).unwrap();
/// function.add_point(2.0, 1.0).unwrap();
///
/// // Evaluate at defined points
/// assert_eq!(function.get_value(0.0).unwrap(), 0.0);
/// assert_eq!(function.get_value(1.0).unwrap(), 2.0);
///
/// // Interpolation between points
/// assert_eq!(function.get_value(0.5).unwrap(), 1.0);
/// assert_eq!(function.get_value(1.5).unwrap(), 1.5);
///
/// // Extrapolation beyond range
/// assert_eq!(function.get_value(-1.0).unwrap(), 0.0);  // Returns first point's y
/// assert_eq!(function.get_value(3.0).unwrap(), 1.0);   // Returns last point's y
/// ```
#[derive(Debug)]
pub struct PiecewiseLinearFunction {
    /// Internal storage of points, kept sorted by x-coordinate.
    points: Vec<(f32, f32)>,
}

impl PiecewiseLinearFunction {
    /// Creates a new piecewise linear function from a vector of points.
    ///
    /// The points will be automatically sorted by their x-coordinates, and any
    /// duplicate x-coordinates will result in the last y-value being kept.
    ///
    /// # Arguments
    ///
    /// * `points` - A vector of (x, y) coordinate pairs
    ///
    /// # Examples
    ///
    /// ```rust
    /// use piecewise_linear_function::PiecewiseLinearFunction;
    ///
    /// // Points can be provided in any order
    /// let points = vec![(2.0, 4.0), (0.0, 0.0), (1.0, 2.0)];
    /// let function = PiecewiseLinearFunction::new(points);
    ///
    /// assert_eq!(function.len(), 3);
    /// assert_eq!(function.get_value(0.5).unwrap(), 1.0);
    /// ```
    #[must_use]
    pub fn new(points: Vec<(f32, f32)>) -> Self {
        let mut fun = Self { points: Vec::new() };

        for (x, y) in points {
            fun.add_point_unchecked(x, y);
        }

        fun
    }

    /// Creates an empty piecewise linear function with no points.
    ///
    /// Points can be added later using [`add_point`](Self::add_point).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use piecewise_linear_function::PiecewiseLinearFunction;
    ///
    /// let mut function = PiecewiseLinearFunction::empty();
    /// assert!(function.is_empty());
    ///
    /// function.add_point(1.0, 1.0).unwrap();
    /// assert!(!function.is_empty());
    /// ```
    #[must_use]
    pub fn empty() -> Self {
        Self { points: Vec::new() }
    }

    /// Adds a point to the function.
    ///
    /// The point will be inserted in the correct position to maintain sorted order
    /// by x-coordinate. If a point with the same x-coordinate already exists,
    /// its y-value will be updated.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the point (must be finite)
    /// * `y` - The y-coordinate of the point (must be finite)
    ///
    /// # Errors
    ///
    /// Returns [`PiecewiseError::InvalidPoint`] if either coordinate is NaN or infinite.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use piecewise_linear_function::PiecewiseLinearFunction;
    ///
    /// let mut function = PiecewiseLinearFunction::empty();
    ///
    /// // Add points in any order
    /// function.add_point(2.0, 4.0).unwrap();
    /// function.add_point(0.0, 0.0).unwrap();
    /// function.add_point(1.0, 2.0).unwrap();
    ///
    /// // Update existing point
    /// function.add_point(1.0, 3.0).unwrap();
    /// assert_eq!(function.get_value(1.0).unwrap(), 3.0);
    ///
    /// // Invalid coordinates are rejected
    /// assert!(function.add_point(f32::NAN, 1.0).is_err());
    /// assert!(function.add_point(1.0, f32::INFINITY).is_err());
    /// ```
    pub fn add_point(&mut self, x: f32, y: f32) -> Result<(), PiecewiseError> {
        if !x.is_finite() || !y.is_finite() {
            return Err(PiecewiseError::InvalidPoint);
        }

        self.add_point_unchecked(x, y);
        Ok(())
    }

    /// Adds a point without validation (internal use only).
    ///
    /// This method assumes the coordinates are valid and finite.
    fn add_point_unchecked(&mut self, x: f32, y: f32) {
        match self
            .points
            .binary_search_by(|&(px, _)| px.partial_cmp(&x).unwrap())
        {
            Ok(pos) => {
                self.points[pos].1 = y;
            }
            Err(pos) => {
                self.points.insert(pos, (x, y));
            }
        }
    }

    /// Returns the number of points in the function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use piecewise_linear_function::PiecewiseLinearFunction;
    ///
    /// let function = PiecewiseLinearFunction::new(vec![(0.0, 1.0), (1.0, 2.0)]);
    /// assert_eq!(function.len(), 2);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Returns `true` if the function contains no points.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use piecewise_linear_function::PiecewiseLinearFunction;
    ///
    /// let empty_function = PiecewiseLinearFunction::empty();
    /// assert!(empty_function.is_empty());
    ///
    /// let function = PiecewiseLinearFunction::new(vec![(0.0, 1.0)]);
    /// assert!(!function.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Returns the range of y-values in the function.
    ///
    /// This returns the minimum and maximum y-values among all defined points.
    /// Note that interpolated values between points might fall outside this range.
    ///
    /// # Returns
    ///
    /// * `Some((min_y, max_y))` if the function has points
    /// * `None` if the function is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// use piecewise_linear_function::PiecewiseLinearFunction;
    ///
    /// let function = PiecewiseLinearFunction::new(vec![
    ///     (0.0, 1.0),
    ///     (1.0, 5.0),
    ///     (2.0, 2.0)
    /// ]);
    ///
    /// assert_eq!(function.range(), Some((1.0, 5.0)));
    ///
    /// let empty_function = PiecewiseLinearFunction::empty();
    /// assert_eq!(empty_function.range(), None);
    /// ```
    #[must_use]
    pub fn range(&self) -> Option<(f32, f32)> {
        if self.points.is_empty() {
            return None;
        }

        let mut min_y = self.points[0].1;
        let mut max_y = self.points[0].1;

        for &(_, y) in &self.points {
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }

        Some((min_y, max_y))
    }

    /// Evaluates the function at the given x-coordinate, returning 0.0 for empty functions.
    ///
    /// This is a convenience method that returns a default value instead of an error
    /// when the function is empty.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate to evaluate
    ///
    /// # Returns
    ///
    /// The function value at x, or 0.0 if the function is empty or x is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use piecewise_linear_function::PiecewiseLinearFunction;
    ///
    /// let function = PiecewiseLinearFunction::new(vec![(0.0, 1.0), (1.0, 2.0)]);
    /// assert_eq!(function.get_value_or_default(0.5), 1.5);
    ///
    /// let empty_function = PiecewiseLinearFunction::empty();
    /// assert_eq!(empty_function.get_value_or_default(0.5), 0.0);
    /// ```
    pub fn get_value_or_default(&self, x: f32) -> f32 {
        self.get_value(x).unwrap_or(0.0)
    }

    /// Evaluates the function at the given x-coordinate.
    ///
    /// The function uses linear interpolation between defined points. For x-values
    /// outside the defined range, it returns the y-value of the nearest endpoint.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate to evaluate (must be finite)
    ///
    /// # Returns
    ///
    /// * `Ok(y)` - The function value at x
    /// * `Err(PiecewiseError::EmptyFunction)` - If the function has no points
    /// * `Err(PiecewiseError::InvalidPoint)` - If x is NaN or infinite
    ///
    /// # Examples
    ///
    /// ```rust
    /// use piecewise_linear_function::PiecewiseLinearFunction;
    ///
    /// let function = PiecewiseLinearFunction::new(vec![
    ///     (0.0, 0.0),
    ///     (2.0, 4.0),
    ///     (4.0, 2.0)
    /// ]);
    ///
    /// // Exact matches
    /// assert_eq!(function.get_value(0.0).unwrap(), 0.0);
    /// assert_eq!(function.get_value(2.0).unwrap(), 4.0);
    ///
    /// // Linear interpolation
    /// assert_eq!(function.get_value(1.0).unwrap(), 2.0);
    /// assert_eq!(function.get_value(3.0).unwrap(), 3.0);
    ///
    /// // Extrapolation (returns nearest endpoint)
    /// assert_eq!(function.get_value(-1.0).unwrap(), 0.0);
    /// assert_eq!(function.get_value(5.0).unwrap(), 2.0);
    ///
    /// // Error cases
    /// let empty = PiecewiseLinearFunction::empty();
    /// assert!(empty.get_value(1.0).is_err());
    /// assert!(function.get_value(f32::NAN).is_err());
    /// ```
    pub fn get_value(&self, x: f32) -> Result<f32, PiecewiseError> {
        if self.points.is_empty() {
            return Err(PiecewiseError::EmptyFunction);
        }

        if !x.is_finite() {
            return Err(PiecewiseError::InvalidPoint);
        }

        // edge case, smaller than the existing values
        if x <= self.points[0].0 {
            return Ok(self.points[0].1);
        }

        // edge case, greater than the existing values
        if x >= self.points[self.points.len() - 1].0 {
            return Ok(self.points[self.points.len() - 1].1);
        }

        let pos = self
            .points
            .binary_search_by(|&(px, _)| px.partial_cmp(&x).unwrap());

        match pos {
            Ok(index) => {
                // Exact match found
                Ok(self.points[index].1)
            }
            Err(index) => {
                let (x0, y0) = self.points[index - 1];
                let (x1, y1) = self.points[index];

                // Linear interpolation
                let interpolated_y = y0 + (x - x0) * (y1 - y0) / (x1 - x0);
                Ok(interpolated_y)
            }
        }
    }
}

impl Default for PiecewiseLinearFunction {
    /// Creates an empty piecewise linear function.
    ///
    /// This is equivalent to calling [`PiecewiseLinearFunction::empty()`].
    fn default() -> Self {
        Self::empty()
    }
}

impl FromIterator<(f32, f32)> for PiecewiseLinearFunction {
    /// Creates a piecewise linear function from an iterator of points.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use piecewise_linear_function::PiecewiseLinearFunction;
    ///
    /// let points = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 1.5)];
    /// let function: PiecewiseLinearFunction = points.into_iter().collect();
    ///
    /// assert_eq!(function.len(), 3);
    /// assert_eq!(function.get_value(0.5).unwrap(), 1.5);
    /// ```
    fn from_iter<T: IntoIterator<Item = (f32, f32)>>(iter: T) -> Self {
        let points: Vec<_> = iter.into_iter().collect();
        Self::new(points)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let mut piecewise = PiecewiseLinearFunction::empty();

        piecewise.add_point(0.0, 1.0).unwrap();
        piecewise.add_point(2.0, 3.0).unwrap();
        piecewise.add_point(5.0, 2.0).unwrap();

        assert_eq!(piecewise.get_value(0.0).unwrap(), 1.0);
        assert_eq!(piecewise.get_value(1.0).unwrap(), 2.0);
        assert_eq!(piecewise.get_value(3.5).unwrap(), 2.5);
        assert_eq!(piecewise.get_value(6.0).unwrap(), 2.0);
    }

    #[test]
    fn test_from_vec() {
        let points = vec![(0.0, 1.0), (5.0, 2.0), (2.0, 3.0)]; // Unsorted
        let piecewise = PiecewiseLinearFunction::new(points);

        assert_eq!(piecewise.len(), 3);
        assert_eq!(piecewise.get_value(1.0).unwrap(), 2.0);
    }

    #[test]
    fn test_empty_function() {
        let piecewise = PiecewiseLinearFunction::empty();
        assert!(piecewise.get_value(1.0).is_err());
        assert!(piecewise.is_empty());
    }

    #[test]
    fn test_duplicate_x_values() {
        let mut piecewise = PiecewiseLinearFunction::empty();
        piecewise.add_point(1.0, 2.0).unwrap();
        piecewise.add_point(1.0, 3.0).unwrap(); // Overwrites the previous value

        assert_eq!(piecewise.len(), 1);
        assert_eq!(piecewise.get_value(1.0).unwrap(), 3.0);
    }

    #[test]
    fn test_range() {
        let piecewise = PiecewiseLinearFunction::new(vec![(1.0, 5.0), (3.0, 2.0), (5.0, 8.0)]);

        assert_eq!(piecewise.range(), Some((2.0, 8.0)));
    }

    #[test]
    fn test_invalid_values() {
        let mut piecewise = PiecewiseLinearFunction::empty();
        assert!(piecewise.add_point(f32::NAN, 1.0).is_err());
        assert!(piecewise.add_point(1.0, f32::INFINITY).is_err());

        piecewise.add_point(1.0, 1.0).unwrap();
        assert!(piecewise.get_value(f32::NAN).is_err());
    }

    #[test]
    fn test_from_iterator() {
        let points = vec![(0.0, 1.0), (2.0, 3.0), (5.0, 2.0)];
        let piecewise: PiecewiseLinearFunction = points.into_iter().collect();

        assert_eq!(piecewise.len(), 3);
        assert_eq!(piecewise.get_value(1.0).unwrap(), 2.0);
    }

    #[test]
    fn test_get_value_or_default() {
        let function = PiecewiseLinearFunction::new(vec![(0.0, 1.0), (1.0, 2.0)]);
        assert_eq!(function.get_value_or_default(0.5), 1.5);

        let empty_function = PiecewiseLinearFunction::empty();
        assert_eq!(empty_function.get_value_or_default(0.5), 0.0);
    }
}
