//! Voltage converter module with sound feedback
//!
//! This module provides a `Converter` struct that simulates a voltage converter
//! with audio feedback during startup and shutdown operations.

use lotus_extra::rand::gen_f32;
use lotus_script::{rand::gen_f64, time::delta};

use crate::{
    api::sound::Sound, elements::std::timer::Timer,
    management::enums::target_enums::SwitchingTarget,
};

/// A voltage converter with sound feedback that handles startup and shutdown sequences.
///
/// The `Converter` manages voltage conversion with configurable minimum voltage thresholds,
/// startup/shutdown timing, and provides audio feedback through volume control during
/// state transitions.
///
/// # Examples
///
/// ```rust
/// use pandemist_vehicle_elements::converter::Converter;
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

//====================================

#[derive(Debug, Default)]
pub struct ManualConverterBuilder {
    state: bool,

    const_min_voltage_norm: f32,
    const_startup_time: f32,
    const_shutdown_time: f32,

    snd_turn_on: Sound,
    snd_turn_off: Sound,

    sound: Sound,
}

impl ManualConverterBuilder {
    pub fn add_start_sound(mut self, snd_turn_on_name: &str) -> Self {
        self.snd_turn_on = Sound::new_simple(Some(snd_turn_on_name));
        self
    }

    pub fn add_end_sound(mut self, snd_turn_off_name: &str) -> Self {
        self.snd_turn_off = Sound::new_simple(Some(snd_turn_off_name));
        self
    }

    pub fn add_run_sound(mut self, snd_run_name: &str) -> Self {
        self.sound = Sound::new_simple(Some(snd_run_name));
        self
    }

    pub fn init(mut self, on: bool) -> Self {
        if on {
            self.state = true;
        }
        self
    }

    pub fn build(self) -> ManualConverter {
        ManualConverter {
            state: self.state,
            switching_timer: Timer::new(),
            target: SwitchingTarget::Neutral,
            target_last: SwitchingTarget::Neutral,
            const_min_voltage_norm: self.const_min_voltage_norm,
            const_startup_time: self.const_startup_time,
            const_shutdown_time: self.const_shutdown_time,
            snd_turn_on: self.snd_turn_on,
            snd_turn_off: self.snd_turn_off,
            sound_vol: 0.0,
            sound: self.sound,
            ouput_voltage_norm: 0.0,
        }
    }
}

#[derive(Debug, Default)]
pub struct ManualConverter {
    pub state: bool,

    switching_timer: Timer,

    pub target: SwitchingTarget,
    target_last: SwitchingTarget,

    const_min_voltage_norm: f32,
    const_startup_time: f32,
    const_shutdown_time: f32,

    snd_turn_on: Sound,
    snd_turn_off: Sound,

    sound_vol: f32,
    sound: Sound,
    pub ouput_voltage_norm: f32,
}

impl ManualConverter {
    pub fn builder(
        min_voltage_norm: f32,
        startup_time: f32,
        shutdown_time: f32,
    ) -> ManualConverterBuilder {
        ManualConverterBuilder {
            state: false,
            const_min_voltage_norm: min_voltage_norm,
            const_startup_time: startup_time,
            const_shutdown_time: shutdown_time,
            snd_turn_on: Sound::new_simple(None),
            snd_turn_off: Sound::new_simple(None),
            sound: Sound::new_simple(None),
        }
    }

