use lotus_script::content::ContentId;

use crate::mocks::sound::{IndipendentSound, Sound};

#[derive(Debug, Default)]
pub struct AnnoucementPlayer {
    name_id: String,

    snd: Sound,

    normal_annoucement_uid: usize,
}

impl AnnoucementPlayer {
    pub fn new(name: String) -> Self {
        AnnoucementPlayer {
            name_id: name.clone(),
            snd: Sound::new(format!("snd_{}", name)),
            ..Default::default()
        }
    }

    pub fn play_glued_announcement(&mut self, s: String) {
        let mut indi_sound = IndipendentSound::new(format!("Ansage"));

        let tokens: Vec<&str> = s.split('+').collect();

        let last_index = if tokens.len() % 2 == 0 {
            tokens.len()
        } else {
            tokens.len() - 1
        };

        for i in (0..last_index).step_by(2) {
            if let (Ok(uid), Ok(sub_id)) = (tokens[i].parse::<i32>(), tokens[i + 1].parse::<i32>())
            {
                indi_sound.add_sound(ContentId {
                    user_id: uid,
                    sub_id: sub_id,
                    version: 0.0,
                });
            }
        }

        self.snd.play_glued_sound(indi_sound);
    }

    pub fn prepare_normal_annoucement(&mut self, uid: usize) {
        self.normal_annoucement_uid = uid;
    }

    pub fn play_normal_annoucement(&mut self, subid: usize) {
        self.snd
            .play_indipendent_sound(self.normal_annoucement_uid, subid);
    }
}
