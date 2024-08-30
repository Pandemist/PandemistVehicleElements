use crate::mocks::{
    animation::Animation,
    key_event::{KeyEvent, KeyEventCab},
    sound::{Sound, SoundTarget},
};

#[derive(Debug)]
pub struct FGNotbremse {
    name: String,
    pos: f32,
    pub value: bool,
    key_toggle: KeyEvent,
    key_pull: KeyEvent,
    key_release: KeyEvent,
    btn_anim: Animation,

    snd_pull: Sound,
    snd_reset: Sound,
}

impl FGNotbremse {
    pub fn new(name: &str, cab_side: KeyEventCab, sound_id: &str) -> Self {
        Self {
            name: name.to_string(),
            pos: 0.0,
            value: false,
            key_toggle: KeyEvent::new(format!("{}_toggle", name), cab_side),
            key_pull: KeyEvent::new(format!("{}_pull", name), cab_side),
            key_release: KeyEvent::new(format!("{}_reset", name), cab_side),
            btn_anim: Animation::new(format!("{}_anim", name)),
            snd_pull: Sound::new(format!("{}_pull", sound_id)),
            snd_reset: Sound::new(format!("{}_reset", sound_id)),
        }
    }

    pub fn tick(&mut self) {
        if self.key_toggle.is_just_pressed() {
            self.pos = 1.0 - self.pos;
            self.value = self.pos > 0.5;
            self.sound();
            self.btn_anim.set(self.pos);
        }

        if self.key_pull.is_just_pressed() && !self.value {
            self.pos = 1.0;
            self.value = true;
            self.sound();
            self.btn_anim.set(self.pos);
        }

        if self.key_release.is_just_pressed() && self.value {
            self.pos = 0.0;
            self.value = false;
            self.sound();
            self.btn_anim.set(self.pos);
        }
    }

    fn sound(&mut self) {
        if self.value {
            self.snd_pull.update_target(SoundTarget::Start);
        } else {
            self.snd_reset.update_target(SoundTarget::Start);
        }
    }
}
