use lotus_script::delta;

use crate::mocks::{
    animation::Animation,
    key_event::{KeyEvent, KeyEventCab},
};

#[derive(Debug)]
pub struct OneWayDekadenschalter {
    name: String,
    pos: f32,
    pub value: i32,
    target: f32,
    pre_target: f32,

    speed: f32,
    modulo: i32,

    pos_anim: Animation,
}

impl OneWayDekadenschalter {
    pub fn new(name: &str, speed: f32, modulo: i32) -> Self {
        Self {
            name: name.to_string(),
            pos: 0.0,
            value: 0,
            target: 0.0,
            pre_target: 0.0,
            speed: speed,
            modulo: modulo,
            pos_anim: Animation::new(format!("{}_anim", name)),
        }
    }

    pub fn set(&mut self, new_pos: f32) {
        let pos = new_pos.rem_euclid(self.modulo as f32);

        self.pos = pos;
        self.target = pos;
        self.value = self.pos as i32;
        self.pos_anim.set(self.pos);
    }

    pub fn tick(&mut self, add_pos: f32) -> bool {
        if (self.target - self.pos).abs() > 0.0001 {
            self.pos = (self.pos + self.speed + delta()).min(self.target);

            self.pre_target += add_pos;
        } else {
            if self.pre_target > 0.0 {
                self.target = self.pre_target;
                self.pre_target = 0.0;
            }

            self.target = self.target + add_pos;
        }

        let result = if self.target >= self.modulo as f32 {
            self.target = self.target.rem_euclid(self.modulo as f32);
            true
        } else {
            false
        };

        self.value = self.pos as i32;
        self.pos_anim.set(self.pos);

        result
    }
}

#[derive(Debug)]
pub struct TwoWayDekadenschalter {
    name: String,
    pos: f32,
    pub value: i32,
    target: f32,
    pre_target: f32,
    direction: i8,

    speed: f32,
    modulo: i32,

    pos_anim: Animation,
}

impl TwoWayDekadenschalter {
    pub fn new(name: &str, speed: f32, modulo: i32) -> Self {
        Self {
            name: name.to_string(),
            pos: 0.0,
            value: 0,
            target: 0.0,
            pre_target: 0.0,
            direction: 0,
            speed: speed,
            modulo: modulo,
            pos_anim: Animation::new(format!("{}_anim", name)),
        }
    }

    pub fn set(&mut self, new_pos: f32) {
        let pos = new_pos.rem_euclid(self.modulo as f32);

        self.pos = pos;
        self.target = pos;
        self.value = self.pos as i32;
        self.pos_anim.set(self.pos);
    }

    pub fn tick(&mut self, new_target: f32) {
        if (self.target - self.pos).abs() > 0.0001 {
            self.direction = if self.target > self.pos { 1 } else { -1 };

            if self.direction > 0 {
                self.pos = (self.pos + self.speed + delta()).min(self.target);
            } else {
                self.pos = (self.pos - self.speed + delta()).max(self.target);
            }
        } else {
            self.target = (self.target + new_target).rem_euclid(self.modulo as f32);
            self.direction = new_target.signum() as i8;
        }

        self.pos = self.pos.rem_euclid(self.modulo as f32);

        self.value = self.pos as i32;
        self.pos_anim.set(self.pos);
    }
}

#[derive(Debug)]
pub struct ButtonDekadenschalter {
    name: String,
    pos: f32,
    pub value: i32,
    target: f32,

    speed: f32,
    modulo: i32,

    pos_anim: Animation,

    key_plus: KeyEvent,
    key_minus: KeyEvent,
}

impl ButtonDekadenschalter {
    pub fn new(name: &str, cab: KeyEventCab, speed: f32, modulo: i32) -> Self {
        let button_dekadenschalter = Self {
            name: name.to_string(),
            pos: 0.0,
            value: 0,
            target: 0.0,
            speed: speed,
            modulo: modulo,
            pos_anim: Animation::new(format!("{}_anim", name)),
            key_plus: KeyEvent::new(format!("{}_plus", name), cab),
            key_minus: KeyEvent::new(format!("{}_minus", name), cab),
        };
        button_dekadenschalter
    }

    pub fn set(&mut self, new_pos: f32) {
        let pos = new_pos.rem_euclid(self.modulo as f32);

        self.pos = pos;
        self.target = pos;
        self.value = self.pos as i32;
        self.pos_anim.set(self.pos);
    }

    pub fn tick(&mut self) {
        if (self.target - self.pos).abs() > 0.0001 {
            if self.key_plus.is_just_pressed() {
                self.target += 1.0;
            }

            if self.key_minus.is_just_pressed() {
                self.target -= 1.0;
            }
        }

        if self.target > self.pos {
            self.pos = (self.pos + self.speed + delta()).min(self.target);
        } else {
            self.pos = (self.pos - self.speed + delta()).max(self.target);
        }

        self.value = self.pos.rem_euclid(self.modulo as f32).round() as i32;
        self.pos_anim.set(self.pos);
    }
}
