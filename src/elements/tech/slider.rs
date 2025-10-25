//! # Slider and Rollo Components
//!
//! This module provides interactive slider and rollo (roll-up) components for UI applications.
//! Both components support mouse interaction, keyboard events, animations, and sound effects.
//!
//! ## Features
//!
//! - **Slider**: A draggable slider with customizable bounds, physics simulation, and path following
//! - **Rollo**: A roll-up component that can be pulled and reset, useful for curtains, blinds, or similar UI elements
//! - Physics simulation with force, friction, and bouncing
//! - Mouse and keyboard interaction
//! - Animation and sound integration
//! - Flexible builder pattern for easy configuration
//!
//! ## Quick Start
//!
//! ```rust
//! use slider::{Slider, Rollo};
//! use lotus_script::api::key_event::CockpitSide;
//!
//! // Create a horizontal slider
//! let mut slider = Slider::builder()
//!     .min(0.0)
//!     .max(100.0)
//!     .axis_x()
//!     .key_event("slider_grab", Some(CockpitSide::ACab))
//!     .animation("slider_animation")
//!     .build();
//!
//! // Create a rollo component
//! let mut rollo = Rollo::builder("rollo_animation", "rollo_draw", Some(CockpitSide::ACab))
//!     .mouse_factor(2.0)
//!     .snd_pull("pull_loop_sound")
//!     .snd_reset("reset_sound")
//!     .build();
//!
//! // Update components each frame
//! slider.tick();
//! rollo.tick();
//! ```

use std::rc::Rc;

use lotus_extra::vehicle::CockpitSide;
use lotus_script::{math::Vec2, time::delta};

use crate::{
    api::{animation::Animation, general::mouse_move, key_event::KeyEvent, sound::Sound},
    elements::std::piecewise_linear_function::PiecewiseLinearFunction,
};

/// Builder for creating a [`Slider`] component with customizable properties.
///
/// The builder pattern allows for flexible configuration of slider behavior,
/// including physics properties, input handling, and visual/audio feedback.
///
/// # Example
///
/// ```rust
/// let slider = Slider::builder()
///     .min(-50.0)
///     .max(50.0)
///     .axis_x()
///     .force(10.0)
///     .friction(5.0)
///     .upper_bump_factor(0.8)
///     .lower_bump_factor(0.8)
///     .key_event("grab_slider", Some(CockpitSide::ACab))
///     .animation("slider_pos")
///     .build();
/// ```
pub struct SliderBuilder {
    pos: f32,
    pos_last: f32,

    min: f32,
    max: f32,

    upper_bump_factor: f32,
    lower_bump_factor: f32,

    force: f32,
    friction: f32,
    speed: f32,

    axis: Vec2,

    stay_at_upper: bool,
    stay_at_lower: bool,

    only_while_grab: bool,

    mouse_factor: f32,

    key_grab: KeyEvent,

    path: Option<PiecewiseLinearFunction>,

    pos_anim: Animation,

    snd_open_end: Sound,
    snd_open_end_vol_curve: Rc<dyn Fn(f32) -> f32>,
    snd_close_end: Sound,
    snd_close_end_vol_curve: Rc<dyn Fn(f32) -> f32>,
}

impl SliderBuilder {
    /// Sets the animation for the slider position.
    ///
    /// # Arguments
    ///
    /// * `animation_name` - Name of the animation to use for position updates
    ///
    /// # Example
    ///
    /// ```rust
    /// let slider = Slider::builder()
    ///     .animation("my_slider_animation")
    ///     .build();
    /// ```
    pub fn animation(mut self, animation_name: impl Into<String>) -> Self {
        self.pos_anim = Animation::new(Some(&animation_name.into()));
        self
    }

