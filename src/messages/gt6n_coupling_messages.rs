//! # GT6N Coupling Messages
//!
//! This module provides message types and handlers for inter-car communication
//! in GT6N tram/train simulations. It implements a comprehensive coupling system
//! that allows different cars in a consist to exchange control signals, status
//! information, and operational data.
//!
//! ## Overview
//!
//! The coupling system enables communication between connected train cars through
//! various message types including:
//!
//! - Control signals (throttle, brakes, reverser)
//! - Safety systems (emergency brake, rail brake, spring brake)
//! - Passenger systems (doors, interior lighting, stop requests)
//! - Operational status (car activation, door status)
//! - Maintenance functions (sanding, shunting signals)
//!
//! ## Message Flow
//!
//! Messages are sent across couplings using the `MessageTarget::AcrossCoupling` target,
//! with each message type implementing the `MessageLine` trait for bidirectional
//! communication and state evaluation.

use lotus_script::{
    message::Coupling,
    prelude::{message_type, send_message, Message, MessageTarget},
};
use serde::{Deserialize, Serialize};

use crate::{
    management::enums::{door_enums::DoorTarget, traction_enums::DirectionOfDriving},
    messages::coupling_handler::MessageLine,
};

//===================================================================
//  bag
//===================================================================

/// Message for controlling  bag visibility between connected cars.
///
/// The  bag is a protective cover that can be shown or hidden
/// depending on whether cars are coupled together.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Bag {
    /// Whether the  bag should be visible
    pub value: bool,
}

message_type!(Bag, "Gt6n_Coupler", "BagVisibility");

/// Sends a  bag visibility message to the specified coupling side.
///
/// # Arguments
///
/// * `value` - True to show the  bag, false to hide it
/// * `side` - Which coupling (front or rear) to send the message to
///
/// # Example
///
/// ```rust
/// use lotus_script::message::Coupling;
///
/// // Hide the  bag on the front coupling
/// send__bag(false, Coupling::Front);
/// ```
pub fn send_bag(value: bool, side: Coupling) {
    send_message(
        &(Bag { value }),
        [MessageTarget::AcrossCoupling {
            coupling: side,
            cascade: false,
        }],
    );
}

/// Reader for receiving  bag visibility messages from a specific coupling side.
///
/// This struct maintains the current state of the  bag visibility
/// for one coupling connection.
#[derive(Debug)]
pub struct BagReader {
    /// Which coupling side this reader monitors
    side: Coupling,
    /// Current visibility state of the  bag
    pub value: bool,
}

impl BagReader {
    /// Creates a new  bag reader for the specified coupling side.
    ///
    /// # Arguments
    ///
    /// * `side` - The coupling side to monitor (Front or Rear)
    ///
    /// # Returns
    ///
    /// A new `BagReader` with initial value set to false
    pub fn new(side: Coupling) -> Self {
        Self { side, value: false }
    }

    /// Processes incoming messages and updates the  bag state.
    ///
    /// This method should be called for each received message to update
    /// the internal state when relevant  bag messages arrive.
    ///
    /// # Arguments
    ///
    /// * `msg` - The incoming message to process
    ///
    /// # Panics
    ///
    /// Panics if message handling fails unexpectedly
    pub fn on_message(&mut self, msg: Message) {
        if let Some(msg_side) = msg.source().coupling {
            if msg_side == self.side {
                msg.handle::<Bag>(|m| {
                    self.value = m.value;
                    Ok(())
                })
                .expect("Bag: message handle failed");
            }
        }
    }
}

//===================================================================
// Aufrüstzustand im Zugverband (Car Activation State in Train Consist)
//===================================================================

/// Message indicating whether a car is active/powered up in the train consist.
///
/// This is used to coordinate the operational state across all cars,
/// ensuring proper power distribution and system activation.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CarActiv {
    /// Whether this car is currently active/powered
    pub value: bool,
}

message_type!(CarActiv, "Gt6n_Coupler", "CarActiv");

impl MessageLine for CarActiv {
    /// Evaluates the combined car activation state using OR logic.
    ///
    /// # Arguments
    ///
    /// * `a` - Activation state from one side
    /// * `b` - Activation state from other side
    ///
    /// # Returns
    ///
    /// True if either car is active, false only if both are inactive
    fn evaluate(&self, a: &CarActiv, b: &CarActiv) -> CarActiv {
        CarActiv {
            value: a.value || b.value,
        }
    }

