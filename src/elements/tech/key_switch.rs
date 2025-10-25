//! Key switch system for managing key-operated controls in simulation environments.
//!
//! This module provides a complete key switch implementation with features like:
//! - Key depot management for storing and retrieving keys
//! - Multi-position switches with configurable ranges
//! - Spring-loaded positions that automatically return
//! - Pull-out functionality at extreme positions
//! - Sound effects and animations
//! - Event handling for different interaction types

use std::collections::HashMap;

use lotus_extra::vehicle::CockpitSide;

use crate::api::{
    animation::Animation,
    key_event::KeyEvent,
    sound::Sound,
    variable::{get_var, set_var},
    visible_flag::Visiblility,
};

/// A key depot manages the storage and retrieval of keys for key switches.
///
/// The depot tracks whether a key is available in the inventory using a boolean variable.
/// Keys can be inserted into the depot when removed from switches, and taken out when
/// needed for switch operation.
///
/// # Examples
///
/// ```rust
/// use pandemist_vehicle_elements::KeyDepot;
///
/// let depot = KeyDepot::new("engine_key_inventory");
///
/// // Check if key is available
/// if depot.testfor_key() {
///     // Key is available, can be used
/// }
///
/// // Put key back in depot
/// depot.put_in();
/// ```
#[derive(Debug, Clone)]
pub struct KeyDepot {
    /// The variable name used to track key availability in the inventory
    key_inventory: String,
}

impl KeyDepot {
    /// Creates a new key depot with the specified inventory variable name.
    ///
    /// # Arguments
    ///
    /// * `key_depot` - The variable name to use for tracking key availability
    ///
    /// # Examples
    ///
    /// ```rust
    /// let depot = KeyDepot::new("main_engine_key");
    /// ```
    pub fn new(key_depot: impl Into<String>) -> Self {
        Self {
            key_inventory: key_depot.into(),
        }
    }

    /// Tests if a key is currently available in the depot.
    ///
    /// # Returns
    ///
    /// * `true` if a key is available in the depot
    /// * `false` if no key is available
    #[must_use]
    pub fn testfor_key(&self) -> bool {
        get_var::<bool>(&self.key_inventory)
    }

    /// Puts a key into the depot, making it available for future use.
    ///
    /// This is typically called when a key is removed from a switch.
    pub fn put_in(&self) {
        set_var(&self.key_inventory, true);
    }

    /// Takes a key out of the depot, making it unavailable.
    ///
    /// This is typically called when a key is inserted into a switch.
    pub fn take_out(&self) {
        set_var(&self.key_inventory, false);
    }

    /// Tests for key availability and removes it if present.
    ///
    /// This is an atomic operation that both checks for and consumes a key
    /// if one is available.
    ///
    /// # Returns
    ///
    /// * `true` if a key was available and has been taken
    /// * `false` if no key was available
    #[must_use]
    pub fn test_and_take_out(&self) -> bool {
        if self.testfor_key() {
            self.take_out();
            true
        } else {
            false
        }
    }
}

//---------------------------------------

/// Builder for creating and configuring key switches.
///
/// The builder pattern allows for flexible configuration of key switch properties
/// including position ranges, spring-loaded behavior, pull-out functionality,
/// events, sounds, and animations.
///
/// # Examples
///
/// ```rust
/// use pandemist_vehicle_elements::{KeySwitch, KeyDepot};
///
/// let depot = KeyDepot::new("ignition_key");
/// let switch = KeySwitch::builder(depot, "ignition_anim", "ignition_vis", None)
///     .min(0)
///     .max(3)
///     .min_spring()
///     .pullout_max()
///     .event_turn("ignition_turn")
///     .snd_turn("key_turn_sound")
///     .init(true, 1)
///     .build();
/// ```
pub struct KeySwitchBuilder {
    /// Optional cab side specification for events
    cab_side: Option<CockpitSide>,

    /// Key depot for managing key availability
    key_depot: KeyDepot,
    /// Maximum position value
    max: i32,
    /// Minimum position value
    min: i32,
    /// Current position as floating point
    pos: f32,
    /// Current integer position value
    value: i32,
    /// Previous position value for change detection
    value_last: i32,

