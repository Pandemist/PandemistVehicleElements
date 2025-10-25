//! Speedometer component for animated gauge displays.
//!
//! This module provides a physics-based speedometer implementation with configurable
//! needle and arrow animations. The speedometer uses force and friction parameters
//! to create realistic movement dynamics.

use lotus_extra::math::PiecewiseLinearFunction;
use lotus_script::time::delta;

use crate::api::animation::Animation;

/// Builder for creating a `Speedometer` with customizable parameters.
///
/// The builder pattern allows for flexible configuration of the speedometer's
/// physical properties and animation settings before construction.
///
/// # Examples
///
/// ```rust
/// use pandemist_vehicle_elements::speedometer::Speedometer;
/// use pandemist_vehicle_elements::elements::std::piecewise_linear_function::PiecewiseLinearFunction;
///
/// let speedometer = Speedometer::builder("needle_animation")
///     .force(2.0)
///     .friction(0.8)
///     .add_arrow("arrow_animation")
///     .build();
/// ```
pub struct SpeedometerBuilder {
    needle_pos: f32,
    arrow_pos: f32,

    force: f32,
    friction: f32,

    needle_speed: f32,
    needle_acc: f32,

    arrow_speed: f32,
    arrow_acc: f32,

    needle_path: Option<PiecewiseLinearFunction>,
    arrow_path: Option<PiecewiseLinearFunction>,

    needle_pos_anim: Animation,
    arrow_pos_anim: Animation,
}

impl SpeedometerBuilder {
    /// Adds an arrow component to the speedometer with the specified animation name.
    ///
    /// # Parameters
    ///
    /// * `animation_name` - The name of the animation to use for the arrow
    ///
    /// # Returns
    ///
    /// Returns `self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = Speedometer::builder("needle")
    ///     .add_arrow("my_arrow_animation");
    /// ```
    pub fn add_arrow(mut self, animation_name: impl Into<String>) -> Self {
        self.arrow_pos_anim = Animation::new(Some(&animation_name.into()));
        self
    }

    /// Sets the friction coefficient for the speedometer physics.
    ///
    /// Friction affects how quickly the needle and arrow slow down when approaching
    /// their target positions. Higher values create more damping.
    ///
    /// # Parameters
    ///
    /// * `friction` - The friction coefficient (typically between 0.0 and 1.0)
    ///
    /// # Returns
    ///
    /// Returns `self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = Speedometer::builder("needle")
    ///     .friction(0.5); // Moderate friction
    /// ```
    pub fn friction(mut self, friction: f32) -> Self {
        self.friction = friction;
        self
    }

    /// Sets the force multiplier for the speedometer physics.
    ///
    /// Force determines how quickly the needle and arrow accelerate toward
    /// their target positions. Higher values result in faster movement.
    ///
    /// # Parameters
    ///
    /// * `force` - The force multiplier (positive values recommended)
    ///
    /// # Returns
    ///
    /// Returns `self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = Speedometer::builder("needle")
    ///     .force(2.0); // Strong force for quick response
    /// ```
    pub fn force(mut self, force: f32) -> Self {
        self.force = force;
        self
    }

    /// Sets a custom path function for the needle movement.
    ///
    /// The path function allows for non-linear mapping of the needle position,
    /// enabling curved or custom gauge layouts.
    ///
    /// # Parameters
    ///
    /// * `path` - A piecewise linear function defining the needle's movement path
    ///
    /// # Returns
    ///
    /// Returns `self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pandemist_vehicle_elements::elements::std::piecewise_linear_function::PiecewiseLinearFunction;
    ///
    /// let path = PiecewiseLinearFunction::new(/* path parameters */);
    /// let builder = Speedometer::builder("needle")
    ///     .needle_path(path);
    /// ```
    pub fn needle_path(mut self, path: PiecewiseLinearFunction) -> Self {
        self.needle_path = Some(path);
        self
    }

    /// Sets a custom path function for the arrow movement.
    ///
    /// Similar to needle_path, this allows for non-linear mapping of the arrow position.
    ///
    /// # Parameters
    ///
    /// * `path` - A piecewise linear function defining the arrow's movement path
    ///
    /// # Returns
    ///
    /// Returns `self` for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pandemist_vehicle_elements::elements::std::piecewise_linear_function::PiecewiseLinearFunction;
    ///
    /// let path = PiecewiseLinearFunction::new(/* path parameters */);
    /// let builder = Speedometer::builder("needle")
    ///     .arrow_path(path);
    /// ```
    pub fn arrow_path(mut self, path: PiecewiseLinearFunction) -> Self {
        self.arrow_path = Some(path);
        self
    }

    /// Constructs the final `Speedometer` instance with the configured parameters.
    ///
    /// # Returns
    ///
    /// A new `Speedometer` instance ready for use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let speedometer = Speedometer::builder("needle_animation")
    ///     .force(1.5)
    ///     .friction(0.7)
    ///     .build();
    /// ```
    pub fn build(self) -> Speedometer {
        Speedometer {
            needle_pos: self.needle_pos,
            arrow_pos: self.arrow_pos,

            force: self.force,
            friction: self.friction,

            needle_speed: self.needle_speed,
            needle_acc: self.needle_acc,

            arrow_speed: self.arrow_speed,
            arrow_acc: self.arrow_acc,

            needle_path: self.needle_path,
            arrow_path: self.arrow_path,

            needle_pos_anim: self.needle_pos_anim,
            arrow_pos_anim: self.arrow_pos_anim,
        }
    }
}

