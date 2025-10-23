//! Decade Switch Component
//!
//! This module provides a `DecadeSwitch` component that implements a rotary switch
//! with configurable steps (decades). It's commonly used in aviation and industrial
//! control interfaces where precise value selection is required.
//!
//! The switch supports:
//! - Smooth animated transitions between values
//! - Keyboard input handling for increment/decrement operations
//! - Threshold crossing detection for value changes
//! - Configurable rotation speed and maximum values
//!
//! # Example
//!
//! ```rust
//! use your_crate::DecadeSwitch;
//! use your_crate::api::key_event::KeyEventCab;
//!
//! let mut switch = DecadeSwitch::builder(10, "rotation_anim", Some(KeyEventCab::ACab))
//!     .rotation_speed(2.0)
//!     .button_events("increment", "decrement")
//!     .init_value(5.0)
//!     .build();
//!
//! // In your game loop
//! let value_change = switch.tick(0.0);
//! if value_change != 0 {
//!     println!("Value changed by: {}", value_change);
//! }
//! ```

use lotus_script::time::delta;

use crate::api::{
    animation::Animation,
    key_event::{KeyEvent, KeyEventCab},
};

/// Builder for creating a `DecadeSwitch` with custom configuration.
///
/// This builder allows you to configure various aspects of the decade switch
/// before creating the final instance. All configuration methods can be chained
/// for a fluent API experience.
///
/// # Example
///
/// ```rust
/// let switch = DecadeSwitch::builder(10, "my_animation", None)
///     .rotation_speed(1.5)
///     .button_events("plus_key", "minus_key")
///     .init_value(3.0)
///     .build();
/// ```
pub struct DecadeSwitchBuilder {
    cab_side: Option<KeyEventCab>,

    pos: f32,
    target: f32,
    pre_target: f32,

    step_last: u8,
    new_step: u8,

    value: f32,
    max_value: u8,

    rotation_speed: f32,

    pos_anim: Animation,

    key_plus: KeyEvent,
    key_minus: KeyEvent,
}

impl DecadeSwitchBuilder {
    /// Sets the rotation speed of the decade switch.
    ///
    /// The rotation speed determines how fast the switch animates between positions.
    /// Higher values result in faster transitions.
    ///
    /// # Arguments
    ///
    /// * `rotation_speed` - The speed multiplier for rotations (default: 1.0)
    ///
    /// # Example
    ///
    /// ```rust
    /// let builder = DecadeSwitch::builder(10, "anim", None)
    ///     .rotation_speed(2.0); // Double speed rotations
    /// ```
    pub fn rotation_speed(mut self, rotation_speed: f32) -> Self {
        self.rotation_speed = rotation_speed;
        self
    }

    /// Configures the key events for increment and decrement operations.
    ///
    /// This method sets up the keyboard input handling for the decade switch.
    /// The switch will respond to the specified key events to increment or
    /// decrement its value.
    ///
    /// # Arguments
    ///
    /// * `event_plus_name` - Name of the key event for incrementing
    /// * `event_minus_name` - Name of the key event for decrementing
    ///
    /// # Example
    ///
    /// ```rust
    /// let builder = DecadeSwitch::builder(10, "anim", None)
    ///     .button_events("arrow_up", "arrow_down");
    /// ```
    pub fn button_events(
        mut self,
        event_plus_name: impl Into<String>,
        event_minus_name: impl Into<String>,
    ) -> Self {
        self.key_plus = KeyEvent::new(Some(&event_plus_name.into()), self.cab_side);
        self.key_minus = KeyEvent::new(Some(&event_minus_name.into()), self.cab_side);
        self
    }

    /// Sets the initial value and position of the decade switch.
    ///
    /// This method initializes the switch to a specific value. The position
    /// and target will be set to match this value.
    ///
    /// # Arguments
    ///
    /// * `value` - The initial value for the switch
    ///
    /// # Example
    ///
    /// ```rust
    /// let builder = DecadeSwitch::builder(10, "anim", None)
    ///     .init_value(7.5); // Start at position 7.5
    /// ```
    pub fn init_value(mut self, value: f32) -> Self {
        self.value = value;
        self.pos = value;
        self.target = value;
        self
    }

    /// Builds the final `DecadeSwitch` instance.
    ///
    /// Consumes the builder and returns a configured `DecadeSwitch` ready for use.
    ///
    /// # Returns
    ///
    /// A new `DecadeSwitch` instance with the configured settings.
    pub fn build(self) -> DecadeSwitch {
        DecadeSwitch {
            cab_side: self.cab_side,
            pos: self.pos,
            target: self.target,
            pre_target: self.pre_target,
            value: self.value,
            step_last: self.step_last,
            new_step: self.new_step,
            max_value: self.max_value,
            rotation_speed: self.rotation_speed,
            pos_anim: self.pos_anim,
            key_plus: self.key_plus,
            key_minus: self.key_minus,
        }
    }
}

