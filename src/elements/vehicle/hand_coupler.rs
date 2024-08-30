use lotus_script::delta;

use crate::mocks::{
    animation::Animation,
    key_event::{KeyEvent, KeyEventCab},
    visible_flag::Visiblility,
};

const KUPPLUNG_REFLECT_CLOSE: f32 = 0.95;

#[derive(Debug)]
pub struct HandCoupler {
    id: usize,
    pos: f32,
    cab: Visiblility,
    speed: f32,
    daempfung: f32,
    coupler_anim: Animation,
    hebel_anim: Animation,
    key_klappe: KeyEvent,
    key_grab: KeyEvent,
    key_hebel: KeyEvent,
}

impl HandCoupler {
    pub fn new(id: usize, cab_side: KeyEventCab, daempfung: f32) -> Self {
        HandCoupler {
            id: id,
            pos: 0.0,
            cab: Visiblility::new(format!("vis_kupplungsklappe_{}", id)),
            speed: 0.0,
            daempfung: daempfung,
            coupler_anim: Animation::new(format!("hand_coupler_{}_anim", id)),
            hebel_anim: Animation::new(format!("hand_coupler_{}_hebel_anim", id)),
            key_klappe: KeyEvent::new(format!("hand_coupler_{}_cab", id), cab_side),
            key_grab: KeyEvent::new(format!("hand_coupler_{}_grab", id), cab_side),
            key_hebel: KeyEvent::new(format!("hand_coupler_{}_hebel", id), cab_side),
        }
    }

    pub fn tick(&mut self, force: f32) {
        if self.key_klappe.is_just_pressed() {
            self.cab.make_invisible();
        }

        let grabbing = self.key_grab.is_pressed() || self.key_hebel.is_pressed();
        let hebel_pos = self.key_hebel.is_pressed().into();

        if force > 0.0 && hebel_pos > 0.5 && self.pos > 0.99 {
            self.speed = 0.0;
            return;
        }

        if grabbing && (hebel_pos < 0.5 && self.pos > (KUPPLUNG_REFLECT_CLOSE + 0.001)) {
            self.pos = 1.0;
            return;
        }

        let pos_last = self.pos;

        if grabbing {
            if hebel_pos > 0.5 {
                self.pos = (self.pos + delta()).max(0.0).min(1.0);
            } else {
                self.pos = (self.pos + force).max(0.0).min(KUPPLUNG_REFLECT_CLOSE);
            }

            self.speed = force / delta();
        }

        if hebel_pos < 0.5 && self.pos > KUPPLUNG_REFLECT_CLOSE && self.pos != 1.0 {
            self.pos = KUPPLUNG_REFLECT_CLOSE - 0.001;
            self.speed = -0.2 * self.speed;
        }

        if hebel_pos > 0.5 && self.pos >= 1.0 {
            self.pos = 1.0;
            self.speed = 0.0;
        }

        if pos_last > 0.0 && self.pos <= 0.0 {
            self.pos = 0.0;
            self.speed = 0.0;
            self.cab.make_visible();
        }

        if self.speed.abs() > 0.0 {
            let new_speed = self.speed + (-self.speed.signum() * self.daempfung) * delta();

            if new_speed * self.speed < 0.0 {
                self.speed = 0.0;
            } else {
                self.speed = new_speed;
            }
        }

        self.hebel_anim.set(hebel_pos);
        self.coupler_anim.set(self.pos);
    }
}
