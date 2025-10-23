//! Sound control utilities for lotus_script integration.
//!
//! This module provides various sound control structures and utilities for managing
//! audio playback, volume control, and sound state transitions in lotus_script applications.
//!
//! # Features
//!
//! - Basic sound start/stop control
//! - Volume and pitch manipulation
//! - Gradual volume transitions
//! - Sound sequences with end sounds
//! - Complex sound chains with start, loop, and end sounds
//!
//! # Examples
//!
//! ```rust
//! use sound::{Sound, SoundTarget, SoundWithVol};
//!
//! // Basic sound control
//! let mut basic_sound = Sound::new_simple(Some("my_sound"));
//! basic_sound.start();
//! basic_sound.stop();
//!
//! // Sound with volume control
//! let mut vol_sound = SoundWithVol::new("background_music", "bg_volume", 0.5, 0.2);
//! vol_sound.tick(true); // Gradually increase volume (till 1.0)
//! ```

use lotus_script::{time::delta, var::set_var};

use crate::api::variable::get_var;

/// Represents the target state for sound playback.
///
/// This enum is used to control whether a sound should be started or stopped,
/// and provides convenient conversions to and from various types.
///
/// # Examples
///
/// ```rust
/// use sound::SoundTarget;
///
/// let start_target = SoundTarget::Start;
/// let stop_target = SoundTarget::Stop;
///
/// // Convert from i32
/// let from_int: SoundTarget = 1.into(); // SoundTarget::Start
/// let from_zero: SoundTarget = 0.into(); // SoundTarget::Stop
///
/// // Convert to bool
/// let start_bool: bool = SoundTarget::Start.into(); // true
/// let stop_bool: bool = SoundTarget::Stop.into(); // false
/// ```
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SoundTarget {
    /// Start playing the sound
    Start,
    /// Stop playing the sound (default)
    #[default]
    Stop,
}

impl From<i32> for SoundTarget {
    /// Converts an i32 to a SoundTarget.
    ///
    /// # Arguments
    ///
    /// * `val` - The integer value (1 for Start, any other value for Stop)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sound::SoundTarget;
    /// assert_eq!(SoundTarget::from(1), SoundTarget::Start);
    /// assert_eq!(SoundTarget::from(0), SoundTarget::Stop);
    /// assert_eq!(SoundTarget::from(-1), SoundTarget::Stop);
    /// ```
    fn from(val: i32) -> SoundTarget {
        match val {
            1 => SoundTarget::Start,
            _ => SoundTarget::Stop,
        }
    }
}

impl From<SoundTarget> for bool {
    /// Converts a SoundTarget to a boolean.
    ///
    /// # Arguments
    ///
    /// * `val` - The SoundTarget to convert
    ///
    /// # Returns
    ///
    /// * `true` for SoundTarget::Start
    /// * `false` for SoundTarget::Stop
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sound::SoundTarget;
    /// assert_eq!(bool::from(SoundTarget::Start), true);
    /// assert_eq!(bool::from(SoundTarget::Stop), false);
    /// ```
    fn from(val: SoundTarget) -> bool {
        match val {
            SoundTarget::Start => true,
            SoundTarget::Stop => false,
        }
    }
}

//=========================================================================

/// A basic sound controller that manages sound playback, volume, and pitch.
///
/// This struct provides control over a single sound's playback state, volume, and pitch
/// by interfacing with lotus_script variables. Each aspect (sound, volume, pitch) can
/// be controlled independently through separate variable names.
///
/// # Examples
///
/// ```rust
/// use sound::{Sound, SoundTarget};
///
/// // Create a sound with all controls
/// let mut full_sound = Sound::new(
///     Some("sound_trigger"),
///     Some("sound_volume"),
///     Some("sound_pitch")
/// );
///
/// // Create a simple sound with just playback control
/// let mut simple_sound = Sound::new_simple(Some("simple_sound"));
///
/// // Control playback
/// full_sound.start();
/// full_sound.update_volume(0.8);
/// full_sound.update_pitch(1.2);
/// full_sound.stop();
/// ```
#[derive(Default, Debug, Clone)]
pub struct Sound {
    /// The name of the variable controlling sound playback
    name: Option<String>,
    /// The name of the variable controlling sound volume
    name_vol: Option<String>,
    /// The name of the variable controlling sound pitch
    name_pitch: Option<String>,
}

