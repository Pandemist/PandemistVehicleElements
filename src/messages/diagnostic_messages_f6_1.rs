use lotus_extra::{messages::std::Batteryvoltage, vehicle::CockpitSide};
use lotus_script::prelude::{message_type, send_message, MessageTarget};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiagnosticDeviceSide {
    pub value: CockpitSide,
}

message_type!(DiagnosticDeviceSide, "Pan_Diagnostic", "DeviceSide");

//--------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiagnosticZugbustaufe {
    pub value: f32,
}

message_type!(DiagnosticZugbustaufe, "Pan_Diagnostic_F6_1", "Zugbustaufe");

pub fn snd_zugbustaufe(timer: f32) {
    send_message(
        &DiagnosticZugbustaufe { value: timer },
        [MessageTarget::Broadcast {
            across_couplings: true,
            include_self: false,
        }],
    );
}

//--------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiagnosticASGstate {
    pub value: bool,
    pub cart: i32,
}

message_type!(DiagnosticASGstate, "Pan_Diagnostic_F6_1", "ASG", "MMS");

//--------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticBatVoltage {
    pub value: Batteryvoltage,
    pub car: i32,
}

message_type!(
    DiagnosticBatVoltage,
    "Pan_Diagnostic_F6_1",
    "BatVoltage",
    "MMS"
);

pub struct DiagnosticVoltageSender {
    state_last: bool,
    value_last: f32,
    car: i32,
}

impl DiagnosticVoltageSender {
    pub fn new(car: i32) -> Self {
        Self {
            state_last: false,
            value_last: 0.0,
            car,
        }
    }

    pub fn send(&mut self, state: bool, value: f32) {
        // Sends message if state or value changed significantly
        if (value - self.value_last).abs() > 0.01 || self.state_last != state {
            send_message(
                &DiagnosticBatVoltage {
                    value: match state {
                        true => Batteryvoltage::On(value),
                        false => Batteryvoltage::Off,
                    },
                    car: self.car,
                },
                MessageTarget::broadcast_all(),
            );
            self.value_last = value;
            self.state_last = state;
        }
    }
}
