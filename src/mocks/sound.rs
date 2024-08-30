use lotus_script::{content::ContentId, var::VariableType};

// Soundansteuerung
#[derive(Debug)]
pub enum SoundTarget {
    Start,
    Stop,
    Loop,
    Pause,
    Idle,
}

impl Default for SoundTarget {
    fn default() -> Self {
        SoundTarget::Idle
    }
}

#[derive(Default, Debug)]
pub struct Sound {
    name_id: String,
    sound_volume: f32,
}

impl Sound {
    pub fn new(name: String) -> Self {
        Sound {
            name_id: name,
            sound_volume: 1.0,
            ..Default::default()
        }
    }

    pub fn update_target(&mut self, new_target: SoundTarget) {
        let raw_target = match new_target {
            SoundTarget::Start => 1,
            SoundTarget::Stop => -1,
            SoundTarget::Loop => 1,
            SoundTarget::Pause => -1,
            SoundTarget::Idle => 0,
        };
        raw_target.set(&self.name_id);
    }

    pub fn is_playing(&mut self) -> bool {
        todo!()
    }

    pub fn update_volume(&mut self, new_volume: f32) {
        self.sound_volume = new_volume;
        new_volume.set(&format!("{}_vol", self.name_id));
    }

    pub fn play_glued_sound(&mut self, snd: IndipendentSound) {
        todo!()
    }

    pub fn play_indipendent_sound(&mut self, uid: usize, subid: usize) {
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct IndipendentSound {
    name_id: String,
    sound_volume: f32,
    sounds: Vec<ContentId>,
}

impl IndipendentSound {
    pub fn new(name: String) -> Self {
        IndipendentSound {
            name_id: name,
            sound_volume: 1.0,
            ..Default::default()
        }
    }

    pub fn add_sound(&mut self, id: ContentId) {
        self.sounds.push(id);
    }

    pub fn is_playing(&mut self) -> bool {
        todo!()
    }

    pub fn update_volume(&mut self, new_volume: f32) {
        self.sound_volume = new_volume;
        new_volume.set(&format!("{}_vol", self.name_id));
    }
}
