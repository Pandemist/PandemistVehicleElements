//! Vehicle information utilities for Lotus Script integration.
//!
//! This module provides functions to access and manipulate vehicle-related
//! information and physics data through the Lotus Script.

use lotus_script::{
    math::Vec3,
    var::{get_var, set_var},
    vehicle::{acceleration_vs_ground, velocity_vs_ground},
};

/// Gets the current vehicle number identifier.
///
/// Returns the vehicle number as stored in the Lotus Script variable system.
/// This identifier is typically used to distinguish between different vehicles
/// in multi-vehicle scenarios.
///
/// # Returns
///
/// A `String` containing the vehicle number identifier.
///
/// # Examples
///
/// ```rust
/// let vehicle_id = veh_number();
/// println!("Current vehicle: {}", vehicle_id);
/// ```
#[must_use]
pub fn veh_number() -> String {
    get_var::<String>("veh_number")
}

/// Sets the vehicle number identifier.
///
/// Updates the vehicle number in the Lotus Script variable system.
/// This allows changing the active vehicle identifier at runtime.
///
/// # Arguments
///
/// * `value` - The new vehicle number identifier to set
///
/// # Examples
///
/// ```rust
/// set_veh_number("7351".to_string());
/// ```
pub fn set_veh_number(value: impl Into<String>) {
    set_var("veh_number", value.into());
}

/// Gets the current vehicle registration information.
///
/// Returns the vehicle registration data as stored in the Lotus Script
/// variable system. This typically contains license plate or registration
/// identification information.
///
/// # Returns
///
/// A `String` containing the vehicle registration information.
///
/// # Examples
///
/// ```rust
/// let registration = veh_registration();
/// println!("Vehicle registration: {}", registration);
/// ```
#[must_use]
pub fn veh_registration() -> String {
    get_var::<String>("veh_registration")
}

/// Sets the vehicle registration information.
///
/// Updates the vehicle registration data in the Lotus Script variable system.
/// This allows changing the vehicle's registration information at runtime.
///
/// # Arguments
///
/// * `value` - The new registration information to set
///
/// # Examples
///
/// ```rust
/// set_veh_registration("B-V 3323".to_string());
/// ```
pub fn set_veh_registration(value: impl Into<String>) {
    set_var("veh_registration", value.into());
}

/// Gets the current vehicle velocity relative to the ground.
///
/// Returns the magnitude of the vehicle's velocity vector relative to the
/// ground surface. This is calculated by the Lotus Script physics engine
/// and represents the actual speed of the vehicle.
/// Calls the Lotus Script function:
/// ```rust
/// velocity_vs_ground()
/// ```
///
/// # Returns
///
/// A `f32` value representing the velocity magnitude in meters
/// per second.
///
/// # Examples
///
/// ```rust
/// let speed = v_ground();
/// println!("Current speed: {:.2} m/s", speed);
/// ```
#[must_use]
pub fn v_ground() -> f32 {
    velocity_vs_ground()
}

/// Gets the current vehicle acceleration relative to the ground.
///
/// Returns the magnitude of the vehicle's acceleration vector relative to
/// the ground surface. This represents how quickly the vehicle's velocity
/// is changing and is calculated by the Lotus Script physics engine.
/// Calls the Lotus Script function:
/// ```rust
/// acceleration_vs_ground()
/// ```
///
/// # Returns
///
/// A `f32` value representing the acceleration magnitude in meters per
/// second squared.
///
/// # Examples
///
/// ```rust
/// let accel = a_ground();
/// println!("Current acceleration: {:.2} m/s²", accel);
/// ```
#[must_use]
pub fn a_ground() -> f32 {
    acceleration_vs_ground()
}

/// Gets the current vehicle acceleration as a 3D vector.
///
/// Returns the vehicle's acceleration broken down into X, Y, and Z components.
/// These values are retrieved from the Lotus Script variable system and
/// represent the acceleration in each spatial dimension.
///
/// # Returns
///
/// A `Vec3` struct containing:
/// - `x`: Acceleration along the X-axis
/// - `y`: Acceleration along the Y-axis  
/// - `z`: Acceleration along the Z-axis
///
/// All values are in the simulation's units (typically meters per second squared).
///
/// # Examples
///
/// ```rust
/// let accel_vec = acceleration_vec();
/// println!("Acceleration - X: {:.2}, Y: {:.2}, Z: {:.2}",
///          accel_vec.x, accel_vec.y, accel_vec.z);
/// ```
///
#[must_use]
pub fn acceleration_vec() -> Vec3 {
    Vec3 {
        x: get_var::<f32>("acc_x"),
        y: get_var::<f32>("acc_y"),
        z: get_var::<f32>("acc_z"),
    }
}