    pub fn tick(&mut self, input_voltage_norm: f32, fuse: bool) {
        match (self.target, self.state) {
            (SwitchingTarget::TurnOn(delay), false) => {
                if self.switching_timer.idle() {
                    self.switching_timer.start(delay);
                }
                self.switching_timer.tick();
                if self.switching_timer.finished() {
                    self.switching_timer.reset();
                    self.snd_turn_on.start();
                    self.state = true;
                }
            }
            (SwitchingTarget::TurnOff(delay), true) => {
                if self.switching_timer.idle() {
                    self.switching_timer.start(delay);
                }
                self.switching_timer.tick();
                if self.switching_timer.finished() {
                    self.switching_timer.reset();
                    self.snd_turn_off.start();
                    self.state = false;
                }
            }
            (_, _) => {
                self.switching_timer.reset();
            }
        }

        if self.state {
            self.sound_vol = (self.sound_vol + (1.0 / self.const_startup_time) * delta()).min(1.0);
        } else {
            self.sound_vol = (self.sound_vol - (1.0 / self.const_shutdown_time) * delta()).max(0.0);
        }

        self.sound.update_volume(self.sound_vol);

        self.ouput_voltage_norm = input_voltage_norm * self.sound_vol;
    }
}

//====================================

#[derive(Debug, Default)]
pub struct SubwayConverterBuilder {
    const_time: f32,
    const_vari_max_rate: f32,
    const_vari_max: f32,

    switch_on: bool,

    state: bool,

    offset: f32,
    offset_rate: f32,
    offset_target: f32,
    frequence: f32,
    frequence_rate: f32,
    trafo_pitch: f32,

    snd_turn_on: Sound,
    snd_turn_off: Sound,

    sound: Sound,
    sound_trafo: Sound,
}

impl SubwayConverterBuilder {
    pub fn configure_consts(
        mut self,
        const_time: f32,
        const_vari_max_rate: f32,
        const_vari_max: f32,
    ) -> Self {
        self.const_time = const_time;
        self.const_vari_max_rate = const_vari_max_rate;
        self.const_vari_max = const_vari_max;
        self
    }

    pub fn add_start_sound(mut self, snd_turn_on_name: &str) -> Self {
        self.snd_turn_on = Sound::new_simple(Some(snd_turn_on_name));
        self
    }

    pub fn add_end_sound(mut self, snd_turn_off_name: &str) -> Self {
        self.snd_turn_off = Sound::new_simple(Some(snd_turn_off_name));
        self
    }

    pub fn add_run_sound(mut self, snd_run_name: &str) -> Self {
        self.sound = Sound::new_simple(Some(snd_run_name));
        self
    }

    pub fn add_travo_sound(mut self, snd_vol_name: &str, snd_pitch_name: &str) -> Self {
        self.sound = Sound::new(None, Some(snd_vol_name), Some(snd_pitch_name));
        self
    }

    pub fn init(mut self, on: bool) -> Self {
        if on {
            self.state = true;
        }
        self
    }

    pub fn build(self) -> SubwayConverter {
        SubwayConverter {
            const_time: self.const_time,
            const_vari_max_rate: self.const_vari_max_rate,
            const_vari_max: self.const_vari_max,
            switch_on: self.switch_on,
            state: self.state,
            switching_timer: Timer::new(),
            target: SwitchingTarget::Neutral,
            target_last: SwitchingTarget::Neutral,
            offset: self.offset,
            offset_rate: self.offset_rate,
            offset_target: self.offset_target,
            frequence: self.frequence,
            frequence_rate: self.frequence_rate,
            trafo_pitch: self.trafo_pitch,
            snd_turn_on: self.snd_turn_on,
            snd_turn_off: self.snd_turn_off,
            sound_vol: 0.0,
            sound: self.sound,
            sound_trafo: self.sound_trafo,
            ouput_voltage_norm: 0.0,
        }
    }
}

#[derive(Debug, Default)]
pub struct SubwayConverter {
    const_time: f32,
    const_vari_max_rate: f32,
    const_vari_max: f32,

    switch_on: bool,

    pub state: bool,

    switching_timer: Timer,

    pub target: SwitchingTarget,
    target_last: SwitchingTarget,

    offset: f32,
    offset_rate: f32,
    offset_target: f32,
    frequence: f32,
    frequence_rate: f32,
    trafo_pitch: f32,

    snd_turn_on: Sound,
    snd_turn_off: Sound,

