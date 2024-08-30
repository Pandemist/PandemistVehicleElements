use lotus_script::{delta, rand::gen_f64};

use crate::mocks::{
    animation::Animation,
    generell::mouse_move,
    key_event::{KeyEvent, KeyEventCab},
};

#[derive(Debug, PartialEq)]
pub enum KlappsitzFeder {
    Up(f32),
    Down(f32),
    Random(f32),
    None,
}

#[derive(Debug)]
pub struct Klappsitz {
    name: String,
    pos: f32,
    force: f32,
    deampfung: f32,
    bump_factor: f32,
    speed: f32,

    mouse_factor: f32,

    key_grab: KeyEvent,
    pos_anim: Animation,
}

impl Klappsitz {
    pub fn new(
        name: &str,
        federung: KlappsitzFeder,
        deampfung: f32,
        bump_factor: f32,
        mouse_factor: f32,
    ) -> Self {
        let new_force = match federung {
            KlappsitzFeder::Up(force) => force,
            KlappsitzFeder::Down(force) => -force,
            KlappsitzFeder::Random(force) => {
                if gen_f64() > 0.5 {
                    -force
                } else {
                    force
                }
            }
            KlappsitzFeder::None => 0.0,
        };

        let new_pos = if new_force > 0.0 { 1.0 } else { 0.0 };

        Self {
            name: name.to_string(),
            pos: 0.0,
            force: new_force,
            deampfung: deampfung,
            bump_factor: bump_factor,
            mouse_factor: mouse_factor,
            speed: 0.0,
            key_grab: KeyEvent::new(format!("{}_grab", name), KeyEventCab::None),
            pos_anim: Animation::new(format!("{}_anim", name)),
        }
    }

    pub fn tick(&mut self) {
        if self.key_grab.is_pressed() {
            let hand_delta = mouse_move().y * self.mouse_factor;
            self.pos = (self.pos + hand_delta * delta()).min(1.0).max(0.0);
        } else {
            if self.force > 0.0 {
                if self.pos > 0.01 {
                    self.pos = self.pos + self.speed * delta();
                }

                if self.pos >= 1.0 {
                    self.pos = 1.0;
                    self.speed = if self.bump_factor > 0.0 {
                        -self.bump_factor * self.speed
                    } else {
                        0.0
                    };
                }

                self.speed = self.speed + self.force * delta();
            } else if self.force < 0.0 {
                if self.pos < 0.99 {
                    self.pos = self.pos - self.speed * delta();
                }

                if self.pos <= 0.0 {
                    self.pos = 0.0;
                    self.speed = if self.bump_factor > 0.0 {
                        -self.bump_factor * self.speed
                    } else {
                        0.0
                    };
                }

                self.speed = self.speed + self.force * delta();
            }
        }

        self.pos_anim.set(self.pos);
    }
}
