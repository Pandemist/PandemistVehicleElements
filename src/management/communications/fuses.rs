use std::collections::HashMap;

use crate::elements::tech::switches::Switch;

/// A manager for electrical fuses that controls and monitors switches.
///
/// The `FuseManager` provides a centralized way to manage multiple switches (fuses)
/// by their unique identifiers. It allows registration of switches, periodic updates,
/// and querying of their states.
///
/// # Examples
///
/// ```rust
/// use your_crate::FuseManager;
/// use your_crate::elements::tech::switches::Switch;
///
/// let mut manager = FuseManager::new();
/// let switch = Switch::new(); // Assuming Switch has a new() method
///
/// // Register a fuse with a key
/// manager.register("main_power", switch);
///
/// // Check if a fuse is on
/// if manager.is_on("main_power") {
///     println!("Main power is active");
/// }
///
/// // Update all fuses
/// manager.tick();
/// ```
#[derive(Debug)]
pub struct FuseManager {
    /// Internal storage for fuses mapped by their string identifiers
    fuse: HashMap<String, Switch>,
}

impl FuseManager {
    /// Creates a new empty `FuseManager`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_crate::FuseManager;
    ///
    /// let manager = FuseManager::new();
    /// ```
    pub fn new() -> Self {
        Self {
            fuse: HashMap::new(),
        }
    }

    /// Registers a new switch with the given key.
    ///
    /// If a switch with the same key already exists, it will be replaced.
    ///
    /// # Arguments
    ///
    /// * `key` - A unique identifier for the switch that can be converted to a `String`
    /// * `value` - The `Switch` instance to register
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_crate::FuseManager;
    /// use your_crate::elements::tech::switches::Switch;
    ///
    /// let mut manager = FuseManager::new();
    /// let switch = Switch::new();
    ///
    /// manager.register("emergency_power", switch);
    /// manager.register(String::from("backup_system"), Switch::new());
    /// ```
    pub fn register(&mut self, key: impl Into<String>, value: Switch) {
        self.fuse.insert(key.into(), value);
    }

    /// Updates all registered switches by calling their `tick()` method.
    ///
    /// This method should be called periodically to ensure all switches
    /// update their internal state properly.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_crate::FuseManager;
    ///
    /// let mut manager = FuseManager::new();
    /// // ... register some switches ...
    ///
    /// // In your main loop
    /// manager.tick();
    /// ```
    pub fn tick(&mut self) {
        for ks in self.fuse.values_mut() {
            ks.tick();
        }
    }

    /// Checks if a switch with the given key is currently on.
    ///
    /// Returns `true` if the switch exists and is on, or if no switch
    /// with the given key exists (default behavior). Returns `false`
    /// if the switch exists but is off.
    ///
    /// # Arguments
    ///
    /// * `key` - The identifier of the switch to check
    ///
    /// # Returns
    ///
    /// * `true` - if the switch is on or doesn't exist
    /// * `false` - if the switch exists but is off
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_crate::FuseManager;
    /// use your_crate::elements::tech::switches::Switch;
    ///
    /// let mut manager = FuseManager::new();
    ///
    /// // Non-existent switch returns true (safe default)
    /// assert!(manager.is_on("non_existent"));
    ///
    /// // Register and check a switch
    /// manager.register("test_switch", Switch::new());
    /// let is_active = manager.is_on("test_switch");
    /// ```
    pub fn is_on(&self, key: impl Into<String>) -> bool {
        match self.fuse.get(&key.into()) {
            Some(s) => s.value(true),
            None => true,
        }
    }
}

impl Default for FuseManager {
    /// Creates a new `FuseManager` using the default implementation.
    ///
    /// This is equivalent to calling `FuseManager::new()`.
    fn default() -> Self {
        Self::new()
    }
}