    /// Sets the key event for grabbing/controlling the slider.
    ///
    /// # Arguments
    ///
    /// * `event_name` - Name of the key event
    /// * `cab_side` - Optional cabinet side specification
    ///
    /// # Example
    ///
    /// ```rust
    /// let slider = Slider::builder()
    ///     .key_event("slider_grab", Some(CockpitSide::Left))
    ///     .build();
    /// ```
    pub fn key_event(
        mut self,
        event_name: impl Into<String>,
        cab_side: Option<CockpitSide>,
    ) -> Self {
        self.key_grab = KeyEvent::new(Some(&event_name.into()), cab_side);
        self
    }

    /// Sets the bounce factor when the slider hits the upper bound.
    ///
    /// A value of 0.0 means no bounce (slider stops), while 1.0 means perfect bounce.
    /// Values greater than 1.0 will amplify the bounce.
    ///
    /// # Arguments
    ///
    /// * `value` - Bounce factor (0.0 = no bounce, 1.0 = perfect bounce)
    pub fn upper_bump_factor(mut self, value: f32) -> Self {
        self.upper_bump_factor = value;
        self
    }

    /// Sets the bounce factor when the slider hits the lower bound.
    ///
    /// A value of 0.0 means no bounce (slider stops), while 1.0 means perfect bounce.
    /// Values greater than 1.0 will amplify the bounce.
    ///
    /// # Arguments
    ///
    /// * `value` - Bounce factor (0.0 = no bounce, 1.0 = perfect bounce)
    pub fn lower_bumb_factor(mut self, value: f32) -> Self {
        self.lower_bump_factor = value;
        self
    }

    pub fn stay_at_upper(mut self) -> Self {
        self.stay_at_upper = true;
        self
    }

    pub fn stay_at_lower(mut self) -> Self {
        self.stay_at_lower = true;
        self
    }

    /// Configures the slider to only move while being grabbed.
    ///
    /// When enabled, the slider will not continue moving due to physics
    /// when not actively being controlled.
    pub fn only_while_grab(mut self) -> Self {
        self.only_while_grab = true;
        self
    }

    /// Sets the mouse sensitivity factor.
    ///
    /// Higher values make the slider more sensitive to mouse movement.
    ///
    /// # Arguments
    ///
    /// * `mouse_factor` - Sensitivity multiplier (default: 1.0)
    pub fn mouse_factor(mut self, mouse_factor: f32) -> Self {
        self.mouse_factor = mouse_factor;
        self
    }

    /// Sets the constant force applied to the slider.
    ///
    /// This can be used to simulate gravity or other constant forces.
    ///
    /// # Arguments
    ///
    /// * `force` - Force value in slider units per second squared
    pub fn force(mut self, force: f32) -> Self {
        self.force = force;
        self
    }

    /// Sets the friction coefficient.
    ///
    /// Higher values will slow down the slider more quickly.
    ///
    /// # Arguments
    ///
    /// * `friction` - Friction coefficient
    pub fn friction(mut self, friction: f32) -> Self {
        self.friction = friction;
        self
    }

    /// Configures the slider to move along the X-axis.
    pub fn axis_x(mut self) -> Self {
        self.axis = Vec2 { x: 1.0, y: 0.0 };
        self
    }

    /// Configures the slider to move along the Y-axis.
    pub fn axis_y(mut self) -> Self {
        self.axis = Vec2 { x: 0.0, y: 1.0 };
        self
    }

    /// Sets the maximum value for the slider.
    ///
    /// # Arguments
    ///
    /// * `max` - Maximum slider value
    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    /// Sets the minimum value for the slider.
    ///
    /// # Arguments
    ///
    /// * `min` - Minimum slider value
    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    /// Sets a custom path for the slider to follow.
    ///
    /// The path maps slider positions to visual positions using a piecewise linear function.
    ///
    /// # Arguments
    ///
    /// * `path` - Piecewise linear function defining the slider path
    pub fn path(mut self, path: PiecewiseLinearFunction) -> Self {
        self.path = Some(path);
        self
    }

