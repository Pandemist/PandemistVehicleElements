//! Light control utilities for lotus_script environments.
//!
//! This module provides various light control structures including basic light management,
//! light bulbs with smooth transitions, blink relays, and simple blinkers for different
//! lighting effects and animations.

use lotus_script::{time::delta, var::set_var};

/// A basic light structure that can control brightness through lotus_script variables.
///
/// The `Light` struct provides a simple interface for controlling light sources
/// by setting variables in the lotus_script environment.
#[derive(Debug)]
pub struct Light {
    /// The name of the light variable in the lotus_script environment.
    /// If `None`, the light operations will be ignored.
    name: Option<String>,
}

impl Light {
    /// Creates a new `Light` instance.
    ///
    /// # Arguments
    ///
    /// * `name` - Optional name of the light variable. If `None`, light operations will be ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::Light;
    ///
    /// let light = Light::new(Some("main_light"));
    /// let unnamed_light = Light::new(None);
    /// ```
    pub fn new(name: Option<&str>) -> Self {
        Light {
            name: name.map(|s| s.into()),
        }
    }

    /// Sets the brightness of the light source.
    ///
    /// Updates the lotus_script variable with the new brightness level.
    /// If no name was provided during construction, this operation is ignored.
    ///
    /// # Arguments
    ///
    /// * `new_level` - The new brightness level (typically between 0.0 and 1.0)
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::Light;
    ///
    /// let light = Light::new(Some("room_light"));
    /// light.set_brightness(0.8); // Set to 80% brightness
    /// ```
    pub fn set_brightness(&self, new_level: f32) {
        if let Some(light) = &self.name {
            set_var(light, new_level);
        }
    }
}

//=========================================================================

/// A light bulb with smooth transitions between brightness levels.
///
/// `LightBulb` provides smooth transitions when changing brightness levels,
/// simulating the behavior of real light bulbs that don't instantly change brightness.
/// The transition speed is controlled by the `on_off_speed` parameter.
pub struct LightBulb {
    /// The underlying light control structure
    light: Light,
    /// Speed factor for brightness transitions (higher values = faster transitions)
    on_off_speed: f32,
    /// Current brightness value
    value: f32,
}

impl LightBulb {
    /// Creates a new `LightBulb` instance.
    ///
    /// # Arguments
    ///
    /// * `light_name` - Name of the light variable in the lotus_script environment
    /// * `on_off_speed` - Speed factor for brightness transitions (higher = faster)
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::LightBulb;
    ///
    /// let bulb = LightBulb::new("bedroom_light", 2.0);
    /// ```
    pub fn new(light_name: &str, on_off_speed: f32) -> Self {
        Self {
            light: Light::new(Some(light_name)),
            on_off_speed,
            value: 0.0,
        }
    }

    /// Updates the light bulb's brightness towards the target value.
    ///
    /// This method should be called regularly (typically each frame) to smoothly
    /// transition the light's brightness towards the target value. The transition
    /// speed is determined by the `on_off_speed` parameter and the frame delta time.
    ///
    /// # Arguments
    ///
    /// * `target` - Target brightness level (typically between 0.0 and 1.0)
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::LightBulb;
    ///
    /// let mut bulb = LightBulb::new("living_room", 1.5);
    ///
    /// // In your update loop:
    /// bulb.tick(1.0); // Gradually increase to full brightness
    /// ```
    pub fn tick(&mut self, target: f32) {
        self.value =
            (1.0 - (-self.on_off_speed * delta()).exp()) * (target - self.value) + self.value;
        self.light.set_brightness(self.value);
    }
}

//=========================================================================

/// A blink relay that provides timed on/off cycles with state change detection.
///
/// `BlinkRelais` creates a periodic blinking pattern with configurable intervals
/// and provides feedback when the state changes, making it useful for triggering
/// events at specific blink transitions.
#[derive(Debug, Default)]
pub struct BlinkRelais {
    /// Time interval for one complete blink cycle
    interval: f32,
    /// Duration for which the relay stays "on" during each cycle
    on_time: f32,
    /// Current timer value
    timer: f32,
    /// Current state of the relay
    pub is_on: bool,
    /// Timer value to set when reset() is called
    reset_time: f32,
}

