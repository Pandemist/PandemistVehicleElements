use lotus_script::delta;

use crate::mocks::{
    animation::Animation,
    key_event::{KeyEvent, KeyEventCab},
};

#[derive(Debug)]
pub struct Kurbel {
    name: String,
    pub pos: f32,
    speed: f32,

    min: f32,
    max: f32,

    rotation_anim: Animation,

    key_plus: KeyEvent,
    key_minus: KeyEvent,
}

impl Kurbel {
    pub fn new(name: &str, cab_side: KeyEventCab, speed: f32, min: f32, max: f32) -> Self {
        Kurbel {
            name: name.to_string(),
            pos: 0.0,
            speed: speed,
            min: min,
            max: max,

            rotation_anim: Animation::new(format!("{}_anim", name)),

            key_plus: KeyEvent::new(format!("{}_plus", name), cab_side),
            key_minus: KeyEvent::new(format!("{}_minus", name), cab_side),
        }
    }

    pub fn tick(&mut self) {
        if self.key_plus.is_pressed() {
            self.pos = (self.pos + self.speed * delta()).min(self.max);
            self.rotation_anim.set(self.pos);
        }
        if self.key_minus.is_pressed() {
            self.pos = (self.pos - self.speed * delta()).max(self.min);
            self.rotation_anim.set(self.pos);
        }
    }
}
