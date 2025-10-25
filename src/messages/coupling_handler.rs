//! # Coupling Handler
//!
//! This module provides a universal coupling line system for train communication.
//! It manages bidirectional message passing between coupled train cars, handling
//! both physical coupling state and message routing with permission controls.

use lotus_script::{message::Coupling, prelude::Message};
use serde::{Deserialize, Serialize};

use crate::management::trainbus;

/// A trait for handling message communication between coupled train cars.
///
/// This trait defines the interface for evaluating, sending, and receiving
/// messages across coupling connections. Implementations should handle the
/// specific message types and evaluation logic for their use case.
///
/// # Type Parameters
///
/// * `T` - The type of value being communicated, must be comparable and cloneable
///
/// # Examples
///
/// ```rust,ignore
/// use lotus_script::message::Coupling;
/// use lotus_script::prelude::Message;
///
/// struct MyMessageHandler;
///
/// impl MessageLine<i32> for MyMessageHandler {
///     fn evaluate(&self, a: &i32, b: &i32) -> i32 {
///         a + b  // Simple addition logic
///     }
///
///     fn send(&self, value: i32, side: Coupling) {
///         // Send implementation
///     }
///
///     fn rcv(&self, msg: Message) -> Option<(Coupling, i32)> {
///         // Receive implementation
///         None
///     }
/// }
/// ```
pub trait MessageLine<T: PartialEq + Clone> {
    /// Evaluates two values and returns a combined result.
    ///
    /// This method defines how values from different sources should be
    /// combined or processed. The evaluation logic depends on the specific
    /// use case (e.g., logical operations, arithmetic, priority selection).
    ///
    /// # Arguments
    ///
    /// * `a` - First value to evaluate
    /// * `b` - Second value to evaluate
    ///
    /// # Returns
    ///
    /// The evaluated result of combining the two input values
    fn evaluate(&self, a: &T, b: &T) -> T;

    /// Sends a value to the specified coupling side.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to send
    /// * `side` - The coupling side (Front or Rear) to send to
    fn send(&self, value: T, side: Coupling);

    /// Attempts to receive and parse a message.
    ///
    /// # Arguments
    ///
    /// * `msg` - The incoming message to process
    ///
    /// # Returns
    ///
    /// `Some((side, value))` if the message was successfully parsed and contains
    /// a value for the specified coupling side, `None` otherwise
    fn rcv(&self, msg: Message) -> Option<(Coupling, T)>;
}

//-----------------------------------------------------------------------------------

/// A universal coupling line that manages bidirectional communication between train cars.
///
/// This struct handles the complex logic of coupling state management, message routing,
/// and permission controls for train communication systems. It maintains local and
/// received values, tracks coupling states, and manages when communication is allowed.
///
/// # Type Parameters
///
/// * `T` - The type of value being communicated. Must implement `Default`, `Clone`, `Serialize`, `Deserialize`, and `PartialEq`
/// * `H` - The message handler implementing `MessageLine<T>`
///
/// # Features
///
/// - **Bidirectional Communication**: Handles both front and rear coupling connections
/// - **Permission Control**: Allows enabling/disabling communication per side
/// - **State Management**: Tracks coupling states and prevents unnecessary message sending
/// - **Automatic Updates**: Recalculates and sends values when state changes
///
/// # Examples
///
/// ```rust,ignore
/// use pandemist_vehicle_elements::coupling_handler::{UniversalCouplingLine, MessageLine};
///
/// // Create a coupling line with front coupling allowed, rear disabled
/// let coupling_line = UniversalCouplingLine::new(
///     MyMessageHandler::new(),
///     (true, false)  // (front_allowed, rear_allowed)
/// );
/// ```
pub struct UniversalCouplingLine<
    T: Default + Clone + Serialize + for<'a> Deserialize<'a> + PartialEq,
    H: MessageLine<T>,
> {
    /// The message handler responsible for processing messages
    message_handler: H,

    /// The current local value for this coupling line
    pub local_value: T,
    /// The last values sent to (front, rear) to avoid duplicate sends
    pub last_send: (T, T),
    /// The values received from (front, rear) coupled cars
    pub received: (T, T),

    /// Permission flags for (front, rear) communication
    pub is_allowed: (bool, bool),
    /// Current coupling state for (front, rear) connections
    pub is_coupled: (bool, bool),
}

