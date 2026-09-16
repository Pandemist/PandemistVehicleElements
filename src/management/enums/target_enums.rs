use serde::{Deserialize, Serialize};

use crate::messages::extended_modul_messenges::SimplePantoTarget;

/// Represents the target state for controlling electrical systems.
///
/// This enum is used to specify whether an electrical system should be turned on,
/// turned off, or remain in a neutral state. Each active state (TurnOn/TurnOff)
/// can carry an associated floating-point value for additional control parameters.
///
/// # Examples
///
/// ```
/// use pandemist_vehicle_elements_name::SimpleSwitchingTarget;
///
/// // Create a turn-on target
/// let target = SimpleSwitchingTarget::TurnOn;
///
/// // Create from boolean
/// let target = SimpleSwitchingTarget::from(true);
///
/// // Create from integer with data
/// let target = SimpleSwitchingTarget::new(1);
/// ```
#[derive(Default, Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum SimpleSwitchingTarget {
    /// Turn on the electrical system with the specified intensity/parameter value.
    TurnOn,

    /// Turn off the electrical system with the specified parameter value.
    TurnOff,

    /// Neutral state - no action should be taken.
    #[default]
    Neutral,
}

impl SimpleSwitchingTarget {
    /// Creates a new `SimpleSwitchingTarget` from an integer value and associated data.
    ///
    /// # Arguments
    ///
    /// * `val` - The switching command: -1 for TurnOff, 1 for TurnOn, any other value for Neutral
    /// * `data` - The associated floating-point parameter value
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements_name::SimpleSwitchingTarget;
    ///
    /// let turn_on = SimpleSwitchingTarget::new(1);
    /// let turn_off = SimpleSwitchingTarget::new(-1);
    /// let neutral = SimpleSwitchingTarget::new(0);
    /// ```
    pub fn new(val: i32) -> Self {
        match val {
            -1 => SimpleSwitchingTarget::TurnOff,
            1 => SimpleSwitchingTarget::TurnOn,
            _ => SimpleSwitchingTarget::Neutral,
        }
    }

    /// Creates a new `SimpleSwitchingTarget` from a boolean value and associated data.
    ///
    /// # Arguments
    ///
    /// * `val` - `true` for TurnOn, `false` for TurnOff
    /// * `data` - The associated floating-point parameter value
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements_name::SimpleSwitchingTarget;
    ///
    /// let turn_on = SimpleSwitchingTarget::new_bool(true);
    /// let turn_off = SimpleSwitchingTarget::new_bool(false);
    /// ```
    pub fn new_bool(val: bool) -> Self {
        match val {
            false => SimpleSwitchingTarget::TurnOff,
            true => SimpleSwitchingTarget::TurnOn,
        }
    }

    /// Conditionally returns this target or neutral based on a flag.
    ///
    /// This method is useful for conditional switching logic where the target
    /// should only be applied when certain conditions are met.
    ///
    /// # Arguments
    ///
    /// * `flag` - If `true`, returns `self`; if `false`, returns `Neutral`
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements_name::SimpleSwitchingTarget;
    ///
    /// let target = SimpleSwitchingTarget::TurnOn;
    /// let conditional = target.and(true);  // Returns TurnOn
    /// let disabled = target.and(false);    // Returns Neutral
    /// ```
    pub fn and(self, flag: bool) -> Self {
        if flag {
            self
        } else {
            Self::default()
        }
    }

    /// Combines two `SimpleSwitchingTarget` values according to specific rules.
    ///
    /// The combination rules are:
    /// - If one target is `Neutral`, the other target takes precedence
    /// - If targets conflict (TurnOn vs TurnOff), the first target (`self`) wins
    /// - If targets are the same type, their parameter values are averaged
    ///
    /// # Arguments
    ///
    /// * `other` - The other `SimpleSwitchingTarget` to combine with
    ///
    /// # Returns
    ///
    /// A new `SimpleSwitchingTarget` representing the combination of both inputs
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements_name::SimpleSwitchingTarget;
    ///
    /// let target1 = SimpleSwitchingTarget::TurnOn;
    /// let target2 = SimpleSwitchingTarget::TurnOn;
    /// let combined = target1.combine(target2); // TurnOn
    ///
    /// let target3 = SimpleSwitchingTarget::TurnOff;
    /// let conflicted = target1.combine(target3); // TurnOn - first target wins
    /// ```
    pub fn combine(self, other: SimpleSwitchingTarget) -> SimpleSwitchingTarget {
        use SimpleSwitchingTarget::*;

        match (self, other) {
            // If one is neutral, the other wins
            (Neutral, x) | (x, Neutral) => x,

            // TurnOn vs. TurnOff → self wins
            (TurnOn, TurnOff) => self,
            (TurnOff, TurnOn) => self,
            (TurnOn, TurnOn) => self,
            (TurnOff, TurnOff) => self,
        }
    }
}