    sound_vol: f32,
    sound: Sound,
    sound_trafo: Sound,
    pub ouput_voltage_norm: f32,
}

impl SubwayConverter {
    pub fn builder() -> SubwayConverterBuilder {
        SubwayConverterBuilder {
            const_time: 0.6,
            const_vari_max_rate: 0.03,
            const_vari_max: 0.04,

            switch_on: false,

            state: false,

            offset: 0.0,
            offset_rate: 0.0,
            offset_target: 0.0,
            frequence: 0.0,
            frequence_rate: 0.0,
            trafo_pitch: 0.0,

            snd_turn_on: Sound::new_simple(None),
            snd_turn_off: Sound::new_simple(None),
            sound: Sound::new_simple(None),
            sound_trafo: Sound::new_simple(None),
        }
    }

    pub fn tick(
        &mut self,
        input_voltage_norm: f32,
        battery: bool,
        frequence_modifyer: Option<f32>,
    ) {
        match (self.target, self.state) {
            (SwitchingTarget::TurnOn(delay), false) => {
                if self.switching_timer.idle() {
                    self.switching_timer.start(delay);
                }
                self.switching_timer.tick();
                if self.switching_timer.finished() {
                    self.switching_timer.reset();
                    self.snd_turn_on.start();
                    self.switch_on = true;
                }
            }
            (SwitchingTarget::TurnOff(delay), true) => {
                if self.switching_timer.idle() {
                    self.switching_timer.start(delay);
                }
                self.switching_timer.tick();
                if self.switching_timer.finished() {
                    self.switching_timer.reset();
                    self.snd_turn_off.start();
                    self.switch_on = false;
                }
            }
            (_, _) => {
                self.switching_timer.reset();
            }
        }

        // ----------------------------------------------------
        // Offset berechnen
        // ----------------------------------------------------

        let newoffset = self.offset + self.offset_rate * delta();

        if (newoffset - self.offset_target) * self.offset_rate >= 0.0 {
            self.offset_rate = gen_f32(0.0..=self.const_vari_max_rate);

            self.offset_target = gen_f32(0.0_f32.next_up()..=self.const_vari_max);

            if gen_f64() as f32 >= 0.5 {
                self.offset_target = -self.offset_target;
            }

            if self.offset_rate * (newoffset - self.offset_target) >= 0.0 {
                self.offset_rate = -self.offset_rate;
            }
        }

        self.offset = newoffset;

        // ----------------------------------------------------
        // Aktivierungszustand bestimmen
        // ----------------------------------------------------

        let new_active = self.switch_on && input_voltage_norm > 0.6 && battery;

        if new_active != self.state {
            self.state = new_active;

            self.trafo_pitch = if new_active { 1.0 } else { 0.0 };
        }

        // ----------------------------------------------------
        // Umformer-Frequenz
        // ----------------------------------------------------

        if self.state {
            self.frequence = frequence_modifyer.unwrap_or(0.77);

            if self.frequence < 1.5 {
                self.frequence += self.offset;
            }
        } else {
            self.frequence = 1.0;
        }

        // ----------------------------------------------------
        // snd_trafo_pitch an Sollwert angleichen
        // ----------------------------------------------------

        if delta() < 0.05 {
            let delta_value =
                100.0_f32.min((-100.0_f32).max((self.frequence - self.trafo_pitch) * 100.0));

            let damper = if self.trafo_pitch > 1.5 { 30.0 } else { 20.0 };

            self.frequence_rate += delta_value * delta() - self.frequence_rate * delta() * damper;

            self.trafo_pitch = 0.0_f32.max(self.trafo_pitch + self.frequence_rate * delta());
        } else {
            self.trafo_pitch += 0.7 * (self.frequence - self.trafo_pitch);
        }

        self.sound_trafo.update_pitch(self.trafo_pitch);

        self.ouput_voltage_norm = input_voltage_norm * f32::from(self.frequence > 0.5);
    }
}
