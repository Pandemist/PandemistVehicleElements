use std::collections::HashMap;

use lotus_script::time::delta;

use crate::api::{
    animation::Animation,
    key_event::{KeyEvent, KeyEventCab},
    sound::Sound,
};

#[derive(Debug, PartialEq)]
enum MovementMode {
    None,
    Min,
    Up,
    Down,
}

#[derive(Debug)]
pub struct StagedThrottleLeverBuilder {
    max_snappoint: i32,
    min_snappoint: i32,

    const_speed_normal: f32,
    const_speed_fast: f32,
    const_speed_very_fast: f32,
    const_pause_time: f32,

    stages_plus: Vec<i32>,
    stages_minus: Vec<i32>,
    last_stage_is_emergency_brake: bool,

    constraints: HashMap<i32, (i32, i32)>,

    pos: f32,
    moving_pos: f32,
    snappoint: i32,
    snappoint_last: i32,
    snappoint_target: i32,

    move_speed: f32,
    move_type: MovementMode,
    pause_timer: f32,

    pos_anim: Animation,

    snd_alt_snappoint: HashMap<i32, Sound>,
    snd_snappoint_default: Sound,

    key_throttle: KeyEvent,
    key_neutral: KeyEvent,
    key_brake: KeyEvent,
    key_max_brake: KeyEvent,
}

impl StagedThrottleLeverBuilder {
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

    pub fn pause_time(mut self, pause_time: f32) -> Self {
        self.const_pause_time = pause_time;
        self
    }

    pub fn max_snappoint(mut self, snappoint: i32) -> Self {
        self.max_snappoint = snappoint;
        self
    }

    pub fn min_snappoint(mut self, snappoint: i32) -> Self {
        self.min_snappoint = snappoint;
        self
    }

    pub fn add_stage_plus(mut self, snappoint: i32) -> Self {
        self.stages_plus.push(snappoint);
        self
    }

    pub fn add_stage_minus(mut self, snappoint: i32) -> Self {
        self.stages_minus.push(snappoint);
        self
    }

    pub fn last_stage_is_emergency_brake(mut self) -> Self {
        self.last_stage_is_emergency_brake = true;
        self
    }

    pub fn add_constraint(mut self, level: i32, min_index: i32, max_index: i32) -> Self {
        self.constraints.insert(level, (min_index, max_index));
        self
    }

    pub fn add_alt_sound(mut self, snappoint: i32, sound_name: impl Into<String>) -> Self {
        self.snd_alt_snappoint
            .insert(snappoint, Sound::new_simple(Some(&sound_name.into())));
        self
    }

    pub fn build(self) -> StagedThrottleLever {
        StagedThrottleLever {
            max_snappoint: self.max_snappoint,
            min_snappoint: self.min_snappoint,
            const_speed_normal: self.const_speed_normal,
            const_speed_fast: self.const_speed_fast,
            const_speed_very_fast: self.const_speed_very_fast,
            const_pause_time: self.const_pause_time,
            stages_plus: self.stages_plus,
            stages_minus: self.stages_minus,
            last_stage_is_emergency_brake: self.last_stage_is_emergency_brake,
            constraints: self.constraints,
            pos: self.pos,
            moving_pos: self.moving_pos,
            snappoint: self.snappoint,
            snappoint_last: self.snappoint_last,
            snappoint_target: self.snappoint_target,
            move_speed: self.move_speed,
            move_type: self.move_type,
            pause_timer: self.pause_timer,
            pos_anim: self.pos_anim,
            snd_alt_snappoint: self.snd_alt_snappoint,
            snd_snappoint_default: self.snd_snappoint_default,
            key_throttle: self.key_throttle,
            key_neutral: self.key_neutral,
            key_brake: self.key_brake,
            key_max_brake: self.key_max_brake,
        }
    }
}

#[derive(Debug)]
pub struct StagedThrottleLever {
    pub max_snappoint: i32,
    pub min_snappoint: i32,

    const_speed_normal: f32,
    const_speed_fast: f32,
    const_speed_very_fast: f32,
    const_pause_time: f32,

    stages_plus: Vec<i32>,
    stages_minus: Vec<i32>,
    last_stage_is_emergency_brake: bool,

    constraints: HashMap<i32, (i32, i32)>,

    pub pos: f32,
    moving_pos: f32,
    pub snappoint: i32,
    snappoint_last: i32,
    snappoint_target: i32,

    move_speed: f32,
    move_type: MovementMode,
    pause_timer: f32,

    pos_anim: Animation,

    snd_alt_snappoint: HashMap<i32, Sound>,
    snd_snappoint_default: Sound,

    key_throttle: KeyEvent,
    key_neutral: KeyEvent,
    key_brake: KeyEvent,
    key_max_brake: KeyEvent,
}

impl StagedThrottleLever {
    pub fn builder(
        anim_name: impl Into<String>,
        cab_side: KeyEventCab,
    ) -> StagedThrottleLeverBuilder {
        StagedThrottleLeverBuilder {
            max_snappoint: 0,
            min_snappoint: 0,
            const_speed_normal: 7.5,
            const_speed_fast: 25.0,
            const_speed_very_fast: 40.0,
            const_pause_time: 0.075,
            stages_plus: Vec::new(),
            stages_minus: Vec::new(),
            last_stage_is_emergency_brake: false,
            constraints: HashMap::new(),
            pos: 0.0,
            moving_pos: 0.0,
            snappoint: 0,
            snappoint_last: 0,
            snappoint_target: 0,
            move_speed: 0.0,
            move_type: MovementMode::None,
            pause_timer: 0.0,
            pos_anim: Animation::new(Some(&anim_name.into())),
            snd_alt_snappoint: HashMap::new(),
            snd_snappoint_default: Sound::new_simple(None),
            key_throttle: KeyEvent::new(Some("Throttle"), Some(cab_side)),
            key_neutral: KeyEvent::new(Some("Neutral"), Some(cab_side)),
            key_brake: KeyEvent::new(Some("Brake"), Some(cab_side)),
            key_max_brake: KeyEvent::new(Some("MaxBrake"), Some(cab_side)),
        }
    }

