use lotus_script::delta;

use crate::mocks::{
    animation::Animation,
    key_event::{KeyEvent, KeyEventCab},
};

#[derive(Debug)]
pub struct Notaus {
    name: String,
    pos: f32,
    rot: f32,
    target: bool,
    value: bool,

    pos_anim: Animation,
    rot_anim: Animation,

    key_press: KeyEvent,
    key_release: KeyEvent,
}

impl Notaus {
    pub fn new(name: &str, cab_side: KeyEventCab) -> Self {
        Self {
            name: name.to_string(),
            pos: 0.0,
            rot: 0.0,
            target: false,
            value: false,
            pos_anim: Animation::new(format!("{}_anim", name)),
            rot_anim: Animation::new(format!("{}_rot_anim", name)),
            key_press: KeyEvent::new(format!("{}_press", name), cab_side),
            key_release: KeyEvent::new(format!("{}_release", name), cab_side),
        }
    }

    pub fn tick(&mut self) {
        if self.key_press.is_just_pressed() {
            self.target = true;
        }
        if self.key_release.is_just_pressed() {
            self.target = false;
        }

        if self.target {
            if self.pos < 1.0 {
                self.pos = (self.pos + 20.0 * delta()).min(1.0);
            }
        } else {
            if self.pos >= 1.0 && self.rot < 1.0 {
                self.rot = self.rot + 2.0 * delta();
            } else {
                if self.pos > 0.0 {
                    self.pos = (self.pos + 20.0 * delta()).max(0.0);
                }
                if self.rot > 0.0 {
                    self.rot = (self.rot + 20.0 * delta()).max(0.0);
                }
            }
        }
        self.value = self.pos > 0.5;

        self.pos_anim.set(self.pos);
        self.rot_anim.set(self.rot);
    }
}
