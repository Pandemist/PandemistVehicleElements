//! Window management utilities for creating interactive folding and sliding windows.
//!
//! This module provides two main window types:
//! - [`FoldingWindow`]: A window that can be toggled open/closed with keyboard input
//! - [`SlidingWindow`]: A window that can be dragged along an axis using mouse input
//!
//! Both window types support animations, sound effects, and customizable input handling.
//!
//! # Examples
//!
//! Creating a folding window:
//! ```rust
//! use pandemist_vehicle_elements::windows::FoldingWindow;
//! use lotus_extra::vehicle::CockpitSide;
//!
//! let mut window = FoldingWindow::builder("fold_animation", "toggle_key", Some(CockpitSide::A))
//!     .snd_open("window_open.wav")
//!     .snd_close("window_close.wav")
//!     .build();
//! ```
//!
//! Creating a sliding window:
//! ```rust
//! use pandemist_vehicle_elements::windows::SlidingWindow;
//! use lotus_extra::vehicle::CockpitSide;
//!
//! let mut window = SlidingWindow::builder("slide_animation", "grab_key", Some(CockpitSide::B))
//!     .axis_x()
//!     .mouse_factor(0.5)
//!     .snd_handle_grab("grab.wav")
//!     .build();
//! ```

use std::rc::Rc;

use lotus_extra::vehicle::CockpitSide;
use lotus_script::math::Vec2;

use crate::api::{animation::Animation, general::mouse_move, key_event::KeyEvent, sound::Sound};

/// Builder for creating a [`FoldingWindow`].
///
/// Provides a fluent interface for configuring all aspects of a folding window,
/// including sound effects and animations.
pub struct FoldingWindowBuilder {
    pos: f32,
    value: bool,

    snd_open: Sound,
    snd_close: Sound,

    anim: Animation,

    key_toggle: KeyEvent,
}

