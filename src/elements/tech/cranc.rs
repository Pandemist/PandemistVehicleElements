//! Crank control system for input handling with animation support.
//!
//! This module provides a crank control mechanism that can be used to handle
//! rotary input controls with configurable limits, speed factors, and key bindings.
//! The crank position is automatically animated and can be controlled via key events.

use lotus_script::time::delta;

use crate::api::{
    animation::Animation,
    key_event::{KeyEvent, KeyEventCab},
};

/// Builder for creating a `Crank` instance with customizable parameters.
///
/// The `CrankBuilder` follows the builder pattern to allow flexible configuration
/// of crank properties before creating the final `Crank` instance.
///
/// # Examples
///
/// ```rust
/// # use your_crate::Crank;
/// # use your_crate::api::key_event::KeyEventCab;
/// let crank = Crank::builder("rotation_anim", Some(KeyEventCab::ACab))
///     .factor(2.0)
///     .min(-10.0)
///     .max(10.0)
///     .event_plus("crank_right")
///     .event_minus("crank_left")
///     .build();
/// ```
pub struct CrankBuilder {
    cab_side: Option<KeyEventCab>,

    pos: f32,
    pos_last: f32,
    factor: f32,

    min: f32,
    max: f32,

    rotation_anim: Animation,

    key_plus: KeyEvent,
    key_minus: KeyEvent,
}

impl CrankBuilder {
    /// Sets the speed factor for crank movement.
    ///
    /// The factor determines how fast the crank position changes per frame
    /// when keys are pressed. Higher values result in faster movement.
    ///
    /// # Arguments
    ///
    /// * `factor` - The movement speed multiplier (default: 1.0)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use your_crate::Crank;
    /// let crank = Crank::builder("anim", None)
    ///     .factor(2.5)  // Move 2.5 times faster
    ///     .build();
    /// ```
    pub fn factor(mut self, factor: f32) -> Self {
        self.factor = factor;
        self
    }

    /// Sets the maximum allowed crank position.
    ///
    /// The crank position will be clamped to not exceed this value.
    ///
    /// # Arguments
    ///
    /// * `max` - The maximum position value (default: 1.0)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use your_crate::Crank;
    /// let crank = Crank::builder("anim", None)
    ///     .max(100.0)  // Allow positions up to 100
    ///     .build();
    /// ```
    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    /// Sets the minimum allowed crank position.
    ///
    /// The crank position will be clamped to not go below this value.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum position value (default: -1.0)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use your_crate::Crank;
    /// let crank = Crank::builder("anim", None)
    ///     .min(-50.0)  // Allow positions down to -50
    ///     .build();
    /// ```
    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    /// Sets the key event for increasing the crank position.
    ///
    /// When this key is pressed, the crank will move in the positive direction
    /// according to the configured factor and delta time.
    ///
    /// # Arguments
    ///
    /// * `name` - The name/identifier for the key event
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use your_crate::Crank;
    /// let crank = Crank::builder("anim", None)
    ///     .event_plus("rotate_right")
    ///     .build();
    /// ```
    pub fn event_plus(mut self, name: impl Into<String>) -> Self {
        self.key_plus = KeyEvent::new(Some(&name.into()), self.cab_side);
        self
    }

    /// Sets the key event for decreasing the crank position.
    ///
    /// When this key is pressed, the crank will move in the negative direction
    /// according to the configured factor and delta time.
    ///
    /// # Arguments
    ///
    /// * `name` - The name/identifier for the key event
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use your_crate::Crank;
    /// let crank = Crank::builder("anim", None)
    ///     .event_minus("rotate_left")
    ///     .build();
    /// ```
    pub fn event_minus(mut self, name: impl Into<String>) -> Self {
        self.key_minus = KeyEvent::new(Some(&name.into()), self.cab_side);
        self
    }

    /// Builds and returns the configured `Crank` instance.
    ///
    /// This consumes the builder and creates a `Crank` with all the
    /// configured parameters.
    ///
    /// # Returns
    ///
    /// A new `Crank` instance with the specified configuration.
    pub fn build(self) -> Crank {
        Crank {
            cab_side: self.cab_side,
            pos: self.pos,
            pos_last: self.pos_last,
            factor: self.factor,
            min: self.min,
            max: self.max,
            rotation_anim: self.rotation_anim,
            key_plus: self.key_plus,
            key_minus: self.key_minus,
        }
    }
}

