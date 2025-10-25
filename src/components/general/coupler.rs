//! Coupler system implementation for train simulation.
//!
//! This module provides two types of couplers for train simulation:
//! - `SimpleCoupler`: A basic coupler with mechanical and electrical connection
//! - `HandCoupler`: A manual coupler with realistic physics and user interaction

use lotus_extra::vehicle::CockpitSide;
use lotus_script::{message::Coupling, rand::gen_f64, time::delta};

use crate::{
    api::{
        animation::Animation, coupler::ApiCoupler, general::mouse_move, key_event::KeyEvent,
        mock_enums::CouplingState, visible_flag::Visiblility,
    },
    elements::tech::{buttons::PushButton, switches::Switch},
    messages::gt6n_coupling_messages::send_bag,
};

/// A simple coupler implementation with basic mechanical and electrical coupling.
///
/// `SimpleCoupler` provides a straightforward coupler system that automatically
/// manages electrical parts based on mechanical coupling state. The electrical
/// connection is established when the mechanical coupling is secure.
///
/// # Examples
///
/// ```
/// use lotus_script::message::Coupling;
/// use lotus_extra::vehicle::CockpitSide;
///
/// let coupler = SimpleCoupler::new(
///     CockpitSide::A,
///     Coupling::Front,
///     "electric_parts_anim"
/// );
/// ```
#[derive(Debug)]
pub struct SimpleCoupler {
    /// API interface for coupler operations
    api_coupler: ApiCoupler,
    /// Animation controller for electrical parts
    electric_parts_anim: Animation,
}

impl SimpleCoupler {
    /// Creates a new `SimpleCoupler` instance.
    ///
    /// # Parameters
    ///
    /// * `cab_side` - The cab side this coupler is associated with
    /// * `coupler` - The coupling type (Front/Rear)
    /// * `electric_parts_animation_name` - Name of the animation for electrical parts
    ///
    /// # Returns
    ///
    /// A new `SimpleCoupler` instance ready for use.
    pub fn new(coupler: Coupling, electric_parts_animation_name: &str) -> Self {
        SimpleCoupler {
            api_coupler: ApiCoupler::new(coupler),
            electric_parts_anim: Animation::new(Some(electric_parts_animation_name)),
        }
    }

    /// Updates the coupler state per simulation tick.
    ///
    /// This method should be called every simulation frame to update
    /// the electrical parts animation based on the mechanical coupling state.
    /// When mechanically coupled, the electrical parts animation is set to 1.0,
    /// otherwise to 0.0.
    pub fn tick(&mut self) {
        let electric_parts = if self.mech_coupled() { 1.0 } else { 0.0 };
        self.electric_parts_anim.set(electric_parts);
    }

    /// Checks if the coupler is mechanically coupled.
    ///
    /// # Returns
    ///
    /// `true` if the coupler is mechanically connected to another vehicle,
    /// `false` otherwise.
    pub fn mech_coupled(&self) -> bool {
        self.api_coupler.is_coupled()
    }

    /// Checks if the coupler is electrically coupled.
    ///
    /// Electrical coupling requires both mechanical coupling and the electrical
    /// parts animation to be in position (> 0.8).
    ///
    /// # Returns
    ///
    /// `true` if both mechanical and electrical connections are established,
    /// `false` otherwise.
    pub fn el_coupled(&self) -> bool {
        self.mech_coupled() && (self.electric_parts_anim.pos > 0.8)
    }
}

//======================================================================

/// A manual coupler with realistic physics simulation and user interaction.
///
/// `HandCoupler` provides a detailed simulation of a manual train coupler system
/// with the following features:
/// - Physics-based movement with friction and momentum
/// - Mouse/keyboard control for manual operation
/// - Electrical parts management via switches
/// - Automatic bag insertion system
/// - Visual state management for different coupler components
/// - Locking mechanism to secure the coupler in position
///
/// # Physics Model
///
/// The coupler uses a simple physics simulation with:
/// - Position-based movement (0.0 to 1.0 range)
/// - Velocity with friction damping
/// - Reflection point to prevent over-extension
/// - Locking mechanism at maximum extension
///
/// # Examples
///
/// ```
/// use lotus_script::message::Coupling;
/// use lotus_extra::vehicle::CockpitSide;
///
/// let mut coupler = HandCoupler::new(
///     0,                      // ID
///     Some(CockpitSide::A),   // Cab side
///     Coupling::Front,        // Coupling type
///     0.02,                   // Friction coefficient
///     0.01                    // Mouse sensitivity
/// );
///
/// // In simulation loop:
/// coupler.tick(false); // false = no remote bag state
/// ```
#[derive(Debug)]
pub struct HandCoupler {
    /// Reflection point constant to prevent over-extension (typically 0.99)
    const_coupler_reflect: f32,

    /// Unique identifier for this coupler instance
    id: usize,
    /// API interface for coupler operations
    api_coupler: ApiCoupler,

