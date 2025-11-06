use serde::{Deserialize, Serialize};

use crate::management::enums::target_enums::SwitchingTarget;

/// Represents the switching state of a component or system.
///
/// This enum is commonly used to represent tri-state logic where a component
/// can be explicitly turned off, on, or left in a neutral/default state.
///
/// # Examples
///
/// ```rust
/// use pandemist_vehicle_elements::SwitchingState;
///
/// let state = SwitchingState::default();
/// assert_eq!(state, SwitchingState::Neutral);
///
/// // Converting from i32 values
/// let off_state = SwitchingState::from(-1);
/// let on_state = SwitchingState::from(1);
/// let neutral_state = SwitchingState::from(0);
/// ```
#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum SwitchingState {
    /// Component is explicitly turned off
    Off,
    /// Component is in neutral/default state (default variant)
    #[default]
    Neutral,
    /// Component is explicitly turned on
    On,
}

impl From<SwitchingTarget> for SwitchingState {
    fn from(value: SwitchingTarget) -> Self {
        match value {
            SwitchingTarget::TurnOn(_) => SwitchingState::On,
            SwitchingTarget::TurnOff(_) => SwitchingState::Off,
            SwitchingTarget::Neutral => SwitchingState::Neutral,
        }
    }
}

impl SwitchingState {
    pub fn or(self, other: SwitchingState) -> Self {
        match (self, other) {
            (SwitchingState::Off, SwitchingState::Off) => SwitchingState::Off,
            (SwitchingState::Off, SwitchingState::Neutral) => SwitchingState::Neutral,
            (SwitchingState::Off, SwitchingState::On) => SwitchingState::Neutral,
            (SwitchingState::Neutral, SwitchingState::Off) => SwitchingState::Off,
            (SwitchingState::Neutral, SwitchingState::Neutral) => SwitchingState::Neutral,
            (SwitchingState::Neutral, SwitchingState::On) => SwitchingState::On,
            (SwitchingState::On, SwitchingState::Off) => SwitchingState::Neutral,
            (SwitchingState::On, SwitchingState::Neutral) => SwitchingState::On,
            (SwitchingState::On, SwitchingState::On) => SwitchingState::On,
        }
    }
}

impl From<i32> for SwitchingState {
    /// Converts an i32 value to a SwitchingState.
    ///
    /// # Mapping
    /// - `-1` maps to `SwitchingState::Off`
    /// - `1` maps to `SwitchingState::On`
    /// - Any other value (including `0`) maps to `SwitchingState::Neutral`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pandemist_vehicle_elements::SwitchingState;
    ///
    /// assert_eq!(SwitchingState::from(-1), SwitchingState::Off);
    /// assert_eq!(SwitchingState::from(1), SwitchingState::On);
    /// assert_eq!(SwitchingState::from(0), SwitchingState::Neutral);
    /// assert_eq!(SwitchingState::from(42), SwitchingState::Neutral);
    /// ```
    fn from(val: i32) -> Self {
        match val {
            -1 => SwitchingState::Off,
            1 => SwitchingState::On,
            _ => SwitchingState::Neutral,
        }
    }
}

//------------------------

/// Represents a state that tracks both current status and recent changes.
///
/// This enum is useful for tracking state transitions, particularly when you need
/// to know not just the current state, but also whether it just changed. This is
/// commonly used in event systems, UI updates, or state machines where transition
/// events are important.
///
/// # State Transitions
///
/// The enum captures four distinct states:
/// - `Off`: Currently off and was off previously
/// - `JustOff`: Currently off but was on previously (transition occurred)
/// - `JustOn`: Currently on but was off previously (transition occurred)  
/// - `On`: Currently on and was on previously
///
/// # Examples
///
/// ```rust
/// use pandemist_vehicle_elements::ChangedState;
///
/// let state = ChangedState::default();
/// assert_eq!(state, ChangedState::On);
///
/// // Track state changes
/// let turning_on = ChangedState::to_changed(false, true);
/// assert_eq!(turning_on, ChangedState::JustOn);
///
/// let turning_off = ChangedState::to_changed(true, false);
/// assert_eq!(turning_off, ChangedState::JustOff);
/// ```
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChangedState {
    /// Currently off, was off previously
    Off,
    /// Currently off, was on previously (just turned off)
    JustOff,
    /// Currently on, was off previously (just turned on)
    JustOn,
    /// Currently on, was on previously (default state)
    #[default]
    On,
}

impl ChangedState {
    pub fn is_on(&self) -> bool {
        match self {
            ChangedState::Off | ChangedState::JustOff => false,
            ChangedState::JustOn | ChangedState::On => true,
        }
    }

    pub fn is_off(&self) -> bool {
        match self {
            ChangedState::Off | ChangedState::JustOff => true,
            ChangedState::JustOn | ChangedState::On => false,
        }
    }

    /// Creates a ChangedState by comparing old and new boolean values.
    ///
    /// This method analyzes the transition between two boolean states and returns
    /// the appropriate ChangedState variant that captures both the current state
    /// and whether a transition occurred.
    ///
    /// # Arguments
    ///
    /// * `old` - The previous boolean state
    /// * `new` - The current boolean state
    ///
    /// # Returns
    ///
    /// * `ChangedState::Off` - if both old and new are `false`
    /// * `ChangedState::JustOff` - if old is `true` and new is `false`
    /// * `ChangedState::JustOn` - if old is `false` and new is `true`
    /// * `ChangedState::On` - if both old and new are `true`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pandemist_vehicle_elements::ChangedState;
    ///
    /// // No change, remained off
    /// assert_eq!(ChangedState::to_changed(false, false), ChangedState::Off);
    ///
    /// // No change, remained on
    /// assert_eq!(ChangedState::to_changed(true, true), ChangedState::On);
    ///
    /// // State changed from off to on
    /// assert_eq!(ChangedState::to_changed(false, true), ChangedState::JustOn);
    ///
    /// // State changed from on to off
    /// assert_eq!(ChangedState::to_changed(true, false), ChangedState::JustOff);
    /// ```
    pub fn to_changed(old: bool, new: bool) -> Self {
        if old {
            if new {
                ChangedState::On
            } else {
                ChangedState::JustOff
            }
        } else if new {
            ChangedState::JustOn
        } else {
            ChangedState::Off
        }
    }
}