impl BlinkRelais {
    /// Creates a new `BlinkRelais` instance.
    ///
    /// # Arguments
    ///
    /// * `interval` - Total duration of one blink cycle
    /// * `on_time` - Duration for which the relay stays "on" (must be ≤ interval)
    /// * `reset_time` - Timer value to use when reset() is called
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::BlinkRelais;
    ///
    /// // Blink every 2 seconds, stay on for 0.5 seconds
    /// let mut relay = BlinkRelais::new(2.0, 0.5, 0.0);
    /// ```
    pub fn new(interval: f32, on_time: f32, reset_time: f32) -> Self {
        Self {
            interval,
            on_time,
            timer: 0.0,
            is_on: false,
            reset_time,
        }
    }

    /// Updates the blink relay and returns state change information.
    ///
    /// This method should be called regularly to update the relay's state.
    /// It returns information about state changes that occurred during this tick.
    ///
    /// # Returns
    ///
    /// * `1` - Relay just turned on
    /// * `-1` - Relay just turned off  
    /// * `0` - No state change occurred
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::BlinkRelais;
    ///
    /// let mut relay = BlinkRelais::new(1.0, 0.3, 0.0);
    ///
    /// // In your update loop:
    /// match relay.tick() {
    ///     1 => println!("Relay turned ON"),
    ///     -1 => println!("Relay turned OFF"),
    ///     _ => {} // No change
    /// }
    /// ```
    pub fn tick(&mut self) -> i32 {
        self.timer += delta();
        if self.timer > self.interval {
            self.timer -= self.interval;
        }

        let new_on = self.timer < self.on_time;

        let result = if new_on && !self.is_on {
            1
        } else if !new_on && self.is_on {
            -1
        } else {
            0
        };

        self.is_on = new_on;
        result
    }

    /// Resets the relay to its initial state.
    ///
    /// Sets the timer to the configured reset time and turns the relay off.
    /// This can be used to synchronize the relay or restart its cycle.
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::BlinkRelais;
    ///
    /// let mut relay = BlinkRelais::new(2.0, 0.5, 1.0);
    /// relay.reset(); // Timer set to 1.0, relay turned off
    /// ```
    pub fn reset(&mut self) {
        self.timer = self.reset_time;
        self.is_on = false;
    }
}

//=========================================================================

/// A simple blinker with separate on/off intervals and target-based control.
///
/// `SimpleBlinker` provides a basic blinking functionality with different intervals
/// for the "on" and "off" states. It only blinks when the target is set to `true`,
/// and can be used for simple flashing effects or status indicators.
#[derive(Debug)]
pub struct SimpleBlinker {
    /// Duration to stay in the "on" state
    interval_on: f32,
    /// Duration to stay in the "off" state  
    interval_off: f32,
    /// Current timer value
    timer: f32,
    /// Whether blinking is enabled
    pub target: bool,
    /// Current light state (true = on, false = off)
    pub lighted: bool,
}

impl SimpleBlinker {
    /// Creates a new `SimpleBlinker` instance.
    ///
    /// # Arguments
    ///
    /// * `interval_on` - Duration to stay in the "on" state
    /// * `interval_off` - Duration to stay in the "off" state
    ///
    /// # Returns
    ///
    /// A new `SimpleBlinker` instance with blinking disabled by default.
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::SimpleBlinker;
    ///
    /// let mut blinker = SimpleBlinker::new(0.5, 0.3); // On for 0.5s, off for 0.3s
    /// blinker.target = true; // Enable blinking
    /// ```
    #[must_use]
    pub fn new(interval_on: f32, interval_off: f32) -> Self {
        Self {
            interval_on,
            interval_off,
            timer: 0.0,
            target: false,
            lighted: false,
        }
    }

    /// Updates the blinker state.
    ///
    /// This method should be called regularly to update the blinker's state.
    /// When `target` is `true`, the blinker alternates between on and off states
    /// based on the configured intervals. When `target` is `false`, the light
    /// is turned off and the timer is reset.
    ///
    /// The current state can be read from the `lighted` field.
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::SimpleBlinker;
    ///
    /// let mut blinker = SimpleBlinker::new(1.0, 0.5);
    /// blinker.target = true;
    ///
    /// // In your update loop:
    /// blinker.tick();
    /// if blinker.lighted {
    ///     println!("Light is ON");
    /// } else {
    ///     println!("Light is OFF");
    /// }
    /// ```
    pub fn tick(&mut self) {
        if self.target {
            self.timer += delta();

            if (self.timer > self.interval_on) && self.lighted {
                self.lighted = !self.lighted;
                self.timer -= self.interval_on;
            }
            if (self.timer > self.interval_off) && !self.lighted {
                self.lighted = !self.lighted;
                self.timer -= self.interval_off;
            }
        } else {
            self.lighted = false;
            self.timer = self.interval_off;
        }
    }
}
