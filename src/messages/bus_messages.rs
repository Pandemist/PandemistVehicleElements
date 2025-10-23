use lotus_script::prelude::message_type;
use serde::{Deserialize, Serialize};

//===================================================================
// Engine
//===================================================================

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct EngineInvJ {
    value: f32,
}

message_type!(EngineInvJ, "Std_Engine", "inv_j");

//-----------------------------

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct EngineM {
    value: f32,
}

message_type!(EngineM, "Std_Engine", "M");

//-----------------------------

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct EngineThrottle {
    value: f32,
}

message_type!(EngineThrottle, "Std_Engine", "Throttle");

//-----------------------------

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct EngineRpm {
    value: f32,
}

message_type!(EngineRpm, "Std_Engine", "Rpm");

//-----------------------------

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct EngineStarter {
    value: i32,
}

message_type!(EngineStarter, "Std_Engine", "StarterShutoff");

//===================================================================
// Gearbox
//===================================================================

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct GearboxMode {
    value: f32,
}

message_type!(GearboxMode, "Std_Gearbox", "Rpm");

//-----------------------------

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct GearboxCurrGear {
    value: i32,
}

message_type!(GearboxCurrGear, "Std_Gearbox", "Gear_Current");

//-----------------------------

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct GearboxRpmInput {
    value: f32,
}

message_type!(GearboxRpmInput, "Std_Gearbox", "Rpm_Input");

//-----------------------------

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct GearboxMoutput {
    value: f32,
}

message_type!(GearboxMoutput, "Std_Gearbox", "M_output");

//-----------------------------

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct GearboxRetarder {
    value: i32,
}

message_type!(GearboxRetarder, "Std_Gearbox", "Retarder");

//-----------------------------

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct GearboxInvJinput {
    value: f32,
}

message_type!(GearboxInvJinput, "Std_Gearbox", "Inv_J_Input");

//-----------------------------

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct GearboxRpmOutput {
    value: f32,
}

message_type!(GearboxRpmOutput, "Std_Gearbox", "RpmOutput");

//-----------------------------

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct GearboxMinput {
    value: f32,
}

message_type!(GearboxMinput, "Std_Gearbox", "M_Input");

//-----------------------------

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct GearboxThrottle {
    value: f32,
}

message_type!(GearboxThrottle, "Std_Gearbox", "Throttle");
