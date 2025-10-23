use std::collections::HashMap;

use lotus_script::{
    message::{send_message, Coupling, MessageMeta, MessageTarget},
    prelude::{message_type, Message, MessageType},
};
use serde::{Deserialize, Serialize};

type MessageKey = (String, String);
type Subscriber = Box<dyn FnMut(&Message)>;
type SubscriberList = Vec<Subscriber>;
type SubscriberMap = HashMap<MessageKey, SubscriberList>;

#[derive(Default)]
pub struct MessageMan {
    subscribers: SubscriberMap,
}

impl MessageMan {
    pub fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
        }
    }

    pub fn subscribe<F>(&mut self, meta: &MessageMeta, callback: F)
    where
        F: FnMut(&Message) + 'static,
    {
        let key: MessageKey = (meta.namespace.to_string(), meta.identifier.to_string());
        self.subscribers
            .entry(key)
            .or_default()
            .push(Box::new(callback));
    }

    pub fn record_message(&mut self, msg: Message) {
        let meta = msg.meta();
        let key: MessageKey = (meta.namespace.to_string(), meta.identifier.to_string());

        if let Some(callbacks) = self.subscribers.get_mut(&key) {
            for callback in callbacks {
                callback(&msg);
            }
        }
    }
}

//-----------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct CounterUpdate {
        value: i32,
    }

    //message_type!(TestMessage, "Std", "Batterymainswitch");

    impl MessageType for CounterUpdate {
        const MESSAGE_META: MessageMeta = MessageMeta::new("Std", "Batterymainswitch", None);
    }

    #[derive(Debug)]
    struct Counter {
        value: i32,
    }

    impl Counter {
        fn new() -> Self {
            Self { value: 0 }
        }

        // Alternative Nutzung zu "subscribe"
        fn register(&mut self, man: &mut MessageMan) {
            // SAFETY: Self ist exklusiv referenziert, daher ist *mut self erlaubt
            let self_ptr: *mut Self = self;

            man.subscribe(&CounterUpdate::MESSAGE_META, move |msg| {
                // SAFETY: `self_ptr` ist gültig, solange der Subscriber lebt.
                let counter = unsafe { &mut *self_ptr };

                let update: CounterUpdate = msg.value().unwrap();
                counter.value += update.value;
            });
        }

        // Alternative Nutzung zu "register"
        fn subscribe(manager: &mut MessageMan, state: Rc<RefCell<Self>>) {
            manager.subscribe(&CounterUpdate::MESSAGE_META, move |msg| {
                let payload: CounterUpdate = msg.value().unwrap();
                state.borrow_mut().value += payload.value;
            });
        }
    }

    #[test]
    fn test_counter_subscription() {
        let mut message_man = MessageMan::new();

        // Variante 1
        let counter = Rc::new(RefCell::new(Counter::new()));

        Counter::subscribe(&mut message_man, counter.clone());

        let msg1 = Message::new(&CounterUpdate { value: 10 });
        let msg2 = Message::new(&CounterUpdate { value: -4 });

        message_man.record_message(msg1);
        message_man.record_message(msg2);

        assert_eq!(counter.borrow().value, 6);

        // Variante 2
        let mut counter = Counter::new();

        counter.register(&mut message_man);

        let msg1 = Message::new(&CounterUpdate { value: 10 });
        let msg2 = Message::new(&CounterUpdate { value: -4 });

        message_man.record_message(msg1);
        message_man.record_message(msg2);

        assert_eq!(counter.value, 6);
    }
}

//-----------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct CounterUpdate {
    value: i32,
}

message_type!(CounterUpdate, "Std", "Batterymainswitch");

// Deine Nachrichtenstruktur
#[derive(Debug)]
struct Counter {
    value: i32,
}

impl Counter {
    fn new() -> Self {
        Self { value: 0 }
    }

    fn subscribe_to_updates(&mut self, manager: &mut MessageMan) {
        let value_ptr = &mut self.value as *mut i32;

        manager.subscribe(&CounterUpdate::MESSAGE_META, move |msg| {
            // Safety: Gültig solange Counter nicht moved wird (nur für einfache Demo!)
            let payload: CounterUpdate = msg.value().unwrap();
            unsafe {
                *value_ptr += payload.value;
            }
        });
    }
}

//-----------------------------

pub struct DiffusionSubscriber {
    meta: MessageMeta,
    value: i32,
}