    /// Current position of the coupler (0.0 = retracted, 1.0 = extended)
    pos: f32,
    /// Current movement speed (units per second)
    speed: f32,
    /// Friction coefficient for speed damping
    friction: f32,
    /// Mouse movement sensitivity multiplier
    mouse_factor: f32,

    /// Switch for electrical parts control
    electric_parts: Switch,
    /// Button for uncoupling operation
    uncoupler: PushButton,
    /// Button for locking the coupler in position
    locking: PushButton,

    /// Key event for cab visibility toggle
    key_klap: KeyEvent,
    /// Key event for grabbing/controlling the coupler
    key_grab: KeyEvent,
    /// Key event for bag visibility control
    key_bag: KeyEvent,
    /// Associated cab side for key events
    cab_side: Option<CockpitSide>,

    /// Visibility controller for cab components
    cab_vis: Visiblility,
    /// Visibility controller for coupling bag
    bag_vis: Visiblility,
    /// Visibility controller for coupler itself
    coupler_vis: Visiblility,

    /// Animation controller for coupler hinge A
    coupler_a_anim: Animation,
    /// Animation controller for coupler hinge B
    coupler_b_anim: Animation,

    /// Timer for automatic bag insertion
    bag_timer: f32,
    /// Flag indicating if bag has been manually set
    bag_setted: bool,
    /// Previous coupling state for change detection
    coupled_state_last: bool,
}

impl HandCoupler {
    /// Creates a new `HandCoupler` instance.
    ///
    /// # Parameters
    ///
    /// * `id` - Unique identifier for this coupler
    /// * `cab_side` - Optional cab side association for key events
    /// * `coupler` - The coupling type (Front/Rear)
    /// * `friction` - Friction coefficient for physics simulation (typically 0.01-0.05)
    /// * `mouse_factor` - Mouse sensitivity multiplier (typically 0.005-0.02)
    ///
    /// # Returns
    ///
    /// A new `HandCoupler` instance with all components initialized.
    /// If the coupler starts in a coupled state, it will be automatically
    /// extended to the coupled position.
    #[must_use]
    pub fn new(
        id: usize,
        cab_side: Option<CockpitSide>,
        coupler: Coupling,
        friction: f32,
        mouse_factor: f32,
    ) -> Self {
        let mut s = Self {
            const_coupler_reflect: 0.99,
            id,
            api_coupler: ApiCoupler::new(coupler),
            cab_side,

            pos: 0.0,
            speed: 0.0,
            friction,
            mouse_factor,

            electric_parts: Switch::builder(format!("Coupling_{id}_E_open"), cab_side)
                .event_toggle("Kupplung_E_Teil_Grab")
                .init(coupler.is_coupled())
                .build(),
            uncoupler: PushButton::builder(
                format!("Coupling_{id}_uncoupler"),
                "Kupplung_Uncoupler",
                cab_side,
            )
            .build(),
            locking: PushButton::builder(
                format!("Coupling_{id}_lever"),
                "Kupplung_Lever",
                cab_side,
            )
            .build(),

            key_klap: KeyEvent::new(Some("Kupplung_CabToggle"), cab_side),
            key_grab: KeyEvent::new(Some("Kupplung_Grab"), cab_side),
            key_bag: KeyEvent::new(Some("Kupplung_Bag"), cab_side),

            cab_vis: Visiblility::new(format!("Coupling_{id}_casecap")),
            bag_vis: Visiblility::new(format!("Coupling_{id}_BagVis")),
            coupler_vis: Visiblility::new(format!("Coupling_{id}_vis")),

            coupler_a_anim: Animation::new(Some(&format!("Coupling_{id}_hingeA"))),
            coupler_b_anim: Animation::new(Some(&format!("Coupling_{id}_hingeB"))),

            bag_timer: 0.0,
            bag_setted: false,
            coupled_state_last: false,
        };

        // Initially extend the coupling if the train is already coupled at the start of the simulation
        if s.api_coupler.is_coupled() {
            s.coupler_vis.make_visible();
            s.pos = 1.0;
            s.coupler_a_anim.set(1.0);
            s.coupler_a_anim.set(1.0);
        } else {
            s.cab_vis.make_visible();
        }

        s
    }

