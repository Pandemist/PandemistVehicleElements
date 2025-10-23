//! Steering control module for vehicle simulation.
//!
//! This module provides a high-level interface for managing steering input
//! in vehicle simulations using the lotus_script variable system.

use lotus_script::var::{get_var, set_var};

/// A steering controller that manages steering input for vehicle simulation.
///
/// The `Steering` struct provides a convenient interface to read and write
/// steering values using the underlying lotus_script variable system. It stores
/// a name identifier and provides methods to get and set steering values.
///
/// # Examples
///
/// ```rust
/// use your_crate::Steering;
///
/// let mut steering = Steering::new("main_steering");
///
/// // Set steering to half lock to the right
/// steering.set_steering(0.5);
///
/// // Read current steering value
/// let current_steering = steering.steering();
/// println!("Current steering: {}", current_steering);
/// ```
#[derive(Default, Debug)]
pub struct Steering {
    /// The name identifier for this steering controller
    name: String,
}

impl Steering {
    /// Creates a new steering controller with the specified name.
    ///
    /// # Arguments
    ///
    /// * `name` - A string-like value that will be used as the identifier for this steering controller
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_crate::Steering;
    ///
    /// let steering = Steering::new("player_steering");
    /// let steering_from_string = Steering::new(String::from("ai_steering"));
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Gets the current steering value from the lotus_script variable system.
    ///
    /// Returns the current steering value as a floating-point number, typically
    /// in the range [-1.0, 1.0] where -1.0 represents full left steering,
    /// 0.0 represents center/no steering, and 1.0 represents full right steering.
    ///
    /// # Returns
    ///
    /// The current steering value as an `f32`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_crate::Steering;
    ///
    /// let steering = Steering::new("main");
    /// let value = steering.steering();
    ///
    /// if value > 0.0 {
    ///     println!("Steering right: {}", value);
    /// } else if value < 0.0 {
    ///     println!("Steering left: {}", value.abs());
    /// } else {
    ///     println!("Steering centered");
    /// }
    /// ```
    #[must_use]
    pub fn steering(&self) -> f32 {
        get_var::<f32>("Steering")
    }

    /// Sets the steering value in the lotus_script variable system.
    ///
    /// Updates the steering value to the specified amount. The value typically
    /// should be in the range [-1.0, 1.0] for proper steering behavior, though
    /// the underlying system may accept values outside this range.
    ///
    /// # Arguments
    ///
    /// * `value` - The new steering value to set. Conventionally:
    ///   * `-1.0` = full left steering
    ///   * `0.0` = center/no steering  
    ///   * `1.0` = full right steering
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_crate::Steering;
    ///
    /// let mut steering = Steering::new("player");
    ///
    /// // Steer half-way to the left
    /// steering.set_steering(-0.5);
    ///
    /// // Center the steering
    /// steering.set_steering(0.0);
    ///
    /// // Full right steering
    /// steering.set_steering(1.0);
    /// ```
    pub fn set_steering(&mut self, value: f32) {
        set_var("Steering", value);
    }
}
