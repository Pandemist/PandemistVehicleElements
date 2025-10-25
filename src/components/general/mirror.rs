//! Outside mirror control system for vehicle simulation
//!
//! This module provides functionality for controlling outside mirrors in a vehicle simulation,
//! including both manual (mouse/keyboard) and electric control modes for mirror positioning
//! and arm movement.

use lotus_extra::vehicle::CockpitSide;
use lotus_script::{math::Vec2, time::delta};

use crate::{
    api::{animation::Animation, general::mouse_move, key_event::KeyEvent, sound::Sound},
    management::structs::general_structs::FourDirections,
};

/// Builder for configuring an outside mirror system
///
/// This builder allows you to configure all aspects of an outside mirror,
/// including arm movement, mirror positioning, input handling, and sound effects.
/// Use the builder pattern to set up your mirror configuration before calling `build()`.
///
/// # Example
///
/// ```rust
/// let mirror = OutsideMirror::builder("mirror_x_anim", "mirror_y_anim", Some(cab_side))
///     .add_mirror_arm("arm_animation")
///     .open_speed(2.0)
///     .close_speed(-2.0)
///     .snd_open("mirror_open_sound")
///     .snd_close("mirror_close_sound")
///     .init_arm(false)
///     .mirror_movement_border(Vec2 { x: -10.0, y: -10.0 }, Vec2 { x: 10.0, y: 10.0 })
///     .mirror_speed(Vec2 { x: 5.0, y: 5.0 })
///     .init_pos(Vec2 { x: 0.0, y: 0.0 })
///     .build();
/// ```
pub struct OutsideMirrorBuilder {
    // General ======================
    /// The cab side this mirror belongs to (left/right)
    cab_side: Option<CockpitSide>,

    // Mirror arm ===================
    /// Current position of the mirror arm (0.0 = closed, 1.0 = open)
    pos_arm: f32,
    /// Animation controller for the mirror arm
    pos_arm_anim: Animation,

    // Mirror arm manually ----------
    /// Mouse sensitivity factor for manual arm control
    mouse_factor_arm: f32,
    /// Key event handler for manual arm control
    key_arm: KeyEvent,

    // Mirror arm el ----------------
    /// Speed at which the arm opens (positive value)
    open_speed: f32,
    /// Speed at which the arm closes (negative value)
    close_speed: f32,
    /// Current state of the arm (true = open, false = closed)
    arm_state: bool,
    /// Target state for the arm
    arm_target: bool,
    /// Previous target state (used for sound triggering)
    arm_target_last: bool,
    /// Sound played when arm opens
    snd_open: Sound,
    /// Sound played when arm closes
    snd_close: Sound,

    // Mirror =======================
    /// Current X position of the mirror
    pos_x: f32,
    /// Current Y position of the mirror
    pos_y: f32,
    /// Animation controller for X position
    pos_x_anim: Animation,
    /// Animation controller for Y position
    pos_y_anim: Animation,

    // Mirror movement in general ---
    /// First boundary point for mirror movement area
    mirror_border_1: Vec2,
    /// Second boundary point for mirror movement area
    mirror_border_2: Vec2,

    /// Variance from border 1 for electric movement limits
    mirror_variance_1: Vec2,
    /// Variance from border 2 for electric movement limits
    mirror_variance_2: Vec2,

    // Mirror manual movement -------
    /// Mouse sensitivity for manual mirror control
    mouse_factor_mirror: Vec2,
    /// Key event handler for grabbing the mirror for manual control
    key_grab: KeyEvent,

    // Mirror movement el -----------
    /// Target direction for electric mirror movement
    mirror_target: FourDirections,

    /// Speed of electric mirror movement
    mirror_speed: Vec2,
    /// Sound played during mirror movement
    snd_move: Sound,
    /// Sound played when mirror reaches movement limit
    snd_move_end: Sound,
    // ==============================
}

impl OutsideMirrorBuilder {
    /// Configure the mirror arm with an animation
    ///
    /// # Arguments
    /// * `animation_name` - Name of the animation to use for the mirror arm
    ///
    /// # Returns
    /// Updated builder instance
    pub fn add_mirror_arm(mut self, animation_name: impl Into<String>) -> Self {
        self.pos_arm_anim = Animation::new(Some(&animation_name.into()));
        self
    }