/// A rotary decade switch component with smooth animation and input handling.
///
/// The `DecadeSwitch` represents a rotary control commonly found in aviation
/// and industrial interfaces. It provides:
///
/// - Smooth animated transitions between positions
/// - Keyboard input handling for precise control
/// - Threshold crossing detection for discrete value changes
/// - Configurable maximum values and rotation speeds
///
/// The switch maintains both a continuous position value and discrete step detection,
/// making it suitable for applications that need both smooth visual feedback and
/// discrete value changes.
///
/// # Fields
///
/// - `value`: The current continuous value of the switch
/// - `key_plus`: Key event handler for increment operations
/// - `key_minus`: Key event handler for decrement operations
#[derive(Debug)]
pub struct DecadeSwitch {
    cab_side: Option<KeyEventCab>,

    pub pos: f32,
    pub target: f32,
    pre_target: f32,

    pub step_last: u8,
    pub new_step: u8,

    /// The current continuous value of the decade switch.
    /// This value is automatically wrapped within the range [0, max_value).
    pub value: f32,
    max_value: u8,

    rotation_speed: f32,

    pub pos_anim: Animation,

    /// Key event handler for incrementing the switch value.
    /// Check `is_just_pressed()` to detect increment inputs.
    pub key_plus: KeyEvent,

    /// Key event handler for decrementing the switch value.
    /// Check `is_just_pressed()` to detect decrement inputs.
    pub key_minus: KeyEvent,
}

impl DecadeSwitch {
    pub fn builder(
        max_value: u8,
        animation_name: impl Into<String>,
        cab_side: Option<KeyEventCab>,
    ) -> DecadeSwitchBuilder {
        DecadeSwitchBuilder {
            cab_side,
            pos: 0.0,
            target: 0.0,
            pre_target: 0.0,

            value: 0.0,
            max_value,

            step_last: 0,
            new_step: 0,

            rotation_speed: 1.0,

            pos_anim: Animation::new(Some(&animation_name.into())),

            key_plus: KeyEvent::new(None, cab_side),
            key_minus: KeyEvent::new(None, cab_side),
        }
    }

    pub fn tick(&mut self, add_target: f32) -> f32 {
        if (self.pos - self.target).abs() < 0.001 {
            if self.key_plus.is_just_pressed() {
                self.target += 1.0;
            }

            if self.key_minus.is_just_pressed() {
                self.target -= 1.0;
            }

            if self.pre_target == 0.0 {
                self.pos = self.pos.rem_euclid(self.max_value as f32);
                self.target = self.target.rem_euclid(self.max_value as f32);

                // Runden
                if (self.pos - self.pos.round()).abs() < 0.000001 {
                    self.pos = self.pos.round();
                }
                if (self.target - self.target.round()).abs() < 0.000001 {
                    self.target = self.target.round();
                }
            }

            if self.pre_target.abs() > 0.0 {
                self.target = self.pre_target;
                self.pre_target = 0.0;
            }

            self.target += add_target;
        } else {
            self.pre_target += add_target;
        }

        let pos_last = self.pos;

        if self.target > self.pos {
            self.pos = (self.pos + self.rotation_speed * delta()).min(self.target);
        } else {
            self.pos = (self.pos - self.rotation_speed * delta()).max(self.target);
        }
        self.pos_anim.set(self.pos);

        self.value = self.pos.rem_euclid(self.max_value as f32);

        self.detect_threshold_crossing(pos_last, self.pos)
    }

    fn detect_threshold_crossing(&mut self, pos_last: f32, new_pos: f32) -> f32 {
        let max_val = self.max_value as f32;

        // Normalisiere die Positionen auf den [0, max_val) Bereich für step_last und new_step
        let normalized_last = pos_last.rem_euclid(max_val);
        let normalized_new = new_pos.rem_euclid(max_val);

        self.step_last = normalized_last.floor() as u8;
        self.new_step = normalized_new.floor() as u8;

        if (pos_last < new_pos && normalized_last >= (max_val - 1.0))
            || (pos_last > new_pos && normalized_last < 1.0)
        {
            return new_pos - pos_last;
        }

        /*if pos_last < new_pos
            && ((normalized_last >= (max_val - 1.0) && normalized_new < 1.0)
                || (normalized_last >= (max_val - 1.0) && normalized_new >= (max_val - 1.0)))
        {
            return new_pos - pos_last;
        }

        if pos_last > new_pos
            && ((normalized_last < 1.0 && normalized_new >= (max_val - 1.0))
                || (normalized_last < 1.0 && normalized_new < 1.0))
        {
            return new_pos - pos_last;
        }*/

        0.0

        /*let max_val = self.max_value as f32;
        let movement = new_pos - pos_last;

        self.step_last = (pos_last.rem_euclid(max_val).floor() as u8) % self.max_value;
        self.new_step = (new_pos.rem_euclid(max_val).floor() as u8) % self.max_value;

        // Forward crossing: last step to first step, or crossing max boundary
        if (self.step_last == self.max_value - 1 && self.new_step == 0 && new_pos > pos_last)
            || (pos_last < max_val && new_pos >= max_val)
        {
            let overflow = new_pos - pos_last.floor() - 1.0;
            return overflow.max(0.0);
        }

        // Backward crossing: first step to last step, or crossing zero boundary
        if (pos_last >= 0.0 && new_pos < 0.0)
            || (self.step_last == 0 && self.new_step == self.max_value - 1 && new_pos < pos_last)
        {
            let underflow = new_pos - pos_last.ceil();
            return underflow.min(0.0);
        }

        0.0*/
    }
}
