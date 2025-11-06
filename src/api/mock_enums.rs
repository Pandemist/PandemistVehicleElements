//! Mock enums for vehicle simulation and train control systems.
//!
//! This module provides enumerations for representing various states and conditions
//! in vehicle simulation applications, particularly for train and rail vehicle systems.

/// Represents the current state of a coupling mechanism (clutch).
///
/// This enum describes the operational status of a vehicle's coupling system,
/// which is used to connect or disconnect mechanical components.
///
/// # Examples
///
/// ```
/// use pandemist_vehicle_elements_name::CouplingState;
///
/// let coupling = CouplingState::Ready;
/// match coupling {
///     CouplingState::Deactivated => println!("Coupling is off"),
///     CouplingState::Ready => println!("Coupling is ready to engage"),
///     CouplingState::Coupled => println!("Coupling is actively engaged"),
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum CouplingState {
    /// The coupling system is completely deactivated and cannot engage
    Deactivated,
    /// The coupling system is active and ready to engage when needed
    Ready,
    /// The coupling is currently engaged and transmitting power/motion
    Coupled,
}

/// Represents the initialization state of a vehicle system.
///
/// This enum defines the startup sequence stages for a vehicle, from completely
/// powered down to fully operational. The enum implements ordering to allow
/// comparison of initialization levels.
///
/// # Examples
///
/// ```
/// use pandemist_vehicle_elements_name::VehicleInitState;
///
/// let current_state = VehicleInitState::Setuped;
/// let target_state = VehicleInitState::ReadyToDrive;
///
/// if current_state < target_state {
///     println!("Still initializing...");
/// }
///
/// // Convert to integer representation
/// let state_code: i32 = current_state.into();
/// assert_eq!(state_code, 1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum VehicleInitState {
    /// Vehicle is completely powered down with all systems inactive
    ColdAndDark = 0,
    /// Vehicle systems are configured but not yet ready for operation
    Setuped = 1,
    /// Vehicle is fully initialized and ready for normal operation
    ReadyToDrive = 2,
}

impl From<VehicleInitState> for i32 {
    /// Converts a `VehicleInitState` to its corresponding integer representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements_name::VehicleInitState;
    ///
    /// let state = VehicleInitState::ReadyToDrive;
    /// let code: i32 = state.into();
    /// assert_eq!(code, 2);
    /// ```
    fn from(value: VehicleInitState) -> Self {
        match value {
            VehicleInitState::ColdAndDark => 0,
            VehicleInitState::Setuped => 1,
            VehicleInitState::ReadyToDrive => 2,
        }
    }
}

/// Represents the initial position of the player/operator in the vehicle.
///
/// This enum defines where the player or operator starts their session within
/// the vehicle, typically used in simulation or training applications.
///
/// # Examples
///
/// ```
/// use pandemist_vehicle_elements_name::PlayerInitPos;
///
/// let start_position = PlayerInitPos::CabFront;
/// match start_position {
///     PlayerInitPos::CabFront => println!("Starting in front cab"),
///     PlayerInitPos::CabRear => println!("Starting in rear cab"),
///     PlayerInitPos::NotHere => println!("Player not in vehicle"),
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum PlayerInitPos {
    /// Player starts in the front operator cab
    CabFront,
    /// Player starts in the rear operator cab
    CabRear,
    /// Player is not currently positioned in the vehicle
    NotHere,
}

/// Represents the connection status to the third rail power system.
///
/// In rail systems, the third rail provides electrical power to trains. This enum
/// tracks the current connection state between the vehicle and the power rail.
///
/// # Examples
///
/// ```
/// use your_crate_name::ThirdRailState;
///
/// let power_status = ThirdRailState::Connected;
/// match power_status {
///     ThirdRailState::Disconnnected => println!("No power available"),
///     ThirdRailState::PartwiseConnected => println!("Partial power connection"),
///     ThirdRailState::Connected => println!("Full power available"),
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum ThirdRailState {
    /// No connection to the third rail power system
    Disconnnected,
    /// Partial or intermittent connection to the power system
    PartwiseConnected,
    /// Full connection established with the third rail power system
    Connected,
}
