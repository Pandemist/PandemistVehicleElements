use lotus_script::event::{ButtonEvent, FloatInputEvent};

use super::{internal_enums::DoorState, internal_events::InternalEvents};

pub trait OnButton {
    fn on_button(&mut self, ev: &ButtonEvent);
}

pub trait OnFloat {
    fn on_float(&mut self, ev: FloatInputEvent);
}

pub trait InternalEventListener {
    fn on_internal_event(&mut self, ev: InternalEvents);
}

pub trait ElectricEquipment {
    fn voltage(&self) -> f32;
}

pub trait PassengerDoor {
    fn state(&self) -> &DoorState;
    fn closed(&self) -> bool;
}