    pub fn snd_open_end(
        mut self,
        snd_open_end_name: impl Into<String>,
        volume_name: Option<&str>,
        volume_func: Option<Rc<dyn Fn(f32) -> f32>>,
    ) -> Self {
        self.snd_open_end = Sound::new(Some(&snd_open_end_name.into()), volume_name, None);
        self.snd_open_end_vol_curve = match volume_func {
            Some(curve) => curve,
            None => Rc::new(|x| x),
        };
        self
    }

    pub fn snd_close_end(
        mut self,
        snd_close_end_name: impl Into<String>,
        volume_name: Option<&str>,
        volume_func: Option<Rc<dyn Fn(f32) -> f32>>,
    ) -> Self {
        self.snd_close_end = Sound::new(Some(&snd_close_end_name.into()), volume_name, None);
        self.snd_close_end_vol_curve = match volume_func {
            Some(curve) => curve,
            None => Rc::new(|x| x),
        };
        self
    }

    /// Builds the final [`Slider`] instance with the configured properties.
    pub fn build(self) -> Slider {
        Slider {
            pos: self.pos,
            pos_last: self.pos_last,

            min: self.min,
            max: self.max,

            upper_bump_factor: self.upper_bump_factor,
            lower_bump_factor: self.lower_bump_factor,

            force: self.force,
            friction: self.friction,
            speed: self.speed,

            axis: self.axis,

            stay_at_upper: self.stay_at_upper,
            stay_at_lower: self.stay_at_lower,
            only_while_grab: self.only_while_grab,

            mouse_factor: self.mouse_factor,

            key_grab: self.key_grab,

            path: self.path,

            pos_anim: self.pos_anim,

            snd_open_end: self.snd_open_end,
            snd_open_end_vol_curve: self.snd_open_end_vol_curve,
            snd_close_end: self.snd_close_end,
            snd_close_end_vol_curve: self.snd_close_end_vol_curve,
        }
    }
}

/// A physics-based slider component with customizable behavior.
///
/// The slider supports mouse interaction, keyboard control, physics simulation,
/// and can follow custom paths. It provides smooth movement with configurable
/// friction, bouncing, and force application.
///
/// # Physics Model
///
/// The slider uses a simple physics model with:
/// - **Position**: Current slider position
/// - **Speed**: Current velocity
/// - **Force**: Constant acceleration (e.g., gravity)
/// - **Friction**: Opposes movement
/// - **Bouncing**: Configurable bounce factors at boundaries
///
/// # Example
///
/// ```rust
/// let mut slider = Slider::builder()
///     .min(0.0)
///     .max(100.0)
///     .axis_x()
///     .friction(2.0)
///     .upper_bump_factor(0.5)
///     .build();
///
/// // In your update loop:
/// slider.tick();
/// println!("Slider position: {}", slider.pos);
/// ```
pub struct Slider {
    /// Current position of the slider
    pub pos: f32,
    /// Position from the previous frame (used for delta calculations)
    pub pos_last: f32,

    /// Minimum allowed position
    pub min: f32,
    /// Maximum allowed position  
    pub max: f32,

    /// Bounce factor when hitting the upper bound
    pub upper_bump_factor: f32,
    /// Bounce factor when hitting the lower bound
    pub lower_bump_factor: f32,

    /// Constant force applied to the slider
    pub force: f32,
    pub friction: f32,
    pub speed: f32,

    axis: Vec2,

    stay_at_upper: bool,
    stay_at_lower: bool,
    only_while_grab: bool,

    mouse_factor: f32,

    /// Key event for grabbing/controlling the slider
    pub key_grab: KeyEvent,

    path: Option<PiecewiseLinearFunction>,

    pos_anim: Animation,

    snd_open_end: Sound,
    snd_open_end_vol_curve: Rc<dyn Fn(f32) -> f32>,
    snd_close_end: Sound,
    snd_close_end_vol_curve: Rc<dyn Fn(f32) -> f32>,
}

