//! Push button implementation for interactive applications
//!
//! This module provides a flexible push button system that supports various interaction modes
//! including regular buttons, toggle buttons, hold-to-activate buttons, and rotating buttons.
//! Each button can be configured with animations, sounds, and different behavioral patterns.

use lotus_script::time::delta;

use crate::api::{
    animation::Animation,
    key_event::{KeyEvent, KeyEventCab},
    sound::Sound,
};

/// Defines the different operational modes for push buttons
///
/// Each mode determines how the button responds to user input and how its state changes.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum PushButtonMode {
    /// Standard button behavior: pressed when held down, released when let go
    #[default]
    Regular,
    /// Button toggles its state when pressed and maintains position when held
    PushHold,
    /// Button only toggles its value on press, always returns to unpressed position
    ToggleValueOnlyPress,
    /// Button must be held for a specified duration (in seconds) to maintain its state
    HoldTimed(f32),
    /// Button with rotation animation that can be toggled or controlled via separate events
    RotateReset,
}

/// Builder pattern implementation for creating push buttons with custom configurations
///
/// This builder allows you to configure all aspects of a push button including its
/// visual animations, sound effects, input handling, and operational mode.
pub struct PushButtonBuilder {
    cab_side: Option<KeyEventCab>,

    pos: f32,
    rot: f32,
    value: bool,
    value_last: bool,
    target: bool,

    time: Option<f32>,
    timer: f32,

    key_press: KeyEvent,
    key_release: KeyEvent,
    key_toggle: KeyEvent,

    btn_anim: Animation,
    rot_anim: Animation,

    snd_press: Sound,
    snd_release: Sound,

    mode: PushButtonMode,
}

impl PushButtonBuilder {
    /// Initialize the button in a pressed state
    ///
    /// This method sets the initial position and value based on the button's mode.
    /// For `PushHold` mode, sets position to 0.75 and value to true.
    /// For `RotateReset` mode, sets position to 1.0 and target to true.
    ///
    /// # Returns
    ///
    /// Returns the builder instance for method chaining.
    pub fn init_pressed(mut self, value: bool) -> Self {
        if value {
            if matches!(self.mode, PushButtonMode::PushHold) {
                self.pos = 0.75;
                self.value = true;
                self.btn_anim.set(self.pos);
            } else if matches!(self.mode, PushButtonMode::RotateReset) {
                self.pos = 1.0;
                self.target = true;
                self.btn_anim.set(self.pos);
            } else if matches!(self.mode, PushButtonMode::ToggleValueOnlyPress) {
                self.value = true;
            }
        }
        self
    }

