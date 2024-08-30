use lotus_script::delta;

use crate::mocks::{
    animation::Animation,
    key_event::{KeyEvent, KeyEventCab},
};

const HANDDOOR_REFLECT_CLOSE: f32 = 0.049;

#[derive(Debug)]
pub struct HandDoor {
    id: usize,
    pos: f32,
    speed: f32,
    daempfung: f32,
    door_anim: Animation,
    riegel_anim: Animation,
    klinke_anim: Animation,
    key_grab_a: KeyEvent,
    key_grab_b: KeyEvent,
    key_klinke: KeyEvent,
}

impl HandDoor {
    pub fn new(id: usize, cab_side: KeyEventCab, daempfung: f32) -> Self {
        HandDoor {
            id: id,
            pos: 0.0,
            speed: 0.0,
            daempfung: daempfung,
            door_anim: Animation::new(format!("fahrerraumtuer_{}_door_anim", id)),
            riegel_anim: Animation::new(format!("fahrerraumtuer_{}_riegel_anim", id)),
            klinke_anim: Animation::new(format!("fahrerraumtuer_{}_klinke_anim", id)),
            key_grab_a: KeyEvent::new(format!("fahrerraumtuer_{}_grab_a", id), cab_side),
            key_grab_b: KeyEvent::new(format!("fahrerraumtuer_{}_grab_b", id), cab_side),
            key_klinke: KeyEvent::new(format!("fahrerraumtuer_{}_klinke", id), cab_side),
        }
    }

    pub fn tick(&mut self, force: f32, physic_force: f32) {
        let grabbing_a = self.key_grab_a.is_pressed();
        let grabbing_b = self.key_grab_b.is_pressed();
        let pos_handle = (grabbing_a || grabbing_b || self.key_klinke.is_pressed()).into();

        let pos_riegel = (self.pos > 0.5
            || (self.pos > 0.0 && (self.pos + 0.001) < HANDDOOR_REFLECT_CLOSE))
            as i32 as f32;

        if (force > 0.0 && physic_force > 0.0) && self.pos < 0.01 && pos_handle != 1.0 {
            self.speed = 0.0;
            return;
        }

        if (grabbing_a || grabbing_b) && (pos_riegel != 1.0 && self.pos < 0.005) {
            self.pos = 0.0;
            return;
        }

        if grabbing_a {
            self.pos = if pos_riegel < 0.5 {
                (self.pos + force).max(0.0).min(1.0)
            } else {
                (self.pos + force).max(HANDDOOR_REFLECT_CLOSE).min(1.0)
            };
            self.speed = force / delta();
        } else if grabbing_b {
            self.pos = if pos_riegel < 0.5 {
                (self.pos - force).max(0.0).min(1.0)
            } else {
                (self.pos - force).max(HANDDOOR_REFLECT_CLOSE).min(1.0)
            };
            self.speed = force / delta();
        } else {
            self.pos = if pos_riegel < 0.5 && self.pos >= HANDDOOR_REFLECT_CLOSE {
                (self.pos + self.speed * delta()).max(HANDDOOR_REFLECT_CLOSE)
            } else {
                self.pos + self.speed * delta()
            };

            self.speed = physic_force / delta();
        }

        if self.pos > 1.0 {
            self.pos = 1.0;
            self.speed = -0.2 * self.speed;
        }

        if self.pos < 0.005 {
            self.pos = 0.0;
            self.speed = 0.0;
        }

        if pos_riegel < 0.5 && self.pos < HANDDOOR_REFLECT_CLOSE && self.pos != 0.0 {
            self.pos = HANDDOOR_REFLECT_CLOSE + 0.001;
            self.speed = -0.2 * self.speed;
        }

        if self.pos < HANDDOOR_REFLECT_CLOSE && self.pos > 0.0 {
            self.speed = 0.0;
        }

        if self.speed != 0.0 {
            let new_speed = self.speed + (-self.speed.signum() * self.daempfung) * delta();

            self.speed = if new_speed * self.speed < 0.0 {
                0.0
            } else {
                new_speed
            };
        }

        self.door_anim.set(self.pos);
        self.riegel_anim.set(pos_riegel);
        self.klinke_anim.set(pos_handle);
    }
}
