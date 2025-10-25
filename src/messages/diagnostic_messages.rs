//! # Diagnostic Messages Module
//!
//! This module provides comprehensive diagnostic messaging capabilities for railway vehicle systems.
//! It includes fault detection, state monitoring, and inter-vehicle communication for various
//! subsystems including pantographs, doors, brakes, heating, and electrical systems.
//!
//! ## Features
//!
//! - **Fault Detection**: Comprehensive fault enumeration covering all major vehicle subsystems
//! - **State Change Detection**: Only sends messages when state changes occur to reduce network traffic
//! - **Broadcasting**: Supports both local and cross-coupling message distribution
//! - **Serialization**: All message types are serializable for network transmission
//!
//! ## Usage
//!
//! ```rust
//! use diagnostic_messages::{DiagnosticMessageSender, DiagnosticFaultKind};
//!
//! let mut sender = DiagnosticMessageSender::new();
//! sender.send(DiagnosticFaultKind::HauptschalterA, true, None);
//! ```

use std::collections::HashMap;

use lotus_extra::vehicle::CockpitSide;
use lotus_script::{
    message::{send_message, MessageTarget},
    prelude::message_type,
};
use serde::{Deserialize, Serialize};

use crate::{
    api::vehicle_infos::veh_number,
    management::enums::{door_enums::DoorTarget, state_enums::SwitchingState},
};

