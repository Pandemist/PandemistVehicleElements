use lotus_script::delta;

use crate::mocks::{
    animation::Animation,
    key_event::{KeyEvent, KeyEventCab},
    sound::{Sound, SoundTarget},
};

const KNUEPPEL_SPEED: f32 = 7.5;
const KNUEPPEL_SPEED_MIDI: f32 = 25.0;
const KNUEPPEL_SPEED_FAST: f32 = 40.0;
const KNUEPPEL_PAUSE: f32 = 0.075;

#[derive(Debug, PartialEq)]
enum MovementMode {
    None,
    Min,
    Up,
    Down,
}
#[derive(Debug)]
pub struct Knueppel {
    name: String,

    const_max_raste: i8,
    const_min_raste: i8,

    const_max_serie: i8,
    const_max_parallel: i8,

    knochen_pos: i8,

    pos: f32,
    moving_pos: f32,
    raste: i8,
    raste_last: i8,
    raste_target: i8,

    move_speed: f32,
    move_type: MovementMode,
    pause_timer: f32,

    controller: bool,

    pos_anim: Animation,

    snd_notch_m19: Sound,
    snd_notch_m18: Sound,
    snd_notch_m17: Sound,
    snd_notch_m16: Sound,
    snd_notch_m15: Sound,
    snd_notch_m14: Sound,
    snd_notch_m13: Sound,
    snd_notch_m12: Sound,
    snd_notch_m11: Sound,
    snd_notch_m10: Sound,
    snd_notch_m9: Sound,
    snd_notch_m8: Sound,
    snd_notch_m7: Sound,
    snd_notch_m6: Sound,
    snd_notch_m5: Sound,
    snd_notch_m4: Sound,
    snd_notch_m3: Sound,
    snd_notch_m2: Sound,
    snd_notch_m1: Sound,
    snd_notch_0: Sound,
    snd_notch_1: Sound,
    snd_notch_2: Sound,
    snd_notch_3: Sound,
    snd_notch_4: Sound,
    snd_notch_5: Sound,
    snd_notch_6: Sound,
    snd_notch_7: Sound,
    snd_notch_8: Sound,
    snd_notch_9: Sound,
    snd_notch_10: Sound,
    snd_notch_11: Sound,
    snd_notch_12: Sound,
    snd_notch_13: Sound,
    snd_notch_14: Sound,
    snd_notch_15: Sound,
    snd_notch_16: Sound,
    snd_notch_17: Sound,
    snd_notch_18: Sound,
    snd_notch_19: Sound,
    snd_notch_20: Sound,

    key_throttle: KeyEvent,
    key_neutral: KeyEvent,
    key_brake: KeyEvent,
    key_max_brake: KeyEvent,
}

impl Knueppel {
    fn get_pos_from_raste(&self, raste: i8) -> f32 {
        if raste > 0 {
            (raste / self.const_max_raste) as f32
        } else if raste < 0 {
            (raste.abs() / self.const_min_raste) as f32
        } else {
            0.0
        }
    }

    fn get_raste_from_pos(&self, pos: f32) -> i8 {
        if pos > 0.05 {
            (pos * (self.const_max_raste as f32)).trunc() as i8
        } else if pos < 0.05 {
            (pos * (self.const_max_raste as f32)).trunc() as i8
        } else {
            0
        }
    }

