//! # Wiper Animation System
//!
//! This module provides a flexible windshield wiper animation system with configurable speed levels,
//! delays, and sound effects. The system supports multiple wiper levels with different characteristics
//! and can be used to simulate realistic wiper behavior in games or simulations.
//!
//! ## Features
//!
//! - Multiple configurable wiper speed levels
//! - Customizable animation mappings for main and secondary animations
//! - Sound effects for different wiper states (back, forth, full run)
//! - Configurable delays between wiper runs
//! - Post-run cycles for continuous operation
//! - Voltage-based operation control
//!
//! ## Example
//!
//! ```rust
//! use std::rc::Rc;
//! use std::collections::HashMap;
//!
//! // Define wiper speed levels
//! #[derive(Debug, Clone, PartialEq, Eq, Hash)]
//! enum WiperSpeed {
//!     Off,
//!     Slow,
//!     Fast,
//!     Intermittent,
//! }
//!
//! // Create a wiper system
//! let mut wiper = Wiper::builder("main_wiper_anim")
//!     .add_wiper_level(WiperSpeed::Slow, 0.5, 1.0, 0)
//!     .add_wiper_level(WiperSpeed::Fast, 1.0, 0.2, 0)
//!     .add_wiper_level(WiperSpeed::Intermittent, 0.3, 3.0, 1)
//!     .snd_back("wiper_back")
//!     .snd_forth("wiper_forth")
//!     .snd_full_run("wiper_run")
//!     .build();
//!
//! // Update the wiper system
//! wiper.tick(WiperSpeed::Slow, 12.0); // 12V power
//! ```

use std::{collections::HashMap, rc::Rc};

use lotus_script::time::delta;
use std::hash::Hash;

use crate::api::{animation::Animation, sound::Sound};

/// Configuration for a single wiper speed level.
///
/// This struct defines the behavior characteristics of a wiper at a specific speed setting,
/// including motor speed, timing delays, and post-run behavior.
#[derive(Debug)]
pub struct WiperLevel {
    /// Speed multiplier for the wiper motor (0.0 to 1.0+)
    motor_speed: f32,
    /// Delay in seconds between wiper runs
    run_delay: f32,
    /// Number of additional runs to perform after the target changes
    post_runs: u32,
}

/// Builder pattern implementation for creating a `Wiper` instance.
///
/// The `WiperBuilder` provides a fluent interface for configuring all aspects of the wiper system,
/// including animations, sound effects, speed levels, and behavior settings.
///
/// # Type Parameters
///
/// * `T` - The type used to identify different wiper levels (e.g., enum variants)
///
/// # Example
///
/// ```rust
/// use std::rc::Rc;
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// enum WiperMode {
///     Off,
///     Intermittent,
///     Low,
///     High,
/// }
///
/// let wiper = Wiper::builder("wiper_animation")
///     .add_wiper_level(WiperMode::Low, 0.5, 1.0, 0)
///     .add_wiper_level(WiperMode::High, 1.0, 0.5, 0)
///     .main_anim_mapping(Rc::new(|pos| pos * 180.0)) // Convert to degrees
///     .snd_back("wiper_sound_back")
///     .build();
/// ```
pub struct WiperBuilder<T> {
    main_anim: Animation,
    main_anim_mapping: Rc<dyn Fn(f32) -> f32>,
    secondary_anim: Animation,
    secondary_anim_mapping: Rc<dyn Fn(f32) -> f32>,

    levels: HashMap<T, WiperLevel>,

    current_target: Option<T>,

    delay_timer: f32,
    motor_pos: f32,
    runs: u32,

    change_target_instant: bool,
    motor_at_start_pos: bool,

    snd_back: Sound,
    snd_forth: Sound,
    snd_full_run: Sound,
}

impl<T: Eq + Hash + Clone> WiperBuilder<T> {
    /// Sets a custom mapping function for the main animation.
    ///
    /// The mapping function transforms the motor position (0.0 to 1.0) into the desired
    /// animation value. This is useful for converting linear motor movement into
    /// angular positions or other coordinate systems.
    ///
    /// # Parameters
    ///
    /// * `mapping` - A function that takes a motor position (0.0-1.0) and returns the mapped value
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::rc::Rc;
    ///
    /// let builder = Wiper::builder("wiper")
    ///     .main_anim_mapping(Rc::new(|pos| {
    ///         // Convert linear position to sine wave motion
    ///         (pos * std::f32::consts::PI).sin() * 90.0
    ///     }));
    /// ```
    pub fn main_anim_mapping(mut self, mapping: Rc<dyn Fn(f32) -> f32>) -> Self {
        self.main_anim_mapping = mapping;
        self
    }

