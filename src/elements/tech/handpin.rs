//! HandPin module for directional input control with mouse and keyboard support.
//!
//! This module provides a flexible input system that allows users to control directional
//! movement using either mouse input (when grabbed) or keyboard directional keys.
//! The system supports both free movement and constrained single-axis movement.

use lotus_script::time::delta;

use crate::{
    api::{
        animation::Animation,
        general::mouse_move,
        key_event::{KeyEvent, KeyEventCab},
    },
    management::structs::general_structs::FourDirections,
};

/// Builder for creating a `HandPin` instance with customizable settings.
///
/// The `HandPinBuilder` follows the builder pattern to provide a fluent interface
/// for configuring a HandPin before construction. It allows setting mouse sensitivity,
/// key bindings, and movement constraints.
///
/// # Examples
///
/// ```rust
/// let hand_pin = HandPin::builder("x_anim", "y_anim", Some(KeyEventCab::ACab))
///     .mouse_factor(2.0)
///     .event_grab("mouse_grab")
///     .only_one_direction()
///     .build();
/// ```
pub struct HandPinBuilder {
    cab_side: Option<KeyEventCab>,

    pos_x: f32,
    pos_y: f32,
    mouse_factor: f32,
    direction: FourDirections,
    direction_last: FourDirections,

    x_move_anim: Animation,
    y_move_anim: Animation,

    key_grab: KeyEvent,
    key_target_n: KeyEvent,
    key_target_s: KeyEvent,
    key_target_e: KeyEvent,
    key_target_w: KeyEvent,

    only_one_direction: bool,
}

impl HandPinBuilder {
    /// Constrains movement to only one axis at a time.
    ///
    /// When enabled, the HandPin will only allow movement along one axis (X or Y)
    /// at a time, preventing diagonal movement. The system determines the primary
    /// axis based on which coordinate is closer to zero.
    ///
    /// # Returns
    ///
    /// Returns `self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let hand_pin = HandPin::builder("x_anim", "y_anim", None)
    ///     .only_one_direction()
    ///     .build();
    /// ```
    pub fn only_one_direction(mut self) -> Self {
        self.only_one_direction = true;
        self
    }

    /// Sets the mouse sensitivity factor.
    ///
    /// The mouse factor determines how sensitive the HandPin is to mouse movement.
    /// A factor of 1.0 provides normal sensitivity, values greater than 1.0 increase
    /// sensitivity, and values between 0.0 and 1.0 decrease sensitivity.
    ///
    /// # Arguments
    ///
    /// * `factor` - The mouse sensitivity multiplier (typically between 0.1 and 5.0)
    ///
    /// # Returns
    ///
    /// Returns `self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let hand_pin = HandPin::builder("x_anim", "y_anim", None)
    ///     .mouse_factor(1.5) // 50% more sensitive
    ///     .build();
    /// ```
    pub fn mouse_factor(mut self, factor: f32) -> Self {
        self.mouse_factor = factor;
        self
    }

    /// Sets the key event for grabbing/enabling mouse control.
    ///
    /// When this key is pressed, the HandPin will respond to mouse movement
    /// instead of keyboard directional input.
    ///
    /// # Arguments
    ///
    /// * `name` - The name/identifier of the key event
    ///
    /// # Returns
    ///
    /// Returns `self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let hand_pin = HandPin::builder("x_anim", "y_anim", None)
    ///     .event_grab("left_mouse_button")
    ///     .build();
    /// ```
    pub fn event_grab(mut self, name: impl Into<String>) -> Self {
        self.key_grab = KeyEvent::new(Some(&name.into()), self.cab_side);
        self
    }

    /// Overrides the default east (right) direction key.
    ///
    /// # Arguments
    ///
    /// * `name` - The name/identifier of the key event for eastward movement
    ///
    /// # Returns
    ///
    /// Returns `self` for method chaining.
    pub fn event_override_e(mut self, name: impl Into<String>) -> Self {
        self.key_target_e = KeyEvent::new(Some(&name.into()), self.cab_side);
        self
    }

    /// Overrides the default west (left) direction key.
    ///
    /// # Arguments
    ///
    /// * `name` - The name/identifier of the key event for westward movement
    ///
    /// # Returns
    ///
    /// Returns `self` for method chaining.
    pub fn event_override_w(mut self, name: impl Into<String>) -> Self {
        self.key_target_w = KeyEvent::new(Some(&name.into()), self.cab_side);
        self
    }

