use lotus_script::delta;

use crate::mocks::sound::Sound;

#[derive(Debug, Default)]
pub struct Umformer {
    name_id: String,

    sound_vol: f32,
    sound: Sound,
    pub ouput_norm_voltage: f32,
}

impl Umformer {
    pub fn new(name: String) -> Self {
        Umformer {
            name_id: name.clone(),
            sound: Sound::new(format!("snd_{}", name)),
            ..Default::default()
        }
    }

    pub fn tick(&mut self, input_norm_voltage: f32) {
        self.ouput_norm_voltage = input_norm_voltage;
        if input_norm_voltage > 0.8 {
            self.sound_vol = (self.sound_vol + 0.5 * delta()).max(0.0);
        } else {
            self.sound_vol = (self.sound_vol - 0.25 * delta()).min(1.0);
        }

        self.sound.update_volume(self.sound_vol);
    }
}
