use super::communications::{fuses::FuseManager, local_value_manager::LocalValueManager};

/// A communication manager that coordinates local values and fuse management.
///
/// The `Com` struct serves as the main entry point for communication operations,
/// managing both local value storage and fuse-based safety mechanisms.
///
/// # Examples
///
/// ```
/// use pandemist_vehicle_elements::Com;
///
/// let mut com = Com::new();
/// com.tick();
/// ```
pub struct Com {
    /// Local values storage and management
    pub lv: LocalValueManager,
    /// Fuse manager for safety and protection mechanisms
    pub fuse: FuseManager,
}

impl Com {
    /// Creates a new `Com` instance with default values.
    ///
    /// This initializes both the local values storage and fuse manager
    /// with their default configurations.
    ///
    /// # Returns
    ///
    /// A new `Com` instance ready for use.
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::Com;
    ///
    /// let com = Com::new();
    /// ```
    pub fn new() -> Self {
        Self {
            lv: LocalValueManager::default(),
            fuse: FuseManager::default(),
        }
    }

    /// Performs a single tick operation on the communication system.
    ///
    /// This method advances the internal state of the fuse manager by one tick,
    /// which is typically called in a main loop to update timing-sensitive
    /// operations and safety mechanisms.
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::Com;
    ///
    /// let mut com = Com::new();
    ///
    /// // In your main loop
    /// loop {
    ///     com.tick();
    ///     // Other operations...
    /// }
    /// ```
    pub fn tick(&mut self) {
        self.fuse.tick();
    }
}

impl Default for Com {
    /// Creates a default `Com` instance.
    ///
    /// This is equivalent to calling `Com::new()` and provides a convenient
    /// way to create a `Com` instance using the `Default` trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::Com;
    ///
    /// let com = Com::default();
    /// // or
    /// let com: Com = Default::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}
