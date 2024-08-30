use lotus_script::delta;

use crate::{
    mocks::{
        animation::Animation,
        key_event::{KeyEvent, KeyEventCab},
    },
    structs::structs::FourDirections,
};

#[derive(Debug)]
pub struct HandPin {
    name: String,
    pos_x: f32,
    pos_y: f32,
    pub direction: FourDirections,
    pub selected_side: bool,

    rot_anim: Animation,
    x_move_anim: Animation,
    y_move_anim: Animation,

    key_toggle: KeyEvent,
    key_grab: KeyEvent,
    key_target_o: KeyEvent,
    key_target_u: KeyEvent,
    key_target_r: KeyEvent,
    key_target_l: KeyEvent,
}

impl HandPin {
    pub fn new(name: &str, cab_side: KeyEventCab) -> Self {
        HandPin {
            name: name.to_string(),
            pos_x: 0.0,
            pos_y: 0.0,
            direction: FourDirections::default(),
            selected_side: false,

            rot_anim: Animation::new(format!("{}_rot_anim", name)),
            x_move_anim: Animation::new(format!("{}_x_anim", name)),
            y_move_anim: Animation::new(format!("{}_y_anim", name)),

            key_target_o: KeyEvent::new(format!("{}_target_o", name), cab_side),
            key_target_u: KeyEvent::new(format!("{}_target_u", name), cab_side),
            key_target_r: KeyEvent::new(format!("{}_target_r", name), cab_side),
            key_target_l: KeyEvent::new(format!("{}_target_l", name), cab_side),

            key_toggle: KeyEvent::new(format!("{}_toggle", name), cab_side),
            key_grab: KeyEvent::new(format!("{}_grab", name), cab_side),
        }
    }

    pub fn tick(&mut self, delta_x: f32, delta_y: f32) {
        if self.key_toggle.is_just_pressed() {
            self.selected_side = !self.selected_side;
        }

        if self.key_grab.is_pressed() {
            self.pos_x = (self.pos_x + delta_x * delta()).min(1.0).max(-1.0);
            self.pos_y = (self.pos_y + delta_y * delta()).min(1.0).max(-1.0);
        } else {
            if self.key_target_l.is_pressed() {
                self.pos_x = self.pos_x - 2.0 * delta();
            } else if self.key_target_r.is_pressed() {
                self.pos_x = self.pos_x + 2.0 * delta();
            } else if self.key_target_o.is_pressed() {
                self.pos_y = self.pos_y + 2.0 * delta();
            } else if self.key_target_u.is_pressed() {
                self.pos_y = self.pos_y - 2.0 * delta();
            } else {
                self.pos_x = self.pos_x - (self.pos_x / 2.0);
                self.pos_y = self.pos_y - (self.pos_y / 2.0);
            }

            if self.pos_x.abs() < 0.025 {
                self.pos_x = 0.0;
            }
            if self.pos_y.abs() < 0.025 {
                self.pos_y = 0.0;
            }
        }

        self.direction = FourDirections::new(
            self.pos_y > 0.8,
            self.pos_y < -0.8,
            self.pos_x > 0.8,
            self.pos_x < -0.8,
        );

        self.rot_anim.set(self.selected_side.into());
        self.x_move_anim.set(self.pos_x);
        self.y_move_anim.set(self.pos_y);
    }
}

#[derive(Debug)]
pub struct HandPinAnalog {
    name: String,
    pos_x: f32,
    pos_y: f32,
    pub direction: FourDirections,
    pub selected_side: bool,

    rot_anim: Animation,
    x_move_anim: Animation,
    y_move_anim: Animation,

    key_toggle: KeyEvent,
    key_grab: KeyEvent,
    key_target_o: KeyEvent,
    key_target_u: KeyEvent,
    key_target_r: KeyEvent,
    key_target_l: KeyEvent,
}

impl HandPinAnalog {
    pub fn new(name: &str, cab_side: KeyEventCab) -> Self {
        HandPinAnalog {
            name: name.to_string(),
            pos_x: 0.0,
            pos_y: 0.0,
            direction: FourDirections::default(),
            selected_side: false,

            rot_anim: Animation::new(format!("{}_rot_anim", name)),
            x_move_anim: Animation::new(format!("{}_x_anim", name)),
            y_move_anim: Animation::new(format!("{}_y_anim", name)),

            key_target_o: KeyEvent::new(format!("{}_target_o", name), cab_side),
            key_target_u: KeyEvent::new(format!("{}_target_u", name), cab_side),
            key_target_r: KeyEvent::new(format!("{}_target_r", name), cab_side),
            key_target_l: KeyEvent::new(format!("{}_target_l", name), cab_side),

            key_toggle: KeyEvent::new(format!("{}_toggle", name), cab_side),
            key_grab: KeyEvent::new(format!("{}_grab", name), cab_side),
        }
    }
    pub fn tick(&mut self, delta_x: f32, delta_y: f32) {
        if self.key_toggle.is_just_pressed() {
            self.selected_side = !self.selected_side;
        }

        if self.key_grab.is_pressed() {
            if self.pos_x < 0.025 && self.pos_x > -0.025 {
                self.pos_x = (self.pos_x + delta_x * delta()).min(1.0).max(-1.0);
            }
            if self.pos_y < 0.025 && self.pos_y > -0.025 {
                self.pos_y = (self.pos_y + delta_y * delta()).min(1.0).max(-1.0);
            }
        } else {
            if self.key_target_l.is_pressed() {
                self.pos_x = self.pos_x - 2.0 * delta();
            } else if self.key_target_r.is_pressed() {
                self.pos_x = self.pos_x + 2.0 * delta();
            } else if self.key_target_o.is_pressed() {
                self.pos_y = self.pos_y + 2.0 * delta();
            } else if self.key_target_u.is_pressed() {
                self.pos_y = self.pos_y - 2.0 * delta();
            } else {
                self.pos_x = self.pos_x - (self.pos_x / 2.0);
                self.pos_y = self.pos_y - (self.pos_y / 2.0);
            }

            if self.pos_x.abs() < 0.025 {
                self.pos_x = 0.0;
            }
            if self.pos_y.abs() < 0.025 {
                self.pos_y = 0.0;
            }
        }

        self.direction = FourDirections::new(
            self.pos_y > 0.8,
            self.pos_y < -0.8,
            self.pos_x > 0.8,
            self.pos_x < -0.8,
        );

        self.rot_anim.set(self.selected_side.into());
        self.x_move_anim.set(self.pos_x);
        self.y_move_anim.set(self.pos_y);
    }
}