/// A crank control that handles rotary input with animation support.
///
/// The `Crank` provides a way to handle rotary controls that can be operated
/// via key events. It maintains a position value that can be increased or
/// decreased within configured bounds, and automatically updates an associated
/// animation to reflect the current position.
///
/// # Key Features
///
/// - Configurable position limits (min/max)
/// - Adjustable movement speed factor
/// - Automatic animation updates
/// - Key event integration
/// - Delta time-based smooth movement
///
/// # Usage
///
/// Create a crank using the builder pattern, then call `tick()` each frame
/// to update the crank state based on input:
///
/// ```rust
/// # use your_crate::Crank;
/// let mut crank = Crank::builder("my_rotation", None)
///     .factor(1.5)
///     .min(-180.0)
///     .max(180.0)
///     .event_plus("turn_right")
///     .event_minus("turn_left")
///     .build();
///
/// // In your game loop:
/// crank.tick();
/// println!("Current position: {}", crank.pos);
/// ```
#[derive(Debug)]
pub struct Crank {
    cab_side: Option<KeyEventCab>,

    /// The current position of the crank.
    ///
    /// This value is updated each frame based on input and is clamped
    /// between the configured minimum and maximum values.
    pub pos: f32,

    pos_last: f32,
    factor: f32,
    min: f32,
    max: f32,
    rotation_anim: Animation,
    key_plus: KeyEvent,
    key_minus: KeyEvent,
}

impl Crank {
    /// Creates a new `CrankBuilder` for configuring a crank instance.
    ///
    /// This is the entry point for creating a new crank. Use the returned
    /// builder to configure the crank's properties before calling `build()`.
    ///
    /// # Arguments
    ///
    /// * `animation_name` - The name of the animation to associate with this crank
    /// * `cab_side` - Optional cab side specification for key events
    ///
    /// # Returns
    ///
    /// A `CrankBuilder` instance with default values:
    /// - Initial position: 0.0
    /// - Factor: 1.0
    /// - Min: -1.0
    /// - Max: 1.0
    /// - No key events configured
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use your_crate::Crank;
    /// # use your_crate::api::key_event::KeyEventCab;
    /// // Basic crank with default settings
    /// let crank = Crank::builder("rotation", None).build();
    ///
    /// // Crank with cab-specific key events
    /// let crank = Crank::builder("wheel", Some(KeyEventCab::ACab)).build();
    /// ```
    pub fn builder(
        animation_name: impl Into<String>,
        cab_side: Option<KeyEventCab>,
    ) -> CrankBuilder {
        CrankBuilder {
            cab_side,
            pos: 0.0,
            pos_last: 0.0,
            factor: 1.0,
            min: -1.0,
            max: 1.0,
            rotation_anim: Animation::new(Some(&animation_name.into())),
            key_plus: KeyEvent::new(None, None),
            key_minus: KeyEvent::new(None, None),
        }
    }

    /// Updates the crank state for the current frame.
    ///
    /// This method should be called once per frame to:
    /// - Store the previous position
    /// - Check for key presses and update position accordingly
    /// - Apply movement speed and delta time
    /// - Clamp position to configured bounds
    /// - Update the associated animation
    ///
    /// The position change is calculated using the formula:
    /// `new_position = old_position ± (factor * delta_time)`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use your_crate::Crank;
    /// let mut crank = Crank::builder("my_crank", None)
    ///     .event_plus("right_key")
    ///     .event_minus("left_key")
    ///     .build();
    ///
    /// // In your main loop:
    /// loop {
    ///     crank.tick();  // Update crank state
    ///     
    ///     // Use crank.pos for your game logic
    ///     if crank.pos > 0.5 {
    ///         // Handle crank turned right
    ///     }
    /// }
    /// ```
    pub fn tick(&mut self) {
        self.pos_last = self.pos;

        if self.key_plus.is_pressed() {
            self.pos = (self.pos + self.factor * delta()).min(self.max);
            self.rotation_anim.set(self.pos);
        }
        if self.key_minus.is_pressed() {
            self.pos = (self.pos - self.factor * delta()).max(self.min);
            self.rotation_anim.set(self.pos);
        }
    }
}
