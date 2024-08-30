use lotus_script::var::VariableType;

use super::{
    mock_enums::{PlayerInitPos, VehicleInitState},
    texture::Texture,
};

// Entspricht der Variable RealisticElecSupply
pub fn realisitc_electric_supply() -> bool {
    bool::get("RealisticElecSupply")
}

// Entspricht der Variable InitReadyForMovement
pub fn init_ready_state() -> VehicleInitState {
    match i8::get("InitReadyForMovement") {
        2 => VehicleInitState::ReadyToDrive,
        1 => VehicleInitState::Setuped,
        _ => VehicleInitState::ColdAndDark,
    }
}

// Entspricht der Variable InitPosInTrain
pub fn init_pos_in_train() -> u32 {
    u32::get("InitPosInTrain")
}

// Entspricht der Variable InitCarIsReversed
pub fn init_car_is_reversed() -> bool {
    bool::get("InitCarIsReversed")
}

// Entspricht der Variable InitUserPlaced
pub fn init_user_placed() -> PlayerInitPos {
    match i8::get("InitUserPlaced") {
        1 => PlayerInitPos::FrontCab,
        -1 => PlayerInitPos::BackCab,
        _ => PlayerInitPos::NotHere,
    }
}

// Entspricht der Variable DeadMansSwitch
pub fn deadmans_switch() -> bool {
    bool::get("DeadMansSwitch")
}

// Entspricht der Variable invradius_abs_max
pub fn invradius_abs_max() -> f32 {
    f32::get("invradius_abs_max")
}

// Entspricht der Variable mirrortex_{i}
pub fn mirror_tex(i: usize) -> Texture {
    todo!()
}

// Entspricht der Variable TextureRaindropSet_{x}
pub fn raindrop_tex(i: usize) -> Texture {
    todo!()
}