    /// Adds a secondary animation with its own mapping function.
    ///
    /// Secondary animations can be used for additional visual effects or to drive
    /// other components that should move in sync with the main wiper motion.
    ///
    /// # Parameters
    ///
    /// * `animation_name` - Name of the secondary animation
    /// * `mapping` - Function to map motor position to secondary animation value
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::rc::Rc;
    ///
    /// let builder = Wiper::builder("main_wiper")
    ///     .add_secondary_anim("wiper_shadow", Rc::new(|pos| pos * 0.8));
    /// ```
    pub fn add_secondary_anim(
        mut self,
        animation_name: impl Into<String>,
        mapping: Rc<dyn Fn(f32) -> f32>,
    ) -> Self {
        self.secondary_anim = Animation::new(Some(&animation_name.into()));
        self.secondary_anim_mapping = mapping;
        self
    }

    /// Adds a new wiper speed level configuration.
    ///
    /// Each level defines how the wiper behaves at that specific setting, including
    /// motor speed, delay between runs, and post-run behavior.
    ///
    /// # Parameters
    ///
    /// * `level` - Identifier for this wiper level
    /// * `speed` - Motor speed multiplier (typically 0.1 to 2.0)
    /// * `delay` - Delay in seconds between wiper cycles
    /// * `post_runs` - Number of additional runs after switching away from this level
    ///
    /// # Example
    ///
    /// ```rust
    /// let builder = Wiper::builder("wiper")
    ///     .add_wiper_level("slow", 0.3, 2.0, 1)     // Slow with 2s delay, 1 post-run
    ///     .add_wiper_level("fast", 1.0, 0.5, 0);   // Fast with 0.5s delay, no post-runs
    /// ```
    pub fn add_wiper_level(mut self, level: T, speed: f32, delay: f32, post_runs: u32) -> Self {
        self.levels.insert(
            level,
            WiperLevel {
                motor_speed: speed,
                run_delay: delay,
                post_runs,
            },
        );
        self
    }

    pub fn change_target_instant(mut self) -> Self {
        self.change_target_instant = true;
        self
    }

