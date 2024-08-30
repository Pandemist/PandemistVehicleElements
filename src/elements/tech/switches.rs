use crate::mocks::{
    animation::Animation,
    key_event::{KeyEvent, KeyEventCab},
    sound::{Sound, SoundTarget},
};

#[derive(Debug)]
pub struct Switch {
    name: String,
    pos: f32,
    pub value: bool,
    key_toggle: KeyEvent,
    key_on: KeyEvent,
    key_off: KeyEvent,
    btn_anim: Animation,

    snd_toggle: Sound,
}

impl Switch {
    pub fn new(name: &str, cab_side: KeyEventCab, sound_id: &str, state: bool) -> Self {
        Self {
            name: name.to_string(),
            pos: state.into(),
            value: state,
            key_toggle: KeyEvent::new(format!("{}_toggle", name), cab_side),
            key_on: KeyEvent::new(format!("{}_on", name), cab_side),
            key_off: KeyEvent::new(format!("{}_off", name), cab_side),
            btn_anim: Animation::new(format!("{}_anim", name)),
            snd_toggle: Sound::new(format!("{}_toggle", sound_id)),
        }
    }

    pub fn tick(&mut self) {
        if self.key_toggle.is_just_pressed() {
            self.pos = 1.0 - self.pos;
            self.value = self.pos > 0.5;
            self.snd_toggle.update_target(SoundTarget::Start);
            self.btn_anim.set(self.pos);
        }

        if self.key_on.is_just_pressed() && !self.value {
            self.pos = 1.0;
            self.value = true;
            self.snd_toggle.update_target(SoundTarget::Start);
            self.btn_anim.set(self.pos);
        }

        if self.key_off.is_just_pressed() && self.value {
            self.pos = 0.0;
            self.value = false;
            self.snd_toggle.update_target(SoundTarget::Start);
            self.btn_anim.set(self.pos);
        }
    }
}

#[derive(Debug)]
pub struct StepSwitch {
    name: String,
    max: i32,
    min: i32,
    pos: f32,
    value: i32,

    min_spring: bool,
    max_spring: bool,

    key_anim: Animation,

    key_plus: KeyEvent,
    key_minus: KeyEvent,

    snd_plus: Sound,
    snd_minus: Sound,
}

impl StepSwitch {
    pub fn new(
        name: &str,
        cab_side: KeyEventCab,
        sound_id: String,
        min: i32,
        max: i32,
        min_spring: bool,
        max_spring: bool,
    ) -> Self {
        Self {
            name: name.to_string(),
            max: max,
            min: min,
            pos: 0.0,
            value: 0,

            min_spring: min_spring,
            max_spring: max_spring,

            key_anim: Animation::new(format!("{}_anim", name)),

            key_plus: KeyEvent::new(format!("{}_plus", name), cab_side),
            key_minus: KeyEvent::new(format!("{}_minus", name), cab_side),

            snd_plus: Sound::new(format!("{}_plus", sound_id)),
            snd_minus: Sound::new(format!("{}_minus", sound_id)),
        }
    }

    pub fn init(&mut self, new_pos: i32) {
        if self.min <= new_pos && new_pos >= self.max {
            self.pos = new_pos as f32;
            self.update();
        }
    }

    pub fn set(&mut self, new_pos: i32) {
        if self.min <= new_pos && new_pos >= self.max && self.pos as i32 != new_pos {
            self.snd_plus.update_target(SoundTarget::Start);
            self.pos = new_pos as f32;
            self.update();
        }
    }

    pub fn tick(&mut self) {
        if self.key_plus.is_just_pressed() {
            if self.value < self.max {
                self.pos += 1.0;
                self.snd_plus.update_target(SoundTarget::Start);
                self.update();
            }
        }

        if self.key_plus.is_just_released() && self.max_spring {
            if self.value == self.max {
                self.pos -= 1.0;
                self.snd_minus.update_target(SoundTarget::Start);
                self.update();
            }
        }

        if self.key_minus.is_just_pressed() {
            if self.value > self.min {
                self.pos -= 1.0;
                self.snd_minus.update_target(SoundTarget::Start);
                self.update();
            }
        }

        if self.key_minus.is_just_released() && self.min_spring {
            if self.value == self.min {
                self.pos += 1.0;
                self.snd_plus.update_target(SoundTarget::Start);
                self.update();
            }
        }
    }

    fn update(&mut self) {
        self.value = self.pos as i32;
        self.key_anim.set(self.pos);
    }
}
