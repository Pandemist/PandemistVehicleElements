use crate::{mocks::animation::Animation, structs::traits::OnButton};

#[derive(Default, Debug)]
pub struct Haltewunschtaster {
    name_id: String,

    pos_a: f32,
    pos_b: f32,

    btn_a_pos: Animation,
    btn_b_pos: Animation,

    value_a: bool,
    value_b: bool,

    haltewunsch: bool,
}

impl Haltewunschtaster {
    pub fn new(name: String) -> Self {
        Haltewunschtaster {
            name_id: name.clone(),
            btn_a_pos: Animation::new(format!("{}_a_anim", name)),
            btn_b_pos: Animation::new(format!("{}_b_anim", name)),
            ..Default::default()
        }
    }
    pub fn tick(&mut self, set: bool, reset: bool) {
        self.haltewunsch = self.haltewunsch || set;
        self.haltewunsch = self.haltewunsch && !reset;
    }

    pub fn press_a(&mut self, key: bool) {
        if key {
            self.pos_a = 1.0;
        } else {
            self.pos_a = 0.0;
        }
        self.value_a = self.pos_a > 0.5;
        self.haltewunsch = self.haltewunsch || self.value_a;

        self.btn_a_pos.update_pos(self.pos_a);
    }

    pub fn press_b(&mut self, key: bool) {
        if key {
            self.pos_b = 1.0;
        } else {
            self.pos_b = 0.0;
        }
        self.value_b = self.pos_b > 0.5;
        self.haltewunsch = self.haltewunsch || self.value_b;

        self.btn_b_pos.update_pos(self.pos_b);
    }
}

impl OnButton for Haltewunschtaster {
    fn on_button(&mut self, ev: &lotus_script::event::ButtonEvent) {
        if ev.id == format!("{}_a_press", self.name_id) {
            self.press_a(ev.value);
        } else if ev.id == format!("{}_b_press", self.name_id) {
            self.press_b(ev.value);
        }
    }
}