/// Converts an `i32` value to a `SimpleSwitchingTarget` with zero data.
///
/// # Conversion Rules
///
/// * `-1` → `TurnOff`
/// * `1` → `TurnOn`
/// * Any other value → `Neutral`
///
/// # Examples
///
/// ```
/// use pandemist_vehicle_elements_name::SimpleSwitchingTarget;
///
/// let turn_on: SimpleSwitchingTarget = 1.into();
/// let turn_off: SimpleSwitchingTarget = (-1).into();
/// let neutral: SimpleSwitchingTarget = 0.into();
/// ```
impl From<i32> for SimpleSwitchingTarget {
    fn from(val: i32) -> Self {
        match val {
            -1 => SimpleSwitchingTarget::TurnOff,
            1 => SimpleSwitchingTarget::TurnOn,
            _ => SimpleSwitchingTarget::Neutral,
        }
    }
}

/// Converts a `bool` value to a `SimpleSwitchingTarget` with zero data.
///
/// # Conversion Rules
///
/// * `true` → `TurnOn`
/// * `false` → `TurnOff`
///
/// # Examples
///
/// ```
/// use pandemist_vehicle_elements_name::SimpleSwitchingTarget;
///
/// let turn_on: SimpleSwitchingTarget = true.into();
/// let turn_off: SimpleSwitchingTarget = false.into();
/// ```
impl From<bool> for SimpleSwitchingTarget {
    fn from(val: bool) -> Self {
        match val {
            false => SimpleSwitchingTarget::TurnOff,
            true => SimpleSwitchingTarget::TurnOn,
        }
    }
}

//===================================================================
// Three State
//===================================================================

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreeState {
    On,
    TurnOff,
    #[default]
    Off,
}

impl ThreeState {
    pub fn or(self, other: ThreeState) -> Self {
        match (self, other) {
            (ThreeState::TurnOff, _) => ThreeState::TurnOff,
            (_, ThreeState::TurnOff) => ThreeState::TurnOff,
            (ThreeState::On, _) => ThreeState::On,
            (_, ThreeState::On) => ThreeState::On,
            (ThreeState::Off, ThreeState::Off) => ThreeState::Off,
        }
    }
}

/// Represents the target state for controlling electrical systems.
///
/// This enum is used to specify whether an electrical system should be turned on,
/// turned off, or remain in a neutral state. Each active state (TurnOn/TurnOff)
/// can carry an associated floating-point value for additional control parameters.
///
/// # Examples
///
/// ```
/// use pandemist_vehicle_elements_name::SwitchingTarget;
///
/// // Create a turn-on target with intensity 0.8
/// let target = SwitchingTarget::TurnOn(0.8);
///
/// // Create from boolean
/// let target = SwitchingTarget::from(true);
///
/// // Create from integer with data
/// let target = SwitchingTarget::new(1, 0.5);
/// ```
#[derive(Default, Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum SwitchingTarget {
    /// Turn on the electrical system with the specified intensity/parameter value.
    ///
    /// The `f32` value can represent intensity, duration, or any other relevant parameter.
    TurnOn(f32),

    /// Turn off the electrical system with the specified parameter value.
    ///
    /// The `f32` value can represent fade-out time, priority, or other relevant parameters.
    TurnOff(f32),

    /// Neutral state - no action should be taken.
    ///
    /// This is the default state when no specific switching action is required.
    #[default]
    Neutral,
}

impl SwitchingTarget {
    /// Creates a new `SwitchingTarget` from an integer value and associated data.
    ///
    /// # Arguments
    ///
    /// * `val` - The switching command: -1 for TurnOff, 1 for TurnOn, any other value for Neutral
    /// * `data` - The associated floating-point parameter value
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements_name::SwitchingTarget;
    ///
    /// let turn_on = SwitchingTarget::new(1, 0.75);
    /// let turn_off = SwitchingTarget::new(-1, 0.25);
    /// let neutral = SwitchingTarget::new(0, 0.0);
    /// ```
    pub fn new(val: i32, data: f32) -> Self {
        match val {
            -1 => SwitchingTarget::TurnOff(data),
            1 => SwitchingTarget::TurnOn(data),
            _ => SwitchingTarget::Neutral,
        }
    }

    /// Creates a new `SwitchingTarget` from a boolean value and associated data.
    ///
    /// # Arguments
    ///
    /// * `val` - `true` for TurnOn, `false` for TurnOff
    /// * `data` - The associated floating-point parameter value
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements_name::SwitchingTarget;
    ///
    /// let turn_on = SwitchingTarget::new_bool(true, 1.0);
    /// let turn_off = SwitchingTarget::new_bool(false, 0.5);
    /// ```
    pub fn new_bool(val: bool, data: f32) -> Self {
        match val {
            false => SwitchingTarget::TurnOff(data),
            true => SwitchingTarget::TurnOn(data),
        }
    }

