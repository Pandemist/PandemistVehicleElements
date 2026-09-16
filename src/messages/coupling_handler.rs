use lotus_extra::messages::trainbus;
use lotus_script::{
    message::{Coupling, MessageType},
    prelude::Message,
};

//-----------------------------------------------
// Message handling
pub fn handle_message<MSG: MessageType>(
    msg: &lotus_script::message::Message,
    handler: impl FnOnce(MSG) -> bool,
) -> bool {
    let mut handler_result = true;

    msg.handle(|m: MSG| {
        handler_result = handler(m);
        Ok(())
    })
    .unwrap()
        && handler_result
}

//-----------------------------------------------

pub trait MessageLine: MessageType + Default + PartialEq + Clone {
    fn evaluate(&self, a: &Self, b: &Self) -> Self;

    fn send(&self, value: Self, side: Coupling);

    fn rcv(&self, msg: Message) -> Option<(Coupling, Self)>;
}

//-----------------------------------------------------------------------------------

#[derive(Debug)]
pub struct UniversalCouplingLine<T: MessageLine> {
    pub local_value: T,
    pub last_send: (T, T),
    pub received: (T, T),

    pub is_allowed: (bool, bool),
    pub is_coupled: (bool, bool),
}

//impl<T: Default + Clone + Serialize + for<'a> Deserialize<'a> + PartialEq, H: MessageLine<T>>
impl<T: MessageLine> UniversalCouplingLine<T> {
    pub fn new(allowed: (bool, bool)) -> Self {
        Self {
            //message_handler,
            local_value: T::default(),
            last_send: (T::default(), T::default()),
            received: (T::default(), T::default()),

            is_allowed: (allowed.0, allowed.1),
            is_coupled: (false, false),
        }
    }

    pub fn on_message(&mut self, msg: Message) {
        // Receive e-coupler
        msg.handle::<trainbus::EcouplerState>(|m| {
            self.update_coupler(m.side, m.value);
            Ok(())
        })
        .expect("EcouplerState: message handle failed");

        // Receive value from the clutch
        if msg.source().is_front() || msg.source().is_rear() {
            if let Some((side, value)) = self.local_value.rcv(msg.clone()) {
                if Self::write_to(&mut self.received, &value, side) {
                    self.update();
                }
            }
        }
    }

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

    pub fn update_local(&mut self, value: T) {
        if self.local_value != value {
            self.local_value = value;
            self.update();
        }
    }

    pub fn get_value(&mut self) -> T {
        let ok_0 = self.is_allowed.0 && self.is_coupled.0;
        let ok_1 = self.is_allowed.1 && self.is_coupled.1;

        if ok_0 && ok_1 {
            self.local_value.evaluate(
                &self.local_value,
                &self
                    .local_value
                    .evaluate(&self.received.0, &self.received.1),
            )
        } else if ok_0 {
            self.local_value
                .evaluate(&self.received.0, &self.local_value)
        } else if ok_1 {
            self.local_value
                .evaluate(&self.received.1, &self.local_value)
        } else {
            self.local_value.clone()
        }
    }

    pub fn get_front(&mut self) -> T {
        if self.is_allowed.0 {
            self.received.0.clone()
        } else {
            T::default()
        }
    }

    pub fn get_rear(&mut self) -> T {
        if self.is_allowed.1 {
            self.received.1.clone()
        } else {
            T::default()
        }
    }

    fn update(&mut self) {
        let ok_0 = self.is_allowed.0 && self.is_coupled.0;
        let ok_1 = self.is_allowed.1 && self.is_coupled.1;

        let to_front = if ok_0 {
            self.local_value
                .evaluate(&self.received.1, &self.local_value)
        } else {
            self.local_value.clone()
        };

        let to_rear = if ok_1 {
            self.local_value
                .evaluate(&self.received.0, &self.local_value)
        } else {
            self.local_value.clone()
        };

        if to_front != self.last_send.0 {
            self.send_to(Coupling::Front, to_front);
        }
        if to_rear != self.last_send.1 {
            self.send_to(Coupling::Rear, to_rear);
        }
    }

    fn send_to(&mut self, side: Coupling, value: T) {
        match side {
            Coupling::Front => {
                if self.is_coupled.0 && self.is_allowed.0 {
                    self.last_send.0 = value.clone();
                    self.local_value.send(value, side);
                }
            }
            Coupling::Rear => {
                if self.is_coupled.1 && self.is_allowed.1 {
                    self.last_send.1 = value.clone();
                    self.local_value.send(value, side);
                }
            }
        }
    }

    fn update_coupler(&mut self, side: Coupling, value: bool) {
        match side {
            Coupling::Front => {
                if self.is_coupled.0 != value {
                    self.is_coupled.0 = value;
                    if !value {
                        // Reset control line if no longer coupled
                        self.received.0 = self.local_value.clone();
                    }
                    self.update();
                }
            }
            Coupling::Rear => {
                if self.is_coupled.1 != value {
                    self.is_coupled.1 = value;
                    if !value {
                        // Reset control line if no longer coupled
                        self.received.1 = self.local_value.clone();
                    }
                    self.update();
                }
            }
        }
    }

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

    fn read_from(var: &(T, T), side: Coupling) -> T {
        match side {
            Coupling::Front => var.0.clone(),
            Coupling::Rear => var.1.clone(),
        }
    }
}
