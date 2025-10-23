use lotus_script::{
    message::{send_message, MessageTarget},
    prelude::message_type,
    time::delta,
};
use serde::{Deserialize, Serialize};

//===================================================================
// Vehicle number
//===================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VehNumber {
    pub value: String,
}

message_type!(VehNumber, "Std", "Vehiclenumber");

//---------------------------

pub fn send_veh_number(value: String) {
    send_message(
        &(VehNumber { value }),
        [MessageTarget::Broadcast {
            across_couplings: false,
            include_self: true,
        }],
    );
}

//===================================================================
// Batterymainswitch & Batteryvoltage
//===================================================================

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Batteryvoltage {
    On(f32), // Battery is on, with voltage as f32
    #[default]
    Off, // Battery is off, no voltage needed
}

// Custom PartialEq to only compare state type, not inner values
impl PartialEq for Batteryvoltage {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Batteryvoltage::On(_), Batteryvoltage::On(_))
                | (Batteryvoltage::Off, Batteryvoltage::Off)
        )
    }
}

impl Batteryvoltage {
    pub fn is_on(&self) -> bool {
        match self {
            Batteryvoltage::On(_) => true,
            Batteryvoltage::Off => false,
        }
    }

    pub fn is_off(&self) -> bool {
        !self.is_on()
    }
}

//---------------------------

message_type!(Batteryvoltage, "Std", "Batteryvoltage");

//---------------------------

#[derive(Debug)]
pub struct BatteryvoltageSender {
    targets: Vec<MessageTarget>,
    state_last: bool,
    value_last: f32,
}

impl BatteryvoltageSender {
    pub fn new(targets: impl IntoIterator<Item = MessageTarget>) -> Self {
        Self {
            targets: targets.into_iter().collect::<Vec<_>>(),
            state_last: false,
            value_last: 0.0,
        }
    }

    // This function only sends updates if the values have changed
    pub fn send(&mut self, state: bool, value: f32) {
        // Sends message if state or value changed significantly
        if (value - self.value_last).abs() > 0.01 || self.state_last != state {
            send_message(
                &(match state {
                    true => Batteryvoltage::On(value),
                    false => Batteryvoltage::Off,
                }),
                self.targets.clone(),
            );
            self.value_last = value;
            self.state_last = state;
        }
    }
}

impl Default for BatteryvoltageSender {
    fn default() -> Self {
        BatteryvoltageSender::new(vec![MessageTarget::Broadcast {
            across_couplings: false,
            include_self: true,
        }])
    }
}

//===================================================================
// Power Signal
//===================================================================

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PowerSignalCabin {
    ACab,  // Cabin A (Index = 0)
    BCab,  // Cabin B (Index = 1)
    NoCab, // A other Cab (not in this Car)
}