/// A physics-based speedometer component with needle and optional arrow.
///
/// The `Speedometer` provides realistic gauge movement using force and friction
/// physics simulation. It supports both a primary needle and an optional secondary
/// arrow, each with customizable animation paths.
///
/// # Physics Model
///
/// The speedometer uses a simple physics model where:
/// - Force determines how quickly elements accelerate toward targets
/// - Friction provides damping to prevent oscillation
/// - Positions, velocities, and accelerations are updated each frame
///
/// # Examples
///
/// ```rust
/// use pandemist_vehicle_elements::speedometer::Speedometer;
///
/// // Create a basic speedometer
/// let mut speedometer = Speedometer::builder("needle_anim")
///     .force(2.0)
///     .friction(0.8)
///     .build();
///
/// // Update the speedometer each frame
/// speedometer.tick(75.0, 50.0); // target needle: 75%, target arrow: 50%
/// ```
pub struct Speedometer {
    needle_pos: f32,
    arrow_pos: f32,

    force: f32,
    friction: f32,

    needle_speed: f32,
    needle_acc: f32,

    arrow_speed: f32,
    arrow_acc: f32,

    needle_path: Option<PiecewiseLinearFunction>,
    arrow_path: Option<PiecewiseLinearFunction>,

    needle_pos_anim: Animation,
    arrow_pos_anim: Animation,
}

impl Speedometer {
    /// Creates a new speedometer builder with the specified needle animation name.
    ///
    /// This is the entry point for creating a speedometer. The builder pattern
    /// allows for flexible configuration before final construction.
    ///
    /// # Parameters
    ///
    /// * `animation_name` - The name of the animation to use for the needle
    ///
    /// # Returns
    ///
    /// A new `SpeedometerBuilder` instance for configuration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let speedometer = Speedometer::builder("my_needle_animation")
    ///     .force(1.0)
    ///     .friction(0.5)
    ///     .build();
    /// ```
    pub fn builder(animation_name: impl Into<String>) -> SpeedometerBuilder {
        SpeedometerBuilder {
            needle_pos: 0.0,
            arrow_pos: 0.0,
            force: 0.0,
            friction: 0.0,
            needle_speed: 0.0,
            needle_acc: 0.0,
            arrow_speed: 0.0,
            arrow_acc: 0.0,
            needle_pos_anim: Animation::new(Some(&animation_name.into())),
            arrow_pos_anim: Animation::new(None),
            needle_path: None,
            arrow_path: None,
        }
    }

    /// Updates the speedometer physics simulation for one frame.
    ///
    /// This method should be called once per frame to update the positions of both
    /// the needle and arrow based on their target values and the configured physics
    /// parameters.
    ///
    /// # Parameters
    ///
    /// * `target_needle` - The target position for the needle (typically 0.0 to 100.0)
    /// * `target_arrow` - The target position for the arrow (typically 0.0 to 100.0)
    ///
    /// # Physics Behavior
    ///
    /// - Both needle and arrow use the same force and friction values
    /// - Positions are clamped to prevent negative values
    /// - Speed and acceleration are dampened when hitting the lower bound
    /// - Custom paths are applied if configured
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut speedometer = Speedometer::builder("needle").build();
    ///
    /// // In your game/animation loop:
    /// loop {
    ///     let engine_rpm = get_engine_rpm(); // e.g., 0-100
    ///     let fuel_level = get_fuel_level(); // e.g., 0-100
    ///     
    ///     speedometer.tick(engine_rpm, fuel_level);
    ///     
    ///     // Render the speedometer...
    /// }
    /// ```
    pub fn tick(&mut self, target_needle: f32, target_arrow: f32) {
        // Tachoneedle physics simulation
        let delta_value = (target_needle - self.needle_pos) * self.force;
        self.needle_acc = delta_value - (self.needle_speed * self.friction);

        self.needle_speed += self.needle_acc * delta();
        self.needle_pos += self.needle_speed * delta();

        // Prevent negative positions and dampen bouncing
        if self.needle_pos < 0.0 {
            self.needle_pos = 0.0;
            self.needle_speed *= -0.5;
            self.needle_acc *= 0.5;
        }

        // Apply custom path mapping if configured
        let new_pos = if let Some(ref mut path) = self.needle_path {
            path.get_value_or_default(self.needle_pos)
        } else {
            self.needle_pos
        };
        self.needle_pos_anim.set(new_pos);

        // Tachoarrow physics simulation (identical to needle)
        let delta_value = (target_arrow - self.arrow_pos) * self.force;
        self.arrow_acc = delta_value - (self.arrow_speed * self.friction);

        self.arrow_speed += self.arrow_acc * delta();
        self.arrow_pos += self.arrow_speed * delta();

        // Prevent negative positions and dampen bouncing
        if self.arrow_pos < 0.0 {
            self.arrow_pos = 0.0;
            self.arrow_speed *= -0.5;
            self.arrow_acc *= 0.5;
        }

        // Apply custom path mapping if configured
        let new_pos = if let Some(ref mut path) = self.arrow_path {
            path.get_value_or_default(self.arrow_pos)
        } else {
            self.arrow_pos
        };
        self.arrow_pos_anim.set(new_pos);
    }
}
