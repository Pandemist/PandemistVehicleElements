use crate::{
    mocks::{animation::Animation, bogie::Bogie, sound::Sound},
    structs::internal_enums::SoundTarget,
};

#[derive(Default, Debug)]
pub struct Railbrake {
    _name_id: String,
    _boogie_index: usize,
    force: f32,

    bogie: Bogie,

    snd_up: Sound,
    snd_down: Sound,

    anim: Animation,

    activ_last: bool,
}

impl Railbrake {
    pub fn new(name: String, id: usize, force: f32) -> Self {
        Railbrake {
            _name_id: name.clone(),
            _boogie_index: id,
            force: force,
            bogie: Bogie::new(name.clone(), id),
            snd_up: Sound::new(format!("snd_{}_up", name)),
            snd_down: Sound::new(format!("snd_{}_down", name)),
            anim: Animation::new(format!("railbrake_{}_anim", id)),
            ..Default::default()
        }
    }

    pub fn tick(&mut self, activ: bool) {
        if activ != self.activ_last {
            if activ {
                self.snd_up.update_target(SoundTarget::Start);
                self.bogie.railbrake_force(self.force);
            } else {
                self.snd_down.update_target(SoundTarget::Start);
                self.bogie.railbrake_force(0.0);
            }
            self.anim.update_pos(activ as i32 as f32);
        }
    }
}