    /// Overrides the default north (up) direction key.
    ///
    /// # Arguments
    ///
    /// * `name` - The name/identifier of the key event for northward movement
    ///
    /// # Returns
    ///
    /// Returns `self` for method chaining.
    pub fn event_override_n(mut self, name: impl Into<String>) -> Self {
        self.key_target_n = KeyEvent::new(Some(&name.into()), self.cab_side);
        self
    }

    /// Overrides the default south (down) direction key.
    ///
    /// # Arguments
    ///
    /// * `name` - The name/identifier of the key event for southward movement
    ///
    /// # Returns
    ///
    /// Returns `self` for method chaining.
    pub fn event_override_s(mut self, name: impl Into<String>) -> Self {
        self.key_target_s = KeyEvent::new(Some(&name.into()), self.cab_side);
        self
    }

    /// Builds and returns the configured `HandPin` instance.
    ///
    /// This method consumes the builder and creates a new `HandPin` with all
    /// the configured settings.
    ///
    /// # Returns
    ///
    /// A new `HandPin` instance with the builder's configuration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let hand_pin = HandPin::builder("x_anim", "y_anim", None)
    ///     .mouse_factor(2.0)
    ///     .event_grab("space")
    ///     .build();
    /// ```
    pub fn build(self) -> HandPin {
        HandPin {
            cab_side: self.cab_side,
            pos_x: self.pos_x,
            pos_y: self.pos_y,
            mouse_factor: self.mouse_factor,
            direction: self.direction,
            direction_last: self.direction_last,
            x_move_anim: self.x_move_anim,
            y_move_anim: self.y_move_anim,
            key_grab: self.key_grab,
            key_target_n: self.key_target_n,
            key_target_s: self.key_target_s,
            key_target_e: self.key_target_e,
            key_target_w: self.key_target_w,

            only_one_direction: self.only_one_direction,
        }
    }
}

/// A directional input controller that supports both mouse and keyboard input.
///
/// `HandPin` provides a unified interface for handling directional input, supporting
/// both mouse-based control (when grabbed) and keyboard-based directional movement.
/// The position is normalized to the range [-1.0, 1.0] for both X and Y axes.
///
/// # Features
///
/// - Mouse control with customizable sensitivity
/// - Keyboard directional input with customizable key bindings
/// - Optional single-axis movement constraint
/// - Smooth position interpolation with configurable return-to-center behavior
/// - Integration with animation systems
/// - Directional state tracking for game logic
///
/// # Examples
///
/// ```rust
/// use your_crate::HandPin;
/// use your_crate::api::key_event::KeyEventCab;
///
/// // Create a HandPin with custom settings
/// let mut hand_pin = HandPin::builder("x_movement", "y_movement", Some(KeyEventCab::ACab))
///     .mouse_factor(1.5)
///     .event_grab("left_mouse")
///     .event_override_n("w")
///     .event_override_s("s")
///     .event_override_e("d")
///     .event_override_w("a")
///     .build();
///
/// // Update the HandPin each frame
/// loop {
///     hand_pin.tick();
///     
///     // Use the current direction for game logic
///     if hand_pin.direction.north {
///         // Handle northward movement
///     }
/// }
/// ```
#[derive(Debug)]
pub struct HandPin {
    cab_side: Option<KeyEventCab>,

    pos_x: f32,
    pos_y: f32,
    mouse_factor: f32,
    /// The current directional state based on position thresholds.
    ///
    /// This field indicates which directions are currently active based on
    /// the position exceeding the threshold values (±0.8).
    pub direction: FourDirections,
    direction_last: FourDirections,

    x_move_anim: Animation,
    y_move_anim: Animation,

    /// The key event for enabling mouse control mode.
    pub key_grab: KeyEvent,
    /// The key event for northward (upward) movement.
    pub key_target_n: KeyEvent,
    /// The key event for southward (downward) movement.
    pub key_target_s: KeyEvent,
    /// The key event for eastward (rightward) movement.
    pub key_target_e: KeyEvent,
    /// The key event for westward (leftward) movement.
    pub key_target_w: KeyEvent,

    only_one_direction: bool,
}

