//! Voltage converter module with sound feedback
//!
//! This module provides a `Converter` struct that simulates a voltage converter
//! with audio feedback during startup and shutdown operations.

use lotus_script::time::delta;

use crate::api::sound::Sound;

/// A voltage converter with sound feedback that handles startup and shutdown sequences.
///
/// The `Converter` manages voltage conversion with configurable minimum voltage thresholds,
/// startup/shutdown timing, and provides audio feedback through volume control during
/// state transitions.
///
/// # Examples
///
/// ```rust
/// use your_crate::converter::Converter;
///
/// // Create a converter with custom parameters
/// let mut converter = Converter::new(
///     Some("startup_sound"),  // Sound volume name
///     0.1,                    // Minimum voltage (normalized)
///     2.0,                    // Startup time in seconds
///     1.0,                    // Shutdown time in seconds
/// );
///
/// // Simulate operation with sufficient voltage and active fuse
/// converter.tick(0.8, true);
/// println!("Output voltage: {}", converter.ouput_voltage_norm);
/// ```
#[derive(Debug, Default)]
pub struct Converter {
    /// Minimum normalized voltage required for operation
    const_min_voltage_norm: f32,
    /// Time in seconds for startup sequence
    const_startup_time: f32,
    /// Time in seconds for shutdown sequence
    const_shutdown_time: f32,

    /// Current sound volume level (0.0 to 1.0)
    sound_vol: f32,
    /// Sound instance for audio feedback
    sound: Sound,
    /// Current normalized output voltage (0.0 to 1.0)
    pub ouput_voltage_norm: f32,
}

impl Converter {
    /// Creates a new `Converter` instance with specified parameters.
    ///
    /// # Arguments
    ///
    /// * `snd_vol_name` - Optional name for the sound volume control
    /// * `min_voltage_norm` - Minimum normalized voltage threshold (0.0 to 1.0)
    /// * `startup_time` - Duration in seconds for the startup sequence
    /// * `shutdown_time` - Duration in seconds for the shutdown sequence
    ///
    /// # Returns
    ///
    /// A new `Converter` instance initialized with the provided parameters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Create converter with default sound settings
    /// let converter = Converter::new(None, 0.2, 3.0, 1.5);
    ///
    /// // Create converter with named sound volume
    /// let converter = Converter::new(Some("motor_sound"), 0.15, 2.5, 2.0);
    /// ```
    #[must_use]
    pub fn new(
        snd_vol_name: Option<&str>,
        min_voltage_norm: f32,
        startup_time: f32,
        shutdown_time: f32,
    ) -> Self {
        Self {
            const_min_voltage_norm: min_voltage_norm,
            const_startup_time: startup_time,
            const_shutdown_time: shutdown_time,
            sound: Sound::new(None, snd_vol_name, None),
            sound_vol: 0.0,
            ouput_voltage_norm: 0.0,
        }
    }

    /// Updates the converter state for one simulation tick.
    ///
    /// This method should be called regularly (typically once per frame) to update
    /// the converter's internal state, including voltage passthrough and sound volume
    /// transitions during startup and shutdown sequences.
    ///
    /// # Arguments
    ///
    /// * `input_voltage_norm` - Current normalized input voltage (0.0 to 1.0)
    /// * `fuse` - Whether the fuse is intact/active (true = active, false = blown)
    ///
    /// # Behavior
    ///
    /// - **Output voltage**: Always matches the input voltage regardless of other conditions
    /// - **Sound volume**:
    ///   - Increases gradually during startup when input voltage exceeds minimum and fuse is active
    ///   - Decreases gradually during shutdown when conditions are not met
    ///   - Transition speed is determined by the configured startup/shutdown times
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut converter = Converter::new(None, 0.1, 2.0, 1.0);
    ///
    /// // Normal operation - voltage above threshold, fuse active
    /// converter.tick(0.8, true);
    ///
    /// // Shutdown condition - fuse blown
    /// converter.tick(0.8, false);
    ///
    /// // Shutdown condition - voltage too low
    /// converter.tick(0.05, true);
    /// ```
    pub fn tick(&mut self, input_voltage_norm: f32, fuse: bool) {
        // Always pass through input voltage to output
        self.ouput_voltage_norm = input_voltage_norm;

        // Update sound volume based on operating conditions
        if input_voltage_norm > self.const_min_voltage_norm && fuse {
            // Startup: gradually increase volume
            self.sound_vol = (self.sound_vol + (1.0 / self.const_startup_time) * delta()).min(1.0);
        } else {
            // Shutdown: gradually decrease volume
            self.sound_vol = (self.sound_vol - (1.0 / self.const_shutdown_time) * delta()).max(0.0);
        }

        // Apply volume change to sound system
        self.sound.update_volume(self.sound_vol);
    }
}
