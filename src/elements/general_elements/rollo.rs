use lotus_script::delta;

use crate::{mocks::animation::Animation, structs::traits::OnButton};

#[derive(Debug, Default)]
pub struct Rollo {
    name_id: String,

    anim: Animation,

    pos: f32,
    grabbing: bool,
    reset: bool,
}

impl Rollo {
    pub fn new(name: String) -> Self {
        Rollo {
            name_id: name.clone(),
            anim: Animation::new(format!("{}_anim", name)),
            ..Default::default()
        }
    }

    pub fn tick(&mut self, grabbing_delta: f32) {
        if self.grabbing && grabbing_delta > 0.0 {
            self.pos = (self.pos + grabbing_delta).min(1.0);
        }
        if self.reset {
            self.pos = (self.pos - 3.0 * delta()).max(0.0);
        }

        self.anim.update_pos(self.pos);
    }
}

impl OnButton for Rollo {
    fn on_button(&mut self, ev: &lotus_script::event::ButtonEvent) {
        if ev.id == format!("{}_grab", self.name_id) {
            self.grabbing = ev.value;
        } else if ev.id == format!("{}_reset", self.name_id) {
            self.grabbing = ev.value;
        }
    }
}