impl HandPin {
    /// Creates a new `HandPinBuilder` for configuring a HandPin instance.
    ///
    /// This is the entry point for creating a HandPin. The builder pattern allows
    /// for flexible configuration of the HandPin's behavior before construction.
    ///
    /// # Arguments
    ///
    /// * `animation_x_name` - The name for the X-axis animation
    /// * `animation_y_name` - The name for the Y-axis animation  
    /// * `cab_side` - Optional cabinet side for key events (e.g., for arcade controls)
    ///
    /// # Returns
    ///
    /// A new `HandPinBuilder` instance ready for configuration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_crate::HandPin;
    /// use your_crate::api::key_event::KeyEventCab;
    ///
    /// let builder = HandPin::builder("x_anim", "y_anim", Some(KeyEventCab::ACab));
    /// ```
    pub fn builder(
        animation_x_name: impl Into<String>,
        animation_y_name: impl Into<String>,
        cab_side: Option<KeyEventCab>,
    ) -> HandPinBuilder {
        HandPinBuilder {
            cab_side,
            pos_x: 0.0,
            pos_y: 0.0,
            mouse_factor: 1.0,
            direction: FourDirections::default(),
            direction_last: FourDirections::default(),
            x_move_anim: Animation::new(Some(&animation_x_name.into())),
            y_move_anim: Animation::new(Some(&animation_y_name.into())),
            key_grab: KeyEvent::new(None, None),
            key_target_n: KeyEvent::new(None, None),
            key_target_s: KeyEvent::new(None, None),
            key_target_e: KeyEvent::new(None, None),
            key_target_w: KeyEvent::new(None, None),
            only_one_direction: false,
        }
    }

    /// Updates the HandPin's state for one frame.
    ///
    /// This method should be called once per frame to update the HandPin's position
    /// and directional state. It handles both mouse input (when grabbed) and keyboard
    /// input, applies movement constraints, and updates the associated animations.
    ///
    /// # Behavior
    ///
    /// - **Mouse Mode**: When the grab key is pressed, mouse movement is translated
    ///   to position changes with the configured sensitivity
    /// - **Keyboard Mode**: When grab is not active, directional keys move the position
    ///   at a fixed rate, with automatic return-to-center when no keys are pressed
    /// - **Single Direction**: If enabled, constrains movement to one axis at a time
    /// - **Thresholding**: Positions are clamped to [-1.0, 1.0] and small values
    ///   near zero are snapped to exactly 0.0
    /// - **Direction Updates**: The direction state is updated based on position
    ///   thresholds (±0.8)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut hand_pin = HandPin::builder("x_anim", "y_anim", None).build();
    ///
    /// // Game loop
    /// loop {
    ///     hand_pin.tick(); // Update HandPin state
    ///     
    ///     // Use the updated state for game logic
    ///     match hand_pin.direction {
    ///         dir if dir.north => println!("Moving north!"),
    ///         dir if dir.south => println!("Moving south!"),
    ///         dir if dir.east => println!("Moving east!"),
    ///         dir if dir.west => println!("Moving west!"),
    ///         _ => println!("Centered"),
    ///     }
    /// }
    /// ```
    pub fn tick(&mut self) {
        self.direction_last = self.direction;

        if self.key_grab.is_pressed() {
            let delta_x = mouse_move().x * self.mouse_factor;
            let delta_y = -mouse_move().y * self.mouse_factor;

            if self.only_one_direction {
                if self.pos_y < 0.025 && self.pos_y > -0.025 {
                    self.pos_x = (self.pos_x + delta_x * delta()).clamp(-1.0, 1.0);
                }
                if self.pos_x < 0.025 && self.pos_x > -0.025 {
                    self.pos_y = (self.pos_y + delta_y * delta()).clamp(-1.0, 1.0);
                }
            } else {
                self.pos_x = (self.pos_x + delta_x * delta()).clamp(-1.0, 1.0);
                self.pos_y = (self.pos_y + delta_y * delta()).clamp(-1.0, 1.0);
            }
        } else {
            if self.key_target_w.is_pressed() {
                self.pos_x -= 2.0 * delta();
            } else if self.key_target_e.is_pressed() {
                self.pos_x += 2.0 * delta();
            } else if self.key_target_n.is_pressed() {
                self.pos_y += 2.0 * delta();
            } else if self.key_target_s.is_pressed() {
                self.pos_y -= 2.0 * delta();
            } else {
                self.pos_x = self.pos_x - (self.pos_x / 2.0);
                self.pos_y = self.pos_y - (self.pos_y / 2.0);
            }

            if self.pos_x.abs() < 0.025 {
                self.pos_x = 0.0;
            }
            if self.pos_y.abs() < 0.025 {
                self.pos_y = 0.0;
            }
        }

        self.direction = FourDirections::new(
            self.pos_y > 0.8,
            self.pos_y < -0.8,
            self.pos_x > 0.8,
            self.pos_x < -0.8,
        );

        self.x_move_anim.set(self.pos_x);
        self.y_move_anim.set(self.pos_y);
    }
}
