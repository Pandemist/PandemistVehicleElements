use crate::mocks::{
    animation::Animation,
    key_event::{KeyEvent, KeyEventCab},
    sound::{Sound, SoundTarget},
};

#[derive(Debug)]
pub struct PushButton {
    name: String,
    pos: f32,
    value: bool,
    value_last: bool,
    key_press: KeyEvent,
    btn_anim: Animation,

    snd_press: Sound,
    snd_release: Sound,
}

impl PushButton {
    pub fn new(name: &str, cab_side: KeyEventCab, sound_id: &str) -> Self {
        Self {
            name: name.to_string(),
            pos: 0.0,
            value: false,
            value_last: false,
            key_press: KeyEvent::new(format!("{}_press", name), cab_side),
            btn_anim: Animation::new(format!("{}_anim", name)),
            snd_press: Sound::new(format!("snd_{}_press", sound_id)),
            snd_release: Sound::new(format!("snd_{}_release", sound_id)),
        }
    }

    pub fn tick(&mut self) {
        self.value_last = self.value;

        if self.key_press.is_just_pressed() {
            self.pos = 1.0;
            self.snd_press.update_target(SoundTarget::Start);
            self.update();
        }
        if self.key_press.is_just_released() {
            self.pos = 0.0;
            self.snd_release.update_target(SoundTarget::Start);
            self.update();
        }
    }

    pub fn is_just_pressed(&mut self) -> bool {
        self.value && !self.value_last
    }

    pub fn is_just_released(&mut self) -> bool {
        !self.value && self.value_last
    }

    pub fn is_pressed(&mut self) -> bool {
        self.value
    }

    pub fn is_released(&mut self) -> bool {
        !self.value
    }

    fn update(&mut self) {
        self.value = self.pos > 0.5;
        self.btn_anim.set(self.pos);
    }
}

#[derive(Debug)]
pub struct PushHoldButton {
    name: String,
    pos: f32,
    pub value: bool,

    key_press: KeyEvent,

    btn_anim: Animation,

    snd_press: Sound,
    snd_release: Sound,
}

impl PushHoldButton {
    pub fn new(name: &str, cab_side: KeyEventCab, sound_id: &str) -> Self {
        Self {
            name: name.to_string(),
            pos: 0.0,
            value: false,
            key_press: KeyEvent::new(format!("{}_press", name), cab_side),
            btn_anim: Animation::new(format!("{}_anim", name)),
            snd_press: Sound::new(format!("{}_press", sound_id)),
            snd_release: Sound::new(format!("{}_release", sound_id)),
        }
    }

    pub fn set(&mut self) {
        self.pos = 0.75;
        self.value = true;
        self.btn_anim.set(self.pos);
    }

    pub fn tick(&mut self) {
        if self.key_press.is_just_pressed() {
            self.pos = 1.0;
            self.snd_press.update_target(SoundTarget::Start);
            self.value = !self.value;
            self.btn_anim.set(self.pos);
        }
        if self.key_press.is_just_released() {
            self.pos = if self.value { 0.75 } else { 0.0 };
            self.snd_release.update_target(SoundTarget::Start);
            self.btn_anim.set(self.pos);
        }
    }
}