impl Sound {
    /// Creates a new Sound with full control options.
    ///
    /// # Arguments
    ///
    /// * `name_sound` - Optional variable name for sound playback control
    /// * `name_volume` - Optional variable name for volume control
    /// * `name_pitch` - Optional variable name for pitch control
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sound::Sound;
    /// let sound = Sound::new(
    ///     Some("my_sound"),
    ///     Some("my_volume"),
    ///     Some("my_pitch")
    /// );
    /// ```
    pub fn new(
        name_sound: Option<&str>,
        name_volume: Option<&str>,
        name_pitch: Option<&str>,
    ) -> Self {
        Self {
            name: name_sound.map(|s| s.into()),
            name_vol: name_volume.map(|s| s.into()),
            name_pitch: name_pitch.map(|s| s.into()),
        }
    }

    /// Creates a new Sound with only playback control.
    ///
    /// This is a convenience method for creating sounds that only need
    /// start/stop functionality without volume or pitch control.
    ///
    /// # Arguments
    ///
    /// * `name_sound` - Optional variable name for sound playback control
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sound::Sound;
    /// let sound = Sound::new_simple(Some("simple_sound"));
    /// ```
    pub fn new_simple(name_sound: Option<&str>) -> Self {
        Self {
            name: name_sound.map(|s| s.into()),
            name_vol: None,
            name_pitch: None,
        }
    }

    /// Updates the sound's target state.
    ///
    /// # Arguments
    ///
    /// * `value` - The target state (Start or Stop)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sound::{Sound, SoundTarget};
    /// let mut sound = Sound::new_simple(Some("my_sound"));
    /// sound.update_target(SoundTarget::Start);
    /// sound.update_target(SoundTarget::Stop);
    /// ```
    pub fn update_target(&mut self, value: SoundTarget) {
        if let Some(snd) = &self.name {
            set_var(snd, bool::from(value));
        }
    }

    /// Starts or stops the sound based on a boolean value.
    ///
    /// # Arguments
    ///
    /// * `value` - `true` to start the sound, `false` to stop it
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sound::Sound;
    /// let mut sound = Sound::new_simple(Some("my_sound"));
    /// sound.start_stop(true);  // Start
    /// sound.start_stop(false); // Stop
    /// ```
    pub fn start_stop(&mut self, value: bool) {
        if let Some(snd) = &self.name {
            if value {
                set_var(snd, bool::from(SoundTarget::Start));
            } else {
                set_var(snd, bool::from(SoundTarget::Stop));
            }
        }
    }

    /// Starts the sound playback.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sound::Sound;
    /// let mut sound = Sound::new_simple(Some("my_sound"));
    /// sound.start();
    /// ```
    pub fn start(&mut self) {
        if let Some(snd) = &self.name {
            set_var(snd, bool::from(SoundTarget::Start));
        }
    }

    /// Stops the sound playback.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sound::Sound;
    /// let mut sound = Sound::new_simple(Some("my_sound"));
    /// sound.stop();
    /// ```
    pub fn stop(&mut self) {
        if let Some(snd) = &self.name {
            set_var(snd, bool::from(SoundTarget::Stop));
        }
    }

    /// Updates the sound's volume. Works only on Sounds with volume
    /// control variable.
    ///
    /// # Arguments
    ///
    /// * `value` - The new volume level (typically 0.0 to 1.0)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sound::Sound;
    /// let mut sound = Sound::new(Some("sound"), Some("volume"), None);
    /// sound.update_volume(0.5); // Set to 50% volume
    /// ```
    pub fn update_volume(&mut self, value: f32) {
        if let Some(snd) = &self.name_vol {
            set_var(snd, value);
        }
    }

