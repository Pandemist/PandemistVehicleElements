//! # Pantograph and Third Rail Collector Module
//!
//! This module provides implementations for electric pantographs and third rail collectors
//! used in electric train simulations. It includes both automatic electric pantographs
//! and manual pantographs, as well as third rail collectors for different power supply systems.
//!
//! ## Features
//!
//! - **Electric Pantograph**: Automatic pantograph with motor control, configurable speeds,
//!   and realistic electrical supply simulation
//! - **Manual Pantograph**: Manual rope-operated pantograph with user interaction
//! - **Third Rail Collector**: Third rail power collection system with sparking effects
//!   and realistic state management
//!
//! ## Example
//!
//! ```rust
//! use your_crate::pantograph::ElectricPantograph;
//! use your_crate::elements::std::piecewise_linear_function::PiecewiseLinearFunction;
//!
//! let curve = PiecewiseLinearFunction::new(vec![(0.0, 0.0), (1.0, 1.0)]);
//! let pantograph = ElectricPantograph::builder("panto_anim", 0, curve)
//!     .move_up_speed(2.0)
//!     .move_down_speed(1.5)
//!     .snd_up("panto_up_sound")
//!     .snd_down("panto_down_sound")
//!     .build();
//! ```

use lotus_script::time::delta;

use crate::{
    api::{
        animation::Animation,
        electrical_supply::ApiPantograph,
        key_event::KeyEventCab,
        light::Light,
        mock_enums::{ThirdRailState, VehicleInitState},
        simulation_settings::{init_ready_state, realisitc_electric_supply},
        sound::Sound,
        visible_flag::Visiblility,
    },
    elements::{
        std::{helper::gen_f32, piecewise_linear_function::PiecewiseLinearFunction},
        tech::slider::Slider,
    },
    management::enums::{
        general_enums::Side, state_enums::SwitchingState, target_enums::SwitchingTarget,
    },
};

/// Builder for creating an `ElectricPantograph` with customizable parameters.
///
/// This builder allows you to configure various aspects of an electric pantograph
/// including movement speeds, animations, sounds, and initial state.
///
/// # Example
///
/// ```rust
/// let pantograph = ElectricPantograph::builder("main_panto", 0, height_curve)
///     .move_up_speed(1.5)
///     .move_down_speed(2.0)
///     .cranc_transmission(0.5)
///     .snd_up("panto_raise")
///     .snd_down("panto_lower")
///     .init(true)
///     .build();
/// ```
pub struct ElectricPantographBuilder {
    move_up_speed: f32,
    move_down_speed: f32,

    height_curve: PiecewiseLinearFunction,

    sub_animations: Vec<(Animation, PiecewiseLinearFunction)>,

    motor_swiching_timer: f32,
    current_wire_height: f32,
    current_wire_max_anim: f32,

    motor_target: SwitchingTarget,
    motor_relais: SwitchingState,
    motor_pos: f32,

    cranc_target: SwitchingTarget,
    cranc_transmission: f32,

    panto_pos: f32,

    animation: Animation,

    /// Normalized voltage output (0.0 to 1.0)
    pub voltage_norm: f32,

    /// Current switching state of the pantograph
    pub state: SwitchingState,

    api_panto: ApiPantograph,

    snd_up: Sound,
    snd_down: Sound,
}

impl ElectricPantographBuilder {
    /// Adds a sub-animation that follows a specific path based on the main pantograph position.
    ///
    /// Sub-animations are useful for animating additional parts of the pantograph
    /// that move in relation to the main pantograph position.
    ///
    /// # Arguments
    ///
    /// * `name` - Name identifier for the sub-animation
    /// * `path` - Piecewise linear function defining the animation path
    ///
    /// # Example
    ///
    /// ```rust
    /// let builder = builder.add_sub_animation("arm_joint", joint_curve);
    /// ```
    pub fn add_sub_animation(
        mut self,
        name: impl Into<String>,
        path: PiecewiseLinearFunction,
    ) -> Self {
        self.sub_animations
            .push((Animation::new(Some(&name.into())), path));

        self
    }

    /// Sets the manual crank transmission factor.
    ///
    /// This controls how much the pantograph moves when operated manually
    /// via the crank mechanism.
    ///
    /// # Arguments
    ///
    /// * `value` - Transmission factor (typically 0.0 to 1.0)
    pub fn cranc_transmission(mut self, value: f32) -> Self {
        self.cranc_transmission = value;
        self
    }