    /// Whether key can be pulled out at minimum position
    min_pullout: bool,
    /// Whether key can be pulled out at maximum position
    max_pullout: bool,
    /// Additional positions where key can be pulled out
    pullout_values: Vec<i32>,

    /// Whether minimum position is spring-loaded
    min_spring: bool,
    /// Whether maximum position is spring-loaded
    max_spring: bool,

    /// Animation controller for visual feedback
    key_anim: Animation,

    anim_mapping: HashMap<i32, f32>,
    /// Visibility controller for key presence
    key_visibility: Visiblility,

    /// Event for key turning/rotation
    key_turn: KeyEvent,
    /// Event for incrementing position
    key_plus: KeyEvent,
    /// Event for decrementing position
    key_minus: KeyEvent,
    /// Event for toggling key insertion/removal
    key_toggle: KeyEvent,

    snd_alt: HashMap<i32, Sound>,

    snd_default: Sound,
    /// Sound effect for key insertion
    snd_insert: Sound,
    /// Sound effect for key removal
    snd_takeout: Sound,
}

impl KeySwitchBuilder {
    /// Initializes the key switch with insertion state and position.
    ///
    /// # Arguments
    ///
    /// * `insert` - Whether to attempt key insertion on initialization
    /// * `new_pos` - Initial position to set if key is successfully inserted
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    pub fn init(mut self, insert: bool, new_pos: i32) -> Self {
        if insert && self.key_depot.test_and_take_out() {
            self.key_visibility.make_visible();
        }

        if self.min <= new_pos && new_pos <= self.max {
            self.pos = new_pos as f32;
            self.value = new_pos;

            let pos = match self.anim_mapping.get(&self.value) {
                Some(s) => *s,
                None => self.value as f32,
            };

            self.key_anim.set(self.pos);
        }
        self
    }

    /// Sets the maximum position value for the switch.
    ///
    /// # Arguments
    ///
    /// * `max` - Maximum position value
    pub fn max(mut self, max: i32) -> Self {
        self.max = max;
        self
    }

    /// Sets the minimum position value for the switch.
    ///
    /// # Arguments
    ///
    /// * `min` - Minimum position value
    pub fn min(mut self, min: i32) -> Self {
        self.min = min;
        self
    }

    /// Enables key pull-out functionality at the maximum position.
    ///
    /// When enabled, attempting to increment beyond the maximum position
    /// will remove the key from the switch.
    pub fn pullout_max(mut self) -> Self {
        self.max_pullout = true;
        self
    }

    /// Enables key pull-out functionality at the minimum position.
    ///
    /// When enabled, attempting to decrement below the minimum position
    /// will remove the key from the switch.
    pub fn pullout_min(mut self) -> Self {
        self.min_pullout = true;
        self
    }

    /// Adds an additional position where the key can be pulled out.
    ///
    /// # Arguments
    ///
    /// * `state` - Position value where pull-out is allowed
    pub fn add_pullout_state(mut self, state: i32) -> Self {
        self.pullout_values.push(state);
        self
    }

    /// Enables spring-loaded behavior at the maximum position.
    ///
    /// The switch will automatically return from the maximum position
    /// when the key is released.
    pub fn max_spring(mut self) -> Self {
        self.max_spring = true;
        self
    }

    /// Enables spring-loaded behavior at the minimum position.
    ///
    /// The switch will automatically return from the minimum position
    /// when the key is released.
    pub fn min_spring(mut self) -> Self {
        self.min_spring = true;
        self
    }

    /// Sets the event name for key toggle (insertion/removal) actions.
    ///
    /// # Arguments
    ///
    /// * `name` - Event name for toggle actions
    pub fn event_toggle(mut self, name: impl Into<String>) -> Self {
        self.key_toggle = KeyEvent::new(Some(&name.into()), self.cab_side);
        self
    }

