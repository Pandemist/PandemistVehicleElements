//! General utility functions for interacting with Lotus Script variables and input.
//!
//! This module provides a high-level interface for accessing commonly used
//! variables and input methods in the Lotus Script environment. It wraps
//! the lower-level `lotus_script` crate functionality with convenient,
//! type-safe functions.

use lotus_script::prelude::get_var;
use lotus_script::var::set_var;
use lotus_script::{input::mouse_delta, math::Vec2};

/// Gets the night texture setting.
///
/// This function retrieves the current night texture configuration value,
/// which is typically used to determine rendering behavior during nighttime
/// or low-light scenarios.
///
/// # Returns
///
/// Returns an `i32` representing the night texture setting. The specific
/// values and their meanings depend on the application's texture system.
///
/// # Examples
///
/// ```rust
/// let night_setting = night_tex();
/// println!("Night texture setting: {}", night_setting);
/// ```
///
/// # Note
///
/// This function corresponds to the Lotus Script variable "NightTex".
#[must_use]
pub fn night_tex() -> i32 {
    get_var::<i32>("NightTex")
}

/// Gets the current environment brightness level.
///
/// Retrieves the ambient brightness value for the environment, which is
/// commonly used for lighting calculations and visual effects.
///
/// # Returns
///
/// Returns an `f32` value representing the environment brightness level.
/// Values typically range from 0.0 (completely dark) to 1.0 (fully bright),
/// though the exact range may vary depending on the application.
///
/// # Examples
///
/// ```rust
/// let brightness = env_brightness();
/// if brightness < 0.5 {
///     println!("Environment is relatively dark: {}", brightness);
/// }
/// ```
///
/// # Note
///
/// This function corresponds to the Lotus Script variable "EnvirBrightness".
#[must_use]
pub fn env_brightness() -> f32 {
    get_var::<f32>("EnvirBrightness")
}

/// Gets the surface brightness level.
///
/// Retrieves the brightness value specifically for surface rendering,
/// which may differ from the general environment brightness and is often
/// used for material and surface lighting calculations.
///
/// # Returns
///
/// Returns an `f32` value representing the surface brightness level.
/// Like environment brightness, this typically ranges from 0.0 to 1.0.
///
/// # Examples
///
/// ```rust
/// let surface_bright = surface_brightness();
/// let env_bright = env_brightness();
///
/// if surface_bright != env_bright {
///     println!("Surface brightness ({}) differs from environment brightness ({})",
///              surface_bright, env_bright);
/// }
/// ```
///
/// # Note
///
/// This function corresponds to the Lotus Script variable "EnvirBrightnessSurface".
#[must_use]
pub fn surface_brightness() -> f32 {
    get_var::<f32>("EnvirBrightnessSurface")
}

/// Gets the district lighting value.
///
/// Retrieves the lighting level for a specific district or area,
/// which can be used for location-based lighting effects and
/// environmental storytelling.
///
/// # Returns
///
/// Returns an `f32` value representing the district light level.
/// The interpretation of this value depends on the specific district
/// system implemented in the application.
///
/// # Examples
///
/// ```rust
/// let district_lighting = district_light();
/// match district_lighting {
///     x if x > 0.8 => println!("Well-lit district"),
///     x if x > 0.4 => println!("Moderately lit district"),
///     _ => println!("Poorly lit district"),
/// }
/// ```
///
/// # Note
///
/// This function corresponds to the Lotus Script variable "DistrictLight".
#[must_use]
pub fn district_light() -> f32 {
    get_var::<f32>("DistrictLight")
}

/// Sets a hint message for the user interface.
///
/// This function updates the hint text that is displayed to the user,
/// typically used for providing contextual information, instructions,
/// or status updates.
///
/// # Arguments
///
/// * `hint` - A string slice containing the hint message to display
///
/// # Examples
///
/// ```rust
/// set_hint("Press E to interact");
/// set_hint("Low health - find a safe place to rest");
/// set_hint(""); // Clear the hint
/// ```
///
/// # Note
///
/// This function corresponds to the Lotus Script variable "Hint".
/// The hint will remain displayed until updated with a new value or
/// cleared by passing an empty string.
pub fn set_hint(hint: &str) {
    set_var("Hint", hint);
}

/// Gets the mouse movement delta since the last frame.
///
/// Returns the relative mouse movement as a 2D vector, which is useful
/// for implementing camera controls, cursor movement, or other
/// mouse-based interactions.
///
/// # Returns
///
/// Returns a `Vec2` containing the mouse movement delta:
/// - `x` component: horizontal movement (positive = right, negative = left)
/// - `y` component: vertical movement (positive = up, negative = down)
///
/// # Examples
///
/// ```rust
/// let mouse_delta = mouse_move();
/// if mouse_delta.x.abs() > 0.1 || mouse_delta.y.abs() > 0.1 {
///     println!("Mouse moved: x={}, y={}", mouse_delta.x, mouse_delta.y);
/// }
/// ```
///
/// # Note
///
/// This function corresponds to the Lotus Script variables "Mouse_X" and "Mouse_Y".
/// The delta values are reset each frame, so this should be called once per
/// frame to get accurate movement data.
#[must_use]
pub fn mouse_move() -> Vec2 {
    mouse_delta()
}

/// Gets the current signal state.
///
/// Retrieves the current state of a signal system, which could represent
/// communication status, system state, or any other discrete state information
/// tracked by the application.
///
/// # Returns
///
/// Returns a `u32` value representing the current signal state.
/// The specific meaning of different values depends on the application's
/// signal system implementation.
///
/// # Examples
///
/// ```rust
/// let state = signalstate();
/// match state {
///     0 => println!("Signal inactive"),
///     1 => println!("Signal active"),
///     2 => println!("Signal error"),
///     _ => println!("Unknown signal state: {}", state),
/// }
/// ```
///
/// # Note
///
/// This function corresponds to the Lotus Script variable "Signalstate".
#[must_use]
pub fn signalstate() -> u32 {
    get_var::<u32>("Signalstate")
}
