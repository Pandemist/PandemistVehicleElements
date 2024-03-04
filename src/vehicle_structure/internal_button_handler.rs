use lotus_script::event::ButtonEvent;

use crate::structs::traits::OnButton;

#[derive(Debug, Default)]
pub struct InternalButtonHandler<T>{
	object_list: Vec<T>,
}

impl<T: OnButton> InternalButtonHandler<T> {
	pub fn new() -> Self {
		InternalButtonHandler{
			object_list: Vec::new(),
		}
	}

	pub fn add(&mut self, new: T) {
		self.object_list.push(new);
	}

	pub fn on_event(&mut self, ev: ButtonEvent) {
		for button_object in &mut self.object_list {
			button_object.on_button(&ev);
		}
	}
}