    /// Sends car activation state to the specified coupling.
    ///
    /// # Arguments
    ///
    /// * `value` - Current activation state to transmit
    /// * `side` - Which coupling to send the message through
    fn send(&self, value: CarActiv, side: Coupling) {
        send_message(
            &value,
            [MessageTarget::AcrossCoupling {
                coupling: side,
                cascade: false,
            }],
        );
    }

    /// Receives and processes car activation messages.
    ///
    /// # Arguments
    ///
    /// * `msg` - Incoming message to process
    ///
    /// # Returns
    ///
    /// Some((coupling_side, activation_state)) if message was relevant, None otherwise
    fn rcv(&self, msg: Message) -> Option<(Coupling, CarActiv)> {
        let mut result = None;

        if let Some(side) = msg.source().coupling {
            msg.handle::<CarActiv>(|m| {
                result = Some((side, m));
                Ok(())
            })
            .expect("CarActiv: message handle failed");
        }

        result
    }
}

//===================================================================
// Reverser
//===================================================================

/// Message for transmitting reverser (direction control) state between cars.
///
/// The reverser controls the driving direction and its state needs to be
/// coordinated across the entire train consist.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Reverser {
    /// Current driving direction state
    pub value: DirectionOfDriving,
}

message_type!(Reverser, "Gt6n_Coupler", "Reverser");

/// Handler for reverser state messages across couplings.
///
/// Handles direction flipping for front couplings to ensure consistent
/// direction interpretation across the entire train consist.
pub struct CouplerReverser;

impl MessageLine for Reverser {
    /// Evaluates combined reverser state by merging the two states.
    ///
    /// # Arguments
    ///
    /// * `a` - Reverser state from one side
    /// * `b` - Reverser state from other side
    ///
    /// # Returns
    ///
    /// Merged driving direction state
    fn evaluate(&self, a: &Reverser, b: &Reverser) -> Reverser {
        Reverser {
            value: a.value.clone().merge(&b.value),
        }
    }

    /// Sends reverser state to the specified coupling.
    ///
    /// Automatically flips the direction for front couplings to maintain
    /// consistent direction interpretation throughout the train.
    ///
    /// # Arguments
    ///
    /// * `value` - Current reverser state
    /// * `side` - Which coupling to send through (affects direction interpretation)
    fn send(&self, value: Reverser, side: Coupling) {
        let value = match side {
            Coupling::Front => Reverser {
                value: value.value.flip(),
            },
            Coupling::Rear => Reverser { value: value.value },
        };

        send_message(
            &value,
            [MessageTarget::AcrossCoupling {
                coupling: side,
                cascade: false,
            }],
        );
    }

    /// Receives and processes reverser state messages.
    ///
    /// Handles direction interpretation based on which coupling the message
    /// came from, flipping direction for rear couplings.
    ///
    /// # Arguments
    ///
    /// * `msg` - Incoming message to process
    ///
    /// # Returns
    ///
    /// Some((coupling_side, direction_state)) if message was relevant, None otherwise
    fn rcv(&self, msg: Message) -> Option<(Coupling, Reverser)> {
        let mut result = None;

        if msg.source().is_front() || msg.source().is_rear() {
            let side = if msg.source().is_front() {
                Coupling::Front
            } else {
                Coupling::Rear
            };

            msg.handle::<Reverser>(|m| {
                result = Some((
                    side,
                    match side {
                        Coupling::Front => m,
                        Coupling::Rear => Reverser {
                            value: m.value.flip(),
                        },
                    },
                ));
                Ok(())
            })
            .expect("Reverser: message handle failed");
        }

        result
    }
}

//===================================================================
// Throttle
//===================================================================

/// Message for transmitting throttle position between cars.
///
/// Throttle values are additive across the train consist to coordinate
/// traction effort distribution.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Throttle {
    /// Throttle position value (typically 0.0 to 1.0)
    pub value: f32,
}

message_type!(Throttle, "Gt6n_Coupler", "Throttle");

