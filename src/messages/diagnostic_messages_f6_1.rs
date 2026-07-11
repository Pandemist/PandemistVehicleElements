use std::collections::HashMap;

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

//--------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiagnosticWorkshopMode {
    pub value: bool,
}

message_type!(
    DiagnosticWorkshopMode,
    "Pan_Diagnostic_F6_1",
    "WorkshopMode"
);

//--------------------------------

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum DiagnosticStatusReportKind {
    KeineFahrspannung,
    AutomatAus,
    Notbremse,
    Stromabnehmer,
    TuerNotauf,
    TuerOffen,
    Federspeicher,
    KeineLuft,
    Stoerstrom,
    AntriebAus,
    BMA,
    LufthahnZKE,
}

impl DiagnosticStatusReportKind {
    pub fn get_pos(&self) -> (i32, i32) {
        match self {
            DiagnosticStatusReportKind::KeineFahrspannung => (0, 0),
            DiagnosticStatusReportKind::AutomatAus => (1, 0),
            DiagnosticStatusReportKind::Notbremse => (0, 1),
            DiagnosticStatusReportKind::Stromabnehmer => (1, 1),
            DiagnosticStatusReportKind::TuerNotauf => (0, 2),
            DiagnosticStatusReportKind::TuerOffen => (1, 2),
            DiagnosticStatusReportKind::Federspeicher => (0, 3),
            DiagnosticStatusReportKind::KeineLuft => (1, 3),
            DiagnosticStatusReportKind::Stoerstrom => (0, 4),
            DiagnosticStatusReportKind::AntriebAus => (1, 4),
            DiagnosticStatusReportKind::BMA => (0, 5),
            DiagnosticStatusReportKind::LufthahnZKE => (1, 5),
        }
    }

    pub fn get_text(&self) -> String {
        match self {
            DiagnosticStatusReportKind::KeineFahrspannung => "keine Fahrspg.".to_string(),
            DiagnosticStatusReportKind::AutomatAus => "Automat aus".to_string(),
            DiagnosticStatusReportKind::Notbremse => "Notbremse".to_string(),
            DiagnosticStatusReportKind::Stromabnehmer => "Stromabnehmer".to_string(),
            DiagnosticStatusReportKind::TuerNotauf => "Notöffnung".to_string(),
            DiagnosticStatusReportKind::TuerOffen => "Tür offen".to_string(),
            DiagnosticStatusReportKind::Federspeicher => "Federspeicher".to_string(),
            DiagnosticStatusReportKind::KeineLuft => "keine Luft".to_string(),
            DiagnosticStatusReportKind::Stoerstrom => "Störstrom".to_string(),
            DiagnosticStatusReportKind::AntriebAus => "Antrieb aus".to_string(),
            DiagnosticStatusReportKind::BMA => "Brandalarm".to_string(),
            DiagnosticStatusReportKind::LufthahnZKE => "Lufthahn ZKE".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiagnosticStatusMessage {
    pub veh_number: String,
    pub kind: DiagnosticStatusReportKind,
    pub state: bool,
}

message_type!(
    DiagnosticStatusMessage,
    "Pan_Diagnostic_F6_1",
    "StatusMessage"
);

//--------------------------------

#[derive(Default, Debug)]
pub struct DiagnosticStatusSender {
    value_last: HashMap<(DiagnosticStatusReportKind, String), bool>,
}

impl DiagnosticStatusSender {
    pub fn new() -> Self {
        Self {
            value_last: HashMap::new(),
        }
    }

    pub fn send(&mut self, kind: DiagnosticStatusReportKind, state: bool, veh_number: String) {
        let last_value = self
            .value_last
            .get(&(kind, veh_number.clone()))
            .unwrap_or(&false);
        if state != *last_value {
            send_message(
                &(DiagnosticStatusMessage {
                    veh_number: veh_number.clone(),
                    kind,
                    state,
                }),
                [MessageTarget::broadcast_all()],
            );
            self.value_last.insert((kind, veh_number), state);
        }
    }
}