impl DiffusionSubscriber {
    pub fn new(meta: MessageMeta) -> Self {
        Self { meta, value: 0 }
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn update_local(&mut self, delta: i32) {
        self.apply_and_propagate(delta);
    }

    pub fn register(&mut self, man: &mut MessageMan) {
        // Pointer-Hack wie vorher
        let self_ptr: *mut Self = self;

        man.subscribe(&self.meta, move |msg| {
            let this = unsafe { &mut *self_ptr };

            let src = msg.source();

            let Ok(diff_msg): Result<CounterUpdate, _> = msg.value() else {
                return;
            };

            if src.is_front() {
                this.apply_and_propagate(diff_msg.value);
            } else if src.is_rear() {
                this.apply_and_propagate(-diff_msg.value);
            }
        });
    }

    fn apply_and_propagate(&mut self, delta: i32) {
        if delta != 0 {
            self.value += delta;
            self.send_update();
        }
    }

    fn send_update(&self) {
        let value = self.value;

        send_message(
            &(CounterUpdate { value }),
            [MessageTarget::AcrossCoupling {
                coupling: Coupling::Rear,
                cascade: false,
            }],
        );

        send_message(
            &(CounterUpdate { value: -value }),
            [MessageTarget::AcrossCoupling {
                coupling: Coupling::Front,
                cascade: false,
            }],
        );
    }
}

//-----------------------------

use std::cell::RefCell;
use std::collections::VecDeque;

// Mocking Framework für Sendeoperationen
thread_local! {
    static SENT_MESSAGES: RefCell<VecDeque<(MessageTarget, i32)>> = const { RefCell::new(VecDeque::new()) };
}

#[test]
fn test_diffusion_subscriber() {
    let mut man = MessageMan::new();
    let mut subscriber = DiffusionSubscriber::new(CounterUpdate::MESSAGE_META.clone());
    subscriber.register(&mut man);

    // Initialwert prüfen
    assert_eq!(subscriber.value(), 0);

    // Lokale Änderung durchführen
    subscriber.update_local(10);
    assert_eq!(subscriber.value(), 10);

    SENT_MESSAGES.with(|log| {
        let mut log = log.borrow_mut();
        let sent: Vec<_> = log.drain(..).collect();

        let expected = [
            (
                MessageTarget::AcrossCoupling {
                    coupling: Coupling::Rear,
                    cascade: false,
                },
                10,
            ),
            (
                MessageTarget::AcrossCoupling {
                    coupling: Coupling::Front,
                    cascade: false,
                },
                -10,
            ),
        ];

        assert_eq!(sent.len(), expected.len());
        for (a, b) in sent.iter().zip(expected.iter()) {
            assert_eq!(format!("{:?}", a), format!("{:?}", b));
        }
    });

    // Nachricht von Front: +5
    let msg = Message::new(&CounterUpdate {
        //    source: MessageSource::Front,
        value: 5,
    });
    man.record_message(msg);
    assert_eq!(subscriber.value(), 15);

    SENT_MESSAGES.with(|log| {
        let mut log = log.borrow_mut();
        let sent: Vec<_> = log.drain(..).collect();

        let expected = [
            (
                MessageTarget::AcrossCoupling {
                    coupling: Coupling::Rear,
                    cascade: false,
                },
                15,
            ),
            (
                MessageTarget::AcrossCoupling {
                    coupling: Coupling::Front,
                    cascade: false,
                },
                -15,
            ),
        ];

        assert_eq!(sent.len(), expected.len());
        for (a, b) in sent.iter().zip(expected.iter()) {
            assert_eq!(format!("{:?}", a), format!("{:?}", b));
        }
    });

    // Nachricht von Back: +7 (aber intern: -7)
    let msg = Message::new(&CounterUpdate {
        //    source: MessageSource::Back,
        value: 7,
    });
    man.record_message(msg);
    assert_eq!(subscriber.value(), 8); // 15 - 7

    SENT_MESSAGES.with(|log| {
        let mut log = log.borrow_mut();
        let sent: Vec<_> = log.drain(..).collect();

        let expected = [
            (
                MessageTarget::AcrossCoupling {
                    coupling: Coupling::Rear,
                    cascade: false,
                },
                8,
            ),
            (
                MessageTarget::AcrossCoupling {
                    coupling: Coupling::Front,
                    cascade: false,
                },
                -8,
            ),
        ];

        assert_eq!(sent.len(), expected.len());
        for (a, b) in sent.iter().zip(expected.iter()) {
            assert_eq!(format!("{:?}", a), format!("{:?}", b));
        }
    });
}
