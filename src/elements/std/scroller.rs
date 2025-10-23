//! Animation utilities for smooth scrolling and pointer movement.
//!
//! This module provides two main components for creating smooth animations:
//! - [`Scroller`]: Linear interpolation-based scrolling with constant speed
//! - [`Pointer`]: Physics-based movement with force and friction simulation

use lotus_script::time::delta;

use crate::api::animation::Animation;

/// A linear interpolation-based scroller that smoothly moves towards a target position.
///
/// The `Scroller` provides constant-speed movement between positions, making it ideal
/// for UI scrolling animations where you want predictable, uniform motion.
///
/// # Examples
///
/// ```rust
/// use your_crate::Scroller;
///
/// let mut scroller = Scroller::new(0.0, 100.0, "scroll_animation");
///
/// // Set a target position
/// scroller.target = 500.0;
///
/// // Update in your game loop
/// loop {
///     scroller.tick();
///     // scroller.pos will smoothly move towards the target
/// }
/// ```
#[derive(Debug)]
pub struct Scroller {
    /// Current position of the scroller
    pub pos: f32,
    /// Target position the scroller is moving toward
    pub target: f32,

    /// Movement speed in units per second
    speed: f32,

    /// Animation object for external animation system integration
    pos_anim: Animation,
}

impl Scroller {
    /// Creates a new scroller with the specified initial position and movement speed.
    ///
    /// # Arguments
    ///
    /// * `init_pos` - Initial position of the scroller
    /// * `speed` - Movement speed in units per second
    /// * `anim_name` - Name for the animation object (for external system integration)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let scroller = Scroller::new(0.0, 150.0, "my_scroll_animation");
    /// ```
    pub fn new(init_pos: f32, speed: f32, anim_name: impl Into<String>) -> Self {
        Scroller {
            pos: init_pos,
            target: init_pos,
            speed,
            pos_anim: Animation::new(Some(&anim_name.into())),
        }
    }

    /// Updates the scroller position, moving it towards the target.
    ///
    /// This method should be called once per frame in your game loop.
    /// The scroller will move at the specified speed until it reaches the target position.
    ///
    /// The movement is frame-rate independent, using delta time for smooth animation
    /// regardless of frame rate variations.
    pub fn tick(&mut self) {
        if self.pos < self.target {
            self.pos = (self.pos + self.speed * delta()).min(self.target);
        }
        if self.pos > self.target {
            self.pos = (self.pos - self.speed * delta()).max(self.target);
        }

        self.pos_anim.set(self.pos);
    }
}

/// A physics-based pointer that simulates realistic movement with force and friction.
///
/// The `Pointer` uses a spring-like physics model to create natural, organic movement
/// patterns. It's ideal for cursor tracking, camera following, or any animation that
/// should feel responsive but not overly rigid.
///
/// # Physics Model
///
/// The pointer uses the following physics:
/// - **Force**: Attraction strength towards the target (higher = more responsive)
/// - **Friction**: Damping that slows down movement (higher = less oscillation)
/// - **Acceleration**: Calculated from force and friction
/// - **Velocity**: Integrated from acceleration over time
/// - **Position**: Integrated from velocity over time
///
/// # Examples
///
/// ```rust
/// use your_crate::Pointer;
///
/// // Higher force = more responsive, higher friction = less bouncy
/// let mut pointer = Pointer::new(5.0, 0.8, "cursor_follow");
///
/// // Update in your game loop with target position
/// loop {
///     let target_pos = get_mouse_position();
///     pointer.tick(target_pos);
///     // pointer.pos() will smoothly follow the target
/// }
/// ```
#[derive(Debug)]
pub struct Pointer {
    /// Current position of the pointer
    pos: f32,

    /// Force constant - how strongly the pointer is attracted to the target
    force: f32,
    /// Friction constant - how much the movement is damped
    friction: f32,

    /// Current velocity (speed and direction)
    speed: f32,
    /// Current acceleration
    acc: f32,

    /// Animation object for external animation system integration
    pos_anim: Animation,
}

impl Pointer {
    /// Creates a new physics-based pointer with specified force and friction.
    ///
    /// # Arguments
    ///
    /// * `force` - Attraction force towards target (typical range: 1.0-10.0)
    ///   - Higher values make the pointer more responsive
    ///   - Lower values create more sluggish movement
    /// * `friction` - Damping factor (typical range: 0.1-2.0)
    ///   - Higher values reduce oscillation and overshoot
    ///   - Lower values allow more bouncy, spring-like behavior
    /// * `anim_name` - Name for the animation object
    ///
    /// # Tuning Guidelines
    ///
    /// - **Responsive cursor**: `force: 8.0, friction: 1.2`
    /// - **Smooth camera**: `force: 3.0, friction: 0.8`
    /// - **Bouncy UI element**: `force: 5.0, friction: 0.3`
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Responsive pointer with minimal overshoot
    /// let cursor = Pointer::new(8.0, 1.2, "cursor");
    ///
    /// // Smooth, flowing movement
    /// let camera = Pointer::new(3.0, 0.8, "camera_follow");
    /// ```
    pub fn new(force: f32, friction: f32, anim_name: impl Into<String>) -> Self {
        Self {
            pos: 0.0,
            force,
            friction,
            speed: 0.0,
            acc: 0.0,
            pos_anim: Animation::new(Some(&anim_name.into())),
        }
    }

    /// Updates the pointer physics simulation towards the target position.
    ///
    /// This method should be called once per frame with the desired target position.
    /// The pointer will use physics simulation to smoothly move towards the target.
    ///
    /// # Arguments
    ///
    /// * `target` - The position the pointer should move towards
    ///
    /// # Physics Steps
    ///
    /// 1. Calculate force towards target: `(target - position) * force_constant`
    /// 2. Apply friction: `acceleration = force - (velocity * friction_constant)`
    /// 3. Integrate velocity: `velocity += acceleration * delta_time`
    /// 4. Integrate position: `position += velocity * delta_time`
    ///
    /// The simulation is frame-rate independent using delta time.
    pub fn tick(&mut self, target: f32) {
        let delta_value = (target - self.pos) * self.force;
        self.acc = delta_value - (self.speed * self.friction);

        self.speed += self.acc * delta();
        self.pos += self.speed * delta();

        self.pos_anim.set(self.pos);
    }
}
