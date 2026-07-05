use lotus_extra::messages::trainbus;
use lotus_script::{message::Coupling, prelude::Message};

use crate::messages::coupling_handler::MessageLine;

#[derive(Debug)]
pub struct SingleCouplingLine<T: MessageLine> {
    pub received: T,
    pub last_send: T,
    pub is_allowed: bool,
    pub is_coupled: bool,
    side: Coupling,
}

impl<T: MessageLine> SingleCouplingLine<T> {
    pub fn new(side: Coupling, allowed: bool) -> Self {
        Self {
            received: T::default(),
            last_send: T::default(),
            is_allowed: allowed,
            is_coupled: false,
            side,
        }
    }

    pub fn on_message(&mut self, msg: Message) -> bool {
        let mut changed = false;

        msg.handle::<trainbus::EcouplerState>(|m| {
            if m.side == self.side && self.is_coupled != m.value {
                self.is_coupled = m.value;
                if !m.value {
                    // Bei Entkupplung: Empfangswert zurücksetzen.
                    self.received = T::default();
                }
                changed = true;
            }
            Ok(())
        })
        .expect("EcouplerState: message handle failed");

        if msg.source().is_front() || msg.source().is_rear() {
            if let Some((side, value)) = T::default().rcv(msg.clone()) {
                if side == self.side && self.received != value {
                    self.received = value;
                    changed = true;
                }
            }
        }

        changed
    }

    pub fn update_permit(&mut self, allow: bool) -> bool {
        if self.is_allowed != allow {
            self.is_allowed = allow;
            true
        } else {
            false
        }
    }

    pub fn get_received(&self) -> T {
        if self.is_allowed && self.is_coupled {
            self.received.clone()
        } else {
            T::default()
        }
    }

    pub fn send_value(&mut self, value: T) {
        if self.is_coupled && self.is_allowed && value != self.last_send {
            self.last_send = value.clone();
            T::default().send(value, self.side);
        }
    }
}
