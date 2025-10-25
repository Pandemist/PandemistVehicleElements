//! Standard Plus Messages
//!
//! This module provides message types and utilities for the Standard Plus messaging system.
//! It includes functionality for GPM (General Purpose Module) state management and
//! module-to-cabin communication.

use lotus_script::{
    message::{send_message, MessageTarget},
    prelude::message_type,
};
use serde::{Deserialize, Serialize};

//===================================================================
// Choice between GPM and ZB for IBIS
//===================================================================

/// Message indicating whether GPM (General Purpose Module) is available.
///
/// This message is used to broadcast the availability state of the GPM system
/// within the IBIS (Integrated Board Information System) architecture.
/// It helps components determine whether to use GPM or ZB (Zielbezeichnung) protocols.
///
/// # Examples
///
/// ```rust
/// use pandemist_vehicle_elements::HasGPM;
///
/// let gpm_msg = HasGPM {};
/// // Message will be sent via send_gpm_state() function
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HasGPM;

message_type!(HasGPM, "Pan", "hasGPM");

/// Broadcasts the current GPM availability state to all connected components.
///
/// This function sends a `HasGPM` message to all subscribers in the system,
/// including the sender itself. The broadcast does not cross coupling boundaries,
/// ensuring it stays within the local system context.
///
/// # Behavior
///
/// - Broadcasts to all connected components
/// - Includes the sender in the broadcast (`include_self: true`)
/// - Does not cross coupling boundaries (`across_couplings: false`)
///
/// # Examples
///
/// ```rust
/// use pandemist_vehicle_elements::send_gpm_state;
///
/// // Notify all components about GPM availability
/// send_gpm_state();
/// ```
pub fn send_gpm_state() {
    send_message(
        &(HasGPM),
        [MessageTarget::Broadcast {
            across_couplings: false,
            include_self: true,
        }],
    );
}