impl Slider {
    /// Creates a new [`SliderBuilder`] with default values.
    ///
    /// Default configuration:
    /// - Position: 0.0
    /// - Range: 0.0 to 1.0
    /// - No bouncing
    /// - No physics forces
    /// - Mouse factor: 1.0
    /// - Axis: (0, 0) - must be set via `axis_x()` or `axis_y()`
    pub fn builder() -> SliderBuilder {
        SliderBuilder {
            pos: 0.0,
            pos_last: 0.0,

            min: 0.0,
            max: 1.0,

            upper_bump_factor: 0.0,
            lower_bump_factor: 0.0,

            force: 0.0,
            friction: 0.0,
            speed: 0.0,

            axis: Vec2 { x: 0.0, y: 0.0 },

            stay_at_upper: false,
            stay_at_lower: false,
            only_while_grab: false,

            mouse_factor: 1.0,
            key_grab: KeyEvent::new(None, None),
            path: None,
            pos_anim: Animation::new(None),

            snd_open_end: Sound::new_simple(None),
            snd_open_end_vol_curve: Rc::new(|x| x),
            snd_close_end: Sound::new_simple(None),
            snd_close_end_vol_curve: Rc::new(|x| x),
        }
    }

    /// Updates the animation with the current slider position.
    ///
    /// This method is called internally and applies path transformations if configured.
    fn update(&mut self) {
        let new_pos = if let Some(ref mut path) = self.path {
            path.get_value(self.pos).unwrap()
        } else {
            self.pos
        };

        self.pos_anim.set(new_pos);
    }

    /// Directly sets the slider position.
    ///
    /// This bypasses physics simulation and immediately updates the position
    /// and associated animations.
    ///
    /// # Arguments
    ///
    /// * `new_pos` - New position value
    pub fn set_pos(&mut self, new_pos: f32) {
        self.pos = new_pos;
        self.update();
    }

    /// Updates the slider state for one frame.
    ///
    /// This method should be called once per frame to update the slider's
    /// position based on user input and physics simulation.
    ///
    /// The update process:
    /// 1. Handle mouse input when grabbed
    /// 2. Apply physics simulation (force, friction, bouncing)
    /// 3. Clamp position to bounds
    /// 4. Update animations
    pub fn tick(&mut self) {
        self.pos_last = self.pos;

        let vec_mouse = mouse_move() * self.axis;

        let hand_delta = (vec_mouse.x + vec_mouse.y) * self.mouse_factor;
        if self.key_grab.is_pressed() {
            if self.min > self.pos {
                self.pos = (self.pos + hand_delta)
                    .min(self.max)
                    .max(self.pos.min(self.min));

                /*self.pos = self
                .min
                .max(self.pos)
                .max(self.max.min(self.pos + hand_delta));*/
                self.update();
            } else if self.max < self.pos {
                self.pos = (self.pos + hand_delta)
                    .max(self.min)
                    .min(self.pos.max(self.max));

                /*self.pos = self
                .min
                .max((self.max.min(self.pos)).min(self.pos + hand_delta));*/
                self.update();
            } else {
                self.pos = (self.pos + hand_delta).clamp(self.min, self.max);
                self.update();
            }
            self.speed = hand_delta / delta();
        } else if !self.only_while_grab
            && (self.pos < self.max || !self.stay_at_upper)
            && (self.pos > self.min || !self.stay_at_lower)
        {
            self.pos += self.speed * delta();
        }

        if self.pos > self.max {
            self.snd_open_end
                .update_volume((self.snd_open_end_vol_curve)(self.speed));
            self.snd_open_end.start();

            self.pos = self.max;
            if self.upper_bump_factor > 0.0 {
                self.speed *= -self.upper_bump_factor;
            } else {
                self.speed = 0.0;
            }
        }

        if self.pos < self.min {
            self.snd_close_end
                .update_volume((self.snd_close_end_vol_curve)(self.speed));
            self.snd_close_end.start();

            self.pos = self.min;
            if self.lower_bump_factor > 0.0 {
                self.speed *= -self.lower_bump_factor;
            } else {
                self.speed = 0.0;
            }
        }

        self.speed += self.force * delta();

        if self.speed.abs() > 0.0001 {
            let new_speed = self.speed + (-self.speed.signum() * self.friction) * delta();

            self.speed = if new_speed * self.speed < 0.0 {
                0.0
            } else {
                new_speed
            };
        }

        self.update();
    }
}

