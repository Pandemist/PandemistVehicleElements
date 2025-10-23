use lotus_script::vehicle::{Bogie, VehicleError};

use crate::api::variable::get_var;

/// API wrapper for a train bogie (wheel truck assembly).
///
/// The `ApiBogie` struct provides a safe interface for interacting with
/// train bogies through the lotus_script vehicle system. A bogie is the
/// wheeled truck assembly underneath a railway vehicle that contains the
/// wheelsets and provides suspension.
///
/// # Examples
///
/// ```rust
/// use your_crate::ApiBogie;
///
/// let mut bogie = ApiBogie::new(0);
/// bogie.railbrake_force(1000.0); // Apply 1000N of braking force
/// ```
#[derive(Debug)]
pub struct ApiBogie {
    /// The underlying bogie instance, wrapped in a Result to handle
    /// potential errors during bogie retrieval.
    bogie: Result<Bogie, VehicleError>,
}

impl ApiBogie {
    /// Creates a new `ApiBogie` instance for the bogie with the specified ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the bogie to control
    ///
    /// # Returns
    ///
    /// Returns a new `ApiBogie` instance. The actual bogie retrieval is performed
    /// lazily, and any errors will be handled when methods are called.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_crate::ApiBogie;
    ///
    /// let bogie = ApiBogie::new(0); // Create wrapper for bogie ID 0
    /// ```
    #[must_use]
    pub fn new(id: usize) -> Self {
        Self {
            bogie: Bogie::get(id),
        }
    }

    /// Sets the rail brake force for this bogie.
    ///
    /// This method applies braking force to the bogie's rail brake system.
    /// The force is specified in Newtons. If the bogie could not be retrieved
    /// during initialization, this method will silently do nothing.
    ///
    /// # Arguments
    ///
    /// * `force` - The braking force to apply in Newtons (N). Positive values
    ///   indicate braking force, while negative values may release brakes
    ///   depending on the implementation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_crate::ApiBogie;
    ///
    /// let mut bogie = ApiBogie::new(0);
    /// bogie.railbrake_force(1500.0); // Apply 1500N braking force
    /// bogie.railbrake_force(0.0);    // Release brakes
    /// ```
    pub fn railbrake_force(&mut self, force: f32) {
        if let Ok(b) = self.bogie {
            b.set_rail_brake_force_newton(force);
        }
    }
}

/// Returns the maximum absolute inverse radius value.
///
/// This function retrieves the global variable "invradius_abs_max" which represents
/// the maximum absolute value of the inverse radius. The inverse radius is typically
/// used in railway calculations to determine curve tightness - a higher inverse radius
/// indicates a tighter curve.
///
/// The German comment in the original code indicates this corresponds to the variable
/// "invradius_abs_max".
///
/// # Returns
///
/// Returns the maximum absolute inverse radius as a 32-bit floating point number.
/// The units depend on the specific implementation in the lotus_script system.
///
/// # Examples
///
/// ```rust
/// use your_crate::invradius_abs_max;
///
/// let max_inv_radius = invradius_abs_max();
/// println!("Maximum inverse radius: {}", max_inv_radius);
/// ```
///
/// # Panics
///
/// This function may panic if the underlying variable system cannot retrieve
/// the "invradius_abs_max" variable or if it cannot be converted to f32.
#[must_use]
pub fn invradius_abs_max() -> f32 {
    get_var::<f32>("invradius_abs_max")
}
