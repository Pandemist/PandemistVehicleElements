//! Local values management for train systems.
//!
//! This module provides a type-safe storage system for various train subsystem values
//! using typed map keys. It supports different categories of train system data including
//! structural components, electrical systems, lighting, doors, traction, and cab controls.
//!
//! # Examples
//!
//! ```rust
//! use local_values::{LocalValueManager, WslSpeedometerKmh, WslInteriorLight};
//!
//! let mut values = LocalValueManager::new();
//!
//! // Set speed value
//! values.set(WslSpeedometerKmh, 80.5);
//!
//! // Set interior light state
//! values.set(WslInteriorLight, true);
//!
//! // Retrieve values
//! let speed = values.get(WslSpeedometerKmh);
//! let light_on = values.get_or(WslInteriorLight, false);
//! ```

use typedmap::{TypedMap, TypedMapKey};

/// A type-safe storage container for local train system values.
///
/// `LocalValueManager` provides a convenient wrapper around `TypedMap` for storing
/// and retrieving various train subsystem values using strongly-typed keys.
/// Each key type is associated with a specific value type, ensuring type safety
/// at compile time.
///
/// # Examples
///
/// ```rust
/// use local_values::{LocalValueManager, WslSpeedometerKmh, WslDoorsClosed};
///
/// let mut values = LocalValueManager::new();
///
/// // Store values
/// values.set(WslSpeedometerKmh, 65.0);
/// values.set(WslDoorsClosed, true);
///
/// // Retrieve values
/// if let Some(speed) = values.get(WslSpeedometerKmh) {
///     println!("Current speed: {} km/h", speed);
/// }
///
/// // Get with default value
/// let doors_closed = values.get_or(WslDoorsClosed, false);
/// ```
pub struct LocalValueManager {
    stl: TypedMap,
}

impl LocalValueManager {
    /// Creates a new empty `LocalValueManager` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use local_values::LocalValueManager;
    ///
    /// let values = LocalValueManager::new();
    /// ```
    pub fn new() -> Self {
        Self {
            stl: TypedMap::new(),
        }
    }

    /// Sets a value for the given key.
    ///
    /// If a value for this key already exists, it will be overwritten.
    ///
    /// # Arguments
    ///
    /// * `key` - The typed key to store the value under
    /// * `value` - The value to store
    ///
    /// # Examples
    ///
    /// ```rust
    /// use local_values::{LocalValueManager, WslSpeedometerKmh};
    ///
    /// let mut values = LocalValueManager::new();
    /// values.set(WslSpeedometerKmh, 42.0);
    /// ```
    pub fn set<T: TypedMapKey + 'static>(&mut self, key: T, value: T::Value) {
        self.stl.insert(key, value);
    }

    /// Retrieves a value for the given key.
    ///
    /// Returns `Some(value)` if the key exists, `None` otherwise.
    /// The value type must implement `Clone`.
    ///
    /// # Arguments
    ///
    /// * `key` - The typed key to look up
    ///
    /// # Returns
    ///
    /// `Option<T::Value>` - The stored value if it exists
    ///
    /// # Examples
    ///
    /// ```rust
    /// use local_values::{LocalValueManager, WslSpeedometerKmh};
    ///
    /// let mut values = LocalValueManager::new();
    /// values.set(WslSpeedometerKmh, 42.0);
    ///
    /// match values.get(WslSpeedometerKmh) {
    ///     Some(speed) => println!("Speed: {}", speed),
    ///     None => println!("Speed not set"),
    /// }
    /// ```
    pub fn get<T: TypedMapKey + 'static>(&self, key: T) -> Option<T::Value>
    where
        T::Value: Clone,
    {
        self.stl.get(&key).cloned()
    }

    /// Retrieves a value for the given key, returning a default if not found.
    ///
    /// This is a convenience method that combines `get()` with a default value.
    /// The value type must implement `Clone`.
    ///
    /// # Arguments
    ///
    /// * `key` - The typed key to look up
    /// * `default` - The default value to return if the key is not found
    ///
    /// # Returns
    ///
    /// `T::Value` - The stored value if it exists, otherwise the default value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use local_values::{LocalValueManager, WslSpeedometerKmh};
    ///
    /// let values = LocalValueManager::new();
    /// let speed = values.get_or(WslSpeedometerKmh, 0.0);
    /// assert_eq!(speed, 0.0); // Returns default since key wasn't set
    /// ```
    pub fn get_or<T: TypedMapKey + 'static>(&self, key: T, default: T::Value) -> T::Value
    where
        T::Value: Clone,
    {
        match self.get(key) {
            Some(s) => s,
            None => default,
        }
    }
}

impl Default for LocalValueManager {
    /// Creates a new empty `LocalValueManager` instance.
    ///
    /// This is equivalent to calling `LocalValueManager::new()`.
    fn default() -> Self {
        Self::new()
    }
}