impl From<i32> for PowerSignalCabin {
    fn from(val: i32) -> Self {
        match val {
            0 => PowerSignalCabin::ACab,
            1 => PowerSignalCabin::BCab,
            _ => PowerSignalCabin::NoCab,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PowerSignalState {
    On {
        // Cabin is avitvated
        // quickstart: true = do quickstart, false = do regular start
        // For cabin_id see PowerSignalCabin
        quickstart: bool,
        cabin_id: PowerSignalCabin,
    },
    Off {
        // Cabin is deavitvated
        // For cabin_id see PowerSignalCabin
        cabin_id: PowerSignalCabin,
    },
}

impl PowerSignalState {
    pub fn is_on(&self) -> bool {
        match self {
            PowerSignalState::On {
                quickstart,
                cabin_id,
            } => true,
            PowerSignalState::Off { cabin_id } => false,
        }
    }

    pub fn is_off(&self) -> bool {
        !self.is_on()
    }
}

impl Default for PowerSignalState {
    fn default() -> Self {
        PowerSignalState::Off {
            cabin_id: PowerSignalCabin::NoCab,
        }
    }
}

// Custom PartialEq to only compare state type, not inner values
//impl PartialEq for PowerSignalState {
//    fn eq(&self, other: &Self) -> bool {
//        matches!(
//            (self, other),
//            (PowerSignalState::On { .. }, PowerSignalState::On { .. })
//                | (PowerSignalState::Off { .. }, PowerSignalState::Off { .. })
//        )
//    }
//}

//---------------------------

message_type!(PowerSignalState, "Std", "PowerSignal");

//---------------------------

#[derive(Debug)]
pub struct PowerSignalSender {
    targets: Vec<MessageTarget>,
    state_last: PowerSignalState,
}

impl PowerSignalSender {
    pub fn new(targets: impl IntoIterator<Item = MessageTarget>) -> Self {
        Self {
            targets: targets.into_iter().collect::<Vec<_>>(),
            state_last: PowerSignalState::default(),
        }
    }

    // Send quickstart for cabin_id, only if cabin_activ
    pub fn quickstart(&mut self, cabin_activ: bool, cabin_id: PowerSignalCabin) {
        if cabin_activ {
            send_message(
                &(PowerSignalState::On {
                    quickstart: true,
                    cabin_id,
                }),
                self.targets.clone(),
            );
            self.state_last = PowerSignalState::On {
                quickstart: true,
                cabin_id,
            };
        }
    }

    // Sends state to cabin_id
    // This function only sends updates if the values have changed
    pub fn send(&mut self, state: bool, cabin_id: PowerSignalCabin) {
        let value = match state {
            true => PowerSignalState::On {
                quickstart: false,
                cabin_id,
            },
            false => PowerSignalState::Off { cabin_id },
        };

        let equal = match (value, self.state_last) {
            (
                PowerSignalState::On {
                    quickstart: _,
                    cabin_id: cab1,
                },
                PowerSignalState::On {
                    quickstart: _,
                    cabin_id: cab2,
                },
            ) => cab1 == cab2,
            (
                PowerSignalState::Off { cabin_id: cab1 },
                PowerSignalState::Off { cabin_id: cab2 },
            ) => cab1 == cab2,
            _ => false,
        };

        if !equal {
            self.state_last = value;
            send_message(&(value), self.targets.clone());
        }
    }
}

impl Default for PowerSignalSender {
    fn default() -> Self {
        PowerSignalSender::new(vec![MessageTarget::Broadcast {
            across_couplings: false,
            include_self: true,
        }])
    }
}

//---------------------------

pub fn send_mainswitch(value: PowerSignalState, targets: impl IntoIterator<Item = MessageTarget>) {
    send_message(&(value), targets);
}

//===================================================================
// Light
//===================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Light {
    pub value: f32,
}

message_type!(Light, "Std", "Light");

//---------------------------

#[derive(Debug)]
pub struct LightSender {
    targets: Vec<MessageTarget>,
    value_last: f32,
}

impl LightSender {
    pub fn new(targets: impl IntoIterator<Item = MessageTarget>) -> Self {
        Self {
            targets: targets.into_iter().collect::<Vec<_>>(),
            value_last: 0.0,
        }
    }

    // Sends value, if changed significantly, to all defined targets
    pub fn send(&mut self, value: f32) {
        if (value - self.value_last).abs() > 0.01 {
            send_message(&(Light { value }), self.targets.clone());
            self.value_last = value;
        }
    }
}

impl Default for LightSender {
    fn default() -> Self {
        LightSender::new(vec![MessageTarget::Broadcast {
            across_couplings: false,
            include_self: true,
        }])
    }
}

//===================================================================
// Stop Request
//===================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StopRequest {
    pub value: bool,
}

message_type!(StopRequest, "Std", "StopRequest");

//---------------------------

#[derive(Debug)]
pub struct StopRequestSender {
    targets: Vec<MessageTarget>,
    value_last: bool,
}

impl StopRequestSender {
    pub fn new(targets: impl IntoIterator<Item = MessageTarget>) -> Self {
        Self {
            targets: targets.into_iter().collect::<Vec<_>>(),
            value_last: false,
        }
    }

