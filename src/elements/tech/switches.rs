//! # Switches
//!
//! This module provides interactive switch components for GUI applications,
//! particularly useful for control panels and simulation interfaces.
//!
//! ## Overview
//!
//! The module contains two main switch types:
//! - [`Switch`]: A simple on/off switch with toggle functionality
//! - [`StepSwitch`]: A multi-position switch with discrete steps
//!
//! Both switches support:
//! - Key event handling for user interaction
//! - Sound feedback for different actions
//! - Animation control for visual feedback
//! - Flexible configuration through builder patterns
//!
//! ## Examples
//!
//! ### Basic Switch
//!
//! ```rust
//! use crate::switches::Switch;
//! use crate::api::key_event::KeyEventCab;
//!
//! let mut power_switch = Switch::builder("power_anim", Some(KeyEventCab::CabA))
//!     .init(false)
//!     .event_toggle("SPACE")
//!     .snd_toggle("click_sound")
//!     .build();
//!
//! // In your main loop
//! power_switch.tick();
//! if power_switch.value(true) {
//!     println!("Power is ON");
//! }
//! ```
//!
//! ### Multi-Position StepSwitch
//!
//! ```rust
//! use crate::switches::{StepSwitch, SwitchEventAction};
//! use std::collections::HashMap;
//!
//! let mut mode_switch = StepSwitch::builder("mode_anim", None)
//!     .min(0)
//!     .max(3)
//!     .init(1)
//!     .event("UP", SwitchEventAction::Plus)
//!     .event("DOWN", SwitchEventAction::Minus)
//!     .event("MODE_1", SwitchEventAction::Set(1))
//!     .snd_plus("step_up")
//!     .snd_minus("step_down")
//!     .build();
//!
//! // In your main loop
//! mode_switch.tick();
//! match mode_switch.value(true) {
//!     0 => println!("Off"),
//!     1 => println!("Low"),
//!     2 => println!("Medium"),
//!     3 => println!("High"),
//!     _ => unreachable!(),
//! }
//! ```

use std::collections::HashMap;

use crate::api::{
    animation::Animation,
    key_event::{KeyEvent, KeyEventCab},
    sound::Sound,
};

//=================================================================
// Switch
//=================================================================

/// Builder for creating a [`Switch`] with customizable configuration.
///
/// The builder pattern allows for flexible construction of switch components
/// with various key bindings, sounds, and initial states.
pub struct SwitchBuilder {
    cab_side: Option<KeyEventCab>,

    pos: f32,
    value: bool,
    value_last: bool,

    key_toggle: KeyEvent,
    key_plus: KeyEvent,
    key_minus: KeyEvent,

    btn_anim: Animation,

    snd_toggle: Sound,
    snd_plus: Sound,
    snd_minus: Sound,
}

impl SwitchBuilder {
    /// Sets the initial state of the switch.
    ///
    /// # Arguments
    ///
    /// * `init` - If `true`, the switch starts in the "on" position
    ///
    /// # Example
    ///
    /// ```rust
    /// let switch = Switch::builder("anim", None)
    ///     .init(true)  // Switch starts ON
    ///     .build();
    /// ```
    pub fn init(mut self, init: bool) -> Self {
        if init {
            self.value = true;
            self.pos = 1.0;
            self.btn_anim.set(self.pos);
        }
        self
    }

    /// Sets the key event for toggling the switch state.
    ///
    /// # Arguments
    ///
    /// * `name` - The key event name (e.g., "SPACE", "ENTER")
    ///
    /// # Example
    ///
    /// ```rust
    /// let switch = Switch::builder("anim", None)
    ///     .event_toggle("SPACE")
    ///     .build();
    /// ```
    pub fn event_toggle(mut self, name: impl Into<String>) -> Self {
        self.key_toggle = KeyEvent::new(Some(&name.into()), self.cab_side);
        self
    }

    /// Sets the key event for turning the switch on.
    ///
    /// # Arguments
    ///
    /// * `name` - The key event name
    pub fn event_plus(mut self, name: impl Into<String>) -> Self {
        self.key_plus = KeyEvent::new(Some(&name.into()), self.cab_side);
        self
    }

