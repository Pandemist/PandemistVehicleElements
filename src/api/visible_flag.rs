use lotus_script::var::{get_var, set_var};

/// A visibility flag manager that provides a convenient interface for managing
/// boolean visibility states using lotus_script variables.
///
/// This struct allows you to create named visibility flags that can be toggled
/// between visible and invisible states, with the state persisted through the
/// lotus_script variable system.
///
/// # Examples
///
/// ```
/// use pandemist_vehicle_elements::Visiblility;
///
/// // Create a new visibility flag
/// let mut flag = Visiblility::new("my_component");
///
/// // Make the component visible
/// flag.make_visible();
/// assert!(flag.check());
///
/// // Make it invisible
/// flag.make_invisible();
/// assert!(!flag.check());
///
/// // Set visibility directly
/// flag.set_visbility(true);
/// assert!(flag.check());
/// ```
#[derive(Debug)]
pub struct Visiblility {
    /// The name of the visibility flag variable
    name: String,
}

impl Visiblility {
    /// Creates a new visibility flag with the given name.
    ///
    /// The name will be used as the variable name in the lotus_script
    /// variable system to store the visibility state.
    ///
    /// # Arguments
    ///
    /// * `name` - A string-like value that will be used as the variable name
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::Visiblility;
    ///
    /// let flag = Visiblility::new("main_panel");
    /// let flag2 = Visiblility::new(String::from("sidebar"));
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn new_visible(name: impl Into<String>) -> Self {
        let mut s = Self { name: name.into() };
        s.make_visible();
        s
    }

    /// Checks the current visibility state of the flag.
    ///
    /// Returns `true` if the flag is currently set to visible,
    /// `false` if it's invisible.
    ///
    /// # Returns
    ///
    /// * `bool` - The current visibility state
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::Visiblility;
    ///
    /// let flag = Visiblility::new("status_bar");
    /// let is_visible = flag.check();
    /// ```
    pub fn check(&self) -> bool {
        get_var::<bool>(&self.name)
    }

    /// Sets the visibility flag to visible (true).
    ///
    /// This is a convenience method equivalent to calling
    /// `set_visbility(true)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::Visiblility;
    ///
    /// let mut flag = Visiblility::new("toolbar");
    /// flag.make_visible();
    /// assert!(flag.check());
    /// ```
    pub fn make_visible(&mut self) {
        set_var(&self.name, true);
    }

    /// Sets the visibility state to the specified value.
    ///
    /// This method allows you to directly set the visibility state
    /// to either visible (`true`) or invisible (`false`).
    ///
    /// # Arguments
    ///
    /// * `value` - The visibility state to set (`true` for visible, `false` for invisible)
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::Visiblility;
    ///
    /// let mut flag = Visiblility::new("menu");
    /// flag.set_visbility(true);   // Make visible
    /// flag.set_visbility(false);  // Make invisible
    /// ```
    pub fn set_visbility(&mut self, value: bool) {
        set_var(&self.name, value);
    }

    /// Sets the visibility flag to invisible (false).
    ///
    /// This is a convenience method equivalent to calling
    /// `set_visbility(false)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::Visiblility;
    ///
    /// let mut flag = Visiblility::new("notification");
    /// flag.make_invisible();
    /// assert!(!flag.check());
    /// ```
    pub fn make_invisible(&mut self) {
        set_var(&self.name, false);
    }

    /// Toggles the visibility flag.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::Visiblility;
    ///
    /// let mut flag = Visiblility::new("notification");
    /// flag.toggle_visibility();
    /// assert!(flag.check());
    /// flag.toggle_visibility();
    /// assert!(!flag.check());
    /// ```
    pub fn toggle_visibility(&mut self) {
        set_var(&self.name, !get_var::<bool>(&self.name));
    }
}