impl MessageLine for Throttle {
    /// Evaluates combined throttle value using addition.
    ///
    /// # Arguments
    ///
    /// * `a` - Throttle value from one source
    /// * `b` - Throttle value from another source
    ///
    /// # Returns
    ///
    /// Sum of both throttle values
    fn evaluate(&self, a: &Throttle, b: &Throttle) -> Throttle {
        Throttle {
            value: a.value + b.value,
        }
    }

    /// Sends throttle value to the specified coupling.
    ///
    /// # Arguments
    ///
    /// * `value` - Current throttle position
    /// * `side` - Which coupling to send through
    fn send(&self, value: Throttle, side: Coupling) {
        send_message(
            &value,
            [MessageTarget::AcrossCoupling {
                coupling: side,
                cascade: false,
            }],
        );
    }

    /// Receives and processes throttle messages.
    ///
    /// # Arguments
    ///
    /// * `msg` - Incoming message to process
    ///
    /// # Returns
    ///
    /// Some((coupling_side, throttle_value)) if message was relevant, None otherwise
    fn rcv(&self, msg: Message) -> Option<(Coupling, Throttle)> {
        let mut result = None;

        if let Some(side) = msg.source().coupling {
            msg.handle::<Throttle>(|m| {
                result = Some((side, m));
                Ok(())
            })
            .expect("Throttle: message handle failed");
        }

        result
    }
}

//===================================================================
// Throttle (Rear Console)
//===================================================================

/// Message for transmitting rear console throttle position between cars.
///
/// Separate from main throttle to handle dual-console operations where
/// both front and rear driving positions may be active.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThrottleRear {
    /// Rear console throttle position value
    pub value: f32,
}

message_type!(ThrottleRear, "Gt6n_Coupler", "ThrottleRear");

impl MessageLine for ThrottleRear {
    /// Evaluates combined rear throttle value using addition.
    fn evaluate(&self, a: &ThrottleRear, b: &ThrottleRear) -> ThrottleRear {
        ThrottleRear {
            value: a.value + b.value,
        }
    }

    /// Sends rear throttle value to the specified coupling.
    fn send(&self, value: ThrottleRear, side: Coupling) {
        send_message(
            &value,
            [MessageTarget::AcrossCoupling {
                coupling: side,
                cascade: false,
            }],
        );
    }

    /// Receives and processes rear throttle messages.
    fn rcv(&self, msg: Message) -> Option<(Coupling, ThrottleRear)> {
        let mut result = None;

        if let Some(side) = msg.source().coupling {
            msg.handle::<ThrottleRear>(|m| {
                result = Some((side, m));
                Ok(())
            })
            .expect("ThrottleRear: message handle failed");
        }

        result
    }
}

//===================================================================
// Rail brake
//===================================================================

/// Message for coordinating rail brake activation across cars.
///
/// Rail brakes (electromagnetic track brakes) need to be activated
/// consistently across the entire train for effective braking.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Railbrake {
    /// Whether rail brake is activated
    pub value: bool,
}

message_type!(Railbrake, "Gt6n_Coupler", "Railbrake");

impl MessageLine for Railbrake {
    /// Evaluates rail brake state using OR logic.
    ///
    /// Rail brake is active if either source requests it.
    fn evaluate(&self, a: &Railbrake, b: &Railbrake) -> Railbrake {
        Railbrake {
            value: a.value || b.value,
        }
    }

    /// Sends rail brake state to the specified coupling.
    fn send(&self, value: Railbrake, side: Coupling) {
        send_message(
            &value,
            [MessageTarget::AcrossCoupling {
                coupling: side,
                cascade: false,
            }],
        );
    }

    /// Receives and processes rail brake messages.
    fn rcv(&self, msg: Message) -> Option<(Coupling, Railbrake)> {
        let mut result = None;

        if let Some(side) = msg.source().coupling {
            msg.handle::<Railbrake>(|m| {
                result = Some((side, m));
                Ok(())
            })
            .expect("Railbrake: message handle failed");
        }

        result
    }
}

//===================================================================
// Spring brake
//===================================================================

/// Message for coordinating spring brake (parking brake) activation.
///
/// Spring brakes are safety brakes that engage when air pressure is lost
/// or when explicitly activated for parking.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpringBrake {
    /// Whether spring brake is engaged
    pub value: bool,
}

message_type!(SpringBrake, "Gt6n_Coupler", "SpringBrake");

