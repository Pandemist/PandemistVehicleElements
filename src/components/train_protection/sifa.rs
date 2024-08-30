use crate::mocks::{light::Light, sound::Sound};

#[derive(Debug)]
pub struct Sifa {
    id: usize,
    time: f32,
    timer: f32,

    warning: Sound,
    alarmlight: Light,

    pub zwangsbremsung: bool,
}

impl Sifa {
    pub fn new(new_id: usize, new_time: f32) -> Self {
        Self {
            id: new_id,
            time: new_time,
            timer: 0.0,
            warning: Sound::new(format!("snd_sifa_{}", new_id)),
            alarmlight: Light::new(format!("light_sifa_{}", new_id)),
            zwangsbremsung: false,
        }
    }

    pub fn tick(&mut self, sifa_activ: bool, sifa_btns: bool, reset_cond: bool) {}
}