    pub fn tick(&mut self, constrain_level: i32) {
        // Input
        if self.key_throttle.is_just_pressed() {
            self.snappoint_target = self.next_stage();

            self.move_type = MovementMode::Up;
            self.move_speed = self.const_speed_normal;
        }

        if self.key_neutral.is_just_pressed() {
            self.snappoint_target = 0;
            self.move_type = MovementMode::None;

            self.move_speed = (((self.snappoint_target - self.snappoint).signum()) as f32)
                * self.const_speed_fast;
        }

        if self.key_brake.is_just_pressed() {
            self.snappoint_target = self.prev_stage();
            self.move_type = MovementMode::Down;
            self.move_speed = -self.const_speed_normal;
        }

        if self.key_max_brake.is_just_pressed() {
            self.snappoint_target = self.min_snappoint;
            self.move_type = MovementMode::Min;
            self.move_speed = -self.const_speed_very_fast;
        }

        if (self.key_throttle.is_just_released()
            || self.key_neutral.is_just_released()
            || self.key_brake.is_just_released())
            && (self.move_type == MovementMode::Up || self.move_type == MovementMode::Down)
        {
            self.moving_pos = self.moving_pos.round();
            self.snappoint = self.moving_pos.round() as i32;
            self.snappoint_target = self.snappoint;
            self.move_type = MovementMode::Min;
            self.move_speed = 0.0;
            self.pause_timer = 0.0;
        }

        // Action
        let local_max_snappoint = self
            .constraints
            .get(&constrain_level)
            .map_or(self.max_snappoint, |&(first, _)| first);

        let local_min_snappoint = self
            .constraints
            .get(&constrain_level)
            .map_or(self.min_snappoint, |&(_, last)| last);

        if self.pause_timer > 0.0 {
            self.pause_timer -= delta();
        }

        self.snappoint_target = self
            .snappoint_target
            .clamp(local_min_snappoint, local_max_snappoint);

        if (self.pause_timer <= 0.0)
            && (matches!(self.move_type, MovementMode::Down | MovementMode::Up))
            || matches!(self.move_type, MovementMode::None | MovementMode::Min)
        {
            self.moving_pos = (self.moving_pos + self.move_speed * delta())
                .clamp((self.min_snappoint) as f32, (self.max_snappoint) as f32);

            if ((self.move_speed > 0.0) && (self.moving_pos >= (self.snappoint_target as f32)))
                || ((self.move_speed < 0.0) && (self.moving_pos <= (self.snappoint_target as f32)))
            {
                self.moving_pos = self.snappoint_target as f32;
                self.move_speed = 0.0;
            }

            self.snappoint = self.moving_pos.round() as i32;

            if self.snappoint != self.snappoint_last {
                match self.snd_alt_snappoint.get_mut(&self.snappoint) {
                    Some(s) => {
                        s.start();
                    }
                    None => {
                        self.snd_snappoint_default.start();
                    }
                }

                self.pause_timer = self.const_pause_time;
                self.snappoint_last = self.snappoint;
            }
        }

        self.pos = self.get_pos_from_snappoint(self.moving_pos);

        self.pos_anim.set(self.pos);
    }

    fn get_pos_from_snappoint(&self, value: f32) -> f32 {
        if value > 0.0 {
            value / self.max_snappoint as f32
        } else if value < 0.0 {
            value.abs() / self.min_snappoint as f32
        } else {
            0.0
        }
    }

    fn get_snappoint_from_pos(&self, pos: f32) -> i32 {
        if !(-0.05..=0.05).contains(&pos) {
            (pos * (self.max_snappoint as f32)).trunc() as i32
        } else {
            0
        }
    }

    fn next_stage(&mut self) -> i32 {
        if self.stages_plus.is_empty() {
            return self.max_snappoint;
        }

        let mut next_larger = None;

        for &value in &self.stages_plus {
            if value > self.snappoint {
                match next_larger {
                    None => next_larger = Some(value),
                    Some(current) => {
                        if value < current {
                            next_larger = Some(value);
                        }
                    }
                }
            }
        }

        match next_larger {
            Some(val) => val,
            None => self.max_snappoint,
        }
    }

    fn prev_stage(&mut self) -> i32 {
        if self.stages_minus.is_empty() {
            if self.last_stage_is_emergency_brake {
                return self.min_snappoint + 1;
            } else {
                return self.min_snappoint;
            }
        }

        let mut next_smaller = None;

        for &value in &self.stages_minus {
            if self.snappoint > value {
                match next_smaller {
                    None => next_smaller = Some(value),
                    Some(current) => {
                        if value < current {
                            next_smaller = Some(value);
                        }
                    }
                }
            }
        }

        let stage: i32 = match next_smaller {
            Some(val) => val,
            None => self.min_snappoint,
        };

        if self.last_stage_is_emergency_brake {
            stage.max(self.min_snappoint + 1)
        } else {
            stage
        }
    }
}
