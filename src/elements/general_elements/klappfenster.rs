use lotus_script::event::ButtonEvent;

use crate::{
    mocks::{animation::Animation, sound::Sound},
    structs::{internal_enums::SoundTarget, traits::OnButton},
};
#[derive(Debug, Default)]
pub struct Klappfenster {
    pos: f32,
    value: bool,

    snd_open: Sound,
    snd_close: Sound,

    anim: Animation,
    name_id: String,
}

impl Klappfenster {
    pub fn new(name: String) -> Self {
        Klappfenster {
            name_id: name.clone(),
            anim: Animation::new(format!("{}_anim", name)),
            snd_open: Sound::new(format!("snd_{}_open", name)),
            snd_close: Sound::new(format!("snd_{}_close", name)),
            ..Default::default()
        }
    }

    pub fn toggle(&mut self) {
        self.pos = 1.0 - self.pos;
        self.value = self.pos > 0.5;

        if self.value {
            self.snd_open.update_target(SoundTarget::Start);
        } else {
            self.snd_close.update_target(SoundTarget::Start);
        }

        self.anim.update_pos(self.pos);
    }
}

impl OnButton for Klappfenster {
    fn on_button(&mut self, ev: &ButtonEvent) {
        if ev.id == format!("{}_toggle", self.name_id) {
            if ev.value {
                self.toggle();
            }
        }
    }
}