/// Represents different types of diagnostic faults that can occur in a railway vehicle.
///
/// This enum covers all major subsystems including electrical, mechanical, safety,
/// and comfort systems. Each variant represents a specific fault condition that
/// can be monitored and reported.
///
/// # Examples
///
/// ```rust
/// use diagnostic_messages::DiagnosticFaultKind;
///
/// let fault = DiagnosticFaultKind::HauptschalterA;
/// println!("Fault detected: {:?}", fault);
/// ```
#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum DiagnosticFaultKind {
    /// Bus communication error
    Zugbusfehler,
    /// Multiple driver's cabs equipped
    MehrereFahrerstaendeAufgeruestet,
    /// Direction of travel error
    Fahrtrichtungsfehler,
    /// Main switch A fault
    HauptschalterA,
    /// Main switch B fault
    HauptschalterB,
    /// Pantograph A fault
    StromabnehmerA,
    /// Pantograph B fault
    StromabnehmerB,
    /// Wheelchair lift defect A
    HubliftDefektA,
    /// Wheelchair lift defect B
    HubliftDefektB,
    /// Emergency lighting failure
    AusfallNotbeleuchtung,
    /// Turn signal failure
    BlinkerAusfall,
    /// Automatic braking activated
    AutomatischeBremsung,
    /// Maximum speed exceeded
    VmaxUeberschreitung,
    /// SIFA (driver vigilance system) bypassed
    SifaUeberbrueckt,
    /// Green loop bypassed
    GruenschleifeUeberbrueckt,
    /// Vehicle emergency brake bypassed
    FGnotbremseUeberbrueckt,
    /// KWR (short circuit protection) bypassed
    KwrUeberbrueckt,
    /// Driver emergency brake A
    FahrernotbremseA,
    /// Driver emergency brake B
    FahrernotbremseB,
    /// Passenger emergency brake
    Fahrgastnotbremse,
    /// Emergency door release R1
    NotentriegelungR1,
    /// Emergency door release R2
    NotentriegelungR2,
    /// Emergency door release R3
    NotentriegelungR3,
    /// Emergency door release R4
    NotentriegelungR4,
    /// Emergency door release L1
    NotentriegelungL1,
    /// Emergency door release L2
    NotentriegelungL2,
    /// Emergency door release L3
    NotentriegelungL3,
    /// Emergency door release L4
    NotentriegelungL4,
    /// Drive A failed
    AntriebAausgefallen,
    /// Drive B failed
    AntriebBausgefallen,
    /// Drive C failed
    AntriebCausgefallen,
    /// Drive A grouped out
    AntriebAausgegruppiert,
    /// Drive B grouped out
    AntriebBausgegruppiert,
    /// Drive C grouped out
    AntriebCausgegruppiert,
    /// MMS A defective
    MmsAdefekt,
    /// MMS B defective
    MmsBdefekt,
    /// Wash run mode
    Waschfahrt,
    /// Spring-loaded brake not released
    FederspeicherNichtGeloest,
    /// Switch on exterior lighting A
    AussenbeleuchtungAeinschalten,
    /// Switch on exterior lighting B
    AussenbeleuchtungBeinschalten,
    /// Exterior lighting failed
    AussenbeleuchtungAusgefallen,
    /// Start inhibit
    Anfahrsperre,
    /// Start inhibit doors
    AnfahrsperreTueren,
    /// Spring-loaded brake A grouped out
    FederspeicherAausgruppiert,
    /// Spring-loaded brake B grouped out
    FederspeicherBausgruppiert,
    /// Spring-loaded brake C grouped out
    FederspeicherCausgruppiert,
    /// Spring-loaded brake A disturbed
    FederspeicherAgestoert,
    /// Spring-loaded brake B disturbed
    FederspeicherBgestoert,
    /// Spring-loaded brake C disturbed
    FederspeicherCgestoert,
    /// Rear pantograph
    StromabnehmerHinten,
    /// Main switch off
    HauptschalterAus,
    /// No traction voltage
    KeineFahrspannung,
    /// Keep-warm operation
    Warmhaltebetrieb,
    /// Passenger compartment heating off
    FahrgastraumheizungAus,
    /// Heating/ventilation A malfunction
    HeizungLueftungAstoerung,
    /// Heating/ventilation B malfunction
    HeizungLueftungBstoerung,
    /// Heating/ventilation C malfunction
    HeizungLueftungCstoerung,
    /// Driver's compartment heating off
    FahrerraumheizungAus,
    /// Driver's compartment heating A disturbed
    FahrerraumheizungAgestoert,
    /// Driver's compartment heating B disturbed
    FahrerraumheizungBgestoert,
    /// Sand container A1
    SandbehaelterA1,
    /// Sand container A2
    SandbehaelterA2,
    /// Sand container A3
    SandbehaelterA3,
    /// Sand container B1
    SandbehaelterB1,
    /// Sand container B2
    SandbehaelterB2,
    /// Sand container B3
    SandbehaelterB3,
    /// Brake light A failure
    AusfallBremslichtA,
    /// Brake light B failure
    AusfallBremslichtB,
    /// Window heating A
    ScheibenheizungA,
    /// Window heating B
    ScheibenheizungB,
    /// Main switch 1 failure
    Hauptschalter1ausfall,
    /// Main switch 2 failure
    Hauptschalter2ausfall,
    /// KWR failure
    KwrAusfall,
    /// Train formation error A
    ZugbildungsFehlerA,
    /// Train formation error B
    ZugbildungsFehlerB,
    /// Fuse circuit 4a A
    Sicherungskreis4aA,
    /// Fuse circuit 4b A
    Sicherungskreis4bA,
    /// Fuse circuit 4c A
    Sicherungskreis4cA,
    /// Fuse circuit 4d A
    Sicherungskreis4dA,
    /// Fuse circuit 7a A
    Sicherungskreis7aA,
    /// Fuse circuit 7b A
    Sicherungskreis7bA,
    /// Fuse circuit 7c A
    Sicherungskreis7cA,
    /// Fuse circuit 8a A
    Sicherungskreis8aA,
    /// Fuse circuit 8b A
    Sicherungskreis8bA,
    /// Fuse circuit 4a B
    Sicherungskreis4aB,
    /// Fuse circuit 4b B
    Sicherungskreis4bB,
    /// Fuse circuit 4c B
    Sicherungskreis4cB,
    /// Fuse circuit 4d B
    Sicherungskreis4dB,
    /// Fuse circuit 7a B
    Sicherungskreis7aB,
    /// Fuse circuit 7b B
    Sicherungskreis7bB,
    /// Fuse circuit 7c B
    Sicherungskreis7cB,
    /// Fuse circuit 8a B
    Sicherungskreis8aB,
    /// Fuse circuit 8b B
    Sicherungskreis8bB,
    /// Undefined fault type (fallback for unknown faults)
    #[serde(other)]
    Undifined,
}

/// Represents a complete diagnostic message containing fault information.
///
/// This structure encapsulates all information needed to identify and locate
/// a fault within the vehicle system, including the vehicle number, cabin
/// location, fault type, and current state.
///
/// # Examples
///
/// ```rust
/// use diagnostic_messages::{DiagnosticMessage, DiagnosticFaultKind};
///
/// let message = DiagnosticMessage {
///     veh_number: "1234".to_string(),
///     cabin: None,
///     kind: DiagnosticFaultKind::HauptschalterA,
///     state: true,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiagnosticMessage {
    /// Vehicle identification number
    pub veh_number: String,
    /// Optional cabin identifier where the fault occurred
    pub cabin: Option<CockpitSide>,
    /// Type of diagnostic fault
    pub kind: DiagnosticFaultKind,
    /// Current state of the fault (true = active, false = cleared)
    pub state: bool,
}

//--------------------------------

message_type!(DiagnosticMessage, "Pan_Diagnostic", "Diagnostic", "MMS");

