use crate::mocks::{
    animation::Animation,
    key_event::{KeyEvent, KeyEventCab},
};

#[derive(Debug)]
pub struct Pedal {
    name: String,
    pos: f32,
    value: bool,

    pos_anim: Animation,

    key_press: KeyEvent,
    key_toggle: KeyEvent,
}

impl Pedal {
    pub fn new(name: &str, cab_side: KeyEventCab) -> Self {
        Self {
            name: name.to_string(),
            pos: 0.0,
            value: false,

            pos_anim: Animation::new(format!("{}_anim", name)),

            key_press: KeyEvent::new(format!("{}_press", name), cab_side),
            key_toggle: KeyEvent::new(format!("{}_toggle", name), cab_side),
        }
    }

    pub fn tick(&mut self) {
        if self.key_press.is_just_pressed() {
            self.pos = 1.0;
            self.update();
        }
        if self.key_press.is_just_released() {
            self.pos = 0.0;
            self.update();
        }

        if self.key_toggle.is_just_pressed() {
            self.pos = 1.0 - self.pos;
            self.update();
        }
    }

    fn update(&mut self) {
        self.value = self.pos > 0.5;
        self.pos_anim.set(self.pos);
    }
}