impl FoldingWindowBuilder {
    /// Sets the sound effect to play when the window opens.
    ///
    /// # Arguments
    ///
    /// * `name` - The name/path of the sound file to play
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use pandemist_vehicle_elements::windows::FoldingWindow;
    /// let window = FoldingWindow::builder("anim", "key", None)
    ///     .snd_open("open_sound.wav")
    ///     .build();
    /// ```
    pub fn snd_open(mut self, name: impl Into<String>) -> Self {
        self.snd_open = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Sets the sound effect to play when the window closes.
    ///
    /// # Arguments
    ///
    /// * `name` - The name/path of the sound file to play
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use pandemist_vehicle_elements::windows::FoldingWindow;
    /// let window = FoldingWindow::builder("anim", "key", None)
    ///     .snd_close("close_sound.wav")
    ///     .build();
    /// ```
    pub fn snd_close(mut self, name: impl Into<String>) -> Self {
        self.snd_close = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Consumes the builder and creates a [`FoldingWindow`].
    ///
    /// # Returns
    ///
    /// A fully configured `FoldingWindow` instance.
    pub fn build(self) -> FoldingWindow {
        FoldingWindow {
            pos: self.pos,
            value: self.value,
            snd_open: self.snd_open,
            snd_close: self.snd_close,
            anim: self.anim,
            key_toggle: self.key_toggle,
        }
    }
}

/// A window that can be toggled between open and closed states.
///
/// The folding window responds to keyboard input to toggle its state,
/// playing different sounds and animations for opening and closing.
/// The window's position is represented as a value between 0.0 (closed) and 1.0 (open).
///
/// # Examples
///
/// ```rust
/// use pandemist_vehicle_elements::windows::FoldingWindow;
/// use lotus_extra::vehicle::CockpitSide;
///
/// let mut window = FoldingWindow::builder("fold_animation", "space", Some(CockpitSide::A))
///     .snd_open("open.wav")
///     .snd_close("close.wav")
///     .build();
///
/// // In your game loop:
/// window.tick(); // Call this each frame to handle input and update state
/// ```
#[derive(Debug)]
pub struct FoldingWindow {
    pos: f32,
    value: bool,

    snd_open: Sound,
    snd_close: Sound,

    anim: Animation,

    key_toggle: KeyEvent,
}

impl FoldingWindow {
    /// Creates a new builder for configuring a folding window.
    ///
    /// # Arguments
    ///
    /// * `animation_name` - The name of the animation to use for the folding effect
    /// * `event_name` - The name of the key event that will toggle the window
    /// * `cab_side` - Optional cab side specification for the key event
    ///
    /// # Returns
    ///
    /// A [`FoldingWindowBuilder`] instance for further configuration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pandemist_vehicle_elements::windows::FoldingWindow;
    /// use lotus_extra::vehicle::CockpitSide;
    ///
    /// let builder = FoldingWindow::builder("my_animation", "toggle_key", Some(CockpitSide::A));
    /// ```
    pub fn builder(
        animation_name: impl Into<String>,
        event_name: &str,
        cab_side: Option<CockpitSide>,
    ) -> FoldingWindowBuilder {
        FoldingWindowBuilder {
            pos: 0.0,
            value: false,
            snd_open: Sound::new_simple(None),
            snd_close: Sound::new_simple(None),
            anim: Animation::new(Some(&animation_name.into())),
            key_toggle: KeyEvent::new(Some(event_name), cab_side),
        }
    }

    /// Updates the window state based on input.
    ///
    /// This method should be called once per frame in your game loop.
    /// It handles:
    /// - Detecting key presses for toggling the window
    /// - Playing appropriate sound effects
    /// - Updating the animation state
    /// - Switching between open/closed states
    ///
    /// The window toggles between positions 0.0 (closed) and 1.0 (open),
    /// with the boolean value being `true` when position > 0.5.
    pub fn tick(&mut self) {
        if self.key_toggle.is_just_pressed() {
            self.pos = 1.0 - self.pos;
            self.value = self.pos > 0.5;

            if self.value {
                self.snd_open.start();
            } else {
                self.snd_close.start();
            }

            self.anim.set(self.pos);
        }
    }
}

//===========================================================

/// Builder for creating a [`SlidingWindow`].
///
/// Provides a fluent interface for configuring all aspects of a sliding window,
/// including axis of movement, mouse sensitivity, sound effects, and animations.
pub struct SlidingWindowBuilder {
    pos: f32,
    axis: Vec2,

    key_grabbing: KeyEvent,

    window_anim: Animation,
    switch_anim: Animation,

    snd_handle_grab: Sound,
    snd_handle_release: Sound,
    snd_slide: Sound,
    snd_slide_vol_curve: Rc<dyn Fn(f32) -> f32>,
    snd_slide_upper_end: Sound,
    snd_slide_upper_end_vol_curve: Rc<dyn Fn(f32) -> f32>,
    snd_slide_lower_end: Sound,
    snd_slide_lower_end_vol_curve: Rc<dyn Fn(f32) -> f32>,

    mouse_factor: f32,
}

impl SlidingWindowBuilder {
    /// Sets the mouse sensitivity factor for sliding.
    ///
    /// Higher values make the window more responsive to mouse movement.
    ///
    /// # Arguments
    ///
    /// * `mouse_factor` - Multiplier for mouse movement sensitivity (typically 0.1 to 2.0)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use pandemist_vehicle_elements::windows::SlidingWindow;
    /// let window = SlidingWindow::builder("anim", "key", None)
    ///     .mouse_factor(0.5) // Half sensitivity
    ///     .build();
    /// ```
    pub fn mouse_factor(mut self, mouse_factor: f32) -> Self {
        self.mouse_factor = mouse_factor;
        self
    }

    /// Sets the sound effect to play when grabbing the window handle.
    ///
    /// # Arguments
    ///
    /// * `name` - The name/path of the sound file to play
    pub fn snd_handle_grab(mut self, name: impl Into<String>) -> Self {
        self.snd_handle_grab = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Sets the sound effect to play when releasing the window handle.
    ///
    /// # Arguments
    ///
    /// * `name` - The name/path of the sound file to play
    pub fn snd_handle_release(mut self, name: impl Into<String>) -> Self {
        self.snd_handle_release = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Sets the sound effect to play while sliding the window.
    ///
    /// This sound will loop while the window is being moved.
    ///
    /// # Arguments
    ///
    /// * `name` - The name/path of the sound file to play
    pub fn snd_slide(
        mut self,
        name: impl Into<String>,
        volume_name: Option<&str>,
        volume_func: Option<Rc<dyn Fn(f32) -> f32>>,
    ) -> Self {
        self.snd_slide = Sound::new(Some(&name.into()), volume_name, None);
        self.snd_slide_vol_curve = match volume_func {
            Some(curve) => curve,
            None => Rc::new(|x| x),
        };
        self
    }

    pub fn snd_slide_upper_end(
        mut self,
        name: impl Into<String>,
        volume_name: Option<&str>,
        volume_func: Option<Rc<dyn Fn(f32) -> f32>>,
    ) -> Self {
        self.snd_slide_upper_end = Sound::new(Some(&name.into()), volume_name, None);
        self.snd_slide_upper_end_vol_curve = match volume_func {
            Some(curve) => curve,
            None => Rc::new(|x| x),
        };
        self
    }

    pub fn snd_slide_lower_end(
        mut self,
        name: impl Into<String>,
        volume_name: Option<&str>,
        volume_func: Option<Rc<dyn Fn(f32) -> f32>>,
    ) -> Self {
        self.snd_slide_lower_end = Sound::new(Some(&name.into()), volume_name, None);
        self.snd_slide_lower_end_vol_curve = match volume_func {
            Some(curve) => curve,
            None => Rc::new(|x| x),
        };
        self
    }

    /// Sets the animation for the window handle/switch.
    ///
    /// This animation typically shows the visual state of whether the handle is grabbed.
    ///
    /// # Arguments
    ///
    /// * `animation_name` - The name of the handle animation
    pub fn handle_animation(mut self, animation_name: impl Into<String>) -> Self {
        self.switch_anim = Animation::new(Some(&animation_name.into()));
        self
    }

    /// Configures the window to slide along the X-axis (horizontally).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use pandemist_vehicle_elements::windows::SlidingWindow;
    /// let window = SlidingWindow::builder("anim", "key", None)
    ///     .axis_x() // Slide horizontally
    ///     .build();
    /// ```
    pub fn axis_x(mut self) -> Self {
        self.axis = Vec2 { x: 1.0, y: 0.0 };
        self
    }

    /// Configures the window to slide along the Y-axis (vertically).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use pandemist_vehicle_elements::windows::SlidingWindow;
    /// let window = SlidingWindow::builder("anim", "key", None)
    ///     .axis_y() // Slide vertically
    ///     .build();
    /// ```
    pub fn axis_y(mut self) -> Self {
        self.axis = Vec2 { x: 0.0, y: 1.0 };
        self
    }

    /// Consumes the builder and creates a [`SlidingWindow`].
    ///
    /// # Returns
    ///
    /// A fully configured `SlidingWindow` instance.
    pub fn build(self) -> SlidingWindow {
        SlidingWindow {
            pos: self.pos,
            axis: self.axis,
            key_grabbing: self.key_grabbing,
            window_anim: self.window_anim,
            switch_anim: self.switch_anim,
            snd_handle_grab: self.snd_handle_grab,
            snd_handle_release: self.snd_handle_release,
            snd_slide: self.snd_slide,
            snd_slide_vol_curve: self.snd_slide_vol_curve,
            snd_slide_upper_end: self.snd_slide_upper_end,
            snd_slide_upper_end_vol_curve: self.snd_slide_upper_end_vol_curve,
            snd_slide_lower_end: self.snd_slide_lower_end,
            snd_slide_lower_end_vol_curve: self.snd_slide_lower_end_vol_curve,
            mouse_factor: self.mouse_factor,
            end_snd_played: false,
        }
    }
}

/// A window that can be dragged along a specified axis using mouse input.
///
/// The sliding window responds to a key press to "grab" the window, after which
/// mouse movement will slide the window along the configured axis. The window
/// position is clamped between 0.0 and 1.0.
///
/// # Features
///
/// - Configurable slide axis (X or Y)
/// - Mouse sensitivity control
/// - Sound effects for grabbing, releasing, and sliding
/// - Separate animations for window position and handle state
/// - Automatic sound management (slide sound plays only when moving)
///
/// # Examples
///
/// ```rust
/// use pandemist_vehicle_elements::windows::SlidingWindow;
/// use pandemist_vehicle_elements::api::key_event::CockpitSide;
///
/// let mut window = SlidingWindow::builder("slide_anim", "ctrl", Some(CockpitSide::Left))
///     .axis_x()
///     .mouse_factor(0.8)
///     .snd_handle_grab("grab.wav")
///     .snd_handle_release("release.wav")
///     .snd_slide("slide.wav")
///     .build();
///
/// // In your game loop:
/// window.tick(); // Call this each frame to handle input and update state
/// ```
pub struct SlidingWindow {
    pos: f32,
    axis: Vec2,

    key_grabbing: KeyEvent,

    window_anim: Animation,
    switch_anim: Animation,

    snd_handle_grab: Sound,
    snd_handle_release: Sound,
    snd_slide: Sound,
    snd_slide_vol_curve: Rc<dyn Fn(f32) -> f32>,
    snd_slide_upper_end: Sound,
    snd_slide_upper_end_vol_curve: Rc<dyn Fn(f32) -> f32>,
    snd_slide_lower_end: Sound,
    snd_slide_lower_end_vol_curve: Rc<dyn Fn(f32) -> f32>,

    mouse_factor: f32,

    end_snd_played: bool,
}

impl SlidingWindow {
    /// Creates a new builder for configuring a sliding window.
    ///
    /// # Arguments
    ///
    /// * `animation_name` - The name of the animation to use for the window sliding
    /// * `event_name` - The name of the key event that will grab/release the window
    /// * `cab_side` - Optional cab side specification for the key event
    ///
    /// # Returns
    ///
    /// A [`SlidingWindowBuilder`] instance for further configuration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pandemist_vehicle_elements::windows::SlidingWindow;
    ///
    /// let builder = SlidingWindow::builder("my_slide_anim", "grab_key", Some(CabinSide::A));
    /// ```
    pub fn builder(
        animation_name: impl Into<String>,
        event_name: impl Into<String>,
        cab_side: Option<CockpitSide>,
    ) -> SlidingWindowBuilder {
        SlidingWindowBuilder {
            pos: 0.0,
            axis: Vec2 { x: 0.0, y: 0.0 },
            key_grabbing: KeyEvent::new(Some(&event_name.into()), cab_side),
            window_anim: Animation::new(Some(&animation_name.into())),
            switch_anim: Animation::new(None),
            snd_handle_grab: Sound::new_simple(None),
            snd_handle_release: Sound::new_simple(None),
            snd_slide: Sound::new_simple(None),
            snd_slide_vol_curve: Rc::new(|x| x),
            snd_slide_upper_end: Sound::new_simple(None),
            snd_slide_upper_end_vol_curve: Rc::new(|x| x),
            snd_slide_lower_end: Sound::new_simple(None),
            snd_slide_lower_end_vol_curve: Rc::new(|x| x),
            mouse_factor: 0.0,
        }
    }

    /// Updates the window state based on input.
    ///
    /// This method should be called once per frame in your game loop.
    /// It handles:
    /// - Detecting key presses/releases for grabbing the window handle
    /// - Processing mouse movement when the handle is grabbed
    /// - Clamping window position between 0.0 and 1.0
    /// - Playing appropriate sound effects
    /// - Updating animation states
    /// - Managing the sliding sound (plays only when moving significantly)
    ///
    /// The window position is updated based on mouse movement projected onto
    /// the configured axis, scaled by the mouse factor.
    pub fn tick(&mut self) {
        let window_last = self.pos;

        if self.key_grabbing.is_just_pressed() {
            self.switch_anim.set(1.0);
            self.snd_handle_grab.start();
        }
        if self.key_grabbing.is_just_released() {
            self.switch_anim.set(0.0);
            self.snd_handle_release.start();
        }

        let vec_mouse = mouse_move() * self.axis;

        let hand_delta = (vec_mouse.x + vec_mouse.y) * self.mouse_factor;

        if self.key_grabbing.is_pressed() {
            self.pos += hand_delta;

            if self.pos > 1.0 {
                if !self.end_snd_played {
                    self.end_snd_played = true;
                    self.snd_slide_upper_end
                        .update_volume((self.snd_slide_upper_end_vol_curve)(hand_delta));
                    self.snd_slide_upper_end.start();
                }
                self.pos = 1.0;
            }

            if self.pos < 0.0 {
                if !self.end_snd_played {
                    self.end_snd_played = true;
                    self.snd_slide_lower_end
                        .update_volume((self.snd_slide_lower_end_vol_curve)(hand_delta));
                    self.snd_slide_lower_end.start();
                }
                self.pos = 0.0;
            }
        }

        if !self.key_grabbing.is_pressed() || (self.pos > 0.02 && self.pos < 0.98) {
            self.end_snd_played = false;
        }

        self.snd_slide
            .update_volume((self.snd_slide_vol_curve)(hand_delta));
        self.snd_slide
            .start_stop((window_last - self.pos).abs() > 0.001);

        self.window_anim.set(self.pos);
    }
}
