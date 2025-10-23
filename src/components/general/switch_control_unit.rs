//! Switch Control Unit module for railway automation systems.
//!
//! This module provides a priority-based switch control system that manages
//! routing directions based on requests from multiple sources including
//! vehicle systems and control modules.

use std::collections::HashMap;

use lotus_script::prelude::Message;

use crate::messages::fis_messages::{RoutingCode, RoutingDirection, RoutingRequest};

/// Identifies the source of a switch control request.
///
/// Switch requests can originate from different sources in the railway system,
/// each with different priority levels for conflict resolution.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SwitchSender {
    /// Request from a specific control module identified by its ID
    Modul(u32),
    /// Request from the vehicle system
    Vehicle,
}

/// Priority-based switch control unit for railway routing systems.
///
/// The `SwitchControlUnit` manages routing directions based on requests from
/// multiple sources, resolving conflicts using a configurable priority system.
/// It handles sensor triggers, routing codes, and maintains the current state
/// of switch positions.
///
/// # Examples
///
/// ```rust
/// use your_crate::SwitchControlUnit;
/// use your_crate::SwitchSender;
///
/// // Create a control unit with priority order: Vehicle first, then Module 1
/// let priorities = vec![SwitchSender::Vehicle, SwitchSender::Modul(1)];
/// let mut control_unit = SwitchControlUnit::new(priorities, 42);
///
/// // Process a tick with current request states
/// control_unit.tick(true, false, true);
/// ```
pub struct SwtichControlUnit {
    /// Priority order for resolving conflicting switch requests.
    /// Earlier entries have higher priority.
    priorities: Vec<SwitchSender>,

    /// Current routing direction values from each sender
    values: HashMap<SwitchSender, RoutingDirection>,

    /// Currently active routing direction after priority resolution
    current: RoutingDirection,

    /// Current routing code for the switch
    routing_code: u32,

    /// Sensor ID associated with this switch control unit
    sensor_id: u32,

    /// Whether a vehicle is currently in the trigger zone
    trigger_zone: bool,

    /// Current state of switch request activity
    switch_request_active: bool,

    /// Current state of signal request activity
    signal_request_active: bool,

    /// Current state of routing request activity
    routing_request_active: bool,
}

impl SwtichControlUnit {
    /// Creates a new switch control unit with the specified priority order and sensor ID.
    ///
    /// # Arguments
    ///
    /// * `priorities` - Vector defining the priority order for switch requests.
    ///   Earlier entries have higher priority when resolving conflicts.
    /// * `sensor_id` - The sensor ID that this control unit monitors for trigger events.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_crate::{SwitchControlUnit, SwitchSender};
    ///
    /// let priorities = vec![
    ///     SwitchSender::Vehicle,
    ///     SwitchSender::Modul(1),
    ///     SwitchSender::Modul(2),
    /// ];
    /// let control_unit = SwitchControlUnit::new(priorities, 123);
    /// ```
    pub fn new(priorities: Vec<SwitchSender>, sensor_id: u32) -> Self {
        Self {
            priorities,
            values: HashMap::new(),
            current: RoutingDirection::default(),

            routing_code: 0,

            sensor_id,

            trigger_zone: false,

            switch_request_active: false,
            signal_request_active: false,
            routing_request_active: false,
        }
    }

    /// Updates the control unit with current request activity states.
    ///
    /// This method should be called regularly (typically each control cycle)
    /// to update the internal state with current system conditions.
    ///
    /// # Arguments
    ///
    /// * `switch_request_active` - Whether there is currently an active switch request
    /// * `signal_request_active` - Whether there is currently an active signal request
    /// * `routing_request_active` - Whether there is currently an active routing request
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use your_crate::SwitchControlUnit;
    /// # let mut control_unit = SwitchControlUnit::new(vec![], 1);
    /// // Update with current system state
    /// control_unit.tick(true, false, true);
    /// ```
    pub fn tick(
        &mut self,
        switch_request_active: bool,
        signal_request_active: bool,
        routing_request_active: bool,
    ) {
        self.switch_request_active = switch_request_active;
        self.signal_request_active = signal_request_active;
        self.routing_request_active = routing_request_active;
    }

    /// Determines the current switch target based on priority resolution.
    ///
    /// Iterates through the priority list and returns the routing direction
    /// from the highest-priority sender that has a non-Off value.
    ///
    /// # Returns
    ///
    /// The routing direction from the highest-priority active sender,
    /// or `RoutingDirection::Off` if no active requests exist.
    fn get_current_switch_target(&self) -> RoutingDirection {
        for sender in &self.priorities {
            if let Some(&val) = self.values.get(sender) {
                if val != RoutingDirection::Off {
                    return val;
                }
            }
        }
        RoutingDirection::Off
    }

    /// Processes incoming messages from the railway system.
    ///
    /// Handles different types of PIS (Passenger Information System) messages
    /// including routing requests, direction changes, and routing codes.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to process
    ///
    /// # Message Types Handled
    ///
    /// * `MsgPisRoutingRequest` - Routing requests (TODO: implementation pending)
    /// * `MsgPisRoutingDirection` - Direction changes from vehicle systems
    /// * `MsgPisRoutingCode` - Routing code updates
    ///
    /// # Panics
    ///
    /// Panics if message handling fails for any of the supported message types.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use your_crate::SwitchControlUnit;
    /// # use lotus_script::prelude::Message;
    /// # let mut control_unit = SwitchControlUnit::new(vec![], 1);
    /// # let msg = Message::new(); // This would be a real message in practice
    /// control_unit.on_message(msg);
    /// ```
    pub fn on_message(&mut self, msg: Message) {
        msg.handle::<RoutingRequest>(|m| {
            // TODO: RoutingRequest implementation
            Ok(())
        })
        .expect("RoutingRequest: message handle failed");

        msg.handle::<RoutingDirection>(|m| {
            self.values.insert(SwitchSender::Vehicle, m);
            self.current = self.get_current_switch_target();
            Ok(())
        })
        .expect("RoutingDirection: message handle failed");

        msg.handle::<RoutingCode>(|m| {
            self.routing_code = m.value;
            Ok(())
        })
        .expect("RoutingCode: message handle failed");
    }

    /// Handles sensor trigger events.
    ///
    /// Called when a sensor detects a vehicle entering or leaving a detection zone.
    /// Updates the trigger zone state when the sensor ID matches this control unit's
    /// monitored sensor.
    ///
    /// # Arguments
    ///
    /// * `sensor` - The ID of the sensor that triggered
    /// * `entering` - `true` if a vehicle is entering the zone, `false` if leaving
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use your_crate::SwitchControlUnit;
    /// # let mut control_unit = SwitchControlUnit::new(vec![], 42);
    /// // Vehicle enters the trigger zone for sensor 42
    /// control_unit.on_trigger(42, true);
    ///
    /// // Vehicle leaves the trigger zone
    /// control_unit.on_trigger(42, false);
    ///
    /// // Different sensor - no effect on this control unit
    /// control_unit.on_trigger(99, true);
    /// ```
    pub fn on_trigger(&mut self, sensor: u32, entering: bool) {
        if sensor == self.sensor_id {
            self.trigger_zone = entering;
            // TODO
        }
    }
}
