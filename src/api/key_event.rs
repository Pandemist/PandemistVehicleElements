//! Key event handling for cabin-based control systems.
//!
//! This module provides functionality for handling key events in a dual-cabin environment,
//! where events can be associated with either cabin A or cabin B. It includes support for
//! both physical key events and programmatic injection of events.
//!
//! The main components are:
//! - [`KeyEvent`]: Handles key press/release state tracking with cabin awareness

use lotus_extra::vehicle::CockpitSide;
use lotus_script::action::state;

/// A key event handler that tracks press/release states with cabin awareness.
///
/// `KeyEvent` provides functionality for detecting key press and release events
/// in a dual-cabin system. It supports both physical key events (from the lotus_script
/// state system) and programmatic event injection.
///
/// The event handler maintains state between calls to track transitions between
/// pressed and released states, enabling detection of "just pressed" and "just released"
/// events.
///
/// # Examples
///
/// ```
/// use pandemist_vehicle_elements::KeyEvent;
/// use lotus_extra::vehicle::CockpitSide;
///
/// // Create a key event for cabin A
/// let mut key_event = KeyEvent::new(Some("thrust_lever"), Some(CockpitSide::A));
///
/// // Check if the key was just pressed
/// if key_event.is_just_pressed() {
///     println!("Thrust lever was just pressed in cabin A!");
/// }
///
/// // Inject an event programmatically
/// key_event.injection = true;
/// assert!(key_event.is_pressed());
/// ```
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct KeyEvent {
    /// The name of the key event (corresponds to lotus_script state names)
    name: Option<String>,
    /// Which cabin this event is associated with
    cab_side: Option<CockpitSide>,
    /// Whether this event is currently being injected programmatically
    pub injection: bool,
    /// The previous state of the injection flag (used for edge detection)
    injection_last: bool,
}

impl KeyEvent {
    /// Creates a new `KeyEvent` instance.
    ///
    /// # Arguments
    ///
    /// * `name` - Optional name of the key event (should match lotus_script state names)
    /// * `cab_side` - Optional cabin association for the event
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::KeyEvent;
    /// use lotus_extra::vehicle::CockpitSide;
    ///
    /// // Create an event with both name and cabin
    /// let key_event = KeyEvent::new(Some("engine_start"), Some(CockpitSide::A));
    ///
    /// // Create an event with just a name
    /// let key_event = KeyEvent::new(Some("master_alarm"), None);
    ///
    /// // Create an event for programmatic injection only
    /// let key_event = KeyEvent::new(None, Some(CockpitSide::B));
    /// ```
    pub fn new(name: Option<&str>, cab_side: Option<CockpitSide>) -> Self {
        Self {
            name: name.map(|s| s.into()),
            cab_side,
            injection: false,
            injection_last: false,
        }
    }

    /// Checks if the current event matches the associated cabin.
    ///
    /// This method verifies that the event's cabin assignment matches the
    /// cockpit index from the lotus_script state system.
    ///
    /// # Returns
    ///
    /// * `true` if the cabin matches or if there's no cabin restriction
    /// * `false` if there's a cabin mismatch or no event name is set
    fn matching_cab(&self) -> bool {
        match (self.cab_side, &self.name) {
            (Some(cab), Some(ev)) => {
                if let Some(ev_side) = state(ev).cockpit_index {
                    (cab == CockpitSide::A && ev_side == 0)
                        || (cab == CockpitSide::B && ev_side == 1)
                } else {
                    // No Cabin Index found
                    false
                }
            }

            (_, Some(_)) => true, // Event name available, but no driver level index
            (_, None) => false,   // No event name set, so no
        }
    }