    pub fn new(
        name: &str,
        cab_side: KeyEventCab,
        fahrrasten: i8,
        bremsrasten: i8,
        dauerstufe_serie: i8,
        dauerstufe_parallel: i8,
    ) -> Self {
        Knueppel {
            name: name.to_string(),
            const_max_raste: fahrrasten,
            const_min_raste: bremsrasten,

            const_max_serie: dauerstufe_serie,
            const_max_parallel: dauerstufe_parallel,

            pos_anim: Animation::new(format!("{}_anim", name)),

            snd_notch_m19: Sound::new(format!("snd_{}_raste_m19", name)),
            snd_notch_m18: Sound::new(format!("snd_{}_raste_m18", name)),
            snd_notch_m17: Sound::new(format!("snd_{}_raste_m17", name)),
            snd_notch_m16: Sound::new(format!("snd_{}_raste_m16", name)),
            snd_notch_m15: Sound::new(format!("snd_{}_raste_m15", name)),
            snd_notch_m14: Sound::new(format!("snd_{}_raste_m14", name)),
            snd_notch_m13: Sound::new(format!("snd_{}_raste_m13", name)),
            snd_notch_m12: Sound::new(format!("snd_{}_raste_m12", name)),
            snd_notch_m11: Sound::new(format!("snd_{}_raste_m11", name)),
            snd_notch_m10: Sound::new(format!("snd_{}_raste_m10", name)),
            snd_notch_m9: Sound::new(format!("snd_{}_raste_m9", name)),
            snd_notch_m8: Sound::new(format!("snd_{}_raste_m8", name)),
            snd_notch_m7: Sound::new(format!("snd_{}_raste_m7", name)),
            snd_notch_m6: Sound::new(format!("snd_{}_raste_m6", name)),
            snd_notch_m5: Sound::new(format!("snd_{}_raste_m5", name)),
            snd_notch_m4: Sound::new(format!("snd_{}_raste_m4", name)),
            snd_notch_m3: Sound::new(format!("snd_{}_raste_m3", name)),
            snd_notch_m2: Sound::new(format!("snd_{}_raste_m2", name)),
            snd_notch_m1: Sound::new(format!("snd_{}_raste_m1", name)),
            snd_notch_0: Sound::new(format!("snd_{}_raste_0", name)),
            snd_notch_1: Sound::new(format!("snd_{}_raste_1", name)),
            snd_notch_2: Sound::new(format!("snd_{}_raste_2", name)),
            snd_notch_3: Sound::new(format!("snd_{}_raste_3", name)),
            snd_notch_4: Sound::new(format!("snd_{}_raste_4", name)),
            snd_notch_5: Sound::new(format!("snd_{}_raste_5", name)),
            snd_notch_6: Sound::new(format!("snd_{}_raste_6", name)),
            snd_notch_7: Sound::new(format!("snd_{}_raste_7", name)),
            snd_notch_8: Sound::new(format!("snd_{}_raste_8", name)),
            snd_notch_9: Sound::new(format!("snd_{}_raste_9", name)),
            snd_notch_10: Sound::new(format!("snd_{}_raste_10", name)),
            snd_notch_11: Sound::new(format!("snd_{}_raste_11", name)),
            snd_notch_12: Sound::new(format!("snd_{}_raste_12", name)),
            snd_notch_13: Sound::new(format!("snd_{}_raste_13", name)),
            snd_notch_14: Sound::new(format!("snd_{}_raste_14", name)),
            snd_notch_15: Sound::new(format!("snd_{}_raste_15", name)),
            snd_notch_16: Sound::new(format!("snd_{}_raste_16", name)),
            snd_notch_17: Sound::new(format!("snd_{}_raste_17", name)),
            snd_notch_18: Sound::new(format!("snd_{}_raste_18", name)),
            snd_notch_19: Sound::new(format!("snd_{}_raste_19", name)),
            snd_notch_20: Sound::new(format!("snd_{}_raste_20", name)),

            key_throttle: KeyEvent::new("Throttle".to_string(), cab_side),
            key_neutral: KeyEvent::new("Neutral".to_string(), cab_side),
            key_brake: KeyEvent::new("Brake".to_string(), cab_side),
            key_max_brake: KeyEvent::new("MaxBrake".to_string(), cab_side),
            knochen_pos: 0,
            pos: 0.0,
            moving_pos: 0.0,
            raste: 0,
            raste_last: 0,
            raste_target: 0,
            move_speed: 0.0,
            move_type: MovementMode::None,
            pause_timer: 0.0,
            controller: false,
        }
    }