    // Sends value to all moduls defined targets
    // This function only sends updates if the values have changed
    pub fn send(&mut self, value: bool) {
        if value != self.value_last {
            send_message(&(StopRequest { value }), self.targets.clone());
            self.value_last = value;
        }
    }
}

impl Default for StopRequestSender {
    fn default() -> Self {
        StopRequestSender::new(vec![MessageTarget::Broadcast {
            across_couplings: false,
            include_self: true,
        }])
    }
}

//===================================================================
// Velocity
//===================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Velocity {
    pub value: f32,
}

message_type!(Velocity, "Std", "Velocity");

//---------------------------

#[derive(Debug)]
pub struct VelocitySender {
    targets: Vec<MessageTarget>,
    timer: f32,
}

impl VelocitySender {
    pub fn new(targets: impl IntoIterator<Item = MessageTarget>) -> Self {
        Self {
            targets: targets.into_iter().collect::<Vec<_>>(),
            timer: 0.0,
        }
    }

    // Sends value every 0.3 seconds to the defined target
    // Call this function every tick!
    pub fn tick(&mut self, value: f32) {
        self.timer -= delta();
        if self.timer < 0.0 {
            send_message(&(Velocity { value }), self.targets.clone());
            self.timer += 0.3;
        }
    }
}

impl Default for VelocitySender {
    fn default() -> Self {
        VelocitySender::new(vec![MessageTarget::Broadcast {
            across_couplings: false,
            include_self: true,
        }])
    }
}

//===================================================================
// Door State
//===================================================================

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum DoorSide {
    #[default]
    None, // No side set
    Right, // Doors on the right side are targeted
    Left,  // Doors on the left side are targeted
    Both,  // Doors on the both sides are targeted
}

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum DoorControlTarget {
    #[default]
    Closed, // Doors are not released or opened by the door control unit. Emergency releases etc. are not taken into account here.
    Released(DoorSide), // Doors are released by the door control unit.
    Open(DoorSide),     // Doors are forced open by the door control unit.
}

//---------------------------

message_type!(DoorControlTarget, "Std", "DoorState");

//---------------------------

pub struct DoorStateSender {
    targets: Vec<MessageTarget>,
    value_last: DoorControlTarget,
}

impl DoorStateSender {
    pub fn new(targets: impl IntoIterator<Item = MessageTarget>) -> Self {
        Self {
            targets: targets.into_iter().collect::<Vec<_>>(),
            value_last: DoorControlTarget::default(),
        }
    }

    // Sends value to all moduls defined targets
    // This function only sends updates if the values have changed
    pub fn send(&mut self, value: DoorControlTarget) {
        if value != self.value_last {
            send_message(&(value), self.targets.clone());
            self.value_last = value;
        }
    }
}

impl Default for DoorStateSender {
    fn default() -> Self {
        DoorStateSender::new(vec![MessageTarget::Broadcast {
            across_couplings: false,
            include_self: true,
        }])
    }
}

//---------------------------

pub fn send_door_control_msg(
    value: DoorControlTarget,
    targets: impl IntoIterator<Item = MessageTarget>,
) {
    send_message(&(value), targets);
}

//===================================================================
// Power Hold
//===================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DisplayPowerHold {
    pub value: bool,
}

message_type!(DisplayPowerHold, "Std", "DisplayPowerHold", "ModulParams");

//---------------------------

#[derive(Debug)]
pub struct DisplayPowerHoldSender {
    value_last: bool,
}

impl DisplayPowerHoldSender {
    pub fn send(&mut self, value: bool) {
        if value != self.value_last {
            send_message(
                &(DisplayPowerHold { value }),
                [MessageTarget::Broadcast {
                    across_couplings: true,
                    include_self: false,
                }],
            );
            self.value_last = value;
        }
    }
}