impl<T: Default + Clone + Serialize + for<'a> Deserialize<'a> + PartialEq, H: MessageLine<T>>
    UniversalCouplingLine<T, H>
{
    /// Creates a new universal coupling line with the specified message handler and permissions.
    ///
    /// # Arguments
    ///
    /// * `message_handler` - The handler implementing `MessageLine<T>` for message processing
    /// * `allowed` - A tuple of (front_allowed, rear_allowed) permission flags
    ///
    /// # Returns
    ///
    /// A new `UniversalCouplingLine` instance with default values and the specified configuration
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let coupling_line = UniversalCouplingLine::new(
    ///     MyHandler::new(),
    ///     (true, true)  // Allow both front and rear communication
    /// );
    /// ```
    pub fn new(message_handler: H, allowed: (bool, bool)) -> Self {
        Self {
            message_handler,

            local_value: T::default(),
            last_send: (T::default(), T::default()),
            received: (T::default(), T::default()),

            is_allowed: (allowed.0, allowed.1),
            is_coupled: (false, false),
        }
    }

    /// Processes an incoming message and updates the coupling line state accordingly.
    ///
    /// This method handles two types of messages:
    /// 1. E-coupler messages that update the physical coupling state
    /// 2. Value messages from coupled cars that update received values
    ///
    /// # Arguments
    ///
    /// * `msg` - The incoming message to process
    ///
    /// # Panics
    ///
    /// Panics if an e-coupler message cannot be handled properly
    pub fn on_message(&mut self, msg: Message) {
        // Receive e-coupler
        msg.handle::<trainbus::EcouplerState>(|m| {
            self.update_coupler(m.side, m.value);
            Ok(())
        })
        .expect("EcouplerState: message handle failed");

        // Receive value from the clutch
        if msg.source().is_front() || msg.source().is_rear() {
            if let Some((side, value)) = self.message_handler.rcv(msg.clone()) {
                if Self::write_to(&mut self.received, &value, side) {
                    self.update();
                }
            }
        }
    }

    /// Updates the communication permissions for front and rear couplings.
    ///
    /// If permissions change, this triggers an update cycle to recalculate
    /// and send values based on the new permission state.
    ///
    /// # Arguments
    ///
    /// * `allow_front` - Whether to allow front coupling communication
    /// * `allow_rear` - Whether to allow rear coupling communication
    pub fn update_permit(&mut self, allow_front: bool, allow_rear: bool) {
        if self.is_allowed.0 != allow_front {
            self.is_allowed.0 = allow_front;
            self.update();
        }
        if self.is_allowed.1 != allow_rear {
            self.is_allowed.1 = allow_rear;
            self.update();
        }
    }

    /// Updates the local value and triggers recalculation if the value changed.
    ///
    /// # Arguments
    ///
    /// * `value` - The new local value to set
    pub fn update_local(&mut self, value: T) {
        if self.local_value != value {
            self.local_value = value;
            self.update();
        }
    }

    /// Gets the current evaluated value combining local and received values.
    ///
    /// This method applies the message handler's evaluation logic to combine:
    /// 1. The local value
    /// 2. The evaluated combination of front and rear received values (respecting permissions)
    ///
    /// # Returns
    ///
    /// The evaluated result combining all available values
    pub fn get_value(&mut self) -> T {
        let front = if self.is_allowed.0 {
            &self.received.0
        } else {
            &T::default()
        };

        let rear = if self.is_allowed.1 {
            &self.received.1
        } else {
            &T::default()
        };

        self.message_handler.evaluate(
            &self.local_value,
            &self.message_handler.evaluate(front, rear),
        )
    }

    /// Gets the current value received from the front coupling.
    ///
    /// # Returns
    ///
    /// The front received value if front communication is allowed, otherwise the default value
    pub fn get_front(&mut self) -> T {
        if self.is_allowed.0 {
            self.received.0.clone()
        } else {
            T::default()
        }
    }

    /// Gets the current value received from the rear coupling.
    ///
    /// # Returns
    ///
    /// The rear received value if rear communication is allowed, otherwise the default value
    pub fn get_rear(&mut self) -> T {
        if self.is_allowed.1 {
            self.received.1.clone()
        } else {
            T::default()
        }
    }

    /// Internal method that recalculates and sends values to coupled cars when state changes.
    ///
    /// This method:
    /// 1. Evaluates what values should be sent to front and rear
    /// 2. Compares with last sent values to avoid duplicates
    /// 3. Sends new values if they differ from previously sent ones
    fn update(&mut self) {
        let front = if self.is_allowed.0 {
            &self.received.0
        } else {
            &T::default()
        };

        let rear = if self.is_allowed.1 {
            &self.received.1
        } else {
            &T::default()
        };

        let to_front = if self.is_allowed.1 {
            self.message_handler.evaluate(&self.local_value, rear)
        } else {
            self.local_value.clone()
        };
        let to_rear = if self.is_allowed.0 {
            self.message_handler.evaluate(&self.local_value, front)
        } else {
            self.local_value.clone()
        };

        //let to_front = self.message_handler.evaluate(&self.local_value, rear);
        //let to_rear = self.message_handler.evaluate(&self.local_value, front);

        if to_front != self.last_send.0 {
            self.send_to(Coupling::Front, to_front);
        }
        if to_rear != self.last_send.1 {
            self.send_to(Coupling::Rear, to_rear);
        }
    }

    /// Sends a value to the specified coupling side if conditions are met.
    ///
    /// Values are only sent if:
    /// 1. The specified side is physically coupled
    /// 2. Communication to that side is allowed
    ///
    /// # Arguments
    ///
    /// * `side` - The coupling side to send to
    /// * `value` - The value to send
    fn send_to(&mut self, side: Coupling, value: T) {
        match side {
            Coupling::Front => {
                if self.is_coupled.0 && self.is_allowed.0 {
                    self.last_send.0 = value.clone();
                    self.message_handler.send(value, side);
                }
            }
            Coupling::Rear => {
                if self.is_coupled.1 && self.is_allowed.1 {
                    self.last_send.1 = value.clone();
                    self.message_handler.send(value, side);
                }
            }
        }
    }

    /// Updates the coupling state for the specified side.
    ///
    /// When a coupling is disconnected, the received value for that side
    /// is reset to default and an update cycle is triggered.
    ///
    /// # Arguments
    ///
    /// * `side` - The coupling side to update
    /// * `value` - The new coupling state (true = coupled, false = uncoupled)
    fn update_coupler(&mut self, side: Coupling, value: bool) {
        match side {
            Coupling::Front => {
                if self.is_coupled.0 != value {
                    self.is_coupled.0 = value;
                    if !value {
                        // Reset control line if no longer coupled
                        self.received.0 = T::default();
                    }
                    self.update();
                }
            }
            Coupling::Rear => {
                if self.is_coupled.1 != value {
                    self.is_coupled.1 = value;
                    if !value {
                        // Reset control line if no longer coupled
                        self.received.1 = T::default();
                    }
                    self.update();
                }
            }
        }
    }

    /// Writes a value to the specified side of a tuple if it differs from the current value.
    ///
    /// # Arguments
    ///
    /// * `var` - The mutable tuple to write to
    /// * `value` - The value to write
    /// * `side` - Which side of the tuple to write to
    ///
    /// # Returns
    ///
    /// `true` if the value was written (it differed), `false` if no change was made
    fn write_to(var: &mut (T, T), value: &T, side: Coupling) -> bool {
        if &Self::read_from(var, side) != value {
            match side {
                Coupling::Front => var.0 = value.clone(),
                Coupling::Rear => var.1 = value.clone(),
            }
            true
        } else {
            false
        }
    }

    /// Reads a value from the specified side of a tuple.
    ///
    /// # Arguments
    ///
    /// * `var` - The tuple to read from
    /// * `side` - Which side of the tuple to read from
    ///
    /// # Returns
    ///
    /// A clone of the value from the specified side
    fn read_from(var: &(T, T), side: Coupling) -> T {
        match side {
            Coupling::Front => var.0.clone(),
            Coupling::Rear => var.1.clone(),
        }
    }
}
