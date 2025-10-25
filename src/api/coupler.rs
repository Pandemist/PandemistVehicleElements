use lotus_script::{
    message::Coupling,
    var::{get_var, set_var},
};

use super::mock_enums::CouplingState;

/// API wrapper for train coupler functionality.
///
/// The `ApiCoupler` provides a convenient interface to interact with train couplers,
/// allowing you to check coupling status, manage coupling states, and control
/// coupling position offsets. It supports both front and rear couplers.
///
/// # Examples
///
/// ```rust
/// use lotus_script::message::Coupling;
/// use pandemist_vehicle_elements::ApiCoupler;
/// use pandemist_vehicle_elements::CouplingState;
///
/// let front_coupler = ApiCoupler::new(Coupling::Front);
///
/// // Check if the coupler is currently coupled
/// if front_coupler.is_coupled() {
///     println!("Front coupler is coupled");
/// }
///
/// // Set coupling state to ready
/// front_coupler.set_coupling_state(CouplingState::Ready);
///
/// // Get current coupling state
/// let state = front_coupler.coupling_state();
/// println!("Coupling state: {:?}", state);
/// ```
#[derive(Debug)]
pub struct ApiCoupler {
    /// The underlying coupler (Front or Rear)
    pub coupler: Coupling,
}

impl ApiCoupler {
    /// Creates a new `ApiCoupler` instance.
    ///
    /// # Arguments
    ///
    /// * `coupler` - The coupler type (Front or Rear) to manage
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lotus_script::message::Coupling;
    /// use pandemist_vehicle_elements::ApiCoupler;
    ///
    /// let front_coupler = ApiCoupler::new(Coupling::Front);
    /// let rear_coupler = ApiCoupler::new(Coupling::Rear);
    /// ```
    #[must_use]
    pub fn new(coupler: Coupling) -> Self {
        Self { coupler }
    }

    /// Checks if the coupler is currently coupled to another train car.
    ///
    /// This method corresponds to the script variable `coupled_{a}` where `{a}`
    /// is the coupler index (0 for front, 1 for rear).
    ///
    /// # Returns
    ///
    /// `true` if the coupler is coupled, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lotus_script::message::Coupling;
    /// use pandemist_vehicle_elements::ApiCoupler;
    ///
    /// let coupler = ApiCoupler::new(Coupling::Front);
    /// if coupler.is_coupled() {
    ///     println!("Coupler is connected to another car");
    /// }
    /// ```
    #[must_use]
    pub fn is_coupled(&self) -> bool {
        self.coupler.is_coupled()
    }

    /// Returns the internal index used for this coupler in script variables.
    ///
    /// Front coupler returns 0, rear coupler returns 1. This is used internally
    /// to format variable names for script interaction.
    ///
    /// # Returns
    ///
    /// Index value: 0 for Front coupler, 1 for Rear coupler.
    fn index_by_coupler(&self) -> usize {
        match self.coupler {
            Coupling::Front => 0,
            Coupling::Rear => 1,
        }
    }

    /// Gets the current coupling state.
    ///
    /// This method corresponds to the script variable `couplingState_{a}` where `{a}`
    /// is the coupler index. The state indicates whether the coupler is deactivated,
    /// ready for coupling, or actively coupled.
    ///
    /// # Returns
    ///
    /// The current [`CouplingState`]:
    /// - `CouplingState::Deactivated` - Coupler is disabled
    /// - `CouplingState::Ready` - Coupler is ready to couple
    /// - `CouplingState::Coupled` - Coupler is actively coupled
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lotus_script::message::Coupling;
    /// use pandemist_vehicle_elements::{ApiCoupler, CouplingState};
    ///
    /// let coupler = ApiCoupler::new(Coupling::Front);
    /// match coupler.coupling_state() {
    ///     CouplingState::Coupled => println!("Coupler is connected"),
    ///     CouplingState::Ready => println!("Coupler is ready to connect"),
    ///     CouplingState::Deactivated => println!("Coupler is disabled"),
    /// }
    /// ```
    #[must_use]
    pub fn coupling_state(&self) -> CouplingState {
        match get_var::<u8>(&format!("couplingState_{}", self.index_by_coupler())) {
            2 => CouplingState::Coupled,
            1 => CouplingState::Ready,
            _ => CouplingState::Deactivated,
        }
    }

    /// Sets the coupling state.
    ///
    /// This method updates the script variable `couplingState_{a}` where `{a}`
    /// is the coupler index. Use this to control whether the coupler is active
    /// and ready for coupling operations.
    ///
    /// # Arguments
    ///
    /// * `value` - The desired [`CouplingState`] to set
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lotus_script::message::Coupling;
    /// use pandemist_vehicle_elements::{ApiCoupler, CouplingState};
    ///
    /// let coupler = ApiCoupler::new(Coupling::Front);
    ///
    /// // Activate the coupler and make it ready
    /// coupler.set_coupling_state(CouplingState::Ready);
    ///
    /// // Deactivate the coupler
    /// coupler.set_coupling_state(CouplingState::Deactivated);
    /// ```
    pub fn set_coupling_state(&self, value: CouplingState) {
        let new_value = match value {
            CouplingState::Coupled => 2,
            CouplingState::Ready => 1,
            CouplingState::Deactivated => 0,
        };

        set_var(
            &format!("couplingState_{}", self.index_by_coupler()),
            new_value,
        );
    }

    /// Gets the vertical offset of the coupler.
    ///
    /// This method corresponds to the script variable `couplingOffsetY_{a}` where `{a}`
    /// is the coupler index. The offset represents the vertical displacement of the
    /// coupler from its default position, typically used for height adjustments
    /// when coupling to cars of different heights.
    ///
    /// # Returns
    ///
    /// The vertical offset in meters (or game units).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lotus_script::message::Coupling;
    /// use pandemist_vehicle_elements::ApiCoupler;
    ///
    /// let coupler = ApiCoupler::new(Coupling::Rear);
    /// let offset = coupler.coupling_y_offset();
    /// println!("Current vertical offset: {:.2}m", offset);
    /// ```
    #[must_use]
    pub fn coupling_y_offset(&self) -> f32 {
        get_var::<f32>(&format!("couplingOffsetY_{}", self.index_by_coupler()))
    }

    /// Sets the vertical offset of the coupler.
    ///
    /// This method updates the script variable `couplingOffsetY_{a}` where `{a}`
    /// is the coupler index. Use this to adjust the coupler height for proper
    /// alignment with other train cars that may have different coupler heights.
    ///
    /// # Arguments
    ///
    /// * `value` - The vertical offset in meters (or game units). Positive values
    ///   move the coupler upward, negative values move it downward.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lotus_script::message::Coupling;
    /// use pandemist_vehicle_elements::ApiCoupler;
    ///
    /// let mut coupler = ApiCoupler::new(Coupling::Front);
    ///
    /// // Raise the coupler by 0.1 meters
    /// coupler.set_coupling_y_offset(0.1);
    ///
    /// // Lower the coupler by 0.05 meters
    /// coupler.set_coupling_y_offset(-0.05);
    ///
    /// // Reset to default height
    /// coupler.set_coupling_y_offset(0.0);
    /// ```
    pub fn set_coupling_y_offset(&mut self, value: f32) {
        set_var(
            &format!("couplingOffsetY_{}", self.index_by_coupler()),
            value,
        );
    }
}
