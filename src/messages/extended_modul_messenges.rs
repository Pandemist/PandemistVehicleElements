use lotus_script::message::{message_type, send_message, MessageTarget};
use serde::{Deserialize, Serialize};

//===================================================================
// Panto Initial
//===================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PantoControl {
    Rope,
    Electric,
}

message_type!(PantoControl, "ExtStd", "PantoControl");

//===================================================================
// Panto electric Target
//===================================================================

#[derive(Default, Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum SimplePantoTarget {
    Up,
    Down,
    #[default]
    Neutral,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PantoElectricTarget {
    pub value: SimplePantoTarget,
}

message_type!(PantoElectricTarget, "ExtStd", "PantoElectricTarget");

//===================================================================
// Panto cranc Target
//===================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PantoCrancTarget {
    pub value: SimplePantoTarget,
}

message_type!(PantoCrancTarget, "ExtStd", "PantoCrancTarget");

//===================================================================
// Panto rope Target
//===================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PantoRopeTarget {
    pub value: Option<f32>,
}

message_type!(PantoRopeTarget, "ExtStd", "PantoRopeTarget");

//===================================================================
// Wire height
//===================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PantoWireHeight {
    pub value: Option<f32>,
}

message_type!(PantoWireHeight, "ExtStd", "PantoWireHeight");

#[derive(Debug)]
pub struct WireHeightSender {
    targets: Vec<MessageTarget>,
    value_last: Option<f32>,
}

impl WireHeightSender {
    pub fn new(targets: impl IntoIterator<Item = MessageTarget>) -> Self {
        Self {
            targets: targets.into_iter().collect::<Vec<_>>(),
            value_last: None,
        }
    }

    // Sends value, if changed significantly, to all defined targets
    pub fn send(&mut self, value: Option<f32>) {
        let cond = match (value, self.value_last) {
            (Some(a), Some(b)) => (a - b).abs() > 0.01,
            (_, _) => true,
        };

        if cond {
            send_message(&PantoWireHeight { value }, self.targets.clone());
            self.value_last = value;
        }
    }
}

impl Default for WireHeightSender {
    fn default() -> Self {
        WireHeightSender::new(vec![MessageTarget::Broadcast {
            across_couplings: false,
            include_self: true,
        }])
    }
}

//===================================================================
// Panto Voltage
//===================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PantoVoltage {
    pub value_normalized: f32,
}

message_type!(PantoVoltage, "ExtStd", "PantoVoltage");

//===================================================================
// Panto Ampere
//===================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PantoAmpere {
    pub value_absolut: f32,
    pub value_normalized: f32,
}

message_type!(PantoAmpere, "ExtStd", "PantoAmpere");

//===================================================================
// Panto State
//===================================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PantoStateEnum {
    Raised,  // Gehoben
    Raise,   // Heben
    Lowered, // Gesenkt
    Lower,   // Senken
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PantoState {
    pub state: Option<PantoStateEnum>,
    pub rope: Option<f32>,
    pub contact: bool,
}

message_type!(PantoState, "ExtStd", "PantoState");