    /// Sets the event name for key turning actions.
    ///
    /// # Arguments
    ///
    /// * `name` - Event name for turn actions
    pub fn event_turn(mut self, name: impl Into<String>) -> Self {
        self.key_turn = KeyEvent::new(Some(&name.into()), self.cab_side);
        self
    }

    /// Sets the event name for position increment actions.
    ///
    /// # Arguments
    ///
    /// * `name` - Event name for plus/increment actions
    pub fn event_plus(mut self, name: impl Into<String>) -> Self {
        self.key_plus = KeyEvent::new(Some(&name.into()), self.cab_side);
        self
    }

    /// Sets the event name for position decrement actions.
    ///
    /// # Arguments
    ///
    /// * `name` - Event name for minus/decrement actions
    pub fn event_minus(mut self, name: impl Into<String>) -> Self {
        self.key_minus = KeyEvent::new(Some(&name.into()), self.cab_side);
        self
    }

    pub fn mapping(mut self, map: HashMap<i32, f32>) -> Self {
        self.anim_mapping = map;
        self
    }

    /// Sets the sound effect for key insertion.
    ///
    /// # Arguments
    ///
    /// * `name` - Sound name for insertion effect
    pub fn snd_insert(mut self, name: impl Into<String>) -> Self {
        self.snd_insert = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Sets the sound effect for key removal.
    ///
    /// # Arguments
    ///
    /// * `name` - Sound name for removal effect
    pub fn snd_takeout(mut self, name: impl Into<String>) -> Self {
        self.snd_takeout = Sound::new_simple(Some(&name.into()));
        self
    }

    pub fn snd_default(mut self, name: impl Into<String>) -> Self {
        self.snd_default = Sound::new_simple(Some(&name.into()));
        self
    }

    pub fn add_alt_sound(mut self, position: i32, sound_name: impl Into<String>) -> Self {
        self.snd_alt
            .insert(position, Sound::new_simple(Some(&sound_name.into())));
        self
    }

    /// Builds and returns the configured key switch.
    ///
    /// # Returns
    ///
    /// A fully configured `KeySwitch` instance
    pub fn build(self) -> KeySwitch {
        KeySwitch {
            cab_side: self.cab_side,
            key_depot: self.key_depot,
            max: self.max,
            min: self.min,
            pos: self.pos,
            value: self.value,
            value_last: self.value_last,
            min_pullout: self.min_pullout,
            max_pullout: self.max_pullout,
            pullout_values: self.pullout_values,
            min_spring: self.min_spring,
            max_spring: self.max_spring,
            key_anim: self.key_anim,
            anim_mapping: self.anim_mapping,
            key_visibility: self.key_visibility,
            key_turn: self.key_turn,
            key_plus: self.key_plus,
            key_minus: self.key_minus,
            key_toggle: self.key_toggle,
            snd_alt: self.snd_alt,
            snd_default: self.snd_default,
            snd_insert: self.snd_insert,
            snd_takeout: self.snd_takeout,
        }
    }
}

/// A complete key switch implementation with multi-position support.
///
/// `KeySwitch` provides a realistic key-operated switch with features commonly
/// found in vehicles and machinery:
///
/// - **Multi-position operation**: Supports switches with multiple discrete positions
/// - **Spring-loaded positions**: Positions that automatically return when released
/// - **Pull-out functionality**: Key can be removed at specific positions
/// - **Sound and visual feedback**: Integrated animation and sound effects
/// - **Event-driven interaction**: Responds to various input events
///
/// # Examples
///
/// ```rust
/// use pandemist_vehicle_elements::{KeySwitch, KeyDepot};
///
/// // Create an ignition switch
/// let depot = KeyDepot::new("ignition_key_depot");
/// let mut ignition = KeySwitch::builder(
///     depot,
///     "ignition_animation",
///     "ignition_visibility",
///     None
/// )
/// .min(0)        // Off position
/// .max(3)        // Start position
/// .min_spring()  // Off position springs back
/// .max_spring()  // Start position springs back
/// .pullout_min() // Key can be removed when off
/// .event_turn("ignition_key_turn")
/// .snd_turn("ignition_sound")
/// .init(true, 1) // Insert key and set to position 1
/// .build();
///
/// // In your game loop
/// ignition.tick();
///
/// // Check current position
/// let current_pos = ignition.value(true);
/// println!("Ignition position: {}", current_pos);
/// ```
#[derive(Debug)]
#[expect(clippy::struct_excessive_bools)]
pub struct KeySwitch {
    /// Optional cab side specification for events
    cab_side: Option<CockpitSide>,

