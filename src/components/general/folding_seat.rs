//! # Folding Seat Module
//!
//! This module provides a physics-based folding seat simulation with spring mechanics,
//! mouse interaction, sound effects, and animation support. The seat can be configured
//! to fold up, down, or in a random direction with customizable physics parameters.

use lotus_script::rand::gen_f64;
use lotus_script::time::delta;

use crate::api::{
    animation::Animation,
    general::mouse_move,
    key_event::{KeyEvent, KeyEventCab},
    sound::Sound,
};

/// Builder for creating a `FoldingSeat` with customizable parameters.
///
/// This builder allows you to configure the physics properties, mouse interaction,
/// sound effects, and other parameters of a folding seat before creating the final instance.
///
/// # Examples
///
/// ```rust
/// use your_crate::FoldingSeat;
///
/// let seat = FoldingSeat::builder("seat_animation", "grab_key", None)
///     .spring_up(5.0)
///     .friction(0.8)
///     .mouse_factor(2.0)
///     .snd_upper_end("seat_up_sound")
///     .snd_lower_end("seat_down_sound")
///     .build();
/// ```
pub struct FoldingSeatBuilder {
    /// Current position of the seat (0.0 = fully down, 1.0 = fully up)
    pos: f32,
    /// Spring force applied to the seat (positive = up, negative = down)
    force: f32,
    /// Friction coefficient to slow down movement
    friction: f32,
    /// Factor for bouncing when hitting boundaries
    bump_factor: f32,
    /// Current speed of the seat movement
    speed: f32,
    /// Multiplier for mouse interaction sensitivity
    mouse_factor: f32,
    /// Sound played when seat reaches upper end
    snd_upper_end: Sound,
    /// Sound played when seat reaches lower end
    snd_lower_end: Sound,
    /// Key event for grabbing/controlling the seat
    key_grab: KeyEvent,
    /// Animation controller for visual representation
    animation: Animation,
}

impl FoldingSeatBuilder {
    /// Sets the seat to spring upward with the specified force.
    ///
    /// # Arguments
    ///
    /// * `force` - The upward spring force (will be made positive automatically)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = builder.spring_up(10.0);
    /// ```
    pub fn spring_up(mut self, force: f32) -> Self {
        self.force = force.abs();
        self
    }

    /// Sets the seat to spring downward with the specified force.
    ///
    /// # Arguments
    ///
    /// * `force` - The downward spring force (will be made negative automatically)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = builder.spring_down(8.0);
    /// ```
    pub fn spring_down(mut self, force: f32) -> Self {
        self.force = -force.abs();
        self
    }

    /// Sets the seat to spring in a random direction with the specified force.
    ///
    /// The direction is randomly chosen, and the initial position is set accordingly
    /// (1.0 for upward force, 0.0 for downward force).
    ///
    /// # Arguments
    ///
    /// * `force` - The spring force magnitude
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = builder.spring_random(7.5);
    /// ```
    pub fn spring_random(mut self, force: f32) -> Self {
        self.force = if gen_f64() > 0.5 {
            -force.abs()
        } else {
            force.abs()
        };
        self.pos = if self.force > 0.0 { 1.0 } else { 0.0 };
        self
    }

    /// Sets the friction coefficient for the seat movement.
    ///
    /// Higher values create more resistance to movement.
    ///
    /// # Arguments
    ///
    /// * `friction` - The friction coefficient (typically 0.0 to 1.0)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = builder.friction(0.5);
    /// ```
    pub fn friction(mut self, friction: f32) -> Self {
        self.friction = friction;
        self
    }

    /// Sets the bump factor for bouncing when the seat hits boundaries.
    ///
    /// # Arguments
    ///
    /// * `bump_factor` - Factor determining bounce intensity (0.0 = no bounce, 1.0 = full bounce)
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = builder.bump_factor(0.3);
    /// ```
    pub fn bump_factor(mut self, bump_factor: f32) -> Self {
        self.bump_factor = bump_factor;
        self
    }

    /// Sets the mouse interaction sensitivity factor.
    ///
    /// Higher values make the seat more responsive to mouse movement.
    ///
    /// # Arguments
    ///
    /// * `mouse_factor` - Sensitivity multiplier for mouse interaction
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = builder.mouse_factor(1.5);
    /// ```
    pub fn mouse_factor(mut self, mouse_factor: f32) -> Self {
        self.mouse_factor = mouse_factor;
        self
    }