impl MessageLine for SpringBrake {
    /// Evaluates spring brake state using OR logic for safety.
    fn evaluate(&self, a: &SpringBrake, b: &SpringBrake) -> SpringBrake {
        SpringBrake {
            value: a.value || b.value,
        }
    }

    /// Sends spring brake state to the specified coupling.
    fn send(&self, value: SpringBrake, side: Coupling) {
        send_message(
            &value,
            [MessageTarget::AcrossCoupling {
                coupling: side,
                cascade: false,
            }],
        );
    }

    /// Receives and processes spring brake messages.
    fn rcv(&self, msg: Message) -> Option<(Coupling, SpringBrake)> {
        let mut result = None;

        if let Some(side) = msg.source().coupling {
            msg.handle::<SpringBrake>(|m| {
                result = Some((side, m));
                Ok(())
            })
            .expect("SpringBrake: message handle failed");
        }

        result
    }
}

//===================================================================
// Sanding
//===================================================================

/// Message for coordinating sand dispersal system activation.
///
/// Sanding improves wheel adhesion on slippery rails and should be
/// coordinated across all powered cars in the consist.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sanding {
    /// Whether sanding system is active
    pub value: bool,
}

message_type!(Sanding, "Gt6n_Coupler", "Sanding");

impl MessageLine for Sanding {
    /// Evaluates sanding state using OR logic.
    fn evaluate(&self, a: &Sanding, b: &Sanding) -> Sanding {
        Sanding {
            value: a.value || b.value,
        }
    }

    /// Sends sanding activation state to the specified coupling.
    fn send(&self, value: Sanding, side: Coupling) {
        send_message(
            &value,
            [MessageTarget::AcrossCoupling {
                coupling: side,
                cascade: false,
            }],
        );
    }

    /// Receives and processes sanding messages.
    fn rcv(&self, msg: Message) -> Option<(Coupling, Sanding)> {
        let mut result = None;

        if let Some(side) = msg.source().coupling {
            msg.handle::<Sanding>(|m| {
                result = Some((side, m));
                Ok(())
            })
            .expect("Sanding: message handle failed");
        }

        result
    }
}

//===================================================================
// Emergency brake
//===================================================================

/// Message for coordinating emergency brake activation across the train.
///
/// Emergency brake has highest priority and must be activated immediately
/// across all cars when triggered by any source.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmergencyBrake {
    /// Whether emergency brake is activated
    pub value: bool,
}

message_type!(EmergencyBrake, "Gt6n_Coupler", "EmergencyBrake");

impl MessageLine for EmergencyBrake {
    /// Evaluates emergency brake state using OR logic for safety.
    fn evaluate(&self, a: &EmergencyBrake, b: &EmergencyBrake) -> EmergencyBrake {
        EmergencyBrake {
            value: a.value || b.value,
        }
    }

    /// Sends emergency brake state to the specified coupling.
    fn send(&self, value: EmergencyBrake, side: Coupling) {
        send_message(
            &value,
            [MessageTarget::AcrossCoupling {
                coupling: side,
                cascade: false,
            }],
        );
    }

    /// Receives and processes emergency brake messages.
    fn rcv(&self, msg: Message) -> Option<(Coupling, EmergencyBrake)> {
        let mut result = None;

        if let Some(side) = msg.source().coupling {
            msg.handle::<EmergencyBrake>(|m| {
                result = Some((side, m));
                Ok(())
            })
            .expect("EmergencyBrake: message handle failed");
        }

        result
    }
}

//===================================================================
// Door control
//===================================================================

/// Message for coordinating door operations across the train consist.
///
/// Door control commands need to be synchronized to ensure passenger
/// safety and operational consistency.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DoorControl {
    /// Target door state (open, close, etc.)
    pub value: DoorTarget,
}

message_type!(DoorControl, "Gt6n_Coupler", "DoorControl");

impl MessageLine for DoorControl {
    /// Evaluates combined door control state by merging commands.
    fn evaluate(&self, a: &DoorControl, b: &DoorControl) -> DoorControl {
        DoorControl {
            value: a.value.clone().merge(&b.value),
        }
    }

    /// Sends door control command to the specified coupling.
    fn send(&self, value: DoorControl, side: Coupling) {
        send_message(
            &value,
            [MessageTarget::AcrossCoupling {
                coupling: side,
                cascade: false,
            }],
        );
    }

