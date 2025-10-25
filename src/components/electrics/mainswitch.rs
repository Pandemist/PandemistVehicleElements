//! # Main Switch Module
//!
//! This module provides electrical switch components for simulation purposes, including
//! main switches with automatic and manual control capabilities, and circuit breakers.
//!
//! ## Features
//!
//! - **MainSwitch**: A configurable electrical switch with both manual and automatic operation
//! - **CircuitBreaker**: A simple circuit breaker that trips based on voltage thresholds
//! - **Builder Pattern**: Easy configuration of main switches with fluent API
//! - **Animation Support**: Integration with animation system for visual feedback
//! - **Sound Effects**: Configurable audio feedback for switch operations
//! - **Mouse Control**: Interactive mouse-based switch control
//!
//! ## Example
//!
//! ```rust
//! use crate::mainswitch::MainSwitch;
//! use lotus_extra::vehicle::CockpitSide;
//!
//! let mut switch = MainSwitch::builder(Some(CockpitSide::A))
//!     .init(false)
//!     .mouse_factor(0.5)
//!     .handle_switch("slider_animation", "switch_key")
//!     .state_indicator("state_led")
//!     .snd_turn_on("switch_on.wav")
//!     .snd_turn_off("switch_off.wav")
//!     .build();
//!
//! // Update the switch state
//! switch.tick(1.0); // 1.0V input
//! println!("Switch output: {}V", switch.output);
//! ```

use lotus_extra::vehicle::CockpitSide;
use lotus_script::time::delta;

use crate::{
    api::{animation::Animation, general::mouse_move, key_event::KeyEvent, sound::Sound},
    management::enums::target_enums::SwitchingTarget,
};

/// Builder for creating and configuring a `MainSwitch`.
///
/// The `MainSwitchBuilder` provides a fluent interface for setting up a main switch
/// with various configuration options including animations, sounds, and control parameters.
///
/// # Example
///
/// ```rust
/// let switch = MainSwitch::builder(Some(CockpitSide::A))
///     .init(true)  // Start in ON state
///     .mouse_factor(0.8)
///     .handle_switch("my_slider", "my_key")
///     .state_indicator("status_light")
///     .snd_turn_on("power_on.wav")
///     .build();
/// ```
pub struct MainSwitchBuilder {
    cab_side: Option<CockpitSide>,
    state: bool,
    switching_timer: f32,
    switching_allowed: bool,

    output: f32,

    slider: f32,
    mouse_factor: f32,

    key_grab: KeyEvent,

    slider_anim: Animation,
    state_anim: Animation,

    target: SwitchingTarget,
    target_last: SwitchingTarget,

    snd_turn_on_start: Sound,
    snd_turn_on: Sound,
    snd_turn_off: Sound,
    snd_trigger: Sound,
}

impl MainSwitchBuilder {
    /// Sets the state indicator animation for the switch.
    ///
    /// This animation is used to visually represent the current state of the switch.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the animation to use for the state indicator
    ///
    /// # Example
    ///
    /// ```rust
    /// let builder = MainSwitch::builder(None)
    ///     .state_indicator("led_animation");
    /// ```
    pub fn state_indicator(mut self, name: impl Into<String>) -> Self {
        self.state_anim = Animation::new(Some(&name.into()));
        self
    }

    /// Configures the switch handle animation and key binding.
    ///
    /// Sets up both the visual animation for the switch handle and the key event
    /// that will be used to control the switch manually.
    ///
    /// # Arguments
    ///
    /// * `animation_name` - The name of the animation for the switch handle
    /// * `event_name` - The name of the key event for manual control
    ///
    /// # Example
    ///
    /// ```rust
    /// let builder = MainSwitch::builder(Some(CockpitSide::Right))
    ///     .handle_switch("handle_anim", "main_switch_key");
    /// ```
    pub fn handle_switch(mut self, animation_name: impl Into<String>, event_name: &str) -> Self {
        self.slider_anim = Animation::new(Some(&animation_name.into()));
        self.key_grab = KeyEvent::new(Some(event_name), self.cab_side);
        self
    }

    /// Initializes the switch with a specific state.
    ///
    /// When set to `true`, the switch starts in the ON position with the slider
    /// at maximum position (1.0).
    ///
    /// # Arguments
    ///
    /// * `state` - Initial state of the switch (true = ON, false = OFF)
    ///
    /// # Example
    ///
    /// ```rust
    /// let builder = MainSwitch::builder(None)
    ///     .init(true);  // Start switched ON
    /// ```
    pub fn init(mut self, state: bool) -> Self {
        if state {
            self.state = true;
            self.slider = 0.0;
            self.slider_anim.set(self.slider);
        }
        self
    }