    /// Sets the sound to play when the seat reaches the upper end.
    ///
    /// # Arguments
    ///
    /// * `name` - Name/path of the sound file
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = builder.snd_upper_end("seat_up.wav");
    /// ```
    pub fn snd_upper_end(mut self, name: impl Into<String>) -> Self {
        self.snd_upper_end = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Sets the sound to play when the seat reaches the lower end.
    ///
    /// # Arguments
    ///
    /// * `name` - Name/path of the sound file
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = builder.snd_lower_end("seat_down.wav");
    /// ```
    pub fn snd_lower_end(mut self, name: impl Into<String>) -> Self {
        self.snd_lower_end = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Creates the final `FoldingSeat` instance with all configured parameters.
    ///
    /// # Returns
    ///
    /// A fully configured `FoldingSeat` ready for use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let seat = builder.build();
    /// ```
    pub fn build(self) -> FoldingSeat {
        FoldingSeat {
            pos: self.pos,
            force: self.force,
            friction: self.friction,
            bump_factor: self.bump_factor,
            speed: self.speed,
            mouse_factor: self.mouse_factor,
            snd_upper_end: self.snd_upper_end,
            snd_lower_end: self.snd_lower_end,
            key_grab: self.key_grab,
            animation: self.animation,
        }
    }
}

/// A physics-based folding seat simulation with spring mechanics and user interaction.
///
/// The `FoldingSeat` provides realistic folding seat behavior with:
/// - Spring-based physics simulation
/// - Mouse interaction for manual control
/// - Sound effects for boundary hits
/// - Animation integration
/// - Configurable physics parameters
///
/// # Examples
///
/// ```rust
/// use your_crate::FoldingSeat;
///
/// // Create a seat that springs upward
/// let mut seat = FoldingSeat::builder("seat_anim", "grab_key", None)
///     .spring_up(5.0)
///     .friction(0.8)
///     .build();
///
/// // Update the seat physics each frame
/// loop {
///     seat.tick();
///     // ... render seat at current position
/// }
/// ```
#[derive(Debug)]
pub struct FoldingSeat {
    /// Current position of the seat (0.0 = fully down, 1.0 = fully up)
    pos: f32,
    /// Spring force applied to the seat (positive = up, negative = down)
    force: f32,
    /// Friction coefficient to slow down movement
    friction: f32,
    /// Factor for bouncing when hitting boundaries
    bump_factor: f32,
    /// Current speed of the seat movement
    speed: f32,
    /// Multiplier for mouse interaction sensitivity
    mouse_factor: f32,
    /// Sound played when seat reaches upper end
    snd_upper_end: Sound,
    /// Sound played when seat reaches lower end
    snd_lower_end: Sound,
    /// Key event for grabbing/controlling the seat
    key_grab: KeyEvent,
    /// Animation controller for visual representation
    animation: Animation,
}

impl FoldingSeat {
    /// Creates a new builder for configuring a folding seat.
    ///
    /// # Arguments
    ///
    /// * `animation_name` - Name of the animation to control
    /// * `event_name` - Name of the key event for grabbing the seat
    /// * `cab_side` - Optional cab side specification for the key event
    ///
    /// # Returns
    ///
    /// A `FoldingSeatBuilder` with default values that can be further configured.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let builder = FoldingSeat::builder("seat_animation", "grab_seat", None);
    /// ```
    pub fn builder(
        animation_name: impl Into<String>,
        event_name: &str,
        cab_side: Option<KeyEventCab>,
    ) -> FoldingSeatBuilder {
        FoldingSeatBuilder {
            pos: 0.0,
            force: 0.0,
            friction: 0.0,
            bump_factor: 0.0,
            speed: 0.0,
            mouse_factor: 1.0,
            snd_upper_end: Sound::new_simple(None),
            snd_lower_end: Sound::new_simple(None),
            key_grab: KeyEvent::new(Some(event_name), cab_side),
            animation: Animation::new(Some(&animation_name.into())),
        }
    }

    /// Updates the seat physics simulation for one frame.
    ///
    /// This method should be called once per frame to update the seat's position,
    /// handle user input, apply physics forces, and trigger sound effects.
    ///
    /// The behavior depends on the current state:
    /// - If the grab key is pressed: Manual control via mouse movement
    /// - If spring force is positive: Seat moves upward with physics
    /// - If spring force is negative: Seat moves downward with physics
    /// - If no force: Seat maintains current position
    ///
    /// # Examples
    ///
    /// ```rust
    /// // In your main game loop
    /// seat.tick();
    /// ```
    pub fn tick(&mut self) {
        if self.key_grab.is_pressed() {
            // Manual control mode: user is grabbing the seat
            let hand_delta = mouse_move().y * self.mouse_factor;
            self.pos = (self.pos - hand_delta).clamp(0.0, 1.0);
            self.speed = 0.0;
        } else if self.force > 0.0 {
            // Upward spring force
            if self.pos > 0.01 {
                self.pos += self.speed * delta();
            }

            // Handle upper boundary collision
            if self.pos > 1.0 {
                self.pos = 1.0;
                self.snd_upper_end.start();
                self.speed = if self.bump_factor > 0.0 {
                    -self.bump_factor * self.speed
                } else {
                    0.0
                };
            }

            self.speed += self.force * delta();

            // Snap to exact position near boundary
            if self.pos > 0.999 {
                self.pos = 1.0;
            }
        } else if self.force < 0.0 {
            // Downward spring force
            if self.pos < 0.99 {
                self.pos -= self.speed * delta();
            }

            // Handle lower boundary collision
            if self.pos < 0.0 {
                self.pos = 0.0;
                self.snd_lower_end.start();
                self.speed = if self.bump_factor > 0.0 {
                    -self.bump_factor * self.speed
                } else {
                    0.0
                };
            }

            self.speed -= self.force * delta();

            // Snap to exact position near boundary
            if self.pos < 0.001 {
                self.pos = 0.0;
            }
        } else {
            // No force applied: maintain current position
            self.pos = self.pos.clamp(0.0, 1.0);
        }

        // Update animation with current position
        self.animation.set(self.pos);
    }
}
