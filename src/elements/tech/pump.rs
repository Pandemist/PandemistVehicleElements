use lotus_extra::vehicle::CockpitSide;
use lotus_script::time::delta;

use crate::api::{animation::Animation, key_event::KeyEvent, sound::Sound};

pub struct Pump {
    cab_side: Option<CockpitSide>,

    pos: f32,

    pub rate: f32,
    speed: f32,

    dest: bool,

    anim: Animation,

    snd_toggle: Sound,
    snd_plus: Sound,
    snd_minus: Sound,

    key_pull: KeyEvent,
}

impl Pump {
    pub fn new(
        animation_name: impl Into<String>,
        event_name: impl Into<String>,
        cab_side: Option<CockpitSide>,
        snd_toggle_name: Option<&str>,
        snd_plus_name: Option<&str>,
        snd_minus_name: Option<&str>,
        speed: f32,
    ) -> Self {
        Self {
            cab_side,
            pos: 0.0,
            rate: 0.0,
            speed,
            dest: false,
            anim: Animation::new(Some(&animation_name.into())),
            key_pull: KeyEvent::new(Some(&event_name.into()), cab_side),
            snd_toggle: Sound::new_simple(snd_toggle_name),
            snd_plus: Sound::new_simple(snd_plus_name),
            snd_minus: Sound::new_simple(snd_minus_name),
        }
    }

    pub fn tick(&mut self) {
        if self.key_pull.is_just_pressed() && self.pos == 0.0 {
            self.dest = true;
            self.snd_toggle.start();
            self.snd_plus.start();
        }

        let pos_last = self.pos;

        if self.dest {
            self.pos = (self.pos + self.speed * delta()).min(1.0);
            self.dest = self.pos < 1.0;
            if !self.dest {
                self.snd_minus.start();
            }
        } else {
            self.pos = (self.pos - self.speed * delta()).max(0.0);
        }

        if self.pos != pos_last {
            self.anim.set(self.pos);
        }

        self.rate = (self.pos - pos_last).max(0.0);
    }
}
