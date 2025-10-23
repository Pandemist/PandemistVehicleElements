use serde::{Deserialize, Serialize};

/// Represents the target state for controlling electrical systems.
///
/// This enum is used to specify whether an electrical system should be turned on,
/// turned off, or remain in a neutral state. Each active state (TurnOn/TurnOff)
/// can carry an associated floating-point value for additional control parameters.
///
/// # Examples
///
/// ```
/// use your_crate_name::SwitchingTarget;
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
    /// use your_crate_name::SwitchingTarget;
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
    /// use your_crate_name::SwitchingTarget;
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
    /// use your_crate_name::SwitchingTarget;
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
    /// use your_crate_name::SwitchingTarget;
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
/// use your_crate_name::SwitchingTarget;
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
/// use your_crate_name::SwitchingTarget;
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