//======================================================================
// Rollo
//======================================================================

/// Builder for creating a [`Rollo`] component with customizable properties.
///
/// The rollo (roll-up) component simulates elements like curtains, blinds, or
/// other UI elements that can be pulled down and reset.
///
/// # Example
///
/// ```rust
/// let rollo = Rollo::builder("curtain_animation", "pull_curtain", Some(CockpitSide::ACab))
///     .mouse_factor(1.5)
///     .snd_pull("curtain_pull")
///     .snd_reset("curtain_reset")
///     .up_with_reset_switch("reset_animation", "reset_curtain")
///     .build();
/// ```
pub struct RolloBuilder {
    cab_side: Option<CockpitSide>,

    pos_rollo: f32,

    mouse_factor: f32,

    key_draw: KeyEvent,
    key_reset: KeyEvent,

    rollo_anim: Animation,
    reset_anim: Animation,

    pull_loop_sound: Sound,
    pull_single_sound: Sound,
    pull_step_last: f32,
    pull_step_width: f32,
    reset_sound: Sound,

    only_pull: bool,
    reset_flag: bool,
}

impl RolloBuilder {
    /// Sets the mouse sensitivity factor.
    ///
    /// Higher values make the rollo more sensitive to mouse movement.
    ///
    /// # Arguments
    ///
    /// * `mouse_factor` - Sensitivity multiplier
    pub fn mouse_factor(mut self, mouse_factor: f32) -> Self {
        self.mouse_factor = mouse_factor;
        self
    }

    /// Sets the sound effect for pulling the rollo.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the sound effect
    pub fn snd_pull_loop(mut self, name: impl Into<String>) -> Self {
        self.pull_loop_sound = Sound::new_simple(Some(&name.into()));
        self
    }