    pub fn get_volume(&mut self) -> f32 {
        if let Some(snd) = &self.name_vol {
            get_var::<f32>(snd)
        } else {
            0.0
        }
    }

    /// Updates the sound's pitch. Works only on Sounds with pitch
    /// control variable.
    ///
    /// # Arguments
    ///
    /// * `value` - The new pitch multiplier (1.0 = normal pitch)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sound::Sound;
    /// let mut sound = Sound::new(Some("sound"), None, Some("pitch"));
    /// sound.update_pitch(1.5); // 1.5x pitch (higher)
    /// sound.update_pitch(0.8); // 0.8x pitch (lower)
    /// ```
    pub fn update_pitch(&mut self, value: f32) {
        if let Some(snd) = &self.name_pitch {
            set_var(snd, value);
        }
    }
}

//=========================================================================

/// A sound controller with automatic volume transitions.
///
/// This struct manages a sound that gradually increases or decreases its volume
/// based on a trigger state. The volume changes smoothly over time using
/// configurable increase and decrease rates.
///
/// # Examples
///
/// ```rust
/// use sound::SoundWithVol;
///
/// // Create a sound that fades in/out at different rates
/// let mut fade_sound = SoundWithVol::new(
///     "background_music",    // Sound variable name
///     "bg_volume",          // Volume variable name
///     2.0,                  // Increase rate (per second)
///     0.5                   // Decrease rate (per second)
/// );
///
/// // In your game loop:
/// fade_sound.tick(true);  // Gradually increase volume
/// fade_sound.tick(false); // Gradually decrease volume
/// ```
pub struct SoundWithVol {
    /// Rate at which volume increases per second
    increase: f32,
    /// Rate at which volume decreases per second
    decrease: f32,
    /// The underlying sound controller
    snd: Sound,
    /// Current volume level (0.0 to 1.0)
    vol: f32,
}

impl SoundWithVol {
    /// Creates a new SoundWithVol with specified fade rates.
    ///
    /// # Arguments
    ///
    /// * `snd_vol_name` - Variable name for volume control
    /// * `inc` - Volume increase rate per second
    /// * `dec` - Volume decrease rate per second
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sound::SoundWithVol;
    /// let sound = SoundWithVol::new("music_vol", 1.0, 0.5);
    /// ```
    pub fn new(snd_vol_name: &str, inc: f32, dec: f32) -> Self {
        Self {
            snd: Sound::new(None, Some(snd_vol_name), None),
            increase: inc,
            decrease: dec,
            vol: 0.0,
        }
    }

    /// Updates the volume based on the trigger state.
    ///
    /// This method should be called every frame/tick. It will gradually
    /// adjust the volume towards the target based on the trigger state.
    ///
    /// # Arguments
    ///
    /// * `trigger` - `true` to fade in, `false` to fade out
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sound::SoundWithVol;
    /// let mut sound = SoundWithVol::new("music", "music_vol", 1.0, 0.5);
    ///
    /// // In your game loop:
    /// sound.tick(player_in_area); // Fade in when player enters area
    /// ```
    pub fn tick(&mut self, trigger: bool) {
        if trigger {
            self.vol = (self.vol + self.increase * delta()).min(1.0);
        } else {
            self.vol = (self.vol - self.decrease * delta()).max(0.0);
        }
        self.snd.update_volume(self.vol);
    }
}

//=========================================================================

/// A sound controller that plays an end sound when the main sound stops.
///
/// This struct manages two sounds: a main sound and an end sound. When the
/// trigger changes from active to inactive, it stops the main sound and
/// starts the end sound, creating seamless audio transitions.
///
/// # Examples
///
/// ```rust
/// use sound::SoundWithEnd;
///
/// let mut engine_sound = SoundWithEnd::new(
///     "engine_running",     // Main sound
///     "engine_shutdown"     // End/shutdown sound
/// );
///
/// // In your game loop:
/// engine_sound.tick(engine_active); // Manages both sounds automatically
/// ```
pub struct SoundWithEnd {
    /// Previous trigger state for detecting changes
    trigger_last: bool,
    /// Main sound controller
    snd: Sound,
    /// End sound controller
    snd_end: Sound,
}