    /// Key depot for managing key availability
    key_depot: KeyDepot,
    /// Maximum position value
    max: i32,
    /// Minimum position value
    min: i32,
    /// Current position as floating point
    pos: f32,
    /// Current integer position value
    value: i32,
    /// Previous position value for change detection
    value_last: i32,

    /// Whether key can be pulled out at minimum position
    min_pullout: bool,
    /// Whether key can be pulled out at maximum position
    max_pullout: bool,
    /// Additional positions where key can be pulled out
    pullout_values: Vec<i32>,

    /// Whether minimum position is spring-loaded
    min_spring: bool,
    /// Whether maximum position is spring-loaded
    max_spring: bool,

    /// Animation controller for visual feedback
    key_anim: Animation,

    anim_mapping: HashMap<i32, f32>,
    /// Visibility controller for key presence
    key_visibility: Visiblility,

    /// Event for key turning/rotation - publicly accessible for external binding
    pub key_turn: KeyEvent,
    /// Event for incrementing position - publicly accessible for external binding
    pub key_plus: KeyEvent,
    /// Event for decrementing position - publicly accessible for external binding
    pub key_minus: KeyEvent,
    /// Event for toggling key insertion/removal - publicly accessible for external binding
    pub key_toggle: KeyEvent,

    snd_alt: HashMap<i32, Sound>,

    snd_default: Sound,
    /// Sound effect for key insertion
    snd_insert: Sound,
    /// Sound effect for key removal
    snd_takeout: Sound,
}

impl KeySwitch {
    /// Creates a new key switch builder with the specified configuration.
    ///
    /// # Arguments
    ///
    /// * `key_depot` - The key depot to use for key management
    /// * `animation_name` - Name of the animation to control
    /// * `visibility_name` - Name of the visibility flag to control
    /// * `cab_side` - Optional cab side specification for events
    ///
    /// # Returns
    ///
    /// A `KeySwitchBuilder` instance for further configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// let depot = KeyDepot::new("starter_key");
    /// let builder = KeySwitch::builder(
    ///     depot,
    ///     "starter_key_anim",
    ///     "starter_key_visible",
    ///     Some(CockpitSide::A)
    /// );
    /// ```
    pub fn builder(
        key_depot: KeyDepot,
        animation_name: impl Into<String>,
        visibility_name: impl Into<String>,
        cab_side: Option<CockpitSide>,
    ) -> KeySwitchBuilder {
        KeySwitchBuilder {
            cab_side,
            key_depot,
            max: 1,
            min: 0,
            pos: 0.0,
            value: 0,
            value_last: 0,
            min_pullout: false,
            max_pullout: false,
            pullout_values: vec![],
            min_spring: false,
            max_spring: false,
            key_anim: Animation::new(Some(&animation_name.into())),
            anim_mapping: HashMap::new(),
            key_visibility: Visiblility::new(visibility_name),
            key_turn: KeyEvent::new(None, None),
            key_plus: KeyEvent::new(None, None),
            key_minus: KeyEvent::new(None, None),
            key_toggle: KeyEvent::new(None, None),
            snd_alt: HashMap::new(),
            snd_default: Sound::new_simple(None),
            snd_insert: Sound::new_simple(None),
            snd_takeout: Sound::new_simple(None),
        }
    }

    /// Updates the internal state after position changes.
    ///
    /// This method synchronizes the integer value with the floating-point position
    /// and updates the associated animation.
    fn update(&mut self) {
        self.pos = match self.anim_mapping.get(&self.value) {
            Some(s) => *s,
            None => self.value as f32,
        };

        self.key_anim.set(self.pos);
    }

