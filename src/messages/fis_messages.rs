use std::fmt;

use lotus_script::{content::ContentId, prelude::message_type};
use serde::{Deserialize, Serialize};
use time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TerminusText {
    line_1: String,
    line_2: String,
}

impl From<[String; 2]> for TerminusText {
    fn from(val: [String; 2]) -> Self {
        let line_1 = val.first().unwrap_or(&"".to_string()).to_string();
        let line_2 = val.get(1).unwrap_or(&"".to_string()).to_string();

        TerminusText { line_1, line_2 }
    }
}

impl TerminusText {
    pub fn new(line_1: impl Into<String>, line_2: impl Into<String>) -> Self {
        TerminusText {
            line_1: line_1.into(),
            line_2: line_2.into(),
        }
    }
    pub fn new_empty() -> Self {
        TerminusText {
            line_1: "".to_string(),
            line_2: "".to_string(),
        }
    }
    pub fn is_multiline(&self) -> bool {
        !self.line_2.is_empty()
    }
    pub fn line_1(&self) -> String {
        self.line_1.clone()
    }
    pub fn line_2(&self) -> String {
        self.line_2.clone()
    }
    pub fn is_empty(&self) -> bool {
        self.line_1.is_empty() && self.line_2.is_empty()
    }
}

//-----------------------------
// Lotus Messages
//-----------------------------

//===========================================================
// Current line number & specialchar (entered by hand, or by line/route default)
// Mandatory

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LineSpecialchar {
    pub line: u32,
    pub specialchar: u32,
}

message_type!(LineSpecialchar, "Std_Pis", "LineSpecialchar", "TrainBus");

//-----------------------------
// Current course
// Mandatory

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Course {
    pub value: u32,
}

message_type!(Course, "Std_Pis", "Course", "TrainBus");

//-----------------------------
// Current route
// Mandatory

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Route {
    pub value: u32,
}

message_type!(Route, "Std_Pis", "Route", "TrainBus");

//-----------------------------
// Current target code or overwritten text
// Mandatory

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Terminus {
    pub code: u32,
    pub override_front: Option<TerminusText>,
    pub override_side: Option<TerminusText>,
}

message_type!(Terminus, "Std_Pis", "Terminus", "TrainBus");

//-----------------------------
// Code for routes and points request
// Mandatory

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoutingCode {
    pub value: u32,
}

message_type!(RoutingCode, "Std_Pis", "RoutingCode", "TrainBus");

//-----------------------------
// Text to explicitly describe simple interior displays
// Optional

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StopText {
    pub value: TerminusText,
}

message_type!(StopText, "Std_Pis", "StopText", "TrainBus");

//-----------------------------
// Stop on the currently set route (0 based)
// Mandatory

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StopIndex {
    pub value: Option<u32>,
}

message_type!(StopIndex, "Std_Pis", "StopIndex", "TrainBus");

//-----------------------------
// Parameters for validators
// Mandatory

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Validator {
    pub zone: Option<u32>,
    pub stop_index: Option<u32>,
}

message_type!(Validator, "Std_Pis", "Validator", "TrainBus");

//-----------------------------
// Deviation of the timestamp in the terminal from that of the simulation
// Optional

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimeDelta {
    pub value: Duration,
}

message_type!(TimeDelta, "Std_Pis", "TimeDelta", "TrainBus");

//-----------------------------
// Content IDs of the announcements that are to be played
// Optional

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Announcement {
    pub value: Vec<ContentId>,
}

message_type!(Announcement, "Std_Pis", "Announcement", "TrainBus");

//-----------------------------
// Switch control commands, can also come from the vehicle
// Optional

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum RoutingDirection {
    Left,
    Right,
    Straight,
    #[default]
    Off,
}

impl fmt::Display for RoutingDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RoutingDirection::Left => write!(f, "Left"),
            RoutingDirection::Right => write!(f, "Right"),
            RoutingDirection::Straight => write!(f, "Straight"),
            RoutingDirection::Off => write!(f, "Off"),
        }
    }
}

message_type!(RoutingDirection, "Std_Pis", "RoutingDirection", "TrainBus");

//-----------------------------
// Signal request
// Optional

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoutingRequest {}

message_type!(RoutingRequest, "Std_Pis", "RoutingRequest", "TrainBus");

//===========================================================
// Send displayed Terminus to Vehicle

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NowDisplaying {
    pub code: Option<u32>,
}

message_type!(NowDisplaying, "Std_Pis", "Displaying");
