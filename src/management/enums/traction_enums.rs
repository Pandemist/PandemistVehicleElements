use serde::{Deserialize, Serialize};

/// Represents the driving direction state of a vehicle or object.
///
/// This structure tracks whether movement is possible or active in forward and/or backward directions.
/// It's particularly useful for vehicle control systems, robotics, or any application that needs
/// to track bidirectional movement capabilities.
///
/// # Examples
///
/// ```
/// use pandemist_vehicle_elements::DirectionOfDriving;
///
/// // Create a forward-only state
/// let forward_only = DirectionOfDriving::new(true, false);
/// assert!(forward_only.is_one());
///
/// // Create from a numeric value
/// let from_positive: DirectionOfDriving = 5.0f32.into();
/// assert!(from_positive.forward);
/// assert!(!from_positive.backward);
///
/// // Merge two states
/// let backward_only = DirectionOfDriving::new(false, true);
/// let combined = forward_only.merge(&backward_only);
/// assert!(combined.is_both());
/// ```
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct DirectionOfDriving {
    /// Indicates if forward movement is enabled/active
    pub forward: bool,
    /// Indicates if backward movement is enabled/active
    pub backward: bool,
}

impl DirectionOfDriving {
    /// Creates a new `DirectionOfDriving` with the specified forward and backward states.
    ///
    /// # Arguments
    ///
    /// * `forward` - Whether forward movement is enabled/active
    /// * `backward` - Whether backward movement is enabled/active
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::DirectionOfDriving;
    ///
    /// let state = DirectionOfDriving::new(true, false);
    /// assert!(state.forward);
    /// assert!(!state.backward);
    /// ```
    pub fn new(forward: bool, backward: bool) -> Self {
        DirectionOfDriving { forward, backward }
    }

    /// Returns a new state with forward and backward directions swapped.
    ///
    /// This is useful for reversing the direction logic or mirroring movement capabilities.
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::DirectionOfDriving;
    ///
    /// let original = DirectionOfDriving::new(true, false);
    /// let flipped = original.flip();
    ///
    /// assert!(!flipped.forward);
    /// assert!(flipped.backward);
    /// ```
    pub fn flip(&self) -> Self {
        DirectionOfDriving {
            forward: self.backward,
            backward: self.forward,
        }
    }

    /// Merges this state with another using logical OR for both directions.
    ///
    /// The resulting state will have forward movement enabled if either state has it enabled,
    /// and similarly for backward movement.
    ///
    /// # Arguments
    ///
    /// * `other` - The other `DirectionOfDriving` to merge with
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::DirectionOfDriving;
    ///
    /// let forward_only = DirectionOfDriving::new(true, false);
    /// let backward_only = DirectionOfDriving::new(false, true);
    /// let merged = forward_only.merge(&backward_only);
    ///
    /// assert!(merged.forward);
    /// assert!(merged.backward);
    /// assert!(merged.is_both());
    /// ```
    pub fn merge(&self, other: &DirectionOfDriving) -> Self {
        DirectionOfDriving {
            forward: self.forward || other.forward,
            backward: self.backward || other.backward,
        }
    }

    /// Returns `true` if neither forward nor backward movement is enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::DirectionOfDriving;
    ///
    /// let none_state = DirectionOfDriving::new(false, false);
    /// assert!(none_state.is_none());
    ///
    /// let some_state = DirectionOfDriving::new(true, false);
    /// assert!(!some_state.is_none());
    /// ```
    pub fn is_none(&self) -> bool {
        !(self.forward || self.backward)
    }

    /// Returns `true` if exactly one direction (forward XOR backward) is enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::DirectionOfDriving;
    ///
    /// let forward_only = DirectionOfDriving::new(true, false);
    /// assert!(forward_only.is_one());
    ///
    /// let both = DirectionOfDriving::new(true, true);
    /// assert!(!both.is_one());
    ///
    /// let none = DirectionOfDriving::new(false, false);
    /// assert!(!none.is_one());
    /// ```
    pub fn is_one(&self) -> bool {
        self.forward ^ self.backward
    }

    /// Returns `true` if both forward and backward movement are enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::DirectionOfDriving;
    ///
    /// let both = DirectionOfDriving::new(true, true);
    /// assert!(both.is_both());
    ///
    /// let forward_only = DirectionOfDriving::new(true, false);
    /// assert!(!forward_only.is_both());
    /// ```
    pub fn is_both(&self) -> bool {
        self.forward && self.backward
    }
}

/// Converts a `f32` value to a `DirectionOfDriving`.
///
/// - Positive values (> 0.0) result in forward movement enabled
/// - Negative values (< 0.0) result in backward movement enabled  
/// - Zero results in no movement enabled
///
/// # Examples
///
/// ```
/// use pandemist_vehicle_elements::DirectionOfDriving;
///
/// let forward_state: DirectionOfDriving = 1.5f32.into();
/// assert!(forward_state.forward && !forward_state.backward);
///
/// let backward_state: DirectionOfDriving = (-2.0f32).into();
/// assert!(!backward_state.forward && backward_state.backward);
///
/// let zero_state: DirectionOfDriving = 0.0f32.into();
/// assert!(zero_state.is_none());
/// ```
impl From<f32> for DirectionOfDriving {
    fn from(val: f32) -> Self {
        DirectionOfDriving {
            forward: val > 0.0,
            backward: val < 0.0,
        }
    }
}

/// Converts an `i32` value to a `DirectionOfDriving`.
///
/// - Positive values (> 0) result in forward movement enabled
/// - Negative values (< 0) result in backward movement enabled
/// - Zero results in no movement enabled
///
/// # Examples
///
/// ```
/// use pandemist_vehicle_elements::DirectionOfDriving;
///
/// let forward_state: DirectionOfDriving = 5i32.into();
/// assert!(forward_state.forward && !forward_state.backward);
///
/// let backward_state: DirectionOfDriving = (-3i32).into();
/// assert!(!backward_state.forward && backward_state.backward);
///
/// let zero_state: DirectionOfDriving = 0i32.into();
/// assert!(zero_state.is_none());
/// ```
impl From<i32> for DirectionOfDriving {
    fn from(val: i32) -> Self {
        DirectionOfDriving {
            forward: val > 0,
            backward: val < 0,
        }
    }
}

/// Converts a `DirectionOfDriving` to an `f32` value.
///
/// - Forward-only state converts to `1.0`
/// - Backward-only state converts to `-1.0`
/// - All other states (none, both) convert to `0.0`
///
/// # Examples
///
/// ```
/// use pandemist_vehicle_elements::DirectionOfDriving;
///
/// let forward_state = DirectionOfDriving::new(true, false);
/// let forward_val: f32 = forward_state.into();
/// assert_eq!(forward_val, 1.0);
///
/// let backward_state = DirectionOfDriving::new(false, true);
/// let backward_val: f32 = backward_state.into();
/// assert_eq!(backward_val, -1.0);
///
/// let both_state = DirectionOfDriving::new(true, true);
/// let both_val: f32 = both_state.into();
/// assert_eq!(both_val, 0.0);
/// ```
impl From<DirectionOfDriving> for f32 {
    fn from(val: DirectionOfDriving) -> Self {
        if val.forward && !val.backward {
            1.0
        } else if !val.forward && val.backward {
            -1.0
        } else {
            0.0
        }
    }
}
