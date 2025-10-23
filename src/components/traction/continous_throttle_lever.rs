use lotus_script::time::delta;

use crate::api::{
    animation::Animation,
    key_event::{KeyEvent, KeyEventCab},
    sound::Sound,
};

#[derive(Debug, PartialEq)]
pub enum ThrottleMode {
    EmBrake,
    Brake,
    Neutral,
    Throttle,
}

#[derive(Debug, Clone)]
pub struct RastPoint {
    pub id: i32,
    pub upper_bound: f32,
}

#[derive(Debug)]
pub struct ContinuousThrottleLeverBuilder {
    speed: f32,
    target: f32,
    pos: f32,
    pos_last: f32,
    mode: ThrottleMode,
    snappoint: i32,
    snappoint_last: i32,
    snappoint_new: i32,
    zero_snappoint: Option<i32>,
    snappoint_config: Vec<RastPoint>,

    const_speed_normal: f32,
    const_speed_fast: f32,
    const_speed_very_fast: f32,

    pos_anim: Animation,

    snd_notch_neutral: Sound,
    snd_notch_end: Sound,
    snd_notch_other: Sound,

    key_throttle: KeyEvent,
    key_neutral: KeyEvent,
    key_brake: KeyEvent,
    key_max_brake: KeyEvent,
}

impl ContinuousThrottleLeverBuilder {
    pub fn speed_profil(
        mut self,
        const_speed_normal: f32,
        const_speed_fast: f32,
        const_speed_very_fast: f32,
    ) -> Self {
        self.const_speed_normal = const_speed_normal;
        self.const_speed_fast = const_speed_fast;
        self.const_speed_very_fast = const_speed_very_fast;
        self
    }

    pub fn add_snappoint_config(mut self, id: i32, upper_bound: f32) -> Self {
        self.snappoint_config.push(RastPoint { id, upper_bound });
        self
    }

    pub fn snd_notch_neutral(mut self, sound_name: impl Into<String>) -> Self {
        self.snd_notch_neutral = Sound::new_simple(Some(&sound_name.into()));
        self
    }
    pub fn snd_notch_end(mut self, sound_name: impl Into<String>) -> Self {
        self.snd_notch_end = Sound::new_simple(Some(&sound_name.into()));
        self
    }
    pub fn snd_notch_other(mut self, sound_name: impl Into<String>) -> Self {
        self.snd_notch_other = Sound::new_simple(Some(&sound_name.into()));
        self
    }

    pub fn build(self) -> ContinuousThrottleLever {
        let mut s = ContinuousThrottleLever {
            speed: self.speed,
            target: self.target,
            pos: self.pos,
            pos_last: self.pos_last,
            mode: self.mode,
            snappoint: self.snappoint,
            snappoint_last: self.snappoint_last,
            snappoint_new: self.snappoint_new,
            zero_snappoint: self.zero_snappoint,
            snappoint_config: self.snappoint_config,
            const_speed_normal: self.const_speed_normal,
            const_speed_fast: self.const_speed_fast,
            const_speed_very_fast: self.const_speed_very_fast,
            pos_anim: self.pos_anim,
            snd_notch_neutral: self.snd_notch_neutral,
            snd_notch_end: self.snd_notch_end,
            snd_notch_other: self.snd_notch_other,
            key_throttle: self.key_throttle,
            key_neutral: self.key_neutral,
            key_brake: self.key_brake,
            key_max_brake: self.key_max_brake,
        };

        s.snappoint_update();
        s.snappoint = s.snappoint_new;

        s
    }
}

#[derive(Debug)]
pub struct ContinuousThrottleLever {
    speed: f32,
    target: f32,
    pub pos: f32,
    pos_last: f32,
    mode: ThrottleMode,
    pub snappoint: i32,
    pub snappoint_last: i32,
    snappoint_new: i32,
    zero_snappoint: Option<i32>,
    snappoint_config: Vec<RastPoint>,

    const_speed_normal: f32,
    const_speed_fast: f32,
    const_speed_very_fast: f32,

    pos_anim: Animation,

    snd_notch_neutral: Sound,
    snd_notch_end: Sound,
    snd_notch_other: Sound,

    key_throttle: KeyEvent,
    key_neutral: KeyEvent,
    key_brake: KeyEvent,
    key_max_brake: KeyEvent,
}

impl ContinuousThrottleLever {
    pub fn builder(
        anim_name: impl Into<String>,
        cab_side: KeyEventCab,
    ) -> ContinuousThrottleLeverBuilder {
        ContinuousThrottleLeverBuilder {
            speed: 0.0,
            target: 0.0,
            pos: 0.0,
            pos_last: 0.0,
            mode: ThrottleMode::Neutral,
            snappoint: 0,
            snappoint_last: 0,
            snappoint_new: 0,
            zero_snappoint: None,
            snappoint_config: Vec::new(),
            const_speed_normal: 1.0,
            const_speed_fast: 5.0,
            const_speed_very_fast: 20.0,

            pos_anim: Animation::new(Some(&anim_name.into())),

            snd_notch_neutral: Sound::new_simple(None),
            snd_notch_end: Sound::new_simple(None),
            snd_notch_other: Sound::new_simple(None),

            key_throttle: KeyEvent::new(Some("Throttle"), Some(cab_side)),
            key_neutral: KeyEvent::new(Some("Neutral"), Some(cab_side)),
            key_brake: KeyEvent::new(Some("Brake"), Some(cab_side)),
            key_max_brake: KeyEvent::new(Some("MaxBrake"), Some(cab_side)),
        }
    }

