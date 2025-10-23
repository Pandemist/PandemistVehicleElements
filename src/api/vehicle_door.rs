//! Vehicle door management system for Lotus Script integration.
//!
//! This module provides functionality to control and monitor vehicle doors,
//! interfacing with the Lotus Script variable system for state management.

use lotus_script::var::{get_var, set_var};

/// Represents a single vehicle door with entry and exit capabilities.
///
/// The `VehicleDoor` struct manages the state of a vehicle door, including
/// whether it's open, available for entry/exit, and handles communication
/// with the Lotus Script system through predefined variables.
///
/// # Examples
///
/// ```rust
/// use your_crate::VehicleDoor;
///
/// // Create a new door with ID 1, entry enabled, exit disabled
/// let mut door = VehicleDoor::new(1, true, false);
///
/// // Check if someone is requesting to enter
/// if door.request_in() {
///     log::info!("STOP REQUEST");
/// }
/// ```
#[derive(Default, Debug)]
#[expect(clippy::struct_excessive_bools)]
pub struct VehicleDoor {
    /// Unique identifier for this door instance, refering to the door index from the content tool
    id: usize,
    /// Whether entry through this door is currently available
    entry_available: bool,
    /// Whether exit through this door is currently available
    exit_available: bool,
    /// Last known open state to detect changes
    open_last: bool,
    /// Last known released state to detect changes
    released_last: bool,
}

impl VehicleDoor {
    /// Creates a new `VehicleDoor` instance.
    ///
    /// # Arguments
    ///
    /// * `index` - Unique identifier for the door
    /// * `entry_init` - Initial state for entry availability
    /// * `exit_init` - Initial state for exit availability
    ///
    /// # Returns
    ///
    /// A new `VehicleDoor` instance with the specified configuration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let door = VehicleDoor::new(0, true, true);
    /// ```
    #[must_use]
    pub fn new(index: usize, entry_init: bool, exit_init: bool) -> Self {
        Self {
            id: index,
            entry_available: entry_init,
            exit_available: exit_init,
            ..Default::default()
        }
    }

    /// Sets the door open state in the Lotus Script system.
    ///
    /// This method directly updates the `DoorOpen_#` variable in the Lotus Script
    /// system where `#` is the door ID.
    ///
    /// # Arguments
    ///
    /// * `open` - The new open state for the door
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut door = VehicleDoor::new(1, true, true);
    /// door.set_open(true); // Door is now open
    /// ```
    pub fn set_open(&mut self, open: bool) {
        set_var(&format!("DoorOpen_{}", self.id), open);
    }

    /// Updates the door open state only if it has changed.
    ///
    /// This method compares the new state with the last known state and only
    /// updates the Lotus Script variable if there's been a change, reducing
    /// unnecessary variable updates.
    ///
    /// # Arguments
    ///
    /// * `open` - The new open state for the door
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut door = VehicleDoor::new(1, true, true);
    /// door.update_open(true);  // Updates the state
    /// door.update_open(true);  // No update, state unchanged
    /// door.update_open(false); // Updates the state
    /// ```
    pub fn update_open(&mut self, open: bool) {
        if open != self.open_last {
            self.open_last = open;
            self.set_open(open);
        }
    }

    /// Sets the entry availability state for this door.
    ///
    /// Updates both the internal state and the corresponding `DoorEntryAvailable_#`
    /// variable in the Lotus Script system.
    ///
    /// # Arguments
    ///
    /// * `state` - Whether entry should be available through this door
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut door = VehicleDoor::new(1, false, true);
    /// door.set_entry_available(true); // Enable entry
    /// ```
    pub fn set_entry_available(&mut self, state: bool) {
        self.entry_available = state;
        set_var(&format!("DoorEntryAvailable_{}", self.id), state);
    }