impl SoundWithEnd {
    /// Creates a new SoundWithEnd controller.
    ///
    /// # Arguments
    ///
    /// * `snd_name` - Variable name for the main sound
    /// * `snd_end_name` - Variable name for the end sound
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sound::SoundWithEnd;
    /// let sound = SoundWithEnd::new("main_sound", "end_sound");
    /// ```
    pub fn new(snd_name: &str, snd_end_name: &str) -> Self {
        Self {
            trigger_last: false,
            snd: Sound::new_simple(Some(snd_name)),
            snd_end: Sound::new_simple(Some(snd_end_name)),
        }
    }

    /// Updates the sound states based on the trigger.
    ///
    /// It detects changes in the trigger state and manages the
    /// sound transitions accordingly.
    ///
    /// # Arguments
    ///
    /// * `trigger` - Current trigger state
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sound::SoundWithEnd;
    /// let mut sound = SoundWithEnd::new("bell", "bell_end");
    ///
    /// // In your game loop:
    /// sound.tick(door_is_opening); // Handles sound transitions
    /// ```
    pub fn tick(&mut self, trigger: bool) {
        if self.trigger_last != trigger {
            if trigger {
                self.snd.start();
                self.snd_end.stop();
            } else {
                self.snd.stop();
                self.snd_end.start();
            }

            self.trigger_last = trigger;
        }
    }
}

//=========================================================================

/// A complex sound controller with start, loop, and end sounds.
///
/// This struct manages three separate sounds: a start sound (played once when
/// triggered), a loop sound (played continuously while active), and an end
/// sound (played once when deactivated). This creates rich, multi-layered
/// audio experiences.
///
/// # Examples
///
/// ```rust
/// use sound::SoundWithStartAndEnd;
///
/// let mut complex_sound = SoundWithStartAndEnd::new(
///     "machine_startup",    // Start sound
///     "machine_running",    // Loop sound
///     "machine_shutdown"    // End sound
/// );
///
/// // In your game loop:
/// complex_sound.tick(machine_active); // Manages all three sounds
/// ```
pub struct SoundWithStartAndEnd {
    /// Previous trigger state for detecting changes
    trigger_last: bool,
    /// Start sound controller
    snd_start: Sound,
    /// Loop sound controller
    snd: Sound,
    /// End sound controller
    snd_end: Sound,
}

impl SoundWithStartAndEnd {
    /// Creates a new SoundWithStartAndEnd controller.
    ///
    /// # Arguments
    ///
    /// * `snd_start_name` - Variable name for the start sound
    /// * `snd_name` - Variable name for the loop sound
    /// * `snd_end_name` - Variable name for the end sound
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sound::SoundWithStartAndEnd;
    /// let sound = SoundWithStartAndEnd::new(
    ///     "start_sound",
    ///     "loop_sound",
    ///     "end_sound"
    /// );
    /// ```
    pub fn new(snd_start_name: &str, snd_name: &str, snd_end_name: &str) -> Self {
        Self {
            trigger_last: false,
            snd_start: Sound::new_simple(Some(snd_start_name)),
            snd: Sound::new_simple(Some(snd_name)),
            snd_end: Sound::new_simple(Some(snd_end_name)),
        }
    }

    /// Updates all sound states based on the trigger.
    ///
    /// It manages the complex interactions between the start, loop, and end
    /// sounds based on trigger state changes.
    ///
    /// # Arguments
    ///
    /// * `trigger` - Current trigger state
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sound::SoundWithStartAndEnd;
    /// let mut sound = SoundWithStartAndEnd::new(
    ///     "start_sound",
    ///     "loop_sound",
    ///     "end_sound"
    /// );
    ///
    /// // In your game loop:
    /// sound.tick(player_casting_spell); // Manages all sound phases
    /// ```
    pub fn tick(&mut self, trigger: bool) {
        if self.trigger_last != trigger {
            if trigger {
                self.snd_start.start();
            } else {
                self.snd_end.start();
            }

            self.trigger_last = trigger;
        }
    }
}