    /// Set the sound effect for button press events
    ///
    /// # Arguments
    ///
    /// * `name` - The name/path of the sound file to play when the button is pressed
    ///
    /// # Returns
    ///
    /// Returns the builder instance for method chaining.
    pub fn snd_press(mut self, name: impl Into<String>) -> Self {
        self.snd_press = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Set the sound effect for button release events
    ///
    /// # Arguments
    ///
    /// * `name` - The name/path of the sound file to play when the button is released
    ///
    /// # Returns
    ///
    /// Returns the builder instance for method chaining.
    pub fn snd_release(mut self, name: impl Into<String>) -> Self {
        self.snd_release = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Build the final PushButton instance
    ///
    /// Consumes the builder and returns a configured `PushButton` ready for use.
    ///
    /// # Returns
    ///
    /// A fully configured `PushButton` instance.
    pub fn build(self) -> PushButton {
        PushButton {
            cab_side: self.cab_side,
            pos: self.pos,
            rot: self.rot,
            value: self.value,
            value_last: self.value_last,
            target: self.target,
            time: self.time,
            timer: self.timer,
            key_press: self.key_press,
            key_release: self.key_release,
            key_toggle: self.key_toggle,
            btn_anim: self.btn_anim,
            rot_anim: self.rot_anim,
            snd_press: self.snd_press,
            snd_release: self.snd_release,
            mode: self.mode,
        }
    }
}

/// Interactive push button with configurable behavior and visual/audio feedback
///
/// `PushButton` provides a comprehensive button implementation that supports multiple
/// interaction modes, animations, and sound effects. It can be used for various UI
/// elements including standard buttons, toggles, hold-to-activate controls, and
/// rotating switches.
///
/// # Examples
///
/// ```rust
/// // Create a standard button
/// let mut button = PushButton::builder("button_anim", "button_event", None)
///     .snd_press("click.wav")
///     .snd_release("release.wav")
///     .build();
///
/// // Create a toggle button that must be held for 2 seconds
/// let mut hold_button = PushButton::builder_time_till_hold(
///     2.0, "hold_anim", "hold_event", None
/// ).build();
/// ```
#[derive(Debug)]
pub struct PushButton {
    cab_side: Option<KeyEventCab>,

    pos: f32,
    rot: f32,
    /// Current pressed state of the button
    pub value: bool,
    value_last: bool,
    target: bool,

    time: Option<f32>,
    timer: f32,

    /// Key event for button press actions
    pub key_press: KeyEvent,
    /// Key event for button release actions
    pub key_release: KeyEvent,
    /// Key event for button toggle actions
    pub key_toggle: KeyEvent,

    btn_anim: Animation,
    rot_anim: Animation,

    snd_press: Sound,
    snd_release: Sound,

    mode: PushButtonMode,
}

impl PushButton {
    /// Create a builder for a standard push button
    ///
    /// # Arguments
    ///
    /// * `animation_name` - Name of the animation to use for button press/release
    /// * `event_name` - Name of the key event to listen for
    /// * `cab_side` - Optional cabinet side specification for multi-player setups
    ///
    /// # Returns
    ///
    /// A `PushButtonBuilder` configured for regular button behavior.
    pub fn builder(
        animation_name: impl Into<String>,
        event_name: impl Into<String>,
        cab_side: Option<KeyEventCab>,
    ) -> PushButtonBuilder {
        PushButton::builder_intern(animation_name, Some(&event_name.into()), cab_side)
    }

    /// Create a builder for a hold-mode push button
    ///
    /// In hold mode, the button toggles its state when pressed and maintains its
    /// position while held down.
    ///
    /// # Arguments
    ///
    /// * `animation_name` - Name of the animation to use for button press/release
    /// * `event_name` - Name of the key event to listen for
    /// * `cab_side` - Optional cabinet side specification for multi-player setups
    ///
    /// # Returns
    ///
    /// A `PushButtonBuilder` configured for hold mode behavior.
    pub fn builder_hold_mode(
        animation_name: impl Into<String>,
        event_name: impl Into<String>,
        cab_side: Option<KeyEventCab>,
    ) -> PushButtonBuilder {
        let mut btn =
            PushButton::builder_intern(animation_name, Some(&event_name.into()), cab_side);
        btn.mode = PushButtonMode::PushHold;
        btn
    }

    /// Create a builder for a toggle button that only changes value on press
    ///
    /// This mode toggles the button's logical state on each press but always
    /// returns to the unpressed visual position.
    ///
    /// # Arguments
    ///
    /// * `animation_name` - Name of the animation to use for button press/release
    /// * `event_name` - Name of the key event to listen for
    /// * `cab_side` - Optional cabinet side specification for multi-player setups
    ///
    /// # Returns
    ///
    /// A `PushButtonBuilder` configured for toggle-on-press behavior.
    pub fn builder_toggle_value_on_press(
        animation_name: impl Into<String>,
        event_name: impl Into<String>,
        cab_side: Option<KeyEventCab>,
    ) -> PushButtonBuilder {
        let mut btn =
            PushButton::builder_intern(animation_name, Some(&event_name.into()), cab_side);
        btn.mode = PushButtonMode::ToggleValueOnlyPress;
        btn
    }

    /// Create a builder for a timed hold button
    ///
    /// The button must be held down for the specified duration to maintain its
    /// activated state. If released before the time expires, it returns to inactive.
    ///
    /// # Arguments
    ///
    /// * `time` - Duration in seconds the button must be held
    /// * `animation_name` - Name of the animation to use for button press/release
    /// * `event_name` - Name of the key event to listen for
    /// * `cab_side` - Optional cabinet side specification for multi-player setups
    ///
    /// # Returns
    ///
    /// A `PushButtonBuilder` configured for timed hold behavior.
    pub fn builder_time_till_hold(
        time: f32,
        animation_name: impl Into<String>,
        event_name: impl Into<String>,
        cab_side: Option<KeyEventCab>,
    ) -> PushButtonBuilder {
        let mut btn =
            PushButton::builder_intern(animation_name, Some(&event_name.into()), cab_side);
        btn.mode = PushButtonMode::HoldTimed(time);
        btn
    }

    /// Create a builder for a rotating button with toggle functionality
    ///
    /// This creates a button that can rotate and be toggled using a separate toggle event.
    /// The button includes both press animation and rotation animation.
    ///
    /// # Arguments
    ///
    /// * `animation_name` - Name of the main button animation
    /// * `rotation_animation_name` - Name of the rotation animation
    /// * `toggle_event_name` - Name of the toggle key event
    /// * `cab_side` - Optional cabinet side specification for multi-player setups
    ///
    /// # Returns
    ///
    /// A `PushButtonBuilder` configured for rotating toggle behavior.
    pub fn builder_rotate_on_release_toggle(
        animation_name: impl Into<String>,
        rotation_animation_name: impl Into<String>,
        toggle_event_name: impl Into<String>,
        cab_side: Option<KeyEventCab>,
    ) -> PushButtonBuilder {
        let mut btn = PushButton::builder_intern(animation_name, None, cab_side);
        btn.mode = PushButtonMode::RotateReset;
        btn.rot_anim = Animation::new(Some(&rotation_animation_name.into()));
        btn.key_toggle = KeyEvent::new(Some(&toggle_event_name.into()), cab_side);
        btn
    }

    /// Create a builder for a rotating button with separate press/release events
    ///
    /// This creates a button that can rotate and has separate events for press and release actions.
    ///
    /// # Arguments
    ///
    /// * `animation_name` - Name of the main button animation
    /// * `rotation_animation_name` - Name of the rotation animation
    /// * `press_event_name` - Name of the press key event
    /// * `release_event_name` - Name of the release key event
    /// * `cab_side` - Optional cabinet side specification for multi-player setups
    ///
    /// # Returns
    ///
    /// A `PushButtonBuilder` configured for rotating press/release behavior.
    pub fn builder_rotate_on_release_press_release(
        animation_name: impl Into<String>,
        rotation_animation_name: impl Into<String>,
        press_event_name: impl Into<String>,
        release_event_name: impl Into<String>,
        cab_side: Option<KeyEventCab>,
    ) -> PushButtonBuilder {
        let mut btn =
            PushButton::builder_intern(animation_name, Some(&press_event_name.into()), cab_side);
        btn.mode = PushButtonMode::RotateReset;
        btn.rot_anim = Animation::new(Some(&rotation_animation_name.into()));
        btn.key_release = KeyEvent::new(Some(&release_event_name.into()), cab_side);
        btn
    }

    /// Internal builder creation method
    ///
    /// Creates a basic builder with default settings that can be customized by the public builder methods.
    fn builder_intern(
        animation_name: impl Into<String>,
        event_name: Option<&str>,
        cab_side: Option<KeyEventCab>,
    ) -> PushButtonBuilder {
        PushButtonBuilder {
            cab_side,
            pos: 0.0,
            rot: 0.0,
            value: false,
            value_last: false,
            target: false,
            time: None,
            timer: 0.0,
            key_press: KeyEvent::new(event_name, cab_side),
            key_release: KeyEvent::new(None, cab_side),
            key_toggle: KeyEvent::new(None, cab_side),
            btn_anim: Animation::new(Some(&animation_name.into())),
            rot_anim: Animation::new(None),
            snd_press: Sound::new_simple(None),
            snd_release: Sound::new_simple(None),

            mode: PushButtonMode::Regular,
        }
    }

    /// Check if the button was just pressed this frame
    ///
    /// Returns true only on the frame when the button transitions from unpressed to pressed.
    ///
    /// # Returns
    ///
    /// `true` if the button was just pressed, `false` otherwise.
    pub fn is_just_pressed(&mut self) -> bool {
        self.value && !self.value_last
    }

    /// Check if the button was just released this frame
    ///
    /// Returns true only on the frame when the button transitions from pressed to unpressed.
    ///
    /// # Returns
    ///
    /// `true` if the button was just released, `false` otherwise.
    pub fn is_just_released(&mut self) -> bool {
        !self.value && self.value_last
    }

    /// Check if the button is currently pressed
    ///
    /// # Returns
    ///
    /// `true` if the button is currently in the pressed state, `false` otherwise.
    pub fn is_pressed(&mut self) -> bool {
        self.value
    }

    /// Check if the button is currently released
    ///
    /// # Returns
    ///
    /// `true` if the button is currently in the released state, `false` otherwise.
    pub fn is_released(&mut self) -> bool {
        !self.value
    }

    /// Manually set the button to pressed state
    ///
    /// This method programmatically activates the button, setting its position and value
    /// according to its current mode. Only works for `PushHold` mode.
    pub fn set(&mut self) {
        if self.mode == PushButtonMode::PushHold {
            self.pos = 0.75;
            self.value = true;
            self.btn_anim.set(self.pos);
        }
    }

    /// Update the button state for the current frame
    ///
    /// This method should be called once per frame to update the button's state,
    /// handle input events, update animations, and manage mode-specific behavior.
    /// It processes key events, updates timers, manages animations, and plays sounds.
    pub fn tick(&mut self) {
        self.value_last = self.value;

        if self.key_press.is_just_pressed() {
            match self.mode {
                PushButtonMode::Regular | PushButtonMode::HoldTimed(_) => {
                    self.pos = 1.0;
                    self.value = self.pos > 0.5;
                    self.snd_press.start();
                }
                PushButtonMode::PushHold | PushButtonMode::ToggleValueOnlyPress => {
                    self.pos = 1.0;
                    self.value = !self.value;
                    self.snd_press.start();
                }
                PushButtonMode::RotateReset => {
                    if !self.target {
                        self.snd_press.start();
                    }
                    self.target = true;
                }
            }
            self.btn_anim.set(self.pos);
        }

        if matches!(self.mode, PushButtonMode::HoldTimed(_)) {
            if self.key_press.is_pressed() {
                self.timer += delta();
            } else {
                self.timer = 0.0;
            }
        }

        if self.key_press.is_just_released() {
            match self.mode {
                PushButtonMode::Regular => {
                    self.pos = 0.0;
                    self.value = self.pos > 0.5;
                    self.snd_release.start();
                }
                PushButtonMode::ToggleValueOnlyPress => {
                    self.pos = 0.0;
                    self.snd_release.start();
                }
                PushButtonMode::PushHold => {
                    self.pos = if self.value { 0.75 } else { 0.0 };
                    self.snd_release.start();
                }
                PushButtonMode::HoldTimed(t) => {
                    if self.timer > t {
                        self.pos = if self.value { 0.75 } else { 0.0 };
                    } else {
                        self.pos = 0.0;
                    }
                    self.snd_release.start();
                }
                _ => {}
            }
            self.btn_anim.set(self.pos);
        }

        if self.key_release.is_just_pressed() && matches!(self.mode, PushButtonMode::RotateReset) {
            if self.target {
                self.snd_release.start();
            }
            self.target = true;
        }

        if self.key_toggle.is_just_pressed() && matches!(self.mode, PushButtonMode::RotateReset) {
            self.target = !self.target;
            if self.target {
                self.snd_press.start();
            } else {
                self.snd_release.start();
            }
        }

        // Animation movement (Bewegung)

        if matches!(self.mode, PushButtonMode::RotateReset) {
            if self.target {
                if self.pos < 1.0 {
                    self.pos = (self.pos + 20.0 * delta()).min(1.0);
                }
            } else if self.pos >= 1.0 && self.rot < 1.0 {
                self.rot += 2.0 * delta();
            } else {
                if self.pos > 0.0 {
                    self.pos = (self.pos - 20.0 * delta()).max(0.0);
                }
                if self.rot > 0.0 {
                    self.rot = (self.rot - 20.0 * delta()).max(0.0);
                }
            }
            self.value = self.pos > 0.5;

            self.btn_anim.set(self.pos);
            self.rot_anim.set(self.rot);
        }
    }

    /// Get the button's value with additional permission check
    ///
    /// Returns the button's pressed state only if the `allowed` parameter is true.
    /// This can be useful for implementing button lockouts or conditional behavior.
    ///
    /// # Arguments
    ///
    /// * `allowed` - Whether the button's value should be considered valid
    ///
    /// # Returns
    ///
    /// `true` if the button is pressed AND allowed, `false` otherwise.
    pub fn value(&self, allowed: bool) -> bool {
        self.value && allowed
    }
}