    /// Set the mouse sensitivity factor for manual arm control
    ///
    /// # Arguments
    /// * `value` - Sensitivity multiplier for mouse movement affecting arm position
    ///
    /// # Returns
    /// Updated builder instance
    pub fn mouse_factor_arm(mut self, value: f32) -> Self {
        self.mouse_factor_arm = value;
        self
    }

    /// Configure the key event for manual arm control
    ///
    /// # Arguments
    /// * `name` - Name of the key event to bind to arm control
    ///
    /// # Returns
    /// Updated builder instance
    pub fn keyevent_arm(mut self, name: impl Into<String>) -> Self {
        self.key_arm = KeyEvent::new(Some(&name.into()), self.cab_side);
        self
    }

    /// Set the speed at which the mirror arm opens
    ///
    /// # Arguments
    /// * `value` - Opening speed (should be positive)
    ///
    /// # Returns
    /// Updated builder instance
    pub fn open_speed(mut self, value: f32) -> Self {
        self.open_speed = value;
        self
    }

    /// Set the speed at which the mirror arm closes
    ///
    /// # Arguments
    /// * `value` - Closing speed (should be negative)
    ///
    /// # Returns
    /// Updated builder instance
    pub fn close_speed(mut self, value: f32) -> Self {
        self.close_speed = value;
        self
    }

