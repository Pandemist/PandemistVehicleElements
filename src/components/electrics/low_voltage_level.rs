//! Low voltage level management for battery systems.
//!
//! This module provides functionality to manage and monitor low voltage battery systems,
//! including battery main switch control, voltage monitoring, and sound notifications.
//! It supports configurable voltage thresholds, delayed switching operations, and
//! automatic protection against undervoltage conditions.

use lotus_script::time::delta;

use crate::{api::sound::Sound, management::enums::target_enums::SwitchingTarget};

/// Builder for creating and configuring a `LowVoltageLevel` instance.
///
/// This builder provides a fluent interface to configure all parameters of a low voltage
/// battery system before creating the final `LowVoltageLevel` instance. It allows setting
/// voltage thresholds, sound notifications, and initial states.
///
/// # Examples
///
/// ```rust
/// let low_voltage = LowVoltageLevel::builder(12.0)
///     .voltage_max_v(14.4)
///     .voltage_min_v(10.5)
///     .voltage_loss_vs(0.1)
///     .voltage_load_vs(0.2)
///     .snd_battery_on("battery_on_sound")
///     .snd_battery_off("battery_off_sound")
///     .init(false)
///     .build();
/// ```
pub struct LowVoltageLevelBuilder {
    const_battery_voltage_normal_v: f32,
    const_battery_voltage_max_v: f32,
    const_battery_voltage_min_v: f32,
    const_battery_voltage_loss_vs: f32,
    const_battery_voltage_load_vs: f32,

    battery_voltage_abs: f32,
    battery_voltage_abs_last: f32,

    battery_switching_timer: f32,

    battery_mainswitch: bool,
    battery_mainswitch_last: bool,

    low_voltage_abs: f32,
    low_voltage_norm: f32,
    permanent_voltage_abs: f32,
    permanent_voltage_norm: f32,
    permanent_voltage_norm_last: f32,

    snd_battery_on: Sound,
    snd_battery_off: Sound,
}

impl LowVoltageLevelBuilder {
    /// Sets the initial voltage value for the permanent voltage.
    ///
    /// This value is used as the starting point for voltage calculations and
    /// represents the voltage present when the system is initialized.
    ///
    /// # Arguments
    ///
    /// * `value` - The initial voltage value in volts
    ///
    /// # Returns
    ///
    /// Returns `Self` for method chaining
    pub fn voltage_init_v(mut self, value: f32) -> Self {
        self.permanent_voltage_abs = value;
        self
    }

    /// Sets the maximum battery voltage threshold.
    ///
    /// This is the upper limit for battery charging. The battery voltage will not
    /// exceed this value during charging operations.
    ///
    /// # Arguments
    ///
    /// * `value` - The maximum voltage value in volts
    ///
    /// # Returns
    ///
    /// Returns `Self` for method chaining
    pub fn voltage_max_v(mut self, value: f32) -> Self {
        self.const_battery_voltage_max_v = value;
        self
    }

    /// Sets the minimum battery voltage threshold.
    ///
    /// When the battery voltage drops below this threshold, the battery main switch
    /// will automatically turn off to protect the battery from deep discharge.
    ///
    /// # Arguments
    ///
    /// * `value` - The minimum voltage value in volts
    ///
    /// # Returns
    ///
    /// Returns `Self` for method chaining
    pub fn voltage_min_v(mut self, value: f32) -> Self {
        self.const_battery_voltage_min_v = value;
        self
    }

    /// Sets the voltage loss rate per second.
    ///
    /// This represents the natural discharge rate of the battery when the main
    /// switch is active, measured in volts per second.
    ///
    /// # Arguments
    ///
    /// * `value` - The voltage loss rate in volts per second
    ///
    /// # Returns
    ///
    /// Returns `Self` for method chaining
    pub fn voltage_loss_vs(mut self, value: f32) -> Self {
        self.const_battery_voltage_loss_vs = value;
        self
    }

    /// Sets the voltage loading rate per second.
    ///
    /// This represents the charging rate of the battery based on the input voltage
    /// (umformerspannung), measured in volts per second per volt of input.
    ///
    /// # Arguments
    ///
    /// * `value` - The voltage loading rate coefficient
    ///
    /// # Returns
    ///
    /// Returns `Self` for method chaining
    pub fn voltage_load_vs(mut self, value: f32) -> Self {
        self.const_battery_voltage_load_vs = value;
        self
    }