    /// Receives and processes door control messages.
    fn rcv(&self, msg: Message) -> Option<(Coupling, DoorControl)> {
        let mut result = None;

        if let Some(side) = msg.source().coupling {
            msg.handle::<DoorControl>(|m| {
                result = Some((side, m));
                Ok(())
            })
            .expect("DoorControl: message handle failed");
        }

        result
    }
}

//===================================================================
// Door side
//===================================================================

/// Message for coordinating door side operations across the train consist.
///
/// Doorside commands need to be synchronized to ensure passenger
/// safety and operational consistency.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DoorSide {
    pub left: bool,
}

message_type!(DoorSide, "Gt6n_Coupler", "DoorSide");

impl MessageLine for DoorSide {
    /// Evaluates combined door control state by merging commands.
    fn evaluate(&self, a: &DoorSide, b: &DoorSide) -> DoorSide {
        DoorSide {
            left: a.left || b.left,
        }
    }

    /// Sends door control command to the specified coupling.
    fn send(&self, value: DoorSide, side: Coupling) {
        let value = match side {
            Coupling::Front => DoorSide { left: !value.left },
            Coupling::Rear => DoorSide { left: value.left },
        };

        send_message(
            &value,
            [MessageTarget::AcrossCoupling {
                coupling: side,
                cascade: false,
            }],
        );
    }

    /// Receives and processes door control messages.
    fn rcv(&self, msg: Message) -> Option<(Coupling, DoorSide)> {
        let mut result = None;

        if let Some(side) = msg.source().coupling {
            msg.handle::<DoorSide>(|m| {
                result = Some((
                    side,
                    match side {
                        Coupling::Front => m,
                        Coupling::Rear => DoorSide { left: !m.left },
                    },
                ));
                Ok(())
            })
            .expect("DoorSide: message handle failed");
        }

        result
    }
}

//===================================================================
// Powerline voltage
//===================================================================

/// Message for sharing electrical power information between cars.
///
/// Used to coordinate power distribution and monitor electrical
/// system status across the train consist.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PowerlinePower {
    /// Current power level or voltage
    pub value: f32,
}

message_type!(PowerlinePower, "Gt6n_Coupler", "PowerlinePower");

impl MessageLine for PowerlinePower {
    /// Evaluates combined power value using addition.
    fn evaluate(&self, a: &PowerlinePower, b: &PowerlinePower) -> PowerlinePower {
        PowerlinePower {
            value: a.value + b.value,
        }
    }

    /// Sends power information to the specified coupling.
    fn send(&self, value: PowerlinePower, side: Coupling) {
        send_message(
            &value,
            [MessageTarget::AcrossCoupling {
                coupling: side,
                cascade: false,
            }],
        );
    }

    /// Receives and processes powerline power messages.
    fn rcv(&self, msg: Message) -> Option<(Coupling, PowerlinePower)> {
        let mut result = None;

        if let Some(side) = msg.source().coupling {
            msg.handle::<PowerlinePower>(|m| {
                result = Some((side, m));
                Ok(())
            })
            .expect("PowerlinePower: message handle failed");
        }

        result
    }
}

//===================================================================
// Shunting signal
//===================================================================

/// Message for coordinating shunting signal activation during yard operations.
///
/// Shunting signals indicate when the train is being moved at low speed
/// for positioning or coupling operations.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShuntingSignal {
    /// Whether shunting signal is active
    pub value: bool,
}

message_type!(ShuntingSignal, "Gt6n_Coupler", "ShuntingSignal");

impl MessageLine for ShuntingSignal {
    /// Evaluates shunting signal state using OR logic.
    fn evaluate(&self, a: &ShuntingSignal, b: &ShuntingSignal) -> ShuntingSignal {
        ShuntingSignal {
            value: a.value || b.value,
        }
    }

    /// Sends shunting signal state to the specified coupling.
    fn send(&self, value: ShuntingSignal, side: Coupling) {
        send_message(
            &value,
            [MessageTarget::AcrossCoupling {
                coupling: side,
                cascade: false,
            }],
        );
    }

    /// Receives and processes shunting signal messages.
    fn rcv(&self, msg: Message) -> Option<(Coupling, ShuntingSignal)> {
        let mut result = None;

        if let Some(side) = msg.source().coupling {
            msg.handle::<ShuntingSignal>(|m| {
                result = Some((side, m));
                Ok(())
            })
            .expect("ShuntingSignal: message handle failed");
        }

        result
    }
}