    pub fn tick(&mut self) {
        if self.key_throttle.is_just_pressed() {
            self.controller = false;

            if self.raste < 0 {
                self.raste_target = 0;
            } else if self.raste < self.const_max_serie {
                self.raste_target = self.const_max_serie;
            } else if self.raste < self.const_max_parallel {
                self.raste_target = self.const_max_parallel;
            } else {
                self.raste_target = self.const_max_raste;
            }

            self.move_type = MovementMode::Up;
            self.move_speed = KNUEPPEL_SPEED;
        }

        if self.key_neutral.is_just_pressed() {
            self.controller = false;

            self.raste_target = 0;
            self.move_type = MovementMode::None;

            self.move_speed =
                (self.raste_target - self.raste).signum() as f32 * KNUEPPEL_SPEED_MIDI;
        }

        if self.key_brake.is_just_pressed() {
            self.controller = false;

            self.raste_target = self.const_min_raste;
            self.move_type = MovementMode::Down;
            self.move_speed = -KNUEPPEL_SPEED;
        }

        if self.key_max_brake.is_just_pressed() {
            self.controller = false;

            self.raste_target = self.const_min_raste;
            self.move_type = MovementMode::Min;
            self.move_speed = -KNUEPPEL_SPEED_FAST;
        }

        if self.key_throttle.is_just_released()
            || self.key_neutral.is_just_released()
            || self.key_brake.is_just_released()
        {
            self.controller = false;

            if self.move_type == MovementMode::Up || self.move_type == MovementMode::Down {
                self.moving_pos = self.moving_pos.round();
                self.raste = self.moving_pos.round() as i8;
                self.raste_target = self.raste;
                self.move_type = MovementMode::Min;
                self.move_speed = 0.0;
                self.pause_timer = 0.0;
            }
        }

        self.pause_timer = (self.pause_timer - delta()).max(0.0);

        if self.knochen_pos != 0 {
            if (self.knochen_pos == -2) || (self.knochen_pos == -1) {
                self.raste_target = self.raste_target.min(3);
            }

            if (self.knochen_pos == 4)
                || (self.knochen_pos == 3)
                || (self.knochen_pos == -3)
                || (self.knochen_pos == -4)
            {
                self.raste_target = self.raste_target.min(self.const_max_serie);
            }

            if !self.controller {
                if ((self.pause_timer <= 0.0)
                    && ((self.move_type == MovementMode::Down)
                        || (self.move_type == MovementMode::Up)))
                    || ((self.move_type == MovementMode::None)
                        || (self.move_type == MovementMode::Min))
                {
                    self.moving_pos = (self.moving_pos + self.move_speed * delta())
                        .max(self.const_min_raste as f32)
                        .min(self.const_max_raste as f32);

                    if ((self.move_speed > 0.0) && (self.moving_pos >= self.raste_target as f32))
                        || ((self.move_speed < 0.0)
                            && (self.moving_pos <= self.raste_target as f32))
                    {
                        self.moving_pos = self.raste_target as f32;
                        self.move_speed = 0.0;
                    }

                    self.raste = self.raste_target;

                    if self.raste != self.raste_last {
                        match self.raste {
                            20 => self.snd_notch_20.update_target(SoundTarget::Start),
                            19 => self.snd_notch_19.update_target(SoundTarget::Start),
                            18 => self.snd_notch_18.update_target(SoundTarget::Start),
                            17 => self.snd_notch_17.update_target(SoundTarget::Start),
                            16 => self.snd_notch_16.update_target(SoundTarget::Start),
                            15 => self.snd_notch_15.update_target(SoundTarget::Start),
                            14 => self.snd_notch_14.update_target(SoundTarget::Start),
                            13 => self.snd_notch_13.update_target(SoundTarget::Start),
                            12 => self.snd_notch_12.update_target(SoundTarget::Start),
                            11 => self.snd_notch_11.update_target(SoundTarget::Start),
                            10 => self.snd_notch_10.update_target(SoundTarget::Start),
                            9 => self.snd_notch_9.update_target(SoundTarget::Start),
                            8 => self.snd_notch_8.update_target(SoundTarget::Start),
                            7 => self.snd_notch_7.update_target(SoundTarget::Start),
                            6 => self.snd_notch_6.update_target(SoundTarget::Start),
                            5 => self.snd_notch_5.update_target(SoundTarget::Start),
                            4 => self.snd_notch_4.update_target(SoundTarget::Start),
                            3 => self.snd_notch_3.update_target(SoundTarget::Start),
                            2 => self.snd_notch_2.update_target(SoundTarget::Start),
                            1 => self.snd_notch_1.update_target(SoundTarget::Start),
                            0 => self.snd_notch_0.update_target(SoundTarget::Start),
                            -1 => self.snd_notch_m1.update_target(SoundTarget::Start),
                            -2 => self.snd_notch_m2.update_target(SoundTarget::Start),
                            -3 => self.snd_notch_m3.update_target(SoundTarget::Start),
                            -4 => self.snd_notch_m4.update_target(SoundTarget::Start),
                            -5 => self.snd_notch_m5.update_target(SoundTarget::Start),
                            -6 => self.snd_notch_m6.update_target(SoundTarget::Start),
                            -7 => self.snd_notch_m7.update_target(SoundTarget::Start),
                            -8 => self.snd_notch_m8.update_target(SoundTarget::Start),
                            -9 => self.snd_notch_m9.update_target(SoundTarget::Start),
                            -10 => self.snd_notch_m10.update_target(SoundTarget::Start),
                            -11 => self.snd_notch_m11.update_target(SoundTarget::Start),
                            -12 => self.snd_notch_m12.update_target(SoundTarget::Start),
                            -13 => self.snd_notch_m13.update_target(SoundTarget::Start),
                            -14 => self.snd_notch_m14.update_target(SoundTarget::Start),
                            -15 => self.snd_notch_m15.update_target(SoundTarget::Start),
                            -16 => self.snd_notch_m16.update_target(SoundTarget::Start),
                            -17 => self.snd_notch_m17.update_target(SoundTarget::Start),
                            -18 => self.snd_notch_m18.update_target(SoundTarget::Start),
                            -19 => self.snd_notch_m19.update_target(SoundTarget::Start),
                            _ => self.snd_notch_m19.update_target(SoundTarget::Start),
                        }

                        self.pause_timer = KNUEPPEL_PAUSE;
                        self.raste_last = self.raste;
                    }
                }
            } else {
                self.raste = self.moving_pos.trunc() as i8;
                self.raste_last = self.raste;
            }
        }

        self.pos = self.get_pos_from_raste(self.moving_pos.round() as i8);

        self.pos_anim.set(self.pos);
    }

    pub fn axis_input(&mut self, new_value: f32) {
        self.controller = true;

        self.moving_pos = self.get_raste_from_pos(new_value) as f32;
        self.raste = self.moving_pos.trunc() as i8;
        self.raste_target = self.raste;
    }
}