    /// Sets the sound effect for the backward wiper motion.
    ///
    /// This sound plays when the wiper moves from the end position back to the start.
    ///
    /// # Parameters
    ///
    /// * `name` - Name or path of the sound file
    ///
    /// # Example
    ///
    /// ```rust
    /// let builder = Wiper::builder("wiper")
    ///     .snd_back("sounds/wiper_back.wav");
    /// ```
    pub fn snd_back(mut self, name: impl Into<String>) -> Self {
        self.snd_back = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Sets the sound effect for the forward wiper motion.
    ///
    /// This sound plays when the wiper moves from the start position toward the end.
    ///
    /// # Parameters
    ///
    /// * `name` - Name or path of the sound file
    ///
    /// # Example
    ///
    /// ```rust
    /// let builder = Wiper::builder("wiper")
    ///     .snd_forth("sounds/wiper_forth.wav");
    /// ```
    pub fn snd_forth(mut self, name: impl Into<String>) -> Self {
        self.snd_forth = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Sets the sound effect for a complete wiper run.
    ///
    /// This sound plays during the entire wiper cycle and can be used for
    /// continuous motor noise or ambient effects.
    ///
    /// # Parameters
    ///
    /// * `name` - Name or path of the sound file
    ///
    /// # Example
    ///
    /// ```rust
    /// let builder = Wiper::builder("wiper")
    ///     .snd_full_run("sounds/wiper_motor.wav");
    /// ```
    pub fn snd_full_run(mut self, name: impl Into<String>) -> Self {
        self.snd_full_run = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Sets the initial motor position.
    ///
    /// This allows starting the wiper at a specific position rather than the default
    /// starting position (0.0). The position should be between 0.0 and 1.0.
    ///
    /// # Parameters
    ///
    /// * `value` - Initial motor position (0.0 to 1.0)
    ///
    /// # Example
    ///
    /// ```rust
    /// let builder = Wiper::builder("wiper")
    ///     .init_pos(0.5); // Start at middle position
    /// ```
    pub fn init_pos(mut self, value: f32) -> Self {
        self.motor_pos = value;
        self.motor_at_start_pos = self.motor_pos == 0.0;

        let pos_1 = (self.main_anim_mapping)(self.motor_pos);
        self.main_anim.set(pos_1);
        let pos_2 = (self.secondary_anim_mapping)(self.motor_pos);
        self.secondary_anim.set(pos_2);
        self
    }

    /// Consumes the builder and creates a `Wiper` instance.
    ///
    /// This method finalizes the configuration and returns a fully constructed
    /// `Wiper` ready for use.
    ///
    /// # Returns
    ///
    /// A configured `Wiper<T>` instance
    ///
    /// # Example
    ///
    /// ```rust
    /// let wiper = Wiper::builder("my_wiper")
    ///     .add_wiper_level("slow", 0.5, 1.0, 0)
    ///     .build();
    /// ```
    pub fn build(self) -> Wiper<T> {
        Wiper {
            main_anim: self.main_anim,
            main_anim_mapping: self.main_anim_mapping,
            secondary_anim: self.secondary_anim,
            secondary_anim_mapping: self.secondary_anim_mapping,
            levels: self.levels,

            current_target: self.current_target,

            delay_timer: self.delay_timer,

            motor_pos: self.motor_pos,
            runs: self.runs,

            change_target_instant: self.change_target_instant,
            motor_at_start_pos: self.motor_at_start_pos,

            snd_back: self.snd_back,
            snd_forth: self.snd_forth,
            snd_full_run: self.snd_full_run,
        }
    }
}

/// A windshield wiper animation system with configurable speed levels and sound effects.
///
/// The `Wiper` struct manages the state and behavior of a windshield wiper system,
/// including motor position, timing, sound effects, and animation updates. It supports
/// multiple speed levels with different characteristics and provides realistic wiper
/// behavior simulation.
///
/// # Type Parameters
///
/// * `T` - The type used to identify different wiper levels. Must implement `Eq`, `Hash`, and `Clone`.
///
/// # Fields
///
/// The struct contains both public and private fields:
/// - `levels`: Public access to the configured wiper levels
/// - `current_target`: Public access to the current target level
/// - `motor_pos`: Public access to the current motor position
///
/// # Example
///
/// ```rust
/// use std::collections::HashMap;
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// enum WiperSpeed {
///     Off,
///     Intermittent,
///     Low,
///     High,
/// }
///
/// let mut wiper = Wiper::builder("main_wiper")
///     .add_wiper_level(WiperSpeed::Low, 0.5, 1.0, 0)
///     .add_wiper_level(WiperSpeed::High, 1.0, 0.3, 0)
///     .build();
///
/// // Simulate wiper operation
/// loop {
///     wiper.tick(WiperSpeed::Low, 12.0); // 12V operation
///     // ... update game/simulation state
/// }
/// ```
pub struct Wiper<T> {
    main_anim: Animation,
    main_anim_mapping: Rc<dyn Fn(f32) -> f32>,
    secondary_anim: Animation,
    secondary_anim_mapping: Rc<dyn Fn(f32) -> f32>,

    /// Map of configured wiper levels and their settings
    pub levels: HashMap<T, WiperLevel>,

    /// Currently active wiper level target
    pub current_target: Option<T>,

    delay_timer: f32,
    /// Current motor position (0.0 to 1.0)
    pub motor_pos: f32,
    runs: u32,

    change_target_instant: bool,
    motor_at_start_pos: bool,

    snd_back: Sound,
    snd_forth: Sound,
    snd_full_run: Sound,
}

impl<T: Eq + Hash + Clone> Wiper<T> {
    /// Creates a new `WiperBuilder` for configuring a wiper system.
    ///
    /// This is the primary entry point for creating a new wiper. The builder pattern
    /// allows for flexible configuration of all wiper parameters.
    ///
    /// # Parameters
    ///
    /// * `animation_name` - Name of the main animation to drive
    ///
    /// # Returns
    ///
    /// A `WiperBuilder<T>` instance for further configuration
    ///
    /// # Example
    ///
    /// ```rust
    /// let builder = Wiper::builder("windshield_wiper");
    /// ```
    pub fn builder(animation_name: impl Into<String>) -> WiperBuilder<T> {
        WiperBuilder {
            main_anim: Animation::new(Some(&animation_name.into())),
            main_anim_mapping: Rc::new(|x| x),
            secondary_anim: Animation::new(None),
            secondary_anim_mapping: Rc::new(|x| x),

            levels: HashMap::new(),

            current_target: None,

            delay_timer: 0.0,
            motor_pos: 0.0,
            runs: 0,

            change_target_instant: false,
            motor_at_start_pos: true,

            snd_back: Sound::new_simple(None),
            snd_forth: Sound::new_simple(None),
            snd_full_run: Sound::new_simple(None),
        }
    }

    /// Internal method to update animation values based on current motor position.
    ///
    /// This method applies the configured mapping functions to convert the motor
    /// position into appropriate animation values for both main and secondary animations.
    fn update(&mut self) {
        let pos_1 = (self.main_anim_mapping)(self.motor_pos);
        self.main_anim.set(pos_1);
        let pos_2 = (self.secondary_anim_mapping)(self.motor_pos);
        self.secondary_anim.set(pos_2);
    }

    /// Updates the wiper system for one frame/tick.
    ///
    /// This method should be called every frame to update the wiper state, handle
    /// target changes, process motor movement, manage timing, and trigger sound effects.
    /// The wiper will only operate if the voltage is sufficient (>= 0.5V).
    ///
    /// # Parameters
    ///
    /// * `target` - The desired wiper level for this frame
    /// * `voltage` - Current voltage level (must be >= 0.5 for operation)
    ///
    /// # Behavior
    ///
    /// - Voltage below 0.5V stops all wiper operation
    /// - Target changes are handled based on `change_target_instant` setting
    /// - Motor movement is calculated based on the current level's speed
    /// - Sound effects are triggered at appropriate motion points
    /// - Delay timers and post-run cycles are managed automatically
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::collections::HashMap;
    /// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    /// # enum WiperSpeed { Off, Low, High }
    /// # let mut wiper = Wiper::builder("test").build();
    ///
    /// // In your game loop:
    /// loop {
    ///     let current_speed = WiperSpeed::Low;
    ///     let voltage = 12.0;
    ///     
    ///     wiper.tick(current_speed, voltage);
    ///     
    ///     // ... render frame, update other systems
    /// }
    /// ```
    pub fn tick(&mut self, target: T, voltage: f32) {
        // Voltage check - wiper won't operate below 0.5V
        if voltage < 0.5 {
            return;
        }

        // Target change handling
        if self.levels.contains_key(&target) || self.change_target_instant {
            if self.current_target != Some(target.clone()) {
                self.delay_timer = 0.0;
                self.runs = 0;
            }
            self.current_target = Some(target.clone());
        }

        // Main wiper operation loop
        if let Some(real_target) = self.current_target.clone() {
            let level = match self.levels.get(&real_target) {
                Some(level) => level,
                None => return,
            };

            // Handle run delay
            if self.delay_timer > 0.0 {
                self.delay_timer -= delta();
                if self.delay_timer > 0.0 {
                    return;
                }
            }

            // Motor movement calculation
            let motor_pos_last = self.motor_pos;
            self.motor_pos += level.motor_speed * delta();

            // Sound effect triggers
            if self.motor_at_start_pos {
                self.motor_at_start_pos = false;
                self.snd_forth.start();
                self.snd_full_run.start();
            }

            // Mid-point sound trigger (forward motion)
            if self.motor_pos >= 0.5 && motor_pos_last < 0.5 {
                self.snd_back.start();
            }

            // Set minimum runs required
            self.runs = self.runs.max(level.post_runs);

            // Cycle completion handling
            if self.motor_pos >= 1.0 {
                self.motor_pos -= 1.0;

                if self.runs > 0 {
                    self.runs -= 1;
                }

                self.delay_timer = level.run_delay;
                self.motor_at_start_pos = true;

                self.snd_back.start();

                // Check if we should continue running
                if self.runs == 0 {
                    self.current_target = Some(target.clone());
                }

                // Stop operation if no target
                if self.current_target.is_none() {
                    self.delay_timer = 0.0;
                    self.runs = 0;
                    self.snd_back.stop();
                    self.snd_forth.stop();
                    self.snd_full_run.stop();
                }
            }
            self.update();
        } else {
            // No active target - stop all operation
            self.delay_timer = 0.0;
            self.runs = 0;
            self.snd_back.stop();
            self.snd_forth.stop();
            self.snd_full_run.stop();
        }
    }
}