    /// Sets the mouse sensitivity factor for manual control.
    ///
    /// This factor determines how responsive the switch is to mouse movement
    /// when manually operating the switch.
    ///
    /// # Arguments
    ///
    /// * `mouse_factor` - Sensitivity multiplier for mouse input (typically 0.0 to 1.0)
    ///
    /// # Example
    ///
    /// ```rust
    /// let builder = MainSwitch::builder(None)
    ///     .mouse_factor(0.5);  // Half sensitivity
    /// ```
    pub fn mouse_factor(mut self, mouse_factor: f32) -> Self {
        self.mouse_factor = mouse_factor;
        self
    }

    /// Sets the sound effect for when automatic switching begins.
    ///
    /// This sound plays when the switch receives a command to turn on automatically,
    /// before the actual switching occurs (useful for indicating preparation phase).
    ///
    /// # Arguments
    ///
    /// * `name` - Name or path of the sound file
    ///
    /// # Example
    ///
    /// ```rust
    /// let builder = MainSwitch::builder(None)
    ///     .snd_turn_on_start("switch_prepare.wav");
    /// ```
    pub fn snd_turn_on_start(mut self, name: impl Into<String>) -> Self {
        self.snd_turn_on_start = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Sets the sound effect for when the switch turns on.
    ///
    /// This sound plays when the switch actually engages and turns on.
    ///
    /// # Arguments
    ///
    /// * `name` - Name or path of the sound file
    ///
    /// # Example
    ///
    /// ```rust
    /// let builder = MainSwitch::builder(None)
    ///     .snd_turn_on("switch_on.wav");
    /// ```
    pub fn snd_turn_on(mut self, name: impl Into<String>) -> Self {
        self.snd_turn_on = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Sets the sound effect for when the switch turns off.
    ///
    /// This sound plays when the switch disengages and turns off.
    ///
    /// # Arguments
    ///
    /// * `name` - Name or path of the sound file
    ///
    /// # Example
    ///
    /// ```rust
    /// let builder = MainSwitch::builder(None)
    ///     .snd_turn_off("switch_off.wav");
    /// ```
    pub fn snd_turn_off(mut self, name: impl Into<String>) -> Self {
        self.snd_turn_off = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Sets the sound effect for when the switch is triggered.
    ///
    /// This sound plays when the switch is triggered.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the sound file
    ///
    /// # Example
    ///
    /// ```rust
    /// let builder = MainSwitch::builder(None)
    ///     .snd_trigger("snd_trigger");
    /// ```
    pub fn snd_trigger(mut self, name: impl Into<String>) -> Self {
        self.snd_trigger = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Builds and returns the configured `MainSwitch`.
    ///
    /// Consumes the builder and creates a `MainSwitch` instance with all
    /// the configured parameters.
    ///
    /// # Returns
    ///
    /// A fully configured `MainSwitch` ready for use.
    ///
    /// # Example
    ///
    /// ```rust
    /// let switch = MainSwitch::builder(None)
    ///     .init(false)
    ///     .mouse_factor(1.0)
    ///     .build();
    /// ```
    pub fn build(self) -> MainSwitch {
        MainSwitch {
            cab_side: self.cab_side,

            state: self.state,
            switching_timer: self.switching_timer,
            switching_allowed: self.switching_allowed,

            output: self.output,

            slider: self.slider,
            mouse_factor: self.mouse_factor,

            key_grab: self.key_grab,

            slider_anim: self.slider_anim,
            state_anim: self.state_anim,

            target: self.target,
            target_last: self.target,

            snd_turn_on_start: self.snd_turn_on_start,
            snd_turn_on: self.snd_turn_on,
            snd_turn_off: self.snd_turn_off,
            snd_trigger: self.snd_trigger,
        }
    }
}

/// A main electrical switch with both manual and automatic operation capabilities.
///
/// The `MainSwitch` represents a configurable electrical switch that can be operated
/// both manually (via mouse/keyboard input) and automatically (via programmatic commands).
/// It supports animations, sound effects, and various switching modes.
///
/// ## Features
///
/// - **Manual Operation**: Control via mouse movement and keyboard input
/// - **Automatic Operation**: Programmatic switching with configurable delays
/// - **Audio Feedback**: Customizable sound effects for different switch states
/// - **Visual Feedback**: Integration with animation system for visual representation
/// - **Voltage Pass-through**: Multiplies input voltage by switch state (0 or 1)
/// - **Safety Features**: Configurable switching restrictions and minimum positions
///
/// ## States
///
/// - `state`: Current electrical state (true = ON, false = OFF)
/// - `switching_allowed`: Whether automatic switching is permitted
/// - `output`: Current output voltage (input_voltage * state)
/// - `target`: Desired switching target (Neutral, TurnOn, TurnOff)
///
/// ## Example
///
/// ```rust
/// use crate::mainswitch::MainSwitch;
/// use crate::management::enums::target_enums::SwitchingTarget;
///
/// let mut switch = MainSwitch::builder(None)
///     .init(false)
///     .build();
///
/// // Manual operation via tick updates
/// switch.tick(12.0); // 12V input
/// assert_eq!(switch.output, 0.0); // OFF state, no output
///
/// // Automatic switching
/// switch.target = SwitchingTarget::TurnOn(2.0); // Turn on after 2 seconds
///
/// // Update loop (called every frame)
/// loop {
///     switch.tick(12.0);
///     if switch.state {
///         println!("Switch is ON, output: {}V", switch.output);
///         break;
///     }
/// }
/// ```
pub struct MainSwitch {
    cab_side: Option<CockpitSide>,

    /// Current state of the switch (true = ON, false = OFF)
    pub state: bool,
    /// Whether automatic switching operations are allowed
    pub switching_allowed: bool,
    switching_timer: f32,

    /// Current output voltage (input_voltage * state)
    pub output: f32,

    slider: f32,
    mouse_factor: f32,

    key_grab: KeyEvent,

    slider_anim: Animation,
    state_anim: Animation,

    /// Current switching target (what the switch should do)
    pub target: SwitchingTarget,
    target_last: SwitchingTarget,

    snd_turn_on_start: Sound,
    snd_turn_on: Sound,
    snd_turn_off: Sound,
    snd_trigger: Sound,
}

impl MainSwitch {
    /// Creates a new builder for configuring a `MainSwitch`.
    ///
    /// The builder pattern allows for easy configuration of all switch parameters
    /// before creating the final instance.
    ///
    /// # Arguments
    ///
    /// * `cab_side` - Optional cab side for key event handling
    ///
    /// # Returns
    ///
    /// A `MainSwitchBuilder` with default values
    ///
    /// # Example
    ///
    /// ```rust
    /// use crate::api::key_event::CockpitSide;
    ///
    /// let builder = MainSwitch::builder(Some(CockpitSide::Left));
    /// ```
    pub fn builder(cab_side: Option<CockpitSide>) -> MainSwitchBuilder {
        MainSwitchBuilder {
            cab_side,
            state: false,
            switching_allowed: true,
            snd_turn_on_start: Sound::new_simple(None),
            snd_turn_on: Sound::new_simple(None),
            snd_turn_off: Sound::new_simple(None),
            snd_trigger: Sound::new_simple(None),
            switching_timer: 0.0,
            output: 0.0,
            target: SwitchingTarget::Neutral,
            target_last: SwitchingTarget::Neutral,
            slider: 1.0,
            mouse_factor: 0.0,
            key_grab: KeyEvent::new(None, None),
            slider_anim: Animation::new(None),
            state_anim: Animation::new(None),
        }
    }

    /// Updates the switch state and processes all input/output operations.
    ///
    /// This method should be called every frame/tick to update the switch state,
    /// handle user input, process automatic switching commands, and calculate
    /// the output voltage.
    ///
    /// ## Operations performed:
    /// 1. Process manual input (mouse/keyboard)
    /// 2. Handle automatic switching based on target
    /// 3. Update animations
    /// 4. Calculate output voltage
    /// 5. Play appropriate sound effects
    ///
    /// # Arguments
    ///
    /// * `input_voltage` - The input voltage to the switch
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut switch = MainSwitch::builder(None).build();
    ///
    /// // In your main loop:
    /// switch.tick(24.0); // 24V input
    /// println!("Output: {}V", switch.output);
    /// ```
    pub fn tick(&mut self, input_voltage: f32) {
        let slider_last = self.slider;
        let slider_min = if self.switching_allowed { 0.0 } else { 0.15 };

        // Manual switching on and off
        if self.key_grab.is_pressed() {
            self.slider = (self.slider + mouse_move().x * self.mouse_factor).clamp(slider_min, 1.0);
        }
        self.slider_anim.set(self.slider);

        // Manual switch engagement
        if (self.slider <= 0.1 && slider_last > 0.1) && !self.state {
            self.snd_turn_on.start();
            self.state = true;
        }

        // Manual switch disengagement
        if self.slider > 0.1 && slider_last <= 0.1 {
            self.snd_turn_off.start();
            self.state = false;
        }

        // Automatic switching
        match (self.target, self.state) {
            (SwitchingTarget::TurnOn(delay), false) => {
                if self.target_last != self.target {
                    self.snd_turn_on_start.start();
                }
                self.switching_timer += delta();
                if self.switching_timer > delay && self.switching_allowed {
                    self.snd_turn_on.start();
                    self.state = true;
                }
            }
            (SwitchingTarget::TurnOff(delay), true) => {
                self.switching_timer += delta();
                if self.switching_timer > delay && self.switching_allowed {
                    self.snd_turn_off.start();
                    self.state = false;
                }
            }
            (_, _) => {
                self.switching_timer = 0.0;
            }
        }

        // Output voltage calculation
        self.output = input_voltage * self.state as u8 as f32;

        self.target_last = self.target;

        // State indicator animation
        let state_anim_target = 1.0 * (!self.state) as u8 as f32;
        self.state_anim.set(state_anim_target);
    }

    /// Immediately turns off the switch.
    ///
    /// This method provides a way to forcefully turn off the switch, regardless
    /// of its current target or state. It will play the turn-off sound effect
    /// and reset the switching targets to neutral.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut switch = MainSwitch::builder(None)
    ///     .init(true)  // Start ON
    ///     .build();
    ///
    /// assert!(switch.state);
    /// switch.turn_off();
    /// assert!(!switch.state);
    /// ```
    pub fn turn_off(&mut self) {
        if self.state {
            self.snd_trigger.start();
            self.state = false;
            self.target = SwitchingTarget::Neutral;
            self.target_last = SwitchingTarget::Neutral;
        }
    }
}

//=================================================================

/// A simple circuit breaker that trips based on voltage thresholds.
///
/// The `CircuitBreaker` monitors input voltage and automatically opens (trips)
/// when the voltage is outside the acceptable range. It's designed to protect
/// circuits from over-voltage and under-voltage conditions.
///
/// ## Operation
///
/// - **Normal Operation**: Input voltage between 0.8V and 1.2V passes through
/// - **Trip Condition**: Input voltage outside the 0.8V-1.2V range causes the breaker to open
/// - **Output**: Either full input voltage (closed) or 0V (open/tripped)
///
/// ## Example
///
/// ```rust
/// let mut breaker = CircuitBreaker::new();
///
/// // Normal operation
/// breaker.tick(1.0); // 1.0V input
/// assert!(breaker.state);
/// assert_eq!(breaker.output, 1.0);
///
/// // Over-voltage trip
/// breaker.tick(2.0); // 2.0V input (too high)
/// assert!(!breaker.state);
/// assert_eq!(breaker.output, 0.0);
///
/// // Under-voltage trip
/// breaker.tick(0.5); // 0.5V input (too low)
/// assert!(!breaker.state);
/// assert_eq!(breaker.output, 0.0);
/// ```
#[derive(Debug)]
pub struct CircuitBreaker {
    /// Current state of the circuit breaker (true = closed/conducting, false = open/tripped)
    pub state: bool,
    /// Current output voltage (input voltage when closed, 0.0 when open)
    pub output: f32,
}

impl CircuitBreaker {
    /// Creates a new circuit breaker in the open (tripped) state.
    ///
    /// The circuit breaker starts in a safe state with no output until
    /// proper voltage is applied.
    ///
    /// # Returns
    ///
    /// A new `CircuitBreaker` instance
    ///
    /// # Example
    ///
    /// ```rust
    /// let breaker = CircuitBreaker::new();
    /// assert!(!breaker.state); // Starts open
    /// assert_eq!(breaker.output, 0.0);
    /// ```
    pub fn new() -> Self {
        Self {
            state: false,
            output: 0.0,
        }
    }

    /// Updates the circuit breaker state based on input voltage.
    ///
    /// Checks if the input voltage is within the acceptable range (0.8V to 1.2V).
    /// If within range, the breaker closes and passes the voltage through.
    /// If outside range, the breaker opens and blocks all current.
    ///
    /// # Arguments
    ///
    /// * `input_voltage` - The input voltage to monitor
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut breaker = CircuitBreaker::new();
    ///
    /// // Test with normal voltage
    /// breaker.tick(1.0);
    /// assert!(breaker.state);
    /// assert_eq!(breaker.output, 1.0);
    ///
    /// // Test with abnormal voltage
    /// breaker.tick(5.0);
    /// assert!(!breaker.state);
    /// assert_eq!(breaker.output, 0.0);
    /// ```
    pub fn tick(&mut self, input_voltage: f32) {
        self.state = input_voltage > 0.8 && input_voltage < 1.2;
        self.output = input_voltage * self.state as u8 as f32;
    }
}

impl Default for CircuitBreaker {
    /// Creates a circuit breaker with default settings.
    ///
    /// Equivalent to calling `CircuitBreaker::new()`.
    fn default() -> Self {
        Self::new()
    }
}
