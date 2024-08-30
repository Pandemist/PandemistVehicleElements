use lotus_script::delta;

use crate::mocks::{
    animation::Animation,
    key_event::{KeyEvent, KeyEventCab},
    sound::{Sound, SoundTarget},
};

const THROTTLE_LEVER_SPEED: f32 = 1.0;
const THROTTLE_LEVER_SPEED_HIGH: f32 = 5.0;
const THROTTLE_LEVER_SPEED_VERYHIGH: f32 = 20.0;

#[derive(Debug, PartialEq)]
pub enum ThrottleMode {
    EmBrake,
    Brake,
    Neutral,
    Throttle,
}

#[derive(Debug)]
pub struct SollwertgeberGt6n {
    name: String,
    speed: f32,
    target: f32,
    pos: f32,
    mode: ThrottleMode,
    raste: i32,
    raste_new: i32,

    pos_anim: Animation,

    snd_notch_neutral: Sound,
    snd_notch_end: Sound,
    snd_notch_other: Sound,

    key_throttle: KeyEvent,
    key_neutral: KeyEvent,
    key_brake: KeyEvent,
    key_max_brake: KeyEvent,
}

impl SollwertgeberGt6n {
    pub fn new(name: &str, cab_side: KeyEventCab) -> Self {
        SollwertgeberGt6n {
            name: name.to_string(),
            speed: 0.0,
            target: 0.0,
            pos: 0.0,
            mode: ThrottleMode::Neutral,
            raste: 0,
            raste_new: 0,
            pos_anim: Animation::new(format!("{}_anim", name)),
            snd_notch_neutral: Sound::new(format!("snd_{}_neutral", name)),
            snd_notch_end: Sound::new(format!("snd_{}_end", name)),
            snd_notch_other: Sound::new(format!("snd_{}_other", name)),

            key_throttle: KeyEvent::new("Throttle".to_string(), cab_side),
            key_neutral: KeyEvent::new("Neutral".to_string(), cab_side),
            key_brake: KeyEvent::new("Brake".to_string(), cab_side),
            key_max_brake: KeyEvent::new("MaxBrake".to_string(), cab_side),
        }
    }

    pub fn tick(&mut self) {
        if self.key_throttle.is_just_pressed() {
            match self.mode {
                ThrottleMode::Throttle => {
                    self.speed = THROTTLE_LEVER_SPEED;
                    self.target = 1.0;
                }
                ThrottleMode::Neutral => {
                    self.speed = THROTTLE_LEVER_SPEED;
                    self.target = 1.0;
                    self.mode = ThrottleMode::Throttle;
                }
                _ => {
                    if self.pos < -0.15 {
                        self.speed = THROTTLE_LEVER_SPEED;
                        self.target = -0.1;
                        if self.pos < -0.9 {
                            self.pos = -0.9;
                        }
                        self.mode = ThrottleMode::Brake;
                    } else {
                        self.speed = -THROTTLE_LEVER_SPEED_HIGH;
                        self.target = 0.0;
                        self.mode = ThrottleMode::Neutral;
                    }
                }
            }
        }
        if self.key_neutral.is_just_pressed() {
            if self.pos > 0.0 {
                self.speed = -THROTTLE_LEVER_SPEED_HIGH;
            } else {
                self.speed = THROTTLE_LEVER_SPEED_HIGH;
            }
            self.target = 0.0;
            self.mode = ThrottleMode::Neutral;
        }
        if self.key_brake.is_just_pressed() {
            match self.mode {
                ThrottleMode::Throttle => {
                    if self.pos > 0.15 {
                        self.speed = -THROTTLE_LEVER_SPEED;
                        self.target = 0.1;
                    } else {
                        self.speed = -THROTTLE_LEVER_SPEED_HIGH;
                        self.target = 0.0;
                        self.mode = ThrottleMode::Neutral;
                    }
                }
                ThrottleMode::Neutral => {
                    self.speed = -THROTTLE_LEVER_SPEED;
                    self.target = -0.9;
                    self.mode = ThrottleMode::Brake;
                }
                ThrottleMode::Brake => {
                    if self.pos > -0.9 {
                        self.speed = -THROTTLE_LEVER_SPEED;
                        self.target = -0.9;
                    }
                }
                _ => {}
            }
        }
        if self.key_max_brake.is_just_pressed() {
            self.speed = -THROTTLE_LEVER_SPEED_VERYHIGH;
            self.target = -1.0;
            self.mode = ThrottleMode::EmBrake;
        }

        if self.key_throttle.is_just_released()
            || self.key_neutral.is_just_released()
            || self.key_brake.is_just_released()
        {
            match self.mode {
                ThrottleMode::EmBrake => {}
                ThrottleMode::Neutral => {}
                _ => {
                    if self.pos > 0.1 || self.pos < -0.1 {
                        self.target = self.pos;
                        self.speed = 0.0;
                    } else if self.target > 0.0 {
                        self.target = 0.1;
                    } else {
                        self.target = -0.1;
                    }
                }
            }
        }

        self.pos = (self.pos + self.speed * delta()).min(1.0).max(0.0);

        if (self.speed > 0.0 && self.pos >= self.target)
            || (self.speed < 0.0 && self.pos <= self.target)
        {
            self.pos = self.target;
            self.speed = 0.0;
        }

        self.raste_new = if self.pos < -0.95 {
            0
        } else if self.pos < -0.87 {
            1
        } else if self.pos < -0.87 {
            2
        } else if self.pos < -0.05 {
            3
        } else if self.pos < 0.05 {
            4
        } else if self.pos < 0.13 {
            5
        } else if self.pos < 0.97 {
            6
        } else {
            7
        };

        if self.raste_new != self.raste {
            if self.raste == 4 {
                self.snd_notch_neutral.update_target(SoundTarget::Start);
            } else if (self.raste == 6 && self.raste_new == 7)
                || (self.raste == 2 && self.raste_new == 1)
            {
                self.snd_notch_end.update_target(SoundTarget::Start);
            } else if !((self.raste == 6 && self.raste_new == 7)
                || (self.raste == 2 && self.raste_new == 1))
            {
                self.snd_notch_other.update_target(SoundTarget::Start);
            }
        }
        self.raste = self.raste_new;

        self.pos_anim.set(self.pos);
    }

    pub fn axis_input(&mut self, cab_is_vr: bool, new_value: f32) {
        self.speed = 0.0;
        self.target = new_value;

        if cab_is_vr {
            self.pos = 0.0;
            self.raste = 4;
        } else {
            self.pos = new_value;
            self.mode = if self.raste == 0 || new_value < -0.99 {
                ThrottleMode::EmBrake
            } else if self.raste <= 3 {
                ThrottleMode::Brake
            } else if self.raste == 4 {
                ThrottleMode::Neutral
            } else {
                ThrottleMode::Throttle
            };
        }
    }
}