//===================================================================
// Interior light
//===================================================================

/// Message for coordinating interior lighting across the train consist.
///
/// Interior lights should be synchronized to provide consistent
/// passenger experience throughout the train.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InteriorLight {
    /// Whether interior lights are on
    pub value: bool,
}

message_type!(InteriorLight, "Gt6n_Coupler", "InteriorLight");

impl MessageLine for InteriorLight {
    /// Evaluates interior light state using OR logic.
    fn evaluate(&self, a: &InteriorLight, b: &InteriorLight) -> InteriorLight {
        InteriorLight {
            value: a.value || b.value,
        }
    }

    /// Sends interior light state to the specified coupling.
    fn send(&self, value: InteriorLight, side: Coupling) {
        send_message(
            &value,
            [MessageTarget::AcrossCoupling {
                coupling: side,
                cascade: false,
            }],
        );
    }

    /// Receives and processes interior light messages.
    fn rcv(&self, msg: Message) -> Option<(Coupling, InteriorLight)> {
        let mut result = None;

        if let Some(side) = msg.source().coupling {
            msg.handle::<InteriorLight>(|m| {
                result = Some((side, m));
                Ok(())
            })
            .expect("InteriorLight: message handle failed");
        }

        result
    }
}

//===================================================================
// Indicator
//===================================================================

/// Content structure for turn signal/indicator messages.
///
/// Represents the state of directional indicators (turn signals)
/// and hazard warning lights.
#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct Indicator {
    /// Left turn indicator active
    pub left: bool,
    /// Right turn indicator active  
    pub right: bool,
    /// Hazard warning (both indicators) active
    pub warn: bool,
}

impl Indicator {
    /// Creates new indicator content with specified states.
    ///
    /// # Arguments
    ///
    /// * `left` - Left indicator state
    /// * `right` - Right indicator state  
    /// * `warn` - Hazard warning state
    ///
    /// # Returns
    ///
    /// New `BlinkerMessageContent` with specified states
    pub fn new(left: bool, right: bool, warn: bool) -> Self {
        Self { left, right, warn }
    }

    /// Flips left and right indicators for directional consistency.
    ///
    /// Used when transmitting across couplings to maintain correct
    /// directional indication relative to train orientation.
    ///
    /// # Returns
    ///
    /// New content with left/right swapped, warn unchanged
    pub fn flip(self) -> Self {
        Self {
            left: self.right,
            right: self.left,
            warn: self.warn,
        }
    }

    /// Merges this indicator state with another using OR logic.
    ///
    /// # Arguments
    ///
    /// * `other` - Other indicator state to merge with
    ///
    /// # Returns
    ///
    /// Combined state where any indicator is active if either source has it active
    pub fn merge(&self, other: &Indicator) -> Self {
        Self {
            left: self.left || other.left,
            right: self.right || other.right,
            warn: self.warn || other.warn,
        }
    }

    /// Checks if any indicator is currently active.
    ///
    /// # Returns
    ///
    /// True if any indicator (left, right, or warning) is active
    pub fn is_one(&self) -> bool {
        self.left || self.right || self.warn
    }
}

message_type!(Indicator, "Gt6n_Coupler", "Indicator");

impl MessageLine for Indicator {
    /// Evaluates combined indicator state by merging both inputs.
    ///
    /// # Arguments
    ///
    /// * `a` - Indicator state from one source
    /// * `b` - Indicator state from another source
    ///
    /// # Returns
    ///
    /// Merged indicator state using OR logic for each indicator type
    fn evaluate(&self, a: &Indicator, b: &Indicator) -> Indicator {
        a.clone().merge(b)
    }

    /// Sends indicator state to the specified coupling.
    ///
    /// Automatically flips left/right indicators for front couplings
    /// to maintain correct directional indication.
    ///
    /// # Arguments
    ///
    /// * `value` - Current indicator states
    /// * `side` - Which coupling to send through (affects direction interpretation)
    fn send(&self, value: Indicator, side: Coupling) {
        let value = match side {
            Coupling::Front => value.flip(),
            Coupling::Rear => value,
        };

        send_message(
            &value,
            [MessageTarget::AcrossCoupling {
                coupling: side,
                cascade: false,
            }],
        );
    }