    pub fn tick(&mut self) {
        self.pos_last = self.pos;

        if self.key_throttle.is_just_pressed() {
            match self.mode {
                ThrottleMode::Throttle => {
                    self.speed = self.const_speed_normal;
                    self.target = 1.0;
                }
                ThrottleMode::Neutral => {
                    self.speed = self.const_speed_normal;
                    self.target = 1.0;
                    self.mode = ThrottleMode::Throttle;
                }
                _ => {
                    if self.pos < -0.15 {
                        self.speed = self.const_speed_normal;
                        self.target = -0.1;
                        if self.pos < -0.9 {
                            self.pos = -0.9;
                        }
                        self.mode = ThrottleMode::Brake;
                    } else {
                        self.speed = -self.const_speed_fast;
                        self.target = 0.0;
                        self.mode = ThrottleMode::Neutral;
                    }
                }
            }
        }
        if self.key_neutral.is_just_pressed() {
            if self.pos > 0.0 {
                self.speed = -self.const_speed_fast;
            } else {
                self.speed = self.const_speed_fast;
            }
            self.target = 0.0;
            self.mode = ThrottleMode::Neutral;
        }
        if self.key_brake.is_just_pressed() {
            match self.mode {
                ThrottleMode::Throttle => {
                    if self.pos > 0.15 {
                        self.speed = -self.const_speed_normal;
                        self.target = 0.1;
                    } else {
                        self.speed = -self.const_speed_fast;
                        self.target = 0.0;
                        self.mode = ThrottleMode::Neutral;
                    }
                }
                ThrottleMode::Neutral => {
                    self.speed = -self.const_speed_normal;
                    self.target = -0.9;
                    self.mode = ThrottleMode::Brake;
                }
                ThrottleMode::Brake => {
                    if self.pos > -0.9 {
                        self.speed = -self.const_speed_normal;
                        self.target = -0.9;
                    }
                }
                ThrottleMode::EmBrake => {}
            }
        }
        if self.key_max_brake.is_just_pressed() {
            self.speed = -self.const_speed_very_fast;
            self.target = -1.0;
            self.mode = ThrottleMode::EmBrake;
        }

        if self.key_throttle.is_just_released()
            || self.key_neutral.is_just_released()
            || self.key_brake.is_just_released()
        {
            match self.mode {
                ThrottleMode::Neutral | ThrottleMode::EmBrake => {}
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

        self.pos = (self.pos + self.speed * delta()).clamp(-1.0, 1.0);

        if (self.speed > 0.0 && self.pos >= self.target)
            || (self.speed < 0.0 && self.pos <= self.target)
        {
            self.pos = self.target;
            self.speed = 0.0;
        }

        self.update();
    }

    fn snappoint_update(&mut self) {
        self.snappoint_new = self
            .snappoint_config
            .iter()
            .filter(|p| p.upper_bound >= self.pos)
            .min_by(|a, b| {
                (a.upper_bound - self.pos)
                    .partial_cmp(&(b.upper_bound - self.pos))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|p| p.id)
            .unwrap_or(0);
    }

    fn update(&mut self) {
        self.snappoint_update();

        self.snappoint_last = self.snappoint;
        if self.snappoint_new != self.snappoint {
            if self.snappoint == 4 {
                self.snd_notch_neutral.start();
            } else if (self.snappoint == 6 && self.snappoint_new == 7)
                || (self.snappoint == 2 && self.snappoint_new == 1)
            {
                self.snd_notch_end.start();
            } else if !((self.snappoint == 7 && self.snappoint_new == 6)
                || (self.snappoint == 2 && self.snappoint_new == 1))
            {
                self.snd_notch_other.start();
            }
        }
        self.snappoint = self.snappoint_new;

        self.pos_anim.set(self.pos);
    }

    pub fn axis_input(&mut self, cab_is_vr: bool, new_value: f32) {
        self.speed = 0.0;
        self.target = new_value;

        if cab_is_vr {
            self.pos = 0.0;
            self.snappoint = 4;
        } else {
            self.pos = new_value;
            self.mode = if self.snappoint == 0 || new_value < -0.99 {
                ThrottleMode::EmBrake
            } else if self.snappoint <= 3 {
                ThrottleMode::Brake
            } else if self.snappoint == 4 {
                ThrottleMode::Neutral
            } else {
                ThrottleMode::Throttle
            };
        }
    }
}