    /// Sets the key event for turning the switch off.
    ///
    /// # Arguments
    ///
    /// * `name` - The key event name
    pub fn event_minus(mut self, name: impl Into<String>) -> Self {
        self.key_minus = KeyEvent::new(Some(&name.into()), self.cab_side);
        self
    }

    /// Sets the sound to play when toggling the switch.
    ///
    /// # Arguments
    ///
    /// * `name` - The sound resource name
    pub fn snd_toggle(mut self, name: impl Into<String>) -> Self {
        self.snd_toggle = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Sets the sound to play when turning the switch on.
    ///
    /// # Arguments
    ///
    /// * `name` - The sound resource name
    pub fn snd_plus(mut self, name: impl Into<String>) -> Self {
        self.snd_plus = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Sets the sound to play when turning the switch off.
    ///
    /// # Arguments
    ///
    /// * `name` - The sound resource name
    pub fn snd_minus(mut self, name: impl Into<String>) -> Self {
        self.snd_minus = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Builds the final [`Switch`] instance.
    ///
    /// # Returns
    ///
    /// A configured `Switch` ready for use.
    pub fn build(self) -> Switch {
        Switch {
            cab_side: self.cab_side,
            pos: self.pos,
            value: self.value,
            value_last: self.value_last,
            key_toggle: self.key_toggle,
            key_plus: self.key_plus,
            key_minus: self.key_minus,
            btn_anim: self.btn_anim,
            snd_toggle: self.snd_toggle,
            snd_plus: self.snd_plus,
            snd_minus: self.snd_minus,
        }
    }
}

/// A simple two-state switch component.
///
/// The `Switch` represents a basic on/off control that can be toggled
/// via key events. It supports animations and sound feedback for
/// user interactions.
///
/// # Features
///
/// - Toggle functionality with single key press
/// - Separate on/off key events
/// - Animation support for visual feedback
/// - Sound effects for different actions
/// - State change detection
///
/// # Usage
///
/// Switches are typically created using the builder pattern and then
/// updated each frame by calling [`tick()`](Switch::tick).
#[derive(Debug)]
pub struct Switch {
    cab_side: Option<KeyEventCab>,

    pos: f32,
    value: bool,
    value_last: bool,

    /// Key event for toggling the switch
    pub key_toggle: KeyEvent,
    /// Key event for turning the switch on
    pub key_plus: KeyEvent,
    /// Key event for turning the switch off
    pub key_minus: KeyEvent,

    btn_anim: Animation,

    snd_toggle: Sound,
    snd_plus: Sound,
    snd_minus: Sound,
}

impl Switch {
    /// Creates a new switch builder.
    ///
    /// # Arguments
    ///
    /// * `animation_name` - Name of the animation to use for visual feedback
    /// * `cab_side` - Optional cab side for key event handling
    ///
    /// # Returns
    ///
    /// A [`SwitchBuilder`] for configuring the switch.
    ///
    /// # Example
    ///
    /// ```rust
    /// let switch = Switch::builder("power_button_anim", Some(KeyEventCab::ACab))
    ///     .init(false)
    ///     .event_toggle("P")
    ///     .build();
    /// ```
    pub fn builder(
        animation_name: impl Into<String>,
        cab_side: Option<KeyEventCab>,
    ) -> SwitchBuilder {
        SwitchBuilder {
            cab_side,
            pos: 0.0,
            value: false,
            value_last: false,
            key_toggle: KeyEvent::new(None, None),
            key_plus: KeyEvent::new(None, None),
            key_minus: KeyEvent::new(None, None),
            btn_anim: Animation::new(Some(&animation_name.into())),
            snd_toggle: Sound::new_simple(None),
            snd_plus: Sound::new_simple(None),
            snd_minus: Sound::new_simple(None),
        }
    }

    pub fn set(&mut self, target: bool) {
        if target != self.value {
            self.value = target;
            self.pos = self.value as u8 as f32;
            if self.value {
                self.snd_plus.start();
            } else {
                self.snd_minus.start();
            }
            self.snd_toggle.start();
            self.btn_anim.set(self.pos);
        }
    }

    /// Updates the switch state based on key events.
    ///
    /// This method should be called once per frame to handle user input
    /// and update the switch state accordingly.
    ///
    /// # Behavior
    ///
    /// - Toggle key: Switches between on/off states
    /// - Plus key: Turns the switch on (if currently off)
    /// - Minus key: Turns the switch off (if currently on)
    ///
    /// Appropriate sounds are played for each action.
    pub fn tick(&mut self) {
        self.value_last = self.value;

        if self.key_toggle.is_just_pressed() {
            self.pos = 1.0 - self.pos;
            self.value = self.pos > 0.5;
            if self.value {
                self.snd_plus.start();
            } else {
                self.snd_minus.start();
            }
            self.snd_toggle.start();
            self.btn_anim.set(self.pos);
        }

        if self.key_plus.is_just_pressed() && !self.value {
            self.pos = 1.0;
            self.value = true;
            self.snd_plus.start();
            self.snd_toggle.start();
            self.btn_anim.set(self.pos);
        }

        if self.key_minus.is_just_pressed() && self.value {
            self.pos = 0.0;
            self.value = false;
            self.snd_minus.start();
            self.snd_toggle.start();
            self.btn_anim.set(self.pos);
        }
    }

    /// Returns the current switch value, respecting the allowed state.
    ///
    /// # Arguments
    ///
    /// * `allowed` - Whether the switch is allowed to be active
    ///
    /// # Returns
    ///
    /// `true` if the switch is on AND allowed, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// let power_available = true;
    /// if switch.value(power_available) {
    ///     // Switch is on and power is available
    ///     start_engine();
    /// }
    /// ```
    #[must_use]
    pub fn value(&self, allowed: bool) -> bool {
        self.value && allowed
    }

    /// Checks if the switch was just turned on this frame.
    ///
    /// # Returns
    ///
    /// `true` if the switch changed from off to on in the current frame.
    ///
    /// # Example
    ///
    /// ```rust
    /// if switch.is_just_pressed() {
    ///     println!("Switch was just turned on!");
    /// }
    /// ```
    pub fn is_just_pressed(&mut self) -> bool {
        self.value && !self.value_last
    }
}

//=================================================================
// StepSwitch
//=================================================================

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SwitchSoundDirection {
    Plus,
    Minus,
}

/// Defines the action to perform when a switch event is triggered.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SwitchEventAction {
    /// Increment the switch position by one step
    Plus,
    /// Decrement the switch position by one step
    Minus,
    /// Set the switch to a specific position
    Set(i32),
}

/// Builder for creating a [`StepSwitch`] with customizable configuration.
///
/// The step switch builder allows for complex configurations including
/// custom ranges, spring behavior, and animation mappings.
pub struct StepSwitchBuilder {
    cab_side: Option<KeyEventCab>,

    max: i32,
    min: i32,
    pos: f32,
    value: i32,
    value_last: i32,

    min_spring: bool,
    max_spring: bool,
    inv_turn: bool,

    key_anim: Animation,
    anim_mapping: HashMap<i32, f32>,

    key_plus: KeyEvent,
    key_minus: KeyEvent,

    events: HashMap<String, SwitchEventAction>,

    just_changed: Option<i32>,

    snd_default_plus: Sound,
    snd_default_minus: Sound,

    snd_alt: HashMap<i32, (Sound, Option<SwitchSoundDirection>)>,
}

impl StepSwitchBuilder {
    /// Sets the maximum value for the switch.
    ///
    /// # Arguments
    ///
    /// * `max` - The maximum position value (inclusive)
    pub fn max(mut self, max: i32) -> Self {
        self.max = max;
        self
    }

    /// Sets the minimum value for the switch.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum position value (inclusive)
    pub fn min(mut self, min: i32) -> Self {
        self.min = min;
        self
    }

    /// Sets the initial position of the switch.
    ///
    /// # Arguments
    ///
    /// * `value` - The initial position (must be within min/max range)
    pub fn init(mut self, value: i32) -> Self {
        self.value = value;
        self.value_last = value;
        let pos = match self.anim_mapping.get(&self.value) {
            Some(s) => *s,
            None => self.value as f32,
        };
        self.key_anim.set(pos);
        self
    }

    /// Enables spring behavior at the maximum position.
    ///
    /// When enabled, the switch will spring back from the maximum
    /// position when the key is released.
    pub fn max_spring(mut self) -> Self {
        self.inv_turn = false;
        self.max_spring = true;
        self
    }

    /// Enables spring behavior at the minimum position.
    ///
    /// When enabled, the switch will spring back from the minimum
    /// position when the key is released.
    pub fn min_spring(mut self) -> Self {
        self.inv_turn = false;
        self.min_spring = true;
        self
    }

    /// Enables inverse turn behavior.
    ///
    /// When enabled, reaching the maximum position wraps to minimum
    /// and vice versa, creating a circular behavior.
    pub fn inv_turn(mut self) -> Self {
        self.max_spring = false;
        self.min_spring = false;
        self.inv_turn = true;
        self
    }

    /// Adds a key event mapping to the switch.
    ///
    /// # Arguments
    ///
    /// * `name` - The key event name
    /// * `action` - The action to perform when the key is pressed
    ///
    /// # Example
    ///
    /// ```rust
    /// let switch = StepSwitch::builder("anim", None)
    ///     .event("UP", SwitchEventAction::Plus)
    ///     .event("DOWN", SwitchEventAction::Minus)
    ///     .event("HOME", SwitchEventAction::Set(0))
    ///     .build();
    /// ```
    pub fn event(mut self, name: impl Into<String>, action: SwitchEventAction) -> Self {
        self.events.insert(name.into(), action);
        self
    }

    /// Sets a custom mapping between switch positions and animation values.
    ///
    /// # Arguments
    ///
    /// * `map` - HashMap mapping position values to animation positions
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut mapping = HashMap::new();
    /// mapping.insert(0, 0.0);    // Position 0 -> Animation 0.0
    /// mapping.insert(1, 0.3);    // Position 1 -> Animation 0.3
    /// mapping.insert(2, 1.0);    // Position 2 -> Animation 1.0
    ///
    /// let switch = StepSwitch::builder("anim", None)
    ///     .mapping(mapping)
    ///     .build();
    /// ```
    pub fn mapping(mut self, map: HashMap<i32, f32>) -> Self {
        self.anim_mapping = map;
        self
    }

    /// Sets the sound to play when incrementing the switch position.
    ///
    /// # Arguments
    ///
    /// * `name` - The sound resource name
    pub fn snd_default_plus(mut self, name: impl Into<String>) -> Self {
        self.snd_default_plus = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Sets the sound to play when decrementing the switch position.
    ///
    /// # Arguments
    ///
    /// * `name` - The sound resource name
    pub fn snd_default_minus(mut self, name: impl Into<String>) -> Self {
        self.snd_default_minus = Sound::new_simple(Some(&name.into()));
        self
    }

    pub fn add_alt_sound(
        mut self,
        position: i32,
        sound_name: impl Into<String>,
        switching_dir: Option<SwitchSoundDirection>,
    ) -> Self {
        self.snd_alt.insert(
            position,
            (Sound::new_simple(Some(&sound_name.into())), switching_dir),
        );
        self
    }

    /// Builds the final [`StepSwitch`] instance.
    ///
    /// # Returns
    ///
    /// A configured `StepSwitch` ready for use.
    pub fn build(self) -> StepSwitch {
        StepSwitch {
            cab_side: self.cab_side,
            max: self.max,
            min: self.min,
            pos: self.pos,
            value: self.value,
            value_last: self.value_last,
            min_spring: self.min_spring,
            max_spring: self.max_spring,
            inv_turn: self.inv_turn,
            key_anim: self.key_anim,
            anim_mapping: self.anim_mapping,
            key_plus: self.key_plus,
            key_minus: self.key_minus,
            events: self.events,
            just_changed: self.just_changed,
            snd_default_plus: self.snd_default_plus,
            snd_default_minus: self.snd_default_minus,
            snd_alt: self.snd_alt,
        }
    }
}

/// A multi-position switch component with discrete steps.
///
/// The `StepSwitch` provides a switch that can be set to multiple
/// discrete positions within a defined range. It supports various
/// behaviors like spring-back and wrap-around functionality.
///
/// # Features
///
/// - Configurable min/max range
/// - Spring behavior at extremes
/// - Wrap-around (inverse turn) functionality
/// - Custom animation position mapping
/// - Multiple key event bindings
/// - State change detection
///
/// # Common Use Cases
///
/// - Multi-position selector switches
/// - Mode selection controls
/// - Step-wise adjustment controls
/// - Rotary switch simulation
#[derive(Debug)]
pub struct StepSwitch {
    cab_side: Option<KeyEventCab>,

    /// The maximum allowed position
    pub max: i32,
    /// The minimum allowed position
    pub min: i32,
    pos: f32,
    value: i32,
    value_last: i32,

    min_spring: bool,
    max_spring: bool,
    inv_turn: bool,

    key_anim: Animation,
    anim_mapping: HashMap<i32, f32>,

    /// Key event for incrementing position
    pub key_plus: KeyEvent,
    /// Key event for decrementing position
    pub key_minus: KeyEvent,

    events: HashMap<String, SwitchEventAction>,

    just_changed: Option<i32>,

    snd_default_plus: Sound,
    snd_default_minus: Sound,

    snd_alt: HashMap<i32, (Sound, Option<SwitchSoundDirection>)>,
}

impl StepSwitch {
    /// Creates a new step switch builder.
    ///
    /// # Arguments
    ///
    /// * `animation_name` - Name of the animation to use for visual feedback
    /// * `cab_side` - Optional cab side for key event handling
    ///
    /// # Returns
    ///
    /// A [`StepSwitchBuilder`] for configuring the switch.
    ///
    /// # Example
    ///
    /// ```rust
    /// let switch = StepSwitch::builder("mode_selector", None)
    ///     .min(0)
    ///     .max(5)
    ///     .init(2)
    ///     .build();
    /// ```
    pub fn builder(
        animation_name: impl Into<String>,
        cab_side: Option<KeyEventCab>,
    ) -> StepSwitchBuilder {
        StepSwitchBuilder {
            cab_side,
            max: 1,
            min: -1,
            pos: 0.0,
            value: 0,
            value_last: 0,
            min_spring: false,
            max_spring: false,
            inv_turn: false,
            key_anim: Animation::new(Some(&animation_name.into())),
            anim_mapping: HashMap::new(),
            key_plus: KeyEvent::new(None, None),
            key_minus: KeyEvent::new(None, None),
            events: HashMap::new(),
            just_changed: None,
            snd_default_plus: Sound::new_simple(None),
            snd_default_minus: Sound::new_simple(None),
            snd_alt: HashMap::new(),
        }
    }

    /// Initializes the switch to a specific position.
    ///
    /// # Arguments
    ///
    /// * `new_pos` - The position to set (must be within min/max range)
    ///
    /// # Note
    ///
    /// This method will silently ignore positions outside the valid range.
    pub fn init(&mut self, new_value: i32) {
        if (self.min..=self.max).contains(&new_value) {
            self.value = new_value;
            self.pos = new_value as f32;

            let pos = match self.anim_mapping.get(&self.value) {
                Some(s) => *s,
                None => self.value as f32,
            };

            self.update();
        }
    }

    /// Sets the switch to a specific position with sound feedback.
    ///
    /// # Arguments
    ///
    /// * `new_value` - The position to set (must be within min/max range)
    ///
    /// # Note
    ///
    /// This method will play a sound and trigger animations when the
    /// position changes. Invalid positions are ignored.
    pub fn set(&mut self, new_value: i32) {
        if (self.min..=self.max).contains(&new_value) && self.value != new_value {
            self.play_sound(true);
            self.value = new_value;
            self.update();
        }
    }

    /// Returns the new position if the switch just changed.
    ///
    /// # Arguments
    ///
    /// * `allowed` - Whether changes are currently allowed
    ///
    /// # Returns
    ///
    /// `Some(position)` if the switch changed this frame and changes are allowed,
    /// `None` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// if let Some(new_pos) = switch.just_changed(engine_running) {
    ///     println!("Switch moved to position: {}", new_pos);
    /// }
    /// ```
    pub fn just_changed(&self, allowed: bool) -> Option<i32> {
        if allowed {
            self.just_changed
        } else {
            None
        }
    }

    /// Checks if the switch just changed to a specific position.
    ///
    /// # Arguments
    ///
    /// * `allowed` - Whether changes are currently allowed
    /// * `to` - The position to check for
    ///
    /// # Returns
    ///
    /// `true` if the switch just changed to the specified position and
    /// changes are allowed.
    ///
    /// # Example
    ///
    /// ```rust
    /// if switch.just_changed_to(true, 3) {
    ///     println!("Switch was just set to position 3!");
    /// }
    /// ```
    pub fn just_changed_to(&self, allowed: bool, to: i32) -> bool {
        if allowed {
            self.just_changed.unwrap_or_default() == to
        } else {
            false
        }
    }

    /// Internal method to update animation and handle special behaviors.
    fn update(&mut self) {
        if self.inv_turn {
            if self.value > (self.max - 1) {
                self.value = self.min;
            }
            if self.value < (self.min + 1) {
                self.value = self.max - 1;
            }
        }
        self.pos = match self.anim_mapping.get(&self.value) {
            Some(s) => *s,
            None => self.value as f32,
        };
        self.key_anim.set(self.pos);
    }

    /// Updates the switch state based on key events.
    ///
    /// This method should be called once per frame to handle user input
    /// and update the switch state accordingly.
    ///
    /// # Behavior
    ///
    /// The switch responds to all configured key events and applies
    /// the corresponding actions (Plus, Minus, or Set). Spring behavior
    /// is handled on key release events.
    pub fn tick(&mut self) {
        let mut plus_minus = false;

        let mut has_update = false;

        for (key, value) in &self.events {
            let mut ev = KeyEvent::new(Some(key), self.cab_side);
            if ev.is_just_pressed() {
                match value {
                    SwitchEventAction::Plus => {
                        if self.value < self.max {
                            self.value += 1;
                            plus_minus = true;
                            has_update = true;
                        }
                    }
                    SwitchEventAction::Minus => {
                        if self.value > self.min {
                            self.value -= 1;
                            has_update = true;
                        }
                    }
                    SwitchEventAction::Set(new_value) => {
                        if (self.min..=self.max).contains(new_value) && self.value != *new_value {
                            self.value = *new_value;
                            plus_minus = true;
                            has_update = true;
                        }
                    }
                }
            }
            if ev.is_just_released() {
                match value {
                    SwitchEventAction::Plus => {
                        if self.max_spring && self.value == self.max {
                            self.value -= 1;
                            has_update = true;
                        }
                    }
                    SwitchEventAction::Minus => {
                        if self.min_spring && self.value == self.min {
                            self.value += 1;
                            plus_minus = true;
                            has_update = true;
                        }
                    }
                    SwitchEventAction::Set(_) => {}
                }
            }
        }

        if has_update {
            self.play_sound(plus_minus);
            self.update();
        }

        self.just_changed = if self.value_last != self.value {
            Some(self.value)
        } else {
            None
        };

        self.value_last = self.value;
    }

    fn play_sound(&mut self, is_plus: bool) {
        match self.snd_alt.get_mut(&self.value) {
            Some(snd) => match snd.1 {
                Some(dir) => match (dir, is_plus) {
                    (SwitchSoundDirection::Plus, true) | (SwitchSoundDirection::Minus, false) => {
                        snd.0.start()
                    }
                    _ => {
                        if is_plus {
                            self.snd_default_plus.start();
                        } else {
                            self.snd_default_minus.start();
                        }
                    }
                },
                None => snd.0.start(),
            },
            None => {
                if is_plus {
                    self.snd_default_plus.start();
                } else {
                    self.snd_default_minus.start();
                }
            }
        }
    }

    /// Returns the current switch position, respecting the allowed state.
    ///
    /// # Arguments
    ///
    /// * `allowed` - Whether the switch is allowed to be active
    ///
    /// # Returns
    ///
    /// The current position if allowed, `0` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// let system_enabled = true;
    /// match switch.value(system_enabled) {
    ///     0 => handle_off_mode(),
    ///     1 => handle_low_mode(),
    ///     2 => handle_high_mode(),
    ///     _ => handle_unknown_mode(),
    /// }
    /// ```
    pub fn value(&self, allowed: bool) -> i32 {
        if allowed {
            self.value
        } else {
            0
        }
    }
}