    /// Receives and processes indicator messages.
    ///
    /// Handles directional interpretation based on coupling source,
    /// flipping indicators for rear couplings.
    ///
    /// # Arguments
    ///
    /// * `msg` - Incoming message to process
    ///
    /// # Returns
    ///
    /// Some((coupling_side, indicator_state)) if message was relevant, None otherwise
    fn rcv(&self, msg: Message) -> Option<(Coupling, Indicator)> {
        let mut result = None;

        if msg.source().is_front() || msg.source().is_rear() {
            let side = if msg.source().is_front() {
                Coupling::Front
            } else {
                Coupling::Rear
            };

            msg.handle::<Indicator>(|m| {
                result = Some((
                    side,
                    match side {
                        Coupling::Front => m,
                        Coupling::Rear => m.flip(),
                    },
                ));
                Ok(())
            })
            .expect("Indicator: message handle failed");
        }

        result
    }
}

//===================================================================
// Doors closed
//===================================================================

/// Message for reporting door closure status across the train consist.
///
/// Critical safety message used to ensure all doors are properly
/// closed before departure authorization.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DoorsClosed {
    /// Whether all doors on this car are closed
    pub value: bool,
}

message_type!(DoorsClosed, "Gt6n_Coupler", "DoorsClosed");

impl MessageLine for DoorsClosed {
    /// Evaluates overall door closure state using AND logic.
    ///
    /// Doors are considered closed only if ALL cars report doors closed.
    ///
    /// # Arguments
    ///
    /// * `a` - Door closure state from one car
    /// * `b` - Door closure state from another car
    ///
    /// # Returns
    ///
    /// True only if both cars have all doors closed
    fn evaluate(&self, a: &DoorsClosed, b: &DoorsClosed) -> DoorsClosed {
        DoorsClosed {
            value: a.value && b.value,
        }
    }

    /// Sends door closure status to the specified coupling.
    ///
    /// # Arguments
    ///
    /// * `value` - Current door closure status
    /// * `side` - Which coupling to send through
    fn send(&self, value: DoorsClosed, side: Coupling) {
        send_message(
            &value,
            [MessageTarget::AcrossCoupling {
                coupling: side,
                cascade: false,
            }],
        );
    }

    /// Receives and processes door closure messages.
    ///
    /// # Arguments
    ///
    /// * `msg` - Incoming message to process
    ///
    /// # Returns
    ///
    /// Some((coupling_side, doors_closed_status)) if message was relevant, None otherwise
    fn rcv(&self, msg: Message) -> Option<(Coupling, DoorsClosed)> {
        let mut result = None;

        if let Some(side) = msg.source().coupling {
            msg.handle::<DoorsClosed>(|m| {
                result = Some((side, m));
                Ok(())
            })
            .expect("DoorsClosed: message handle failed");
        }

        result
    }
}

//===================================================================
// Buggy request (KiWa - Kinderwagen/Wheelchair request)
//===================================================================

/// Message for wheelchair/buggy accessibility requests across the train.
///
/// Used to coordinate accessibility features and ensure proper
/// accommodation for passengers with mobility devices.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BuggyReqest {
    /// Whether wheelchair/buggy assistance is requested
    pub value: bool,
}

message_type!(BuggyReqest, "Gt6n_Coupler", "KiWaReqest");

impl MessageLine for BuggyReqest {
    /// Evaluates accessibility request state using OR logic.
    ///
    /// Request is active if any car reports an accessibility need.
    ///
    /// # Arguments
    ///
    /// * `a` - Request state from one car
    /// * `b` - Request state from another car
    ///
    /// # Returns
    ///
    /// True if either car has an active accessibility request
    fn evaluate(&self, a: &BuggyReqest, b: &BuggyReqest) -> BuggyReqest {
        BuggyReqest {
            value: a.value || b.value,
        }
    }

    /// Sends accessibility request status to the specified coupling.
    ///
    /// # Arguments
    ///
    /// * `value` - Current request status
    /// * `side` - Which coupling to send through
    fn send(&self, value: BuggyReqest, side: Coupling) {
        send_message(
            &value,
            [MessageTarget::AcrossCoupling {
                coupling: side,
                cascade: false,
            }],
        );
    }