    /// Checks if the key was just pressed (transition from released to pressed).
    ///
    /// This method detects the rising edge of a key press event, returning `true`
    /// only on the frame when the key transitions from released to pressed state.
    /// It considers both physical key events and programmatic injection.
    ///
    /// # Returns
    ///
    /// `true` if the key was just pressed, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::KeyEvent;
    /// use lotus_extra::vehicle::CockpitSide;
    ///
    /// let mut key_event = KeyEvent::new(Some("brake"), Some(CockpitSide::A));
    ///
    /// // This would return true only on the frame when the brake is first pressed
    /// if key_event.is_just_pressed() {
    ///     println!("Brake application started!");
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This method mutates the internal state to track edge transitions.
    /// It should be called once per frame for accurate edge detection.
    #[must_use]
    pub fn is_just_pressed(&mut self) -> bool {
        let action = if let Some(ev) = &self.name {
            state(ev).kind.is_just_pressed() && self.matching_cab()
        } else {
            false
        };

        let result = action || (self.injection && !self.injection_last);
        self.injection_last = self.injection;
        result
    }

    /// Checks if the key was just released (transition from pressed to released).
    ///
    /// This method detects the falling edge of a key press event, returning `true`
    /// only on the frame when the key transitions from pressed to released state.
    /// It considers both physical key events and programmatic injection.
    ///
    /// # Returns
    ///
    /// `true` if the key was just released, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::KeyEvent;
    /// use lotus_extra::vehicle::CockpitSide;
    ///
    /// let mut key_event = KeyEvent::new(Some("throttle"), Some(CockpitSide::B));
    ///
    /// // This would return true only on the frame when the throttle is released
    /// if key_event.is_just_released() {
    ///     println!("Throttle released!");
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This method mutates the internal state to track edge transitions.
    /// It should be called once per frame for accurate edge detection.
    #[must_use]
    pub fn is_just_released(&mut self) -> bool {
        let action = if let Some(ev) = &self.name {
            state(ev).kind.is_just_released() && self.matching_cab()
        } else {
            false
        };

        let result = action || (!self.injection && self.injection_last);
        self.injection_last = self.injection;
        result
    }

    /// Checks if the key is currently pressed.
    ///
    /// This method returns `true` as long as the key is in the pressed state,
    /// regardless of how long it has been pressed. It considers both physical
    /// key events and programmatic injection.
    ///
    /// # Returns
    ///
    /// `true` if the key is currently pressed, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::KeyEvent;
    /// use lotus_extra::vehicle::CockpitSide;
    ///
    /// let mut key_event = KeyEvent::new(Some("gear_lever"), Some(CockpitSide::A));
    ///
    /// // This returns true for as long as the gear lever is held down
    /// if key_event.is_pressed() {
    ///     println!("Gear lever is currently pressed");
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This method mutates the internal state to track edge transitions
    /// for other methods. It should be called once per frame.
    #[must_use]
    pub fn is_pressed(&mut self) -> bool {
        let action = if let Some(ev) = &self.name {
            state(ev).kind.is_pressed() && self.matching_cab()
        } else {
            false
        };

        let result = action || self.injection;
        self.injection_last = self.injection;
        result
    }

    /// Checks if the key is currently released.
    ///
    /// This method returns `true` when the key is not pressed, but only
    /// considers physical key events (ignores programmatic injection).
    ///
    /// # Returns
    ///
    /// `true` if the key is currently released and not being injected, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use pandemist_vehicle_elements::KeyEvent;
    /// use lotus_extra::vehicle::CockpitSide;
    ///
    /// let mut key_event = KeyEvent::new(Some("flaps"), Some(CockpitSide::B));
    ///
    /// if key_event.is_released() {
    ///     println!("Flaps control is not being pressed");
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This method mutates the internal state to track edge transitions
    /// for other methods. It should be called once per frame.
    #[must_use]
    pub fn is_released(&mut self) -> bool {
        let action = if let Some(ev) = &self.name {
            state(ev).kind.is_released() && self.matching_cab()
        } else {
            false
        };

        let result = action && !self.injection;
        self.injection_last = self.injection;
        result
    }
}