    /// Conditionally returns this target or neutral based on a flag.
    ///
    /// This method is useful for conditional switching logic where the target
    /// should only be applied when certain conditions are met.
    ///
    /// # Arguments
    ///
    /// * `flag` - If `true`, returns `self`; if `false`, returns `Neutral`
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements_name::SwitchingTarget;
    ///
    /// let target = SwitchingTarget::TurnOn(0.8);
    /// let conditional = target.and(true);  // Returns TurnOn(0.8)
    /// let disabled = target.and(false);    // Returns Neutral
    /// ```
    pub fn and(self, flag: bool) -> Self {
        if flag {
            self
        } else {
            Self::default()
        }
    }

    /// Combines two `SwitchingTarget` values according to specific rules.
    ///
    /// The combination rules are:
    /// - If one target is `Neutral`, the other target takes precedence
    /// - If targets conflict (TurnOn vs TurnOff), the first target (`self`) wins
    /// - If targets are the same type, their parameter values are averaged
    ///
    /// # Arguments
    ///
    /// * `other` - The other `SwitchingTarget` to combine with
    ///
    /// # Returns
    ///
    /// A new `SwitchingTarget` representing the combination of both inputs
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements_name::SwitchingTarget;
    ///
    /// let target1 = SwitchingTarget::TurnOn(0.6);
    /// let target2 = SwitchingTarget::TurnOn(0.8);
    /// let combined = target1.combine(target2); // TurnOn(0.7) - average of 0.6 and 0.8
    ///
    /// let target3 = SwitchingTarget::TurnOff(0.5);
    /// let conflicted = target1.combine(target3); // TurnOn(0.6) - first target wins
    /// ```
    pub fn combine(self, other: SwitchingTarget) -> SwitchingTarget {
        use SwitchingTarget::*;

        match (self, other) {
            // If one is neutral, the other wins
            (Neutral, x) | (x, Neutral) => x,

            // TurnOn vs. TurnOff → self wins
            (TurnOn(_), TurnOff(_)) => self,
            (TurnOff(_), TurnOn(_)) => self,

            // Similar: Form average
            (TurnOn(a), TurnOn(b)) => TurnOn((a + b) / 2.0),
            (TurnOff(a), TurnOff(b)) => TurnOff((a + b) / 2.0),
        }
    }

    pub fn complex(old: SimpleSwitchingTarget, time: f32) -> SwitchingTarget {
        match &old {
            SimpleSwitchingTarget::TurnOff => SwitchingTarget::TurnOff(time),
            SimpleSwitchingTarget::TurnOn => SwitchingTarget::TurnOn(time),
            SimpleSwitchingTarget::Neutral => SwitchingTarget::Neutral,
        }
    }
}

impl From<SwitchingTarget> for SimpleSwitchingTarget {
    fn from(val: SwitchingTarget) -> Self {
        match val {
            SwitchingTarget::TurnOff(_) => SimpleSwitchingTarget::TurnOff,
            SwitchingTarget::TurnOn(_) => SimpleSwitchingTarget::TurnOn,
            SwitchingTarget::Neutral => SimpleSwitchingTarget::Neutral,
        }
    }
}

/// Converts an `i32` value to a `SwitchingTarget` with zero data.
///
/// # Conversion Rules
///
/// * `-1` → `TurnOff(0.0)`
/// * `1` → `TurnOn(0.0)`
/// * Any other value → `Neutral`
///
/// # Examples
///
/// ```
/// use pandemist_vehicle_elements_name::SwitchingTarget;
///
/// let turn_on: SwitchingTarget = 1.into();
/// let turn_off: SwitchingTarget = (-1).into();
/// let neutral: SwitchingTarget = 0.into();
/// ```
impl From<i32> for SwitchingTarget {
    fn from(val: i32) -> Self {
        match val {
            -1 => SwitchingTarget::TurnOff(0.0),
            1 => SwitchingTarget::TurnOn(0.0),
            _ => SwitchingTarget::Neutral,
        }
    }
}

impl From<SimplePantoTarget> for SwitchingTarget {
    fn from(val: SimplePantoTarget) -> Self {
        match val {
            SimplePantoTarget::Down => SwitchingTarget::TurnOff(0.0),
            SimplePantoTarget::Up => SwitchingTarget::TurnOn(0.0),
            _ => SwitchingTarget::Neutral,
        }
    }
}

/// Converts a `bool` value to a `SwitchingTarget` with zero data.
///
/// # Conversion Rules
///
/// * `true` → `TurnOn(0.0)`
/// * `false` → `TurnOff(0.0)`
///
/// # Examples
///
/// ```
/// use pandemist_vehicle_elements_name::SwitchingTarget;
///
/// let turn_on: SwitchingTarget = true.into();
/// let turn_off: SwitchingTarget = false.into();
/// ```
impl From<bool> for SwitchingTarget {
    fn from(val: bool) -> Self {
        match val {
            false => SwitchingTarget::TurnOff(0.0),
            true => SwitchingTarget::TurnOn(0.0),
        }
    }
}
