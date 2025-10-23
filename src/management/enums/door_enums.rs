//! Door control enumerations for door management systems.
//!
//! This module provides enumerations for controlling and representing the state
//! of doors in various systems, including side selection, step targeting,
//! door control commands, and door state representation.

use serde::{Deserialize, Serialize};

/// Specifies which side(s) of a door system to target for operations.
///
/// Used to control which door panels should be affected by door operations
/// in systems with multiple door panels (e.g., left/right sliding doors).
///
/// # Examples
///
/// ```
/// use your_crate::DoorSideTarget;
///
/// let target = DoorSideTarget::Left;
/// assert_eq!(target, DoorSideTarget::Left);
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DoorSideTarget {
    /// No specific side targeted (default)
    #[default]
    None,
    /// Target the left door panel
    Left,
    /// Target the right door panel
    Right,
    /// Target both door panels simultaneously
    Both,
}

//------------------------

/// Specifies the step or platform level to target for door operations.
///
/// Used in systems where doors can operate at different height levels,
/// such as public transportation vehicles with multiple boarding levels.
///
/// # Examples
///
/// ```
/// use your_crate::DoorStepTarget;
///
/// let step = DoorStepTarget::Street;
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DoorStepTarget {
    /// No specific step level targeted (default)
    #[default]
    None,
    /// Target the high step/platform level
    High,
    /// Target the low step/platform level
    Low,
    /// Target the street level
    Street,
}

//------------------------

/// Represents door control commands and target states.
///
/// This enum defines the various commands that can be sent to a door system
/// to control its operation, from emergency closing to normal opening.
///
/// # Examples
///
/// ```
/// use your_crate::DoorTarget;
///
/// let command = DoorTarget::Open;
/// let flipped = command.flip();
/// assert_eq!(flipped, DoorTarget::Close);
/// ```
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum DoorTarget {
    /// Emergency or rapid door closing
    FastClose,
    /// Normal door closing operation (default)
    #[default]
    Close,
    /// Release door locks or constraints
    Release,
    /// Open the door
    Open,
}

impl DoorTarget {
    /// Flips the door target to its logical opposite.
    ///
    /// This method returns the opposite door operation for the current target:
    /// - `FastClose` and `Close` become `Open`
    /// - `Release` becomes `Close`
    /// - `Open` becomes `Close`
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate::DoorTarget;
    ///
    /// assert_eq!(DoorTarget::Open.flip(), DoorTarget::Close);
    /// assert_eq!(DoorTarget::Close.flip(), DoorTarget::Open);
    /// assert_eq!(DoorTarget::FastClose.flip(), DoorTarget::Open);
    /// assert_eq!(DoorTarget::Release.flip(), DoorTarget::Close);
    /// ```
    pub fn flip(self) -> Self {
        match self {
            DoorTarget::FastClose => DoorTarget::Open,
            DoorTarget::Close => DoorTarget::Open,
            DoorTarget::Release => DoorTarget::Close,
            DoorTarget::Open => DoorTarget::Close,
        }
    }

    pub fn and(self, and: bool) -> Self {
        if and {
            self
        } else {
            DoorTarget::Close
        }
    }

    /// Merges two door targets according to priority rules.
    ///
    /// This method combines two door targets, giving priority to certain operations:
    /// - `Open` commands always take precedence
    /// - `Release` commands take precedence over closing operations
    /// - Existing operations maintain priority when combined with similar operations
    ///
    /// # Arguments
    ///
    /// * `other` - The other `DoorTarget` to merge with this one
    ///
    /// # Returns
    ///
    /// The merged `DoorTarget` based on priority rules
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate::DoorTarget;
    ///
    /// let close = DoorTarget::Close;
    /// let open = DoorTarget::Open;
    /// let merged = close.merge(&open);
    /// assert_eq!(merged, DoorTarget::Open);
    ///
    /// let fast_close = DoorTarget::FastClose;
    /// let release = DoorTarget::Release;
    /// let merged2 = fast_close.merge(&release);
    /// assert_eq!(merged2, DoorTarget::Release);
    /// ```
    pub fn merge(&self, other: &DoorTarget) -> Self {
        match (self, other) {
            // Self takes priority in these cases
            (DoorTarget::FastClose, DoorTarget::Close)
            | (DoorTarget::Release, DoorTarget::FastClose)
            | (DoorTarget::Release, DoorTarget::Release)
            | (DoorTarget::Close, DoorTarget::Close)
            | (DoorTarget::FastClose, DoorTarget::FastClose)
            | (DoorTarget::Release, DoorTarget::Close)
            | (DoorTarget::Open, _) => *self,

            // Other takes priority in these cases
            (DoorTarget::FastClose, DoorTarget::Release)
            | (DoorTarget::Close, DoorTarget::Release)
            | (DoorTarget::Close, DoorTarget::FastClose)
            | (_, DoorTarget::Open) => *other,
        }
    }
}

//------------------------

/// Represents the current state of a door.
///
/// This enum tracks the physical state of a door, distinguishing between
/// definitively closed, definitively open, and intermediate or unknown states.
///
/// # Examples
///
/// ```
/// use your_crate::DoorState;
///
/// let state = DoorState::Open;
/// match state {
///     DoorState::Open => println!("Door is open"),
///     DoorState::Closed => println!("Door is closed"),
///     DoorState::Other => println!("Door is in intermediate state"),
/// }
/// ```
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum DoorState {
    /// Door is fully closed
    Closed,
    /// Door is in an intermediate, unknown, or transitional state (default)
    #[default]
    Other,
    /// Door is fully open
    Open,
}
