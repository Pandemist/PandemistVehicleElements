use crate::structs::internal_enums::SoundTarget;

#[derive(Default, Debug)]

pub struct Sound {
    _name_id: String,
    sound_volume: f32,
}

impl Sound {
    pub fn new(name: String) -> Self {
        Sound {
            _name_id: name,
            sound_volume: 1.0,
            ..Default::default()
        }
    }

    // Setzt den Zustand eines Sounds Start(Loop)/Stop/Pause
    pub fn update_target(&mut self, new_target: SoundTarget) {
        todo!()
    }

    // Setzt die Lautstärke eines Sounds (Bissher: Sound geht bei Lautstärke = 0 aus)
    pub fn update_volume(&mut self, new_volume: f32) {
        self.sound_volume = new_volume;
        todo!()
    }
}