/// Manages the sending of diagnostic messages with state change detection.
///
/// This sender ensures that diagnostic messages are only transmitted when
/// the fault state actually changes, reducing unnecessary network traffic.
/// It maintains a history of the last known state for each fault type.
///
/// # Examples
///
/// ```rust
/// use diagnostic_messages::{DiagnosticMessageSender, DiagnosticFaultKind};
///
/// let mut sender = DiagnosticMessageSender::new();
///
/// // This will send a message (first occurrence)
/// sender.send(DiagnosticFaultKind::HauptschalterA, true, None);
///
/// // This will not send a message (same state)
/// sender.send(DiagnosticFaultKind::HauptschalterA, true, None);
///
/// // This will send a message (state changed)
/// sender.send(DiagnosticFaultKind::HauptschalterA, false, None);
/// ```
#[derive(Default, Debug)]
pub struct DiagnosticMessageSender {
    value_last: HashMap<DiagnosticFaultKind, bool>,
}

impl DiagnosticMessageSender {
    /// Creates a new diagnostic message sender.
    ///
    /// # Returns
    ///
    /// A new `DiagnosticMessageSender` instance with an empty state history.
    pub fn new() -> Self {
        Self {
            value_last: HashMap::new(),
        }
    }

    /// Sends a diagnostic message if the state has changed.
    ///
    /// This method compares the current state with the last known state for
    /// the given fault kind. If the state has changed, it sends a diagnostic
    /// message and updates the internal state history.
    ///
    /// # Arguments
    ///
    /// * `kind` - The type of diagnostic fault
    /// * `state` - The current state of the fault (true = active, false = cleared)
    /// * `cabin` - Optional cabin identifier where the fault occurred
    ///
    /// # Examples
    ///
    /// ```rust
    /// use diagnostic_messages::{DiagnosticMessageSender, DiagnosticFaultKind};
    ///
    /// let mut sender = DiagnosticMessageSender::new();
    /// sender.send(DiagnosticFaultKind::BlinkerAusfall, true, None);
    /// ```
    pub fn send(&mut self, kind: DiagnosticFaultKind, state: bool, cabin: Option<CockpitSide>) {
        let last_value = self.value_last.get(&kind).unwrap_or(&false);
        if state != *last_value {
            send_message(
                &(DiagnosticMessage {
                    veh_number: veh_number(),
                    cabin,
                    kind,
                    state,
                }),
                [MessageTarget::Broadcast {
                    across_couplings: false,
                    include_self: true,
                }],
            );
            self.value_last.insert(kind, state);
        }
    }
}

/// Sends a diagnostic fault message across vehicle couplings.
///
/// This function broadcasts a diagnostic message to all coupled vehicles
/// in the train formation, excluding the sending vehicle itself.
///
/// # Arguments
///
/// * `value` - The diagnostic message to send
///
/// # Examples
///
/// ```rust
/// use diagnostic_messages::{send_diagnostic_flaut_msg, DiagnosticMessage, DiagnosticFaultKind};
///
/// let message = DiagnosticMessage {
///     veh_number: "1234".to_string(),
///     cabin: None,
///     kind: DiagnosticFaultKind::Zugbusfehler,
///     state: true,
/// };
/// send_diagnostic_flaut_msg(message);
/// ```
pub fn send_diagnostic_flaut_msg(value: DiagnosticMessage) {
    send_message(
        &(value),
        [MessageTarget::Broadcast {
            across_couplings: true,
            include_self: false,
        }],
    );
}

//--------------------------------

/// Message for anti-slide protection override status.
///
/// This message indicates whether the anti-slide protection system
/// has been manually overridden by the operator.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MsgDiagnosticAntiSlideOverride {
    /// Override status (true = overridden, false = normal operation)
    pub value: bool,
}

message_type!(
    MsgDiagnosticAntiSlideOverride,
    "Pan_Diagnostic",
    "AntiSlideProtectionOverride",
    "MMS"
);

/// Sends an anti-slide protection override message.
///
/// This function broadcasts the anti-slide override status to all
/// coupled vehicles in the train formation.
///
/// # Arguments
///
/// * `value` - Override status (true = overridden, false = normal)
///
/// # Examples
///
/// ```rust
/// use diagnostic_messages::send_antislide_override;
///
/// // Indicate anti-slide protection is overridden
/// send_antislide_override(true);
/// ```
pub fn send_antislide_override(value: bool) {
    send_message(
        &(MsgDiagnosticAntiSlideOverride { value }),
        [MessageTarget::Broadcast {
            across_couplings: true,
            include_self: false,
        }],
    );
}

//--------------------------------

