use lotus_script::event::ButtonEvent;

use crate::{mocks::animation::Animation, structs::traits::OnButton};

#[derive(Default, Debug)]
pub struct Notbremgriff {
    name_id: String,

    pos: f32,
    value: bool,

    anim: Animation,
}

impl Notbremgriff {
    pub fn new(name: String) -> Self {
        Notbremgriff {
            name_id: name.clone(),
            anim: Animation::new(format!("{}_anim", name)),
            ..Default::default()
        }
    }

    pub fn toggle(&mut self) {
        self.pos = 1.0 - self.pos;
        self.update_value();
    }

    pub fn on(&mut self) -> bool {
        if !self.value {
            self.pos = 1.0;
            self.update_value();
            return true;
        }
        false
    }

    pub fn off(&mut self) -> bool {
        if self.value {
            self.pos = 0.0;
            self.update_value();
            return true;
        }
        false
    }

    fn update_value(&mut self) {
        self.value = self.pos > 0.5;
        self.anim.update_pos(self.pos);
    }
}

impl OnButton for Notbremgriff {
    fn on_button(&mut self, ev: &ButtonEvent) {
        if ev.id == format!("{}_toggle", self.name_id) {
            if ev.value {
                self.toggle();
            }
        } else if ev.id == format!("{}_on", self.name_id) {
            if ev.value {
                self.on();
            }
        } else if ev.id == format!("{}_off", self.name_id) {
            if ev.value {
                self.off();
            }
        }
    }
}

#[test]
fn test_notbremsgriff() {
    let mut nb = Notbremgriff::default();
    nb.toggle();
    assert_eq!(nb.pos, 1.0);
    assert_eq!(nb.value, true);

    nb.toggle();
    assert_eq!(nb.pos, 0.0);
    assert_eq!(nb.value, false);

    let mut nb = Notbremgriff::default();
    assert_eq!(nb.on(), true);
    assert_eq!(nb.pos, 1.0);
    assert_eq!(nb.value, true);
    assert_eq!(nb.on(), false);

    assert_eq!(nb.off(), true);
    assert_eq!(nb.pos, 0.0);
    assert_eq!(nb.value, false);
    assert_eq!(nb.off(), false);
}
