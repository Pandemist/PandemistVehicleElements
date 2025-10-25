use crate::management::enums::general_enums::CabActivState;

/// A struct representing movement or state in four cardinal directions.
///
/// This struct is useful for tracking directional states, movement permissions,
/// or boolean flags for up, down, left, and right directions. Each direction
/// is represented by a boolean value.
///
/// # Examples
///
/// ```
/// use pandemist_vehicle_elements_name::FourDirections;
///
/// // Create a new instance allowing movement up and right
/// let directions = FourDirections::new(true, false, true, false);
/// assert!(directions.up);
/// assert!(!directions.down);
/// assert!(directions.right);
/// assert!(!directions.left);
///
/// // Check if any direction is enabled
/// assert!(directions.is_one());
///
/// // Apply conditional logic
/// let restricted = directions.and(false);
/// assert!(!restricted.is_one());
/// ```
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[expect(clippy::struct_excessive_bools)]
pub struct FourDirections {
    /// Indicates whether the upward direction is enabled/allowed
    pub up: bool,
    /// Indicates whether the downward direction is enabled/allowed
    pub down: bool,
    /// Indicates whether the rightward direction is enabled/allowed
    pub right: bool,
    /// Indicates whether the leftward direction is enabled/allowed
    pub left: bool,
}

impl FourDirections {
    /// Creates a new `FourDirections` instance with the specified directional states.
    ///
    /// # Arguments
    ///
    /// * `up` - Whether the upward direction is enabled
    /// * `down` - Whether the downward direction is enabled
    /// * `right` - Whether the rightward direction is enabled
    /// * `left` - Whether the leftward direction is enabled
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements_name::FourDirections;
    ///
    /// let directions = FourDirections::new(true, false, true, false);
    /// assert!(directions.up && directions.right);
    /// assert!(!directions.down && !directions.left);
    /// ```
    #[must_use]
    #[expect(clippy::fn_params_excessive_bools)]
    pub fn new(up: bool, down: bool, right: bool, left: bool) -> Self {
        Self {
            up,
            down,
            right,
            left,
        }
    }

    /// Checks if at least one direction is enabled.
    ///
    /// Returns `true` if any of the four directions (`up`, `down`, `right`, `left`)
    /// is set to `true`, otherwise returns `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements_name::FourDirections;
    ///
    /// let some_directions = FourDirections::new(true, false, false, false);
    /// assert!(some_directions.is_one());
    ///
    /// let no_directions = FourDirections::new(false, false, false, false);
    /// assert!(!no_directions.is_one());
    /// ```
    #[must_use]
    pub fn is_one(&self) -> bool {
        self.up || self.down || self.right || self.left
    }

    /// Conditionally returns the current directions or disables all directions.
    ///
    /// If `allowed` is `true`, returns the current `FourDirections` instance unchanged.
    /// If `allowed` is `false`, returns a new instance with all directions set to `false`.
    ///
    /// This method is useful for applying conditional logic to directional permissions
    /// or states.
    ///
    /// # Arguments
    ///
    /// * `allowed` - Whether the current directional state should be preserved
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements_name::FourDirections;
    ///
    /// let directions = FourDirections::new(true, true, false, false);
    ///
    /// // Preserve current state
    /// let allowed = directions.and(true);
    /// assert_eq!(allowed, directions);
    ///
    /// // Disable all directions
    /// let restricted = directions.and(false);
    /// assert!(!restricted.is_one());
    /// ```
    #[must_use]
    pub fn and(self, allowed: bool) -> Self {
        if allowed {
            self
        } else {
            FourDirections::new(false, false, false, false)
        }
    }
}

pub enum ActivStatePos {
    CabA,
    CabB,
    Train,
}

pub struct TrainActivState {
    cab_a: CabActivState,
    cab_b: CabActivState,
    train: CabActivState,
}

impl TrainActivState {
    pub fn new() -> Self {
        Self {
            cab_a: CabActivState::Off,
            cab_b: CabActivState::Off,
            train: CabActivState::Off,
        }
    }

    pub fn update(&mut self, pos: ActivStatePos, state: CabActivState) {
        match pos {
            ActivStatePos::CabA => self.cab_a = state,
            ActivStatePos::CabB => self.cab_b = state,
            ActivStatePos::Train => self.train = state,
        }
    }

    pub fn state(&self, pos: ActivStatePos) -> CabActivState {
        match pos {
            ActivStatePos::CabA => self.cab_a,
            ActivStatePos::CabB => self.cab_b,
            ActivStatePos::Train => self.train,
        }
    }

    pub fn off(&self, pos: ActivStatePos) -> bool {
        match pos {
            ActivStatePos::CabA => self.cab_a == CabActivState::Off,
            ActivStatePos::CabB => self.cab_b == CabActivState::Off,
            ActivStatePos::Train => self.train == CabActivState::Off,
        }
    }

    pub fn active(&self, pos: ActivStatePos) -> bool {
        match pos {
            ActivStatePos::CabA => self.cab_a > CabActivState::Off,
            ActivStatePos::CabB => self.cab_b > CabActivState::Off,
            ActivStatePos::Train => self.train > CabActivState::Off,
        }
    }

    pub fn runmode(&self, pos: ActivStatePos) -> bool {
        match pos {
            ActivStatePos::CabA => self.cab_a > CabActivState::Star,
            ActivStatePos::CabB => self.cab_b > CabActivState::Star,
            ActivStatePos::Train => self.train > CabActivState::Star,
        }
    }
}

impl Default for TrainActivState {
    fn default() -> Self {
        Self::new()
    }
}
