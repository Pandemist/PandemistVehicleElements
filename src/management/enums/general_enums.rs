use std::fmt;

/// Represents the side of a vehicle or object.
///
/// This enum is commonly used to distinguish between left and right sides
/// in automotive or transportation contexts.
///
/// # Examples
///
/// ```
/// use pandemist_vehicle_elements::Side;
///
/// let left_side = Side::Left;
/// let right_side = Side::Right;
///
/// println!("{}", left_side);  // Prints "L"
/// println!("{}", right_side); // Prints "R"
/// ```
#[derive(Debug, Clone)]
pub enum Side {
    /// The left side
    Left,
    /// The right side
    Right,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Side::Left => write!(f, "L"),
            Side::Right => write!(f, "R"),
        }
    }
}

//------------------------

/// Represents the target state or mode of a windshield wiper system.
///
/// This enum defines the various operational modes that a wiper system
/// can be set to, from completely off to different speed settings and
/// special cleaning modes.
///
/// # Examples
///
/// ```
/// use pandemist_vehicle_elements::WiperTarget;
///
/// let wiper_state = WiperTarget::Normal;
///
/// match wiper_state {
///     WiperTarget::Off => println!("Wipers are off"),
///     WiperTarget::Normal => println!("Wipers running at normal speed"),
///     _ => println!("Other wiper mode"),
/// }
/// ```
#[derive(Debug, Default, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WiperTarget {
    /// Wipers are turned off (default state)
    #[default]
    Off,
    /// Direct wiper operation mode
    Interval,
    /// Normal speed wiper operation
    Normal,
    /// Fast speed wiper operation
    Fast,
    /// Special cleaning mode for windshield washing
    Cleaning,
}

//------------------------

/// Represents the activation state of a cab or driver compartment system.
///
/// This enum is typically used in railway or heavy vehicle contexts to
/// indicate the current operational state of the cab control systems.
///
/// # Examples
///
/// ```
/// use pandemist_vehicle_elements::CabActivState;
///
/// let cab_state = CabActivState::VR;
///
/// if cab_state != CabActivState::Off {
///     println!("Cab system is active");
/// }
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CabActivState {
    /// Cab system is deactivated (default state)
    #[default]
    Off,
    /// Star activation mode
    Star,
    /// VR activation mode
    VR,
}

//------------------------

/// Represents the formation switch position in a train consist.
///
/// This enum defines the role of a locomotive or train unit within
/// a multi-unit train formation, determining its control and operational
/// responsibilities.
///
/// # Examples
///
/// ```
/// use pandemist_vehicle_elements::TrainFormationSwitch;
///
/// let formation_role = TrainFormationSwitch::Leading;
///
/// match formation_role {
///     TrainFormationSwitch::TractionLeader => {
///         println!("This unit provides traction control");
///     },
///     TrainFormationSwitch::Leading => {
///         println!("This is the leading unit");
///     },
///     TrainFormationSwitch::Following => {
///         println!("This unit follows the leader");
///     },
/// }
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrainFormationSwitch {
    /// Unit acts as the traction leader (default role)
    #[default]
    TractionLeader,
    /// Unit is in the leading position
    Leading,
    /// Unit is in a following position
    Following,
}