    /// Updates the coupler state and handles user input per simulation tick.
    ///
    /// This method handles all coupler operations including:
    /// - Mouse/keyboard input processing
    /// - Physics simulation (position, velocity, friction)
    /// - Coupling/uncoupling logic
    /// - Animation updates
    /// - Visibility management
    /// - Automatic bag insertion system
    ///
    /// # Parameters
    ///
    /// * `remote_bag_state` - Whether the remote car has a bag installed
    ///
    /// # Physics Behavior
    ///
    /// - When grabbed: Position follows mouse input directly
    /// - When released: Momentum continues with friction damping
    /// - Locking switch prevents retraction beyond reflection point
    /// - Automatic engagement at maximum position when conditions are met
    ///
    /// # Bag System
    ///
    /// The bag system simulates the protective covers placed over couplers:
    /// - Bags are automatically shown after a random delay when coupling
    /// - Manual bag hiding via key press
    /// - Coordination with remote car bag state
    pub fn tick(&mut self, remote_bag_state: bool) {
        let hand_delta = mouse_move().x * self.mouse_factor;

        self.electric_parts.tick();
        self.uncoupler.tick();
        self.locking.tick();

        if self.key_klap.is_just_pressed() {
            self.cab_vis.make_invisible();
        }

        let grabbing = self.key_grab.is_pressed() || self.locking.is_pressed();
        let locking_switch_pos: f32 = self.locking.is_pressed().into();

        // Function of the decoupling lever
        if self.uncoupler.is_pressed()
            && (self.api_coupler.coupling_state() == CouplingState::Coupled)
        {
            self.api_coupler
                .set_coupling_state(CouplingState::Deactivated);
        }

        // Animation of the coupler lever
        // Stop moving when the coupler is in the end position
        if hand_delta > 0.0 && locking_switch_pos > 0.5 && self.pos > self.const_coupler_reflect {
            self.speed = 0.0;
            self.pos = 1.0;
        }

        // Engage in max end position, if via latching point
        if grabbing && (locking_switch_pos < 0.5 && self.pos > self.const_coupler_reflect) {
            self.pos = 1.0;
            self.speed = 0.0;
        }

        let pos_last = self.pos;

        // Move clutch
        if grabbing {
            if self.pos < self.const_coupler_reflect || locking_switch_pos > 0.5 {
                self.pos = (self.pos + hand_delta).clamp(0.0, 1.0);
            }

            self.speed = hand_delta / delta();
        } else if self.pos < self.const_coupler_reflect || locking_switch_pos > 0.5 {
            self.pos = (self.pos + self.speed).clamp(0.0, 1.0);
        }

        // Coupling is folded out (set max.)
        if locking_switch_pos > 0.5 && self.pos >= 1.0 {
            self.pos = 1.0;
            self.speed = 0.0;
        }

        // Coupling was folded in (set min.)
        if pos_last > 0.0 && self.pos <= 0.0 {
            self.pos = 0.0;
            self.speed = 0.0;
            self.cab_vis.make_visible();
        }

        // Set the visibility of the clutch yourself
        if self.cab_vis.check() {
            self.pos = 0.0;
            self.speed = 0.0;
            self.coupler_vis.make_invisible();
        } else {
            self.coupler_vis.make_visible();
        }

        // Adjust speed
        if self.speed.abs() > 0.0 {
            let new_speed = self.speed + (-self.speed.signum() * self.friction) * delta();

            if new_speed * self.speed < 0.0 {
                self.speed = 0.0;
            } else {
                self.speed = new_speed;
            }
        }

        // Set animation of the coupler
        self.coupler_a_anim.set(self.pos.clamp(0.0, 1.0));
        self.coupler_b_anim
            .set(((self.pos * 1.1) - 0.1).clamp(0.0, 1.0));

        // Set Y offset
        if self.coupler_b_anim.pos < 1.0 {
            let coupler_y_vz = match self.cab_side {
                Some(side) => side.to_mul(),
                None => 1.0,
            };
            self.api_coupler.set_coupling_y_offset(0.59 * coupler_y_vz);
        } else {
            self.api_coupler.set_coupling_y_offset(0.0);
        }

        // Hide bag
        if self.key_bag.is_just_pressed() || !self.api_coupler.is_coupled() {
            self.bag_vis.make_invisible();
        }

        if self.api_coupler.is_coupled() {
            // Determine whether a bag is installed
            if self.bag_vis.check() || remote_bag_state {
                self.bag_setted = true;
            }

            if !self.bag_setted && self.bag_timer >= 0.0 {
                self.bag_timer -= delta();
            }

            // Inform the other car that this car has inserted the bag
            if (self.bag_timer < 0.0) && !self.bag_setted && !remote_bag_state {
                self.bag_vis.make_visible();
                send_bag(true, self.api_coupler.coupler);
            }
        } else {
            self.bag_setted = false;
        }

        if self.api_coupler.is_coupled() && !self.coupled_state_last {
            self.bag_timer = gen_f64() as f32;
            self.coupled_state_last = self.api_coupler.is_coupled();
        }
    }

    /// Checks if the coupler is mechanically coupled.
    ///
    /// # Returns
    ///
    /// `true` if the coupler is mechanically connected to another vehicle,
    /// `false` otherwise.
    pub fn mech_coupled(&self) -> bool {
        self.api_coupler.is_coupled()
    }

    /// Checks if the coupler is electrically coupled.
    ///
    /// Electrical coupling requires both mechanical coupling and the electrical
    /// parts switch to be in the closed position.
    ///
    /// # Returns
    ///
    /// `true` if both mechanical and electrical connections are established,
    /// `false` otherwise.
    pub fn el_coupled(&self) -> bool {
        self.mech_coupled() && self.electric_parts.value(true)
    }
}
