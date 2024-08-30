use lotus_script::delta;

use crate::mocks::sound::Sound;

#[derive(Debug, Default)]
pub struct Umformer {
    const_min_voltage_norm: f32,
    const_startup_time: f32,
    const_shutdown_time: f32,

    id: usize,

    sound_vol: f32,
    sound: Sound,
    pub ouput_norm_voltage: f32,
}

impl Umformer {
    pub fn new(id: usize, min_voltage_norm: f32, startup_time: f32, shutdown_time: f32) -> Self {
        Umformer {
            id: id,
            const_min_voltage_norm: min_voltage_norm,
            const_startup_time: startup_time,
            const_shutdown_time: shutdown_time,
            sound: Sound::new(format!("snd_umformer_{}", id)),
            ..Default::default()
        }
    }

    pub fn tick(&mut self, input_norm_voltage: f32) {
        self.ouput_norm_voltage = input_norm_voltage;
        if input_norm_voltage > self.const_min_voltage_norm {
            self.sound_vol = (self.sound_vol + self.const_startup_time * delta()).max(0.0);
        } else {
            self.sound_vol = (self.sound_vol - self.const_shutdown_time * delta()).min(1.0);
        }

        self.sound.update_volume(self.sound_vol);
    }
}