/// Message for driver's air conditioning diagnostic information.
///
/// This message contains diagnostic data related to the driver's
/// compartment air conditioning system.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MsgDiagnosticDriverAirCon {
    /// Air conditioning diagnostic value (units depend on specific measurement)
    pub value: f32,
}

message_type!(
    MsgDiagnosticDriverAirCon,
    "Pan_Diagnostic",
    "DriverAirCon",
    "MMS"
);

//--------------------------------

/// Message for pantograph switching state.
///
/// This message reports the current switching state of the vehicle's
/// pantograph system used for power collection from overhead lines.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiagnosticPantoState {
    /// Current switching state of the pantograph
    pub value: SwitchingState,
}

message_type!(DiagnosticPantoState, "Pan_Diagnostic", "PantoState", "MMS");

/// Manages sending of pantograph state messages with change detection.
///
/// This sender only transmits pantograph state messages when the state
/// actually changes, preventing unnecessary network traffic.
#[derive(Default, Debug)]
pub struct DiagnosticPantoStateSender {
    value_last: SwitchingState,
}

impl DiagnosticPantoStateSender {
    /// Sends a pantograph state message if the state has changed.
    ///
    /// # Arguments
    ///
    /// * `value` - The current pantograph switching state
    ///
    /// # Examples
    ///
    /// ```rust
    /// use diagnostic_messages::MsgDiagnosticPantoStateSender;
    /// use crate::management::enums::state_enums::SwitchingState;
    ///
    /// let mut sender = MsgDiagnosticPantoStateSender::default();
    /// sender.send(SwitchingState::On);
    /// ```
    pub fn send(&mut self, value: SwitchingState) {
        if value != self.value_last {
            send_message(
                &(DiagnosticPantoState { value }),
                [MessageTarget::Broadcast {
                    across_couplings: false,
                    include_self: true,
                }],
            );
            self.value_last = value;
        }
    }
}

//--------------------------------

/// Message for door state diagnostic information.
///
/// This message reports the current target state of the vehicle's door system,
/// indicating the intended door operation mode.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiagnosticDoorState {
    /// Current door target state
    pub value: DoorTarget,
}

message_type!(DiagnosticDoorState, "Pan_Diagnostic", "DoorState", "MMS");

/// Manages sending of door state messages with change detection.
///
/// This sender only transmits door state messages when the state
/// actually changes, preventing unnecessary network traffic.
#[derive(Default, Debug)]
pub struct DiagnosticDoorStateSender {
    value_last: DoorTarget,
}

impl DiagnosticDoorStateSender {
    /// Sends a door state message if the state has changed.
    ///
    /// # Arguments
    ///
    /// * `value` - The current door target state
    ///
    /// # Examples
    ///
    /// ```rust
    /// use diagnostic_messages::MsgDiagnosticDoorStateSender;
    /// use crate::management::enums::door_enums::DoorTarget;
    ///
    /// let mut sender = MsgDiagnosticDoorStateSender::default();
    /// sender.send(DoorTarget::Open);
    /// ```
    pub fn send(&mut self, value: DoorTarget) {
        if value != self.value_last {
            send_message(
                &(DiagnosticDoorState { value }),
                [MessageTarget::Broadcast {
                    across_couplings: false,
                    include_self: true,
                }],
            );
            self.value_last = value;
        }
    }
}

//--------------------------------

/// Message for wheel slip diagnostic information.
///
/// This message indicates whether wheel slip has been detected on the vehicle,
/// which is important for traction control and safety systems.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiagnosticSlip {
    /// Slip detection status (true = slip detected, false = no slip)
    pub value: bool,
}

message_type!(DiagnosticSlip, "Pan_Diagnostic", "Slip", "MMS");

/// Manages sending of slip detection messages with change detection.
///
/// This sender only transmits slip detection messages when the state
/// actually changes, preventing unnecessary network traffic.
#[derive(Default, Debug)]
pub struct DiagnosticSlipSender {
    value_last: bool,
}

impl DiagnosticSlipSender {
    /// Sends a slip detection message if the state has changed.
    ///
    /// # Arguments
    ///
    /// * `value` - The current slip detection state (true = slip detected, false = no slip)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use diagnostic_messages::MsgDiagnosticSlipSender;
    ///
    /// let mut sender = MsgDiagnosticSlipSender::default();
    /// sender.send(true); // Slip detected
    /// ```
    pub fn send(&mut self, value: bool) {
        if value != self.value_last {
            send_message(
                &(DiagnosticSlip { value }),
                [MessageTarget::Broadcast {
                    across_couplings: false,
                    include_self: true,
                }],
            );
            self.value_last = value;
        }
    }
}