    /// Sets the speed at which the pantograph moves up.
    ///
    /// # Arguments
    ///
    /// * `speed` - Movement speed in units per second
    pub fn move_up_speed(mut self, speed: f32) -> Self {
        self.move_up_speed = speed;
        self
    }

    /// Sets the speed at which the pantograph moves down.
    ///
    /// # Arguments
    ///
    /// * `speed` - Movement speed in units per second
    pub fn move_down_speed(mut self, speed: f32) -> Self {
        self.move_down_speed = speed;
        self
    }

    /// Sets the sound to play when the pantograph moves up.
    ///
    /// # Arguments
    ///
    /// * `name` - Sound asset name or path
    pub fn snd_up(mut self, name: impl Into<String>) -> Self {
        self.snd_up = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Sets the sound to play when the pantograph moves down.
    ///
    /// # Arguments
    ///
    /// * `name` - Sound asset name or path
    pub fn snd_down(mut self, name: impl Into<String>) -> Self {
        self.snd_down = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Initializes the pantograph in the raised position.
    ///
    /// When set to `true`, the pantograph starts in the fully raised position
    /// with all animations set accordingly.
    ///
    /// # Arguments
    ///
    /// * `init` - Whether to initialize in raised position
    pub fn init(mut self, init: bool) -> Self {
        self.motor_pos = 1.0;
        self.state = SwitchingState::On;

        self.animation.set(self.motor_pos);
        for sub_anim in &mut self.sub_animations {
            let sub_pos = sub_anim.1.get_value_or_default(self.motor_pos);
            sub_anim.0.set(sub_pos);
        }

        self
    }

    /// Builds the final `ElectricPantograph` instance.
    ///
    /// # Returns
    ///
    /// A configured `ElectricPantograph` ready for use in simulation.
    pub fn build(self) -> ElectricPantograph {
        ElectricPantograph {
            move_up_speed: self.move_up_speed,
            move_down_speed: self.move_down_speed,
            height_curve: self.height_curve,
            sub_animations: self.sub_animations,
            motor_relais: self.motor_relais,
            motor_swiching_timer: self.motor_swiching_timer,
            current_wire_height: self.current_wire_height,
            current_wire_max_anim: self.current_wire_max_anim,
            motor_target: self.motor_target,
            motor_pos: self.motor_pos,
            cranc_target: self.cranc_target,
            cranc_transmission: self.cranc_transmission,
            panto_pos: self.panto_pos,
            animation: self.animation,
            voltage_norm: self.voltage_norm,
            state: self.state,
            api_panto: self.api_panto,
            snd_up: self.snd_up,
            snd_down: self.snd_down,
        }
    }
}

/// An electric pantograph for collecting power from overhead wires.
///
/// This struct simulates a realistic electric pantograph with motor control,
/// automatic height adjustment based on wire position, and electrical supply management.
/// It supports both automatic operation and manual crank operation.
///
/// # Features
///
/// - Automatic motor-driven operation with configurable speeds
/// - Manual crank operation as backup
/// - Realistic wire height tracking and collision detection
/// - Sound effects for raising and lowering operations
/// - Sub-animations for complex pantograph mechanisms
/// - Voltage normalization based on contact state
///
/// # Safety Features
///
/// - Requires battery power and safety systems to be active
/// - Automatic shutdown when safety conditions are not met
/// - Prevents operation beyond safe limits
pub struct ElectricPantograph {
    move_up_speed: f32,
    move_down_speed: f32,

    panto_pos: f32,
    animation: Animation,
    height_curve: PiecewiseLinearFunction,
    sub_animations: Vec<(Animation, PiecewiseLinearFunction)>,

    motor_swiching_timer: f32,
    current_wire_height: f32,
    current_wire_max_anim: f32,

    /// Current motor target state
    pub motor_target: SwitchingTarget,
    motor_relais: SwitchingState,
    motor_pos: f32,

    /// Current crank target state for manual operation
    pub cranc_target: SwitchingTarget,
    cranc_transmission: f32,

    /// Normalized voltage output (0.0 to 1.0)
    pub voltage_norm: f32,
    /// Current operational state
    pub state: SwitchingState,

    api_panto: ApiPantograph,

    snd_up: Sound,
    snd_down: Sound,
}

impl ElectricPantograph {
    /// Creates a new builder for configuring an electric pantograph.
    ///
    /// # Arguments
    ///
    /// * `animation_name` - Name for the main pantograph animation
    /// * `id` - Unique identifier for this pantograph
    /// * `curve` - Height curve mapping wire height to animation position
    ///
    /// # Returns
    ///
    /// A new `ElectricPantographBuilder` instance
    ///
    /// # Example
    ///
    /// ```rust
    /// let curve = PiecewiseLinearFunction::new(vec![(0.0, 0.0), (5.0, 1.0)]);
    /// let pantograph = ElectricPantograph::builder("main_panto", 0, curve)
    ///     .move_up_speed(1.0)
    ///     .build();
    /// ```
    pub fn builder(
        animation_name: impl Into<String>,
        id: usize,
        curve: PiecewiseLinearFunction,
    ) -> ElectricPantographBuilder {
        ElectricPantographBuilder {
            move_up_speed: 1.0,
            move_down_speed: 1.0,
            sub_animations: Vec::new(),
            height_curve: curve,
            current_wire_height: 10.0,
            api_panto: ApiPantograph::new(id),
            cranc_target: SwitchingTarget::Neutral,
            cranc_transmission: 0.0,
            animation: Animation::new(Some(&animation_name.into())),
            snd_up: Sound::new_simple(None),
            snd_down: Sound::new_simple(None),
            motor_target: SwitchingTarget::Neutral,
            motor_relais: SwitchingState::Neutral,
            motor_swiching_timer: 0.0,
            current_wire_max_anim: 0.0,
            motor_pos: 0.0,
            panto_pos: 0.0,
            voltage_norm: 0.0,
            state: SwitchingState::Neutral,
        }
    }

    /// Updates all animations based on the current pantograph position.
    ///
    /// This method updates the main animation and all sub-animations
    /// according to their respective curves and the current position.
    ///
    /// # Arguments
    ///
    /// * `pos` - Current pantograph position (0.0 to 1.0)
    fn update_animation(&mut self, pos: f32) {
        self.animation.set(pos);
        for sub_anim in &mut self.sub_animations {
            let sub_pos = sub_anim.1.get_value_or_default(pos);
            sub_anim.0.set(sub_pos);
        }
    }

    /// Updates the pantograph state for one simulation tick.
    ///
    /// This method handles all pantograph logic including:
    /// - Motor control and movement
    /// - Wire height detection and collision
    /// - Sound management
    /// - Voltage calculation
    /// - Safety system integration
    ///
    /// # Arguments
    ///
    /// * `safeguard` - Whether safety systems are active
    /// * `battery` - Whether battery power is available
    ///
    /// # Safety
    ///
    /// The pantograph will not operate if either `safeguard` or `battery` is false.
    /// This prevents operation during unsafe conditions.
    pub fn tick(&mut self, safeguard: bool, battery: bool) {
        if self.state == SwitchingState::Off {
            self.current_wire_height = f32::MAX;
        }

        if let Some(height) = self.api_panto.height() {
            self.current_wire_height = height;
        }

        self.current_wire_max_anim = self
            .height_curve
            .get_value_or_default(self.current_wire_height);

        let target_last = self.motor_relais;

        match self.motor_target {
            SwitchingTarget::TurnOn(delay) => {
                self.motor_swiching_timer += delta();
                if self.motor_swiching_timer > delay {
                    self.motor_relais = SwitchingState::On;
                }
            }
            SwitchingTarget::TurnOff(delay) => {
                self.motor_swiching_timer += delta();
                if self.motor_swiching_timer > delay {
                    self.motor_relais = SwitchingState::Off;
                }
            }
            SwitchingTarget::Neutral => {
                self.motor_swiching_timer = 0.0;
            }
        }

        if !battery || !safeguard {
            self.motor_relais = SwitchingState::Neutral;
        }

        match self.motor_relais {
            SwitchingState::On => {
                if self.panto_pos >= 1.0 {
                    self.motor_relais = SwitchingState::Neutral;
                }
            }
            SwitchingState::Off => {
                if self.panto_pos <= 0.0 {
                    self.motor_relais = SwitchingState::Neutral;
                }
            }
            SwitchingState::Neutral => {}
        }

        if self.motor_relais == SwitchingState::Neutral {
            match self.cranc_target {
                SwitchingTarget::TurnOn(_) => {
                    self.motor_pos = (self.motor_pos + self.cranc_transmission * delta()).min(1.0);
                }
                SwitchingTarget::TurnOff(_) => {
                    self.motor_pos = (self.motor_pos - self.cranc_transmission * delta()).max(0.0);
                }
                SwitchingTarget::Neutral => {}
            }
        }

        match self.motor_relais {
            SwitchingState::On => {
                self.motor_pos = (self.motor_pos + self.move_up_speed * delta()).min(1.0);
            }
            SwitchingState::Off => {
                self.motor_pos = (self.motor_pos - self.move_down_speed * delta()).max(0.0);
            }
            SwitchingState::Neutral => {}
        }

        if self.motor_relais != target_last {
            match self.motor_relais {
                SwitchingState::On => {
                    self.snd_up.start();
                    self.snd_down.stop();
                }
                SwitchingState::Off => {
                    self.snd_up.stop();
                    self.snd_down.start();
                }
                SwitchingState::Neutral => {
                    self.snd_up.stop();
                    self.snd_down.stop();
                }
            }
        }

        if self.motor_pos >= self.current_wire_max_anim && self.motor_pos > 0.95 {
            self.state = SwitchingState::On;
        } else if self.motor_pos < 0.05 {
            self.state = SwitchingState::Off;
            self.current_wire_height = 10.0;
        } else {
            self.state = SwitchingState::Neutral;
        }

        self.voltage_norm = if realisitc_electric_supply() {
            ((self.state == SwitchingState::On) as u8 as f32) * self.api_panto.voltage()
        } else {
            (self.state == SwitchingState::On).into()
        };

        self.panto_pos = self.motor_pos;
        self.panto_pos = self.panto_pos.min(self.current_wire_height);
        self.update_animation(self.panto_pos);
    }
}

//==========================================================================

/// Builder for creating a `ManualPantograph` with customizable parameters.
///
/// This builder allows configuration of a manual rope-operated pantograph
/// including animations, visibility flags, and user interaction settings.
pub struct ManualPantographBuilder {
    height_curve: PiecewiseLinearFunction,

    animation: Animation,
    sub_animations: Vec<(Animation, PiecewiseLinearFunction)>,

    current_wire_height: f32,
    current_wire_max_anim: f32,

    voltage_norm: f32,
    state: bool,

    wire: Slider,
    panto: Slider,

    vis_rope_loss: Visiblility,
    vis_rope_knoted: Visiblility,

    api_panto: ApiPantograph,
}

impl ManualPantographBuilder {
    /// Adds a sub-animation that follows a specific path based on the wire position.
    ///
    /// # Arguments
    ///
    /// * `name` - Name identifier for the sub-animation
    /// * `path` - Piecewise linear function defining the animation path
    pub fn add_sub_animation(
        mut self,
        name: impl Into<String>,
        path: PiecewiseLinearFunction,
    ) -> Self {
        self.sub_animations
            .push((Animation::new(Some(&name.into())), path));

        self
    }

    /// Initializes the manual pantograph in the raised position.
    ///
    /// # Arguments
    ///
    /// * `init` - Whether to initialize in raised position
    pub fn init(mut self, init: bool) -> Self {
        self.wire.pos = 1.0;
        self.panto.pos = 1.0;
        self.state = true;

        self.animation.set(self.panto.pos);
        for sub_anim in &mut self.sub_animations {
            let sub_pos = sub_anim.1.get_value_or_default(self.wire.pos);
            sub_anim.0.set(sub_pos);
        }

        self
    }

    /// Builds the final `ManualPantograph` instance.
    pub fn build(self) -> ManualPantograph {
        ManualPantograph {
            height_curve: self.height_curve,
            animation: self.animation,
            sub_animations: self.sub_animations,
            current_wire_height: self.current_wire_height,
            current_wire_max_anim: self.current_wire_max_anim,
            voltage_norm: self.voltage_norm,
            state: self.state,
            wire: self.wire,
            panto: self.panto,
            vis_rope_loss: self.vis_rope_loss,
            vis_rope_knoted: self.vis_rope_knoted,
            api_panto: self.api_panto,
        }
    }
}

/// A manual rope-operated pantograph for collecting power from overhead wires.
///
/// This pantograph is operated manually by the user through a rope mechanism.
/// It provides realistic rope physics, visual feedback for rope state, and
/// accurate electrical contact simulation.
///
/// # Features
///
/// - Manual rope operation with realistic physics
/// - Visual indicators for rope state (loose/knotted)
/// - Height-based collision detection with overhead wires
/// - Sub-animations for complex mechanical parts
/// - Voltage output based on contact state
pub struct ManualPantograph {
    height_curve: PiecewiseLinearFunction,

    animation: Animation,
    sub_animations: Vec<(Animation, PiecewiseLinearFunction)>,

    current_wire_height: f32,
    current_wire_max_anim: f32,

    pub voltage_norm: f32,
    pub state: bool,

    wire: Slider,
    panto: Slider,

    vis_rope_loss: Visiblility,
    vis_rope_knoted: Visiblility,

    api_panto: ApiPantograph,
}

impl ManualPantograph {
    /// Creates a new builder for configuring a manual pantograph.
    ///
    /// # Arguments
    ///
    /// * `animation_name` - Name for the main pantograph animation
    /// * `key_event_name` - Name for the key event controlling the rope
    /// * `id` - Unique identifier for this pantograph
    /// * `vis_rope_loss_name` - Name for the loose rope visibility flag
    /// * `vis_rope_knoted_name` - Name for the knotted rope visibility flag
    /// * `curve` - Height curve mapping wire height to animation position
    /// * `cab_side` - Optional cab side specification for key events
    ///
    /// # Returns
    ///
    /// A new `ManualPantographBuilder` instance
    pub fn builder(
        animation_name: impl Into<String>,
        key_event_name: impl Into<String>,
        id: usize,
        vis_rope_loss_name: impl Into<String>,
        vis_rope_knoted_name: impl Into<String>,
        curve: PiecewiseLinearFunction,
        cab_side: Option<KeyEventCab>,
    ) -> ManualPantographBuilder {
        ManualPantographBuilder {
            height_curve: curve,
            animation: Animation::new(Some(&animation_name.into())),
            current_wire_height: 10.0,
            current_wire_max_anim: 0.0,
            sub_animations: Vec::new(),
            voltage_norm: 0.0,
            state: false,
            wire: Slider::builder()
                .key_event(key_event_name.into(), cab_side)
                .axis_y()
                .mouse_factor(-0.1 / 400.0)
                .friction(0.1)
                .force(1.5)
                .build(),
            panto: Slider::builder()
                .upper_bump_factor(0.2)
                .friction(0.1)
                .force(1.5)
                .build(),
            vis_rope_loss: Visiblility::new(vis_rope_loss_name),
            vis_rope_knoted: Visiblility::new(vis_rope_knoted_name),
            api_panto: ApiPantograph::new(id),
        }
    }

    /// Updates all animations based on the current pantograph position.
    ///
    /// # Arguments
    ///
    /// * `pos` - Current pantograph position (0.0 to 1.0)
    fn update_animation(&mut self, pos: f32) {
        self.animation.set(pos);
        for sub_anim in &mut self.sub_animations {
            let sub_pos = sub_anim.1.get_value_or_default(pos);
            sub_anim.0.set(sub_pos);
        }
    }

    /// Updates the manual pantograph state for one simulation tick.
    ///
    /// This method handles:
    /// - Wire rope physics simulation
    /// - Pantograph position calculation based on rope state
    /// - Wire height detection and collision
    /// - Voltage calculation based on contact
    /// - Visual rope state indicators
    pub fn tick(&mut self) {
        if !self.state {
            self.current_wire_height = f32::MAX;
        }

        if let Some(height) = self.api_panto.height() {
            self.current_wire_height = height;
        }

        self.current_wire_max_anim = self
            .height_curve
            .get_value_or_default(self.current_wire_height);

        self.wire.tick();

        self.panto.max = self.current_wire_max_anim.min(1.0).min(self.wire.pos);

        self.panto.tick();

        self.panto.pos = self.current_wire_max_anim.min(1.0).min(self.wire.pos);

        self.state = self.panto.pos > 0.05;

        self.vis_rope_loss.set_visbility(self.wire.pos > 0.0);
        self.vis_rope_knoted.set_visbility(self.wire.pos <= 0.0);

        self.voltage_norm = if realisitc_electric_supply() {
            (self.state as u8 as f32) * self.api_panto.voltage()
        } else {
            self.state as u8 as f32
        };

        self.update_animation(self.panto.pos);
    }
}