    /// Receives and processes accessibility request messages.
    ///
    /// # Arguments
    ///
    /// * `msg` - Incoming message to process
    ///
    /// # Returns
    ///
    /// Some((coupling_side, request_status)) if message was relevant, None otherwise
    fn rcv(&self, msg: Message) -> Option<(Coupling, BuggyReqest)> {
        let mut result = None;

        if let Some(side) = msg.source().coupling {
            msg.handle::<BuggyReqest>(|m| {
                result = Some((side, m));
                Ok(())
            })
            .expect("BuggyReqest: message handle failed");
        }

        result
    }
}

//===================================================================
// Buggy reset (KiWa Reset)
//===================================================================

/// Message for resetting wheelchair/buggy accessibility system state.
///
/// Used to clear accessibility requests and reset related systems
/// after passenger needs have been accommodated.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BuggyReset {
    /// Whether to reset the accessibility system
    pub value: bool,
}

message_type!(BuggyReset, "Gt6n_Coupler", "BuggyReset");

impl MessageLine for BuggyReset {
    /// Evaluates reset command using OR logic.
    ///
    /// Reset occurs if any car sends a reset command.
    ///
    /// # Arguments
    ///
    /// * `a` - Reset command from one car
    /// * `b` - Reset command from another car
    ///
    /// # Returns
    ///
    /// True if either car requests a reset
    fn evaluate(&self, a: &BuggyReset, b: &BuggyReset) -> BuggyReset {
        BuggyReset {
            value: a.value || b.value,
        }
    }

    /// Sends accessibility reset command to the specified coupling.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to perform reset
    /// * `side` - Which coupling to send through
    fn send(&self, value: BuggyReset, side: Coupling) {
        send_message(
            &value,
            [MessageTarget::AcrossCoupling {
                coupling: side,
                cascade: false,
            }],
        );
    }

    /// Receives and processes accessibility reset messages.
    ///
    /// # Arguments
    ///
    /// * `msg` - Incoming message to process
    ///
    /// # Returns
    ///
    /// Some((coupling_side, reset_command)) if message was relevant, None otherwise
    fn rcv(&self, msg: Message) -> Option<(Coupling, BuggyReset)> {
        let mut result = None;

        if let Some(side) = msg.source().coupling {
            msg.handle::<BuggyReset>(|m| {
                result = Some((side, m));
                Ok(())
            })
            .expect("BuggyReset: message handle failed");
        }

        result
    }
}

//===================================================================
// Stop request
//===================================================================

/// Message for passenger stop requests across the train consist.
///
/// Allows passengers in any car to request a stop at the next station,
/// with the request being propagated throughout the entire train.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StopRequest {
    /// Whether a passenger has requested a stop
    pub value: bool,
}

message_type!(StopRequest, "Gt6n_Coupler", "StopRequest");

impl MessageLine for StopRequest {
    /// Evaluates stop request state using OR logic.
    ///
    /// Stop is requested if any passenger in any car has requested it.
    ///
    /// # Arguments
    ///
    /// * `a` - Stop request state from one car
    /// * `b` - Stop request state from another car
    ///
    /// # Returns
    ///
    /// True if either car has an active stop request
    fn evaluate(&self, a: &StopRequest, b: &StopRequest) -> StopRequest {
        StopRequest {
            value: a.value || b.value,
        }
    }

    /// Sends stop request status to the specified coupling.
    ///
    /// # Arguments
    ///
    /// * `value` - Current stop request status
    /// * `side` - Which coupling to send through
    fn send(&self, value: StopRequest, side: Coupling) {
        send_message(
            &value,
            [MessageTarget::AcrossCoupling {
                coupling: side,
                cascade: false,
            }],
        );
    }

    /// Receives and processes stop request messages.
    ///
    /// # Arguments
    ///
    /// * `msg` - Incoming message to process
    ///
    /// # Returns
    ///
    /// Some((coupling_side, stop_request_status)) if message was relevant, None otherwise
    fn rcv(&self, msg: Message) -> Option<(Coupling, StopRequest)> {
        let mut result = None;

        if let Some(side) = msg.source().coupling {
            msg.handle::<StopRequest>(|m| {
                result = Some((side, m));
                Ok(())
            })
            .expect("StopRequest: message handle failed");
        }

        result
    }
}
