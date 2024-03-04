use crate::structs::{internal_events::InternalEvents, traits::InternalEventListener};

pub struct InternalEventBus {
	listener: Vec<Box<dyn InternalEventListener>>,
}

impl InternalEventBus{
	pub fn new() -> Self {
		InternalEventBus{
			listener: Vec::new(),
		}
	}

	pub fn trigger_event(&mut self, ev: InternalEvents) {
		for event_listener in &mut self.listener {
			event_listener.on_internal_event(ev.clone());
		}
	}

	pub fn register_handler(&mut self, listener: Box<dyn InternalEventListener>) {
		self.listener.push(listener);
	}
}