    pub fn snd_pull_single(mut self, name: impl Into<String>, step_width: f32) -> Self {
        self.pull_step_width = step_width;
        self.pull_single_sound = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Sets the sound effect for resetting the rollo.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the sound effect
    pub fn snd_reset(mut self, name: impl Into<String>) -> Self {
        self.reset_sound = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Configures the rollo to only allow pulling (not pushing) with a reset switch.
    ///
    /// When enabled, the rollo can only be pulled down and must be reset using
    /// a separate key event. This is useful for curtains or blinds that spring back.
    ///
    /// # Arguments
    ///
    /// * `animation_name` - Name of the reset animation
    /// * `event_name` - Name of the reset key event
    pub fn up_with_reset_switch(
        mut self,
        animation_name: impl Into<String>,
        event_name: &str,
    ) -> Self {
        self.reset_anim = Animation::new(Some(&animation_name.into()));
        self.key_reset = KeyEvent::new(Some(event_name), self.cab_side);
        self.only_pull = true;
        self
    }

    /// Builds the final [`Rollo`] instance with the configured properties.
    pub fn build(self) -> Rollo {
        Rollo {
            cab_side: self.cab_side,
            pos_rollo: self.pos_rollo,
            mouse_factor: self.mouse_factor,
            key_draw: self.key_draw,
            key_reset: self.key_reset,
            rollo_anim: self.rollo_anim,
            reset_anim: self.reset_anim,
            pull_single_sound: self.pull_single_sound,
            pull_step_last: self.pull_step_last,
            pull_step_width: self.pull_step_width,
            pull_loop_sound: self.pull_loop_sound,
            reset_sound: self.reset_sound,
            reset_flag: self.reset_flag,
            only_pull: self.only_pull,
        }
    }
}

/// A rollo (roll-up) component for curtains, blinds, and similar UI elements.
///
/// The rollo can be pulled down with mouse movement and optionally reset
/// with a separate control. It supports sound effects and animations for
/// both pulling and resetting actions.
///
/// # Behavior Modes
///
/// - **Bidirectional**: Can be pulled and pushed in both directions
/// - **Pull-only with reset**: Can only be pulled down, requires reset button to retract
///
/// # Example
///
/// ```rust
/// let mut rollo = Rollo::builder("blind_animation", "pull_blind", None)
///     .mouse_factor(2.0)
///     .build();
///
/// // In your update loop:
/// rollo.tick();
/// ```
pub struct Rollo {
    cab_side: Option<CockpitSide>,

    pos_rollo: f32,

    mouse_factor: f32,

    key_draw: KeyEvent,
    key_reset: KeyEvent,

    rollo_anim: Animation,
    reset_anim: Animation,

    pull_loop_sound: Sound,
    pull_single_sound: Sound,
    pull_step_last: f32,
    pull_step_width: f32,
    reset_sound: Sound,

    reset_flag: bool,
    only_pull: bool,
}

impl Rollo {
    /// Creates a new [`RolloBuilder`] with the specified core properties.
    ///
    /// # Arguments
    ///
    /// * `animation_name` - Name of the main rollo animation
    /// * `event_name` - Name of the key event for pulling the rollo
    /// * `cab_side` - Optional cabinet side specification
    ///
    /// # Example
    ///
    /// ```rust
    /// let builder = Rollo::builder("rollo_animation", "pull_event", Some(CockpitSide::ACab));
    /// ```
    pub fn builder(
        animation_name: impl Into<String>,
        event_name: &str,
        cab_side: Option<CockpitSide>,
    ) -> RolloBuilder {
        RolloBuilder {
            cab_side,
            pos_rollo: 0.0,
            mouse_factor: 1.0,
            rollo_anim: Animation::new(Some(&animation_name.into())),
            pull_loop_sound: Sound::new_simple(None),
            pull_single_sound: Sound::new_simple(None),
            pull_step_last: 0.0,
            pull_step_width: 0.0,
            reset_sound: Sound::new_simple(None),
            key_draw: KeyEvent::new(Some(event_name), cab_side),
            reset_flag: false,
            reset_anim: Animation::new(None),
            key_reset: KeyEvent::new(None, None),
            only_pull: false,
        }
    }

    /// Updates the rollo state for one frame.
    ///
    /// This method should be called once per frame to update the rollo's
    /// position based on user input and handle sound effects.
    ///
    /// The update process:
    /// 1. Handle reset button (if configured)
    /// 2. Process mouse input for pulling/pushing
    /// 3. Apply automatic reset (if enabled)
    /// 4. Update sound effects based on movement
    /// 5. Update animations
    pub fn tick(&mut self) {
        let rollo_last = self.pos_rollo;

        if self.key_reset.is_just_pressed() && self.only_pull {
            self.reset_flag = true;
            self.reset_sound.start();
        }

        if self.key_draw.is_pressed() {
            let hand_delta = mouse_move().y * self.mouse_factor;

            if self.only_pull {
                self.pos_rollo = (self.pos_rollo + (hand_delta.max(0.0))).clamp(0.0, 1.0);
            } else {
                self.pos_rollo = (self.pos_rollo + hand_delta).clamp(0.0, 1.0);
            }
        }

        if self.reset_flag {
            self.pos_rollo = (self.pos_rollo - 3.0 * delta()).clamp(0.0, 1.0);
        }

        if self.pos_rollo <= 0.0 {
            self.reset_flag = false;
        }

        if self.pull_step_width < (self.pull_step_last - self.pos_rollo).abs() {
            self.pull_step_last = self.pos_rollo;
            if !self.reset_flag {
                self.pull_single_sound.start();
            }
        }

        self.pull_loop_sound.start_stop(rollo_last < self.pos_rollo);

        self.rollo_anim.set(self.pos_rollo);
        self.reset_anim
            .set(self.key_reset.is_pressed() as u8 as f32);
    }
}