    /// Configure the sound played when the arm opens
    ///
    /// # Arguments
    /// * `name` - Name of the sound file/resource
    ///
    /// # Returns
    /// Updated builder instance
    pub fn snd_open(mut self, name: impl Into<String>) -> Self {
        self.snd_open = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Configure the sound played when the arm closes
    ///
    /// # Arguments
    /// * `name` - Name of the sound file/resource
    ///
    /// # Returns
    /// Updated builder instance
    pub fn snd_close(mut self, name: impl Into<String>) -> Self {
        self.snd_close = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Initialize the arm position and state
    ///
    /// # Arguments
    /// * `target` - Initial state of the arm (true = open, false = closed)
    ///
    /// # Returns
    /// Updated builder instance
    pub fn init_arm(mut self, target: bool) -> Self {
        self.pos_arm = target as u8 as f32;
        self.pos_arm_anim.set(self.pos_arm);

        self.arm_state = target;
        self.arm_target = target;
        self.arm_target_last = target;
        self
    }

    /// Set the movement boundaries for the mirror
    ///
    /// # Arguments
    /// * `p1` - First boundary point (typically top-left or minimum bounds)
    /// * `p2` - Second boundary point (typically bottom-right or maximum bounds)
    ///
    /// # Returns
    /// Updated builder instance
    pub fn mirror_movement_border(mut self, p1: Vec2, p2: Vec2) -> Self {
        self.mirror_border_1 = p1;
        self.mirror_border_2 = p2;
        self
    }

    /// Configure the key event for manual mirror control
    ///
    /// # Arguments
    /// * `name` - Name of the key event to bind to mirror grabbing
    ///
    /// # Returns
    /// Updated builder instance
    pub fn keyevent_mirror(mut self, name: impl Into<String>) -> Self {
        self.key_grab = KeyEvent::new(Some(&name.into()), self.cab_side);
        self
    }

    /// Set the mouse sensitivity for manual mirror control
    ///
    /// # Arguments
    /// * `value` - 2D sensitivity vector for X and Y mouse movement
    ///
    /// # Returns
    /// Updated builder instance
    pub fn mouse_factor_mirror(mut self, value: Vec2) -> Self {
        self.mouse_factor_mirror = value;
        self
    }

    /// Set variance values for electric movement limits
    ///
    /// These values define how close to the borders the electric movement
    /// should stop, creating a buffer zone.
    ///
    /// # Arguments
    /// * `v1` - Variance from border 1
    /// * `v2` - Variance from border 2
    ///
    /// # Returns
    /// Updated builder instance
    pub fn mirror_movement_variance(mut self, v1: Vec2, v2: Vec2) -> Self {
        self.mirror_variance_1 = v1;
        self.mirror_variance_2 = v2;
        self
    }

    /// Set the speed of electric mirror movement
    ///
    /// # Arguments
    /// * `value` - 2D speed vector for X and Y movement
    ///
    /// # Returns
    /// Updated builder instance
    pub fn mirror_speed(mut self, value: Vec2) -> Self {
        self.mirror_speed = value;
        self
    }

    /// Configure the sound played during mirror movement
    ///
    /// # Arguments
    /// * `name` - Name of the sound file/resource
    ///
    /// # Returns
    /// Updated builder instance
    pub fn snd_move(mut self, name: impl Into<String>) -> Self {
        self.snd_move = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Configure the sound played when mirror reaches movement limit
    ///
    /// # Arguments
    /// * `name` - Name of the sound file/resource
    ///
    /// # Returns
    /// Updated builder instance
    pub fn snd_move_end(mut self, name: impl Into<String>) -> Self {
        self.snd_move_end = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Initialize the mirror position
    ///
    /// # Arguments
    /// * `pos` - Initial 2D position of the mirror
    ///
    /// # Returns
    /// Updated builder instance
    pub fn init_pos(mut self, pos: Vec2) -> Self {
        self.pos_x = pos.x;
        self.pos_y = pos.y;
        self.pos_x_anim.set(self.pos_x);
        self.pos_y_anim.set(self.pos_y);
        self
    }

    /// Build the final OutsideMirror instance
    ///
    /// Consumes the builder and returns a configured OutsideMirror ready for use.
    ///
    /// # Returns
    /// Configured OutsideMirror instance
    pub fn build(self) -> OutsideMirror {
        OutsideMirror {
            cab_side: self.cab_side,
            pos_arm: self.pos_arm,
            pos_arm_anim: self.pos_arm_anim,
            mouse_factor_arm: self.mouse_factor_arm,
            key_arm: self.key_arm,
            open_speed: self.open_speed,
            close_speed: self.close_speed,
            arm_state: self.arm_state,
            arm_target: self.arm_target,
            arm_target_last: self.arm_target_last,
            snd_open: self.snd_open,
            snd_close: self.snd_close,
            pos_x: self.pos_x,
            pos_y: self.pos_y,
            pos_x_anim: self.pos_x_anim,
            pos_y_anim: self.pos_y_anim,
            mirror_border_1: self.mirror_border_1,
            mirror_border_2: self.mirror_border_2,
            mirror_variance_1: self.mirror_variance_1,
            mirror_variance_2: self.mirror_variance_2,
            mouse_factor_mirror: self.mouse_factor_mirror,
            key_grab: self.key_grab,
            mirror_target: self.mirror_target,
            mirror_speed: self.mirror_speed,
            snd_move: self.snd_move,
            snd_move_end: self.snd_move_end,
        }
    }
}

/// Main outside mirror control system
///
/// This struct represents a complete outside mirror system that can be controlled
/// both manually (via mouse and keyboard input) and electrically (via voltage and
/// programmatic control). It handles mirror arm folding/unfolding and mirror
/// positioning with smooth animations and sound effects.
///
/// The mirror system supports:
/// - Manual control via mouse and keyboard
/// - Electric control via voltage input
/// - Smooth animations for all movements
/// - Sound effects for different actions
/// - Boundary checking and movement limits
///
/// # Usage
///
/// Create an instance using the builder pattern, then call `tick()` in your main loop:
///
/// ```rust
/// let mut mirror = OutsideMirror::builder("x_anim", "y_anim", Some(cab_side))
///     .add_mirror_arm("arm_anim")
///     .open_speed(2.0)
///     .close_speed(-2.0)
///     .build();
///
/// // In your main loop:
/// mirror.tick(12.0); // Pass current voltage
/// ```
pub struct OutsideMirror {
    // General ======================
    /// The cab side this mirror belongs to
    cab_side: Option<CockpitSide>,

    // Mirror arm ===================
    /// Current position of the mirror arm (0.0 = closed, 1.0 = open)
    pos_arm: f32,
    /// Animation controller for the mirror arm
    pos_arm_anim: Animation,

    // Mirror arm manually ----------
    /// Mouse sensitivity factor for manual arm control
    mouse_factor_arm: f32,
    /// Key event handler for manual arm control
    key_arm: KeyEvent,

    // Mirror arm el ----------------
    /// Speed at which the arm opens
    open_speed: f32,
    /// Speed at which the arm closes
    close_speed: f32,
    /// Current state of the arm
    arm_state: bool,
    /// Target state for the arm (publicly accessible for external control)
    pub arm_target: bool,
    /// Previous target state (used for sound triggering)
    arm_target_last: bool,
    /// Sound played when arm opens
    snd_open: Sound,
    /// Sound played when arm closes
    snd_close: Sound,

    // Mirror =======================
    /// Current X position of the mirror
    pos_x: f32,
    /// Current Y position of the mirror
    pos_y: f32,
    /// Animation controller for X position
    pos_x_anim: Animation,
    /// Animation controller for Y position
    pos_y_anim: Animation,

    // Mirror movement in general ---
    /// First boundary point for mirror movement area
    mirror_border_1: Vec2,
    /// Second boundary point for mirror movement area
    mirror_border_2: Vec2,

    /// Variance from border 1 for electric movement limits
    mirror_variance_1: Vec2,
    /// Variance from border 2 for electric movement limits
    mirror_variance_2: Vec2,

    // Mirror manual movement -------
    /// Mouse sensitivity for manual mirror control
    mouse_factor_mirror: Vec2,
    /// Key event handler for grabbing the mirror
    key_grab: KeyEvent,

    // Mirror movement el -----------
    /// Target direction for electric mirror movement (publicly accessible)
    pub mirror_target: FourDirections,

    /// Speed of electric mirror movement
    mirror_speed: Vec2,
    /// Sound played during mirror movement
    snd_move: Sound,
    /// Sound played when mirror reaches movement limit
    snd_move_end: Sound,
    // ==============================
}

impl OutsideMirror {
    /// Create a new builder for configuring an outside mirror
    ///
    /// # Arguments
    /// * `anim_x_name` - Name of the animation for X-axis movement
    /// * `anim_y_name` - Name of the animation for Y-axis movement  
    /// * `cab_side` - Optional cab side specification for input handling
    ///
    /// # Returns
    /// A new OutsideMirrorBuilder instance ready for configuration
    ///
    /// # Example
    ///
    /// ```rust
    /// let builder = OutsideMirror::builder("mirror_x", "mirror_y", Some(CockpitSide::Left));
    /// ```
    pub fn builder(
        anim_x_name: impl Into<String>,
        anim_y_name: impl Into<String>,
        cab_side: Option<CockpitSide>,
    ) -> OutsideMirrorBuilder {
        OutsideMirrorBuilder {
            // General ======================
            cab_side,

            // Mirror arm ===================
            pos_arm: 0.0,
            pos_arm_anim: Animation::new(None),

            // Mirror arm manually ----------
            mouse_factor_arm: 0.0,
            key_arm: KeyEvent::new(None, None),

            // Mirror arm el ----------------
            open_speed: 0.0,
            close_speed: 0.0,
            arm_state: false,
            arm_target: false,
            arm_target_last: false,
            snd_open: Sound::new_simple(None),
            snd_close: Sound::new_simple(None),

            // Mirror =======================
            pos_x: 0.0,
            pos_y: 0.0,
            pos_x_anim: Animation::new(Some(&anim_x_name.into())),
            pos_y_anim: Animation::new(Some(&anim_y_name.into())),

            // Mirror movement in general ---
            mirror_border_1: Vec2 { x: 0.0, y: 0.0 },
            mirror_border_2: Vec2 { x: 0.0, y: 0.0 },

            // Mirror manual movement -------
            mouse_factor_mirror: Vec2 { x: 0.0, y: 0.0 },
            key_grab: KeyEvent::new(None, None),

            // Mirror movement el -----------
            mirror_target: FourDirections::default(),

            mirror_variance_1: Vec2 { x: 0.0, y: 0.0 },
            mirror_variance_2: Vec2 { x: 0.0, y: 0.0 },

            mirror_speed: Vec2 { x: 0.0, y: 0.0 },
            snd_move: Sound::new_simple(None),
            snd_move_end: Sound::new_simple(None),
            // ==============================
        }
    }

    /// Update the mirror system for one frame
    ///
    /// This method should be called once per frame in your main game loop.
    /// It handles all mirror logic including:
    /// - Manual control input processing
    /// - Electric control when voltage is sufficient
    /// - Animation updates
    /// - Sound effect triggering
    /// - Boundary checking
    ///
    /// # Arguments
    /// * `voltage` - Current electrical voltage (must be > 0.25 for electric functions)
    ///
    /// # Details
    ///
    /// ## Manual Control
    /// - Mirror arm: Hold the configured key and move mouse horizontally
    /// - Mirror position: Hold the grab key and move mouse in any direction
    ///
    /// ## Electric Control  
    /// - Mirror arm: Set `arm_target` field and ensure voltage > 0.25
    /// - Mirror position: Set direction flags in `mirror_target` field
    ///
    /// ## Voltage Requirements
    /// Electric functions only work when voltage > 0.25, simulating realistic
    /// electrical system behavior where insufficient power disables motors.
    pub fn tick(&mut self, voltage: f32) {
        // Mirror arm (hand)
        if self.key_arm.is_pressed() {
            let hand_delta = mouse_move().x * self.mouse_factor_arm;
            self.pos_arm = (self.pos_arm + hand_delta).clamp(0.0, 1.0);
            self.pos_arm_anim.set(self.pos_arm);
        }

        // Mirror arm (electric)
        if voltage > 0.25 && self.arm_target != self.arm_state {
            match (self.arm_target, self.arm_state) {
                (false, true) => {
                    // Closing the arm
                    if self.arm_target_last {
                        self.snd_close.start();
                    }
                    self.pos_arm = (self.pos_arm - self.close_speed * delta()).max(0.0);
                    self.pos_arm_anim.set(self.pos_arm);

                    if self.pos_arm <= 0.0 {
                        self.arm_state = false;
                    }
                }
                (true, false) => {
                    // Opening the arm
                    if !self.arm_target_last {
                        self.snd_open.start();
                    }
                    self.pos_arm = (self.pos_arm + self.open_speed * delta()).min(1.0);
                    self.pos_arm_anim.set(self.pos_arm);

                    if self.pos_arm >= 1.0 {
                        self.arm_state = true;
                    }
                }
                (_, _) => {}
            }
            self.arm_target_last = self.arm_target;
        }

        // Mirror (hand)
        if self.key_grab.is_pressed() {
            self.pos_x = (self.pos_x + (mouse_move().x * self.mouse_factor_mirror.x))
                .min(self.mirror_border_1.x)
                .max(self.mirror_border_2.x);
            self.pos_y = (self.pos_y + (mouse_move().y * self.mouse_factor_mirror.y))
                .min(self.mirror_border_1.y)
                .max(self.mirror_border_2.y);
            self.pos_x_anim.set(self.pos_x);
            self.pos_y_anim.set(self.pos_y);
        }

        // Mirror (electric)
        if voltage > 0.25 {
            if self.mirror_target.up {
                self.pos_y += self.mirror_speed.y * delta();

                if self.pos_y < self.mirror_border_1.y + self.mirror_variance_1.y {
                    self.pos_y = self.mirror_border_1.y;
                    self.snd_move_end.start();
                }
                self.pos_y_anim.set(self.pos_y);
            } else if self.mirror_target.down {
                self.pos_y -= self.mirror_speed.y * delta();

                if self.pos_y > self.mirror_border_2.y + self.mirror_variance_2.y {
                    self.pos_y = self.mirror_border_2.y;
                    self.snd_move_end.start();
                }
                self.pos_y_anim.set(self.pos_y);
            } else if self.mirror_target.left {
                self.pos_x -= self.mirror_speed.x * delta();

                if self.pos_x < self.mirror_border_1.x + self.mirror_variance_1.x {
                    self.pos_x = self.mirror_border_1.x;
                    self.snd_move_end.start();
                }
                self.pos_x_anim.set(self.pos_x);
            } else if self.mirror_target.right {
                self.pos_x += self.mirror_speed.x * delta();

                if self.pos_x > self.mirror_border_2.x + self.mirror_variance_2.x {
                    self.pos_x = self.mirror_border_2.x;
                    self.snd_move_end.start();
                }
                self.pos_x_anim.set(self.pos_x);
            }
        }

        // Control movement sound based on target state and voltage
        self.snd_move
            .start_stop(self.mirror_target.is_one() && voltage > 0.25);
    }
}
