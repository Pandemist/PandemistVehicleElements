use std::f32::consts::PI;

use lotus_script::delta;

use crate::mocks::animation::Animation;

const WIPERSPEED_RAD_PER_S: f32 = 3.415;
const WIPERSPEED_RAD_PER_S_FAST: f32 = WIPERSPEED_RAD_PER_S * 2.0;
const WIPERINTERVAL: f32 = 3.0;

#[derive(Debug, Default)]
pub struct Scheibenwischer {
    name_id: String,

    pos: f32,
    motor_pos: f32,
    timer: f32,

    anim: Animation,
}

impl Scheibenwischer {
    pub fn new(name: String) -> Self {
        Scheibenwischer {
            name_id: name.clone(),
            motor_pos: 2.0 * PI,
            anim: Animation::new(format!("{}_anim", name)),
            ..Default::default()
        }
    }

    pub fn tick(&mut self, target_level: i32, spannung: f32) {
        if spannung > 0.5 {
            if target_level > 0 {
                self.timer = self.timer + delta();
            } else {
                self.timer = WIPERINTERVAL + 1.0;
            }

            let relais = (self.motor_pos < 2.0 * PI - 0.0001)
                || target_level > 1
                || (self.timer > WIPERINTERVAL && target_level > 0);

            let speed = if target_level > 2 {
                WIPERSPEED_RAD_PER_S_FAST
            } else {
                WIPERSPEED_RAD_PER_S
            };

            if relais {
                self.timer = 0.0;
                if self.motor_pos > 2.0 * PI {
                    self.motor_pos = self.motor_pos - 2.0 * PI;
                }
                self.motor_pos = self.motor_pos + spannung * speed * delta();
            } else {
                self.motor_pos = 2.0 * PI;
            }
        } else {
            self.timer = WIPERINTERVAL * 1.0;
        }

        self.pos = 1.0 - self.motor_pos.cos() / 2.0;

        self.anim.update_pos(self.pos);
    }
}