    /// Processes one tick of switch logic.
    ///
    /// This method should be called every frame or update cycle to handle:
    /// - Event processing for all configured input events
    /// - Spring-loaded position behavior
    /// - Pull-out functionality
    /// - Sound effect triggering
    /// - Animation updates
    ///
    /// The method handles different types of interactions:
    /// - **Turn events**: Toggle between positions or binary operation
    /// - **Plus/Minus events**: Increment/decrement through positions
    /// - **Toggle events**: Insert or remove the key
    /// - **Spring behavior**: Automatic return from spring-loaded positions
    /// - **Pull-out behavior**: Key removal at configured positions
    pub fn tick(&mut self) {
        self.value_last = self.value;

        if self.key_visibility.check() {
            // Handle key turning (binary toggle or rotation)
            if self.key_turn.is_just_pressed() {
                self.value = 1 - self.value;
                self.play_sound(self.value);
                self.update();
            }

            // Handle spring-loaded behavior on key release
            if self.key_turn.is_just_released() {
                if self.max_spring && self.value == self.max {
                    self.value = (1 - self.value).clamp(self.min, self.max);
                    self.play_sound(self.value);
                    self.update();
                }
                if self.min_spring && self.value == self.min {
                    self.value = (1 - self.value).clamp(self.min, self.max);
                    self.play_sound(self.value);
                    self.update();
                }
            }

            // Handle position increment
            if self.key_plus.is_just_pressed() {
                if self.value < self.max {
                    self.value += 1;
                    self.play_sound(self.value);
                    self.update();
                } else if self.value == self.max && self.max_pullout {
                    self.key_visibility.make_invisible();
                    self.key_depot.put_in();
                    self.snd_takeout.start();
                }
            }

            // Handle spring-loaded increment release
            if self.key_plus.is_just_released() && self.max_spring && self.value == self.max {
                self.value -= 1;
                self.play_sound(self.value);
                self.update();
            }

            // Handle position decrement
            if self.key_minus.is_just_pressed() {
                if self.value > self.min {
                    self.value -= 1;
                    self.play_sound(self.value);
                    self.update();
                } else if self.value == self.min && self.min_pullout {
                    self.key_visibility.make_invisible();
                    self.key_depot.put_in();
                    self.snd_takeout.start();
                }
            }

            // Handle spring-loaded decrement release
            if self.key_minus.is_just_released() && self.min_spring && self.value == self.min {
                self.value += 1;
                self.play_sound(self.value);
                self.update();
            }
        }

        // Handle key insertion/removal toggle
        if self.key_toggle.is_just_pressed() {
            if self.key_visibility.check() {
                // Key is inserted, check if current position allows removal
                if self.pullout_values.contains(&self.value)
                    || (self.value == self.min && self.min_pullout)
                    || (self.value == self.max && self.max_pullout)
                {
                    self.key_visibility.make_invisible();
                    self.key_depot.put_in();
                    self.snd_takeout.start();
                }
            } else if self.key_depot.test_and_take_out() {
                // Key is not inserted, try to insert it
                self.key_visibility.make_visible();
                self.snd_insert.start()
            }
        }
    }

    fn play_sound(&mut self, value: i32) {
        match self.snd_alt.get_mut(&value) {
            Some(snd) => snd.start(),
            None => self.snd_default.start(),
        }
    }

    /// Checks if the key is currently inserted in the switch.
    ///
    /// # Returns
    ///
    /// * `true` if the key is inserted and the switch is operational
    /// * `false` if the key is removed
    pub fn is_inserted(&self) -> bool {
        self.key_visibility.check()
    }

    /// Gets the current position value of the switch.
    ///
    /// # Arguments
    ///
    /// * `allowed` - Whether to return the actual value or force zero
    ///
    /// # Returns
    ///
    /// * The current position value if `allowed` is `true`
    /// * `0` if `allowed` is `false` (useful for disabling switch output)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let position = switch.value(switch.is_inserted());
    /// // Returns actual position only if key is inserted
    /// ```
    pub fn value(&self, allowed: bool) -> i32 {
        if allowed {
            self.value
        } else {
            0
        }
    }
}
