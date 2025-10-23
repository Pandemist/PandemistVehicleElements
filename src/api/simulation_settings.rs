//! Simulation settings module for train simulation configuration.
//!
//! This module provides functions to retrieve various simulation settings that control
//! the initial state and behavior of the train simulation. All settings are retrieved
//! from the underlying script system using the lotus_script variable system.

use lotus_script::var::get_var;

use super::mock_enums::{PlayerInitPos, VehicleInitState};

/// Determines if realistic electric supply simulation is enabled.
///
/// Please note: As this feature has not yet been implemented, it is currently still a
/// dummy. As soon as the function is available, it will be supplied with real values
/// from the simulation.
///
/// # Returns
///
/// Returns `true` if realistic electric supply is enabled, `false` otherwise.
///
/// # Examples
///
/// ```
/// use your_crate::simulation_settings::realisitc_electric_supply;
///
/// if realisitc_electric_supply() {
///     println!("Realistic electric supply is enabled");
/// }
/// ```
///
/// # Corresponds to
///
/// Script variable: `"RealisticElecSupply"`
#[must_use]
pub fn realisitc_electric_supply() -> bool {
    //get_var::<bool>("RealisticElecSupply")
    false
}

/// Gets the initial ready state of the vehicle when the simulation starts.
///
/// This function determines the initial operational state of the vehicle, which affects
/// what systems are powered on and what procedures the player needs to follow to
/// begin operating the vehicle.
///
/// Please note: As this feature has not yet been implemented, it is currently still a
/// dummy. As soon as the function is available, it will be supplied with real values
/// from the simulation.
///
/// # Returns
///
/// Returns a [`VehicleInitState`] enum value:
/// - `ReadyToDrive` (2): Vehicle is fully operational and ready to move
/// - `Setuped` (1): Vehicle is powered but requires setup procedures
/// - `ColdAndDark` (default): Vehicle is completely powered down
///
/// # Examples
///
/// ```
/// use your_crate::simulation_settings::init_ready_state;
/// use your_crate::mock_enums::VehicleInitState;
///
/// match init_ready_state() {
///     VehicleInitState::ReadyToDrive => println!("Vehicle is ready to go!"),
///     VehicleInitState::Setuped => println!("Vehicle needs setup procedures"),
///     VehicleInitState::ColdAndDark => println!("Vehicle is powered down"),
/// }
/// ```
///
/// # Corresponds to
///
/// Script variable: `"InitReadyForMovement"`
#[must_use]
pub fn init_ready_state() -> VehicleInitState {
    match get_var::<i8>("InitReadyForMovement") {
        2 => VehicleInitState::ReadyToDrive,
        1 => VehicleInitState::Setuped,
        _ => VehicleInitState::ColdAndDark,
    };
    // TODO! Dummy Value - fix this to return the actual matched value
    VehicleInitState::ReadyToDrive
}

/// Gets the initial position of this vehicle within the train consist.
///
/// In multi-unit train operations, this determines which car in the consist
/// this particular vehicle represents. Position 0 is typically the lead car.
///
/// Please note: As this feature has not yet been implemented, it is currently still a
/// dummy. As soon as the function is available, it will be supplied with real values
/// from the simulation.
///
/// # Returns
///
/// Returns the zero-based position index within the train consist.
///
/// # Examples
///
/// ```
/// use your_crate::simulation_settings::init_pos_in_train;
///
/// let position = init_pos_in_train();
/// if position == 0 {
///     println!("This is the lead car");
/// } else {
///     println!("This is car #{} in the consist", position + 1);
/// }
/// ```
///
/// # Corresponds to
///
/// Script variable: `"InitPosInTrain"`
#[must_use]
pub fn init_pos_in_train() -> usize {
    (get_var::<i32>("InitPosInTrain")).max(0) as usize
}

/// Determines if this car is initially reversed in the train consist.
///
/// When a car is reversed, its front becomes its rear and vice versa.
///
/// Please note: As this feature has not yet been implemented, it is currently still a
/// dummy. As soon as the function is available, it will be supplied with real values
/// from the simulation.
///
/// # Returns
///
/// Returns `true` if the car is reversed, `false` if in normal orientation.
///
/// # Examples
///
/// ```
/// use your_crate::simulation_settings::init_car_is_reversed;
///
/// if init_car_is_reversed() {
///     println!("Car is reversed - controls may be mirrored");
/// } else {
///     println!("Car is in normal orientation");
/// }
/// ```
///
/// # Corresponds to
///
/// Script variable: `"InitCarIsReversed"`
#[must_use]
pub fn init_car_is_reversed() -> bool {
    get_var::<bool>("InitCarIsReversed")
}

/// Gets the initial position of the player within the vehicle.
///
/// This determines where the player character starts when the simulation begins,
/// which affects available controls and viewing angles.
///
/// Please note: As this feature has not yet been implemented, it is currently still a
/// dummy. As soon as the function is available, it will be supplied with real values
/// from the simulation.
///
/// # Returns
///
/// Returns a [`PlayerInitPos`] enum value:
/// - `CabFront` (1): Player starts in the front cab
/// - `CabRear` (-1): Player starts in the rear cab  
/// - `NotHere` (default): Player is not initially placed in this vehicle
///
/// # Examples
///
/// ```
/// use your_crate::simulation_settings::init_user_placed;
/// use your_crate::mock_enums::PlayerInitPos;
///
/// match init_user_placed() {
///     PlayerInitPos::CabFront => println!("Player starts in front cab"),
///     PlayerInitPos::CabRear => println!("Player starts in rear cab"),
///     PlayerInitPos::NotHere => println!("Player not in this vehicle"),
/// }
/// ```
///
/// # Corresponds to
///
/// Script variable: `"InitUserPlaced"`
#[must_use]
pub fn init_user_placed() -> PlayerInitPos {
    match get_var::<i8>("InitUserPlaced") {
        1 => PlayerInitPos::CabFront,
        -1 => PlayerInitPos::CabRear,
        _ => PlayerInitPos::NotHere,
    }
}

/// Determines if the dead man's switch safety system is enabled.
///
/// Please note: As this feature has not yet been implemented, it is currently still a
/// dummy. As soon as the function is available, it will be supplied with real values
/// from the simulation.
///
/// # Returns
///
/// Returns `true` if the dead man's switch is active, `false` if disabled.
///
/// # Examples
///
/// ```
/// use your_crate::simulation_settings::deadmans_switch;
///
/// if deadmans_switch() {
///     println!("Dead man's switch is active - remember to hold the pedal!");
/// } else {
///     println!("Dead man's switch is disabled");
/// }
/// ```
///
/// # Corresponds to
///
/// Script variable: `"DeadMansSwitch"`
#[must_use]
pub fn deadmans_switch() -> bool {
    //get_var::<bool>("DeadMansSwitch")
    true
}