    /// Sets the sound to play when the battery turns on.
    ///
    /// # Arguments
    ///
    /// * `name` - The name/path of the sound file to play
    ///
    /// # Returns
    ///
    /// Returns `Self` for method chaining
    pub fn snd_battery_on(mut self, name: impl Into<String>) -> Self {
        self.snd_battery_on = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Sets the sound to play when the battery turns off.
    ///
    /// # Arguments
    ///
    /// * `name` - The name/path of the sound file to play
    ///
    /// # Returns
    ///
    /// Returns `Self` for method chaining
    pub fn snd_battery_off(mut self, name: impl Into<String>) -> Self {
        self.snd_battery_off = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Sets the initial state of the battery main switch.
    ///
    /// # Arguments
    ///
    /// * `state` - `true` if the battery should start in the "on" state, `false` otherwise
    ///
    /// # Returns
    ///
    /// Returns `Self` for method chaining
    pub fn init(mut self, state: bool) -> Self {
        self.battery_mainswitch = state;
        self.battery_mainswitch_last = state;
        self
    }

    /// Builds and returns the configured `LowVoltageLevel` instance.
    ///
    /// This method consumes the builder and creates a fully configured `LowVoltageLevel`
    /// with all the parameters set through the builder methods. It also performs initial
    /// calculations for voltage normalization and sends the initial battery state.
    ///
    /// # Returns
    ///
    /// A configured `LowVoltageLevel` instance ready for use
    pub fn build(self) -> LowVoltageLevel {
        LowVoltageLevel {
            const_battery_voltage_normal_v: self.const_battery_voltage_normal_v,
            const_battery_voltage_max_v: self.const_battery_voltage_max_v,
            const_battery_voltage_min_v: self.const_battery_voltage_min_v,
            const_battery_voltage_loss_vs: self.const_battery_voltage_loss_vs,
            const_battery_voltage_load_vs: self.const_battery_voltage_load_vs,
            battery_voltage_abs: self.battery_voltage_abs,
            battery_voltage_abs_last: self.battery_voltage_abs_last,
            battery_switching_timer: self.battery_switching_timer,
            battery_mainswitch: self.battery_mainswitch,
            battery_mainswitch_last: self.battery_mainswitch_last,
            low_voltage_norm: self.low_voltage_abs / self.const_battery_voltage_normal_v,
            low_voltage_abs: self.battery_voltage_abs * f32::from(self.battery_mainswitch),
            permanent_voltage_abs: self.permanent_voltage_abs,
            permanent_voltage_norm: self.permanent_voltage_norm,
            permanent_voltage_norm_last: self.permanent_voltage_norm_last,
            snd_battery_on: self.snd_battery_on,
            snd_battery_off: self.snd_battery_off,
        }
    }
}

/// Low voltage battery management system.
///
/// This struct manages a low voltage battery system with features including:
/// - Battery main switch control with delayed switching
/// - Voltage monitoring and normalization
/// - Automatic undervoltage protection
/// - Sound notifications for state changes
/// - Configurable charging and discharging rates
///
/// The system maintains both absolute and normalized voltage values, where normalized
/// values are relative to a configured normal voltage level. This allows for easier
/// integration with systems that expect standardized voltage ranges.
///
/// # Examples
///
/// ```rust
/// use crate::management::enums::target_enums::SwitchingTarget;
///
/// let mut battery_system = LowVoltageLevel::builder(12.0)
///     .voltage_max_v(14.4)
///     .voltage_min_v(10.5)
///     .build();
///
/// // Update the system each frame/tick
/// battery_system.tick(13.8, SwitchingTarget::TurnOn(2.0));
/// ```
#[derive(Default, Debug)]
pub struct LowVoltageLevel {
    /// Normal/reference voltage level in volts
    const_battery_voltage_normal_v: f32,
    /// Maximum allowed battery voltage in volts
    const_battery_voltage_max_v: f32,
    /// Minimum allowed battery voltage in volts (protection threshold)
    const_battery_voltage_min_v: f32,
    /// Voltage loss rate in volts per second
    const_battery_voltage_loss_vs: f32,
    /// Voltage charging rate coefficient
    const_battery_voltage_load_vs: f32,

    /// Current absolute battery voltage
    battery_voltage_abs: f32,
    /// Previous frame's absolute battery voltage
    battery_voltage_abs_last: f32,

    /// Timer for delayed switching operations
    battery_switching_timer: f32,

    /// Current state of the battery main switch
    pub battery_mainswitch: bool,
    /// Previous state of the battery main switch
    pub battery_mainswitch_last: bool,

    /// Low voltage output (affected by main switch state)
    low_voltage_abs: f32,
    /// Normalized low voltage output (0.0 to 1.0+ range)
    pub low_voltage_norm: f32,
    /// Permanent voltage (not affected by main switch)
    permanent_voltage_abs: f32,
    /// Normalized permanent voltage
    pub permanent_voltage_norm: f32,
    /// Previous frame's normalized permanent voltage
    permanent_voltage_norm_last: f32,

    /// Sound played when battery turns on
    snd_battery_on: Sound,
    /// Sound played when battery turns off
    snd_battery_off: Sound,
}

impl LowVoltageLevel {
    /// Creates a new builder for configuring a `LowVoltageLevel` instance.
    ///
    /// This is the primary way to create a new `LowVoltageLevel`. The builder pattern
    /// allows for flexible configuration of all system parameters.
    ///
    /// # Arguments
    ///
    /// * `voltage_normal_v` - The reference voltage level used for normalization
    ///
    /// # Returns
    ///
    /// A `LowVoltageLevelBuilder` instance for configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// let battery_system = LowVoltageLevel::builder(12.0)
    ///     .voltage_max_v(14.4)
    ///     .voltage_min_v(10.5)
    ///     .build();
    /// ```
    pub fn builder(voltage_normal_v: f32) -> LowVoltageLevelBuilder {
        LowVoltageLevelBuilder {
            const_battery_voltage_normal_v: voltage_normal_v,
            const_battery_voltage_max_v: voltage_normal_v,
            const_battery_voltage_min_v: 0.0,
            const_battery_voltage_loss_vs: 0.0,
            const_battery_voltage_load_vs: 0.0,
            battery_voltage_abs: voltage_normal_v,
            battery_voltage_abs_last: voltage_normal_v,
            battery_switching_timer: 0.0,
            battery_mainswitch: false,
            battery_mainswitch_last: false,
            low_voltage_abs: 0.0,
            low_voltage_norm: 0.0,
            permanent_voltage_abs: voltage_normal_v,
            permanent_voltage_norm: 1.0,
            permanent_voltage_norm_last: 0.0,
            snd_battery_on: Sound::new_simple(None),
            snd_battery_off: Sound::new_simple(None),
        }
    }

    /// Updates the battery system state for one simulation tick.
    ///
    /// This method should be called once per frame/update cycle to maintain the battery
    /// system state. It handles:
    /// - Battery main switch control with delays
    /// - Voltage charging and discharging simulation
    /// - Undervoltage protection
    /// - Sound notifications
    /// - Message sending for system integration
    ///
    /// # Arguments
    ///
    /// * `umformerspannung` - Input voltage from the transformer/charger system
    /// * `battery_target` - Desired switching target (turn on, turn off, or neutral)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crate::management::enums::target_enums::SwitchingTarget;
    ///
    /// // Turn on battery with 2-second delay
    /// battery_system.tick(13.8, SwitchingTarget::TurnOn(2.0));
    ///
    /// // Turn off battery with 1-second delay  
    /// battery_system.tick(13.8, SwitchingTarget::TurnOff(1.0));
    ///
    /// // Neutral state (no switching action)
    /// battery_system.tick(13.8, SwitchingTarget::Neutral);
    /// ```
    pub fn tick(&mut self, umformerspannung: f32, battery_target: SwitchingTarget) {
        self.battery_mainswitch_last = self.battery_mainswitch;
        self.battery_voltage_abs_last = self.battery_voltage_abs;

        // Handle battery main switch with delay for turn on/off operations
        match battery_target {
            SwitchingTarget::TurnOn(delay) => {
                self.battery_switching_timer += delta();
                if self.battery_switching_timer > delay {
                    self.battery_mainswitch = true;
                    self.snd_battery_on.start();
                }
            }
            SwitchingTarget::TurnOff(delay) => {
                self.battery_switching_timer += delta();
                if self.battery_switching_timer > delay {
                    self.battery_mainswitch = false;
                    self.snd_battery_off.start();
                }
            }
            SwitchingTarget::Neutral => {
                self.battery_switching_timer = 0.0;
            }
        }

        // Emergency shutdown: turn off battery main switch if voltage drops below minimum
        if self.battery_voltage_abs < self.const_battery_voltage_min_v {
            self.battery_mainswitch = false;
            self.snd_battery_off.start();
        }

        // Update battery voltage based on charging/discharging when main switch is active
        if self.battery_mainswitch {
            // Apply discharge (voltage loss)
            self.battery_voltage_abs =
                (self.battery_voltage_abs - self.const_battery_voltage_loss_vs * delta()).max(0.0);

            // Apply charging from input voltage, clamped to maximum voltage
            self.battery_voltage_abs = self.const_battery_voltage_max_v.min(
                self.battery_voltage_abs
                    + self.const_battery_voltage_load_vs * umformerspannung * delta(),
            );
        }

        // Update output voltages
        self.low_voltage_abs = self.battery_voltage_abs * f32::from(self.battery_mainswitch);
        self.low_voltage_norm = self.low_voltage_abs / self.const_battery_voltage_normal_v;

        self.permanent_voltage_abs = self.battery_voltage_abs;
        self.permanent_voltage_norm =
            self.permanent_voltage_abs / self.const_battery_voltage_normal_v;
    }
}