    /// Sets the exit availability state for this door.
    ///
    /// Updates both the internal state and the corresponding `DoorExitAvailable_#`
    /// variable in the Lotus Script system.
    ///
    /// # Arguments
    ///
    /// * `state` - Whether exit should be available through this door
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut door = VehicleDoor::new(1, true, false);
    /// door.set_exit_available(true); // Enable exit
    /// ```
    pub fn set_exit_available(&mut self, state: bool) {
        self.exit_available = state;
        set_var(&format!("DoorExitAvailable_{}", self.id), state);
    }

    /// Sets the entry released state in the Lotus Script system.
    ///
    /// Updates the `DoorEntryReleased_#` variable to indicate whether
    /// entry has been released/granted for this door.
    ///
    /// # Arguments
    ///
    /// * `state` - The new entry released state
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut door = VehicleDoor::new(1, true, true);
    /// if door.request_in() {
    ///     door.set_entry_released(true);
    /// }
    /// ```
    pub fn set_entry_released(&mut self, state: bool) {
        set_var(&format!("DoorEntryReleased_{}", self.id), state);
    }

    /// Sets the exit released state in the Lotus Script system.
    ///
    /// Updates the `DoorExitReleased_#` variable to indicate whether
    /// exit has been released/granted for this door.
    ///
    /// # Arguments
    ///
    /// * `state` - The new exit released state
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut door = VehicleDoor::new(1, true, true);
    /// if door.request_out() {
    ///     door.set_exit_released(true);
    /// }
    /// ```
    pub fn set_exit_released(&mut self, state: bool) {
        set_var(&format!("DoorExitReleased_{}", self.id), state);
    }

    /// Updates both entry and exit released states only if changed.
    ///
    /// This method efficiently updates both entry and exit released states
    /// simultaneously, but only if the state has actually changed since the
    /// last update.
    ///
    /// # Arguments
    ///
    /// * `state` - The new released state for both entry and exit
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut door = VehicleDoor::new(1, true, true);
    /// door.update_released(true);  // Sets both entry and exit released
    /// door.update_released(true);  // No update, state unchanged
    /// door.update_released(false); // Clears both released states
    /// ```
    pub fn update_released(&mut self, state: bool) {
        if state != self.released_last {
            self.released_last = state;
            self.set_entry_released(state);
            self.set_exit_released(state);
        }
    }

    /// Checks if there's an incoming exit request for this door.
    ///
    /// Reads the `DoorReqIn_#` variable from the Lotus Script system to
    /// determine if someone is requesting to leave through this door.
    ///
    /// # Returns
    ///
    /// `true` if there's an exit request, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let door = VehicleDoor::new(1, true, true);
    ///
    /// if door.request_in() {
    ///     println!("Someone wants to exit through door {}", door.id);
    /// }
    /// ```
    #[must_use]
    pub fn request_in(&self) -> bool {
        get_var::<bool>(&format!("DoorReqIn_{}", self.id))
    }

    /// Checks if there's an outgoing enter request for this door.
    ///
    /// Reads the `DoorReqOut_#` variable from the Lotus Script system to
    /// determine if someone is requesting to enter through this door.
    ///
    /// # Returns
    ///
    /// `true` if there's an enter request, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let door = VehicleDoor::new(1, true, true);
    ///
    /// if door.request_out() {
    ///     println!("Someone wants to enter through door {}", door.id);
    /// }
    /// ```
    #[must_use]
    pub fn request_out(&self) -> bool {
        get_var::<bool>(&format!("DoorReqOut_{}", self.id))
    }

    /// Checks if the door area is currently occupied.
    ///
    /// Reads the `DoorOccupied_#` variable from the Lotus Script system to
    /// determine if the door area is occupied by a person or object.
    ///
    /// # Returns
    ///
    /// `true` if the door is occupied, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let door = VehicleDoor::new(1, true, true);
    ///
    /// if door.occupied() {
    ///     println!("Door {} area is occupied", door.id);
    /// }
    /// ```
    #[must_use]
    pub fn occupied(&self) -> bool {
        get_var::<bool>(&format!("DoorOccupied_{}", self.id))
    }
}
