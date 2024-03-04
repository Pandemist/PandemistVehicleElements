use lotus_script::delta;

use crate::{
    mocks::{animation::Animation, sound::Sound},
    structs::{internal_enums::SoundTarget, traits::OnButton},
};

const MIRRORSPEED_IN: f32 = 1.0;
const MIRRORSPEED_OUT: f32 = 1.0;
const MIRROR_UP_MAX: f32 = 0.2;
const MIRROR_DN_MAX: f32 = -0.3;
const MIRROR_R_MAX: f32 = 0.2;
const MIRROR_L_MAX: f32 = -0.2;
const MIRROR_VARIANZ: f32 = 0.05;
const MIRROR_MOVE_FACTOR: f32 = 0.2;

#[derive(Debug, Default)]
pub struct Mirror {
    name_id: String,

    pos: f32,
    pos_x: f32,
    pos_y: f32,

    anim_arm: Animation,
    anim_x: Animation,
    anim_y: Animation,

    target: bool,
    target_last: bool,
    broken: bool,

    snd_auf: Sound,
    snd_zu: Sound,
    snd_move: Sound,
    snd_end: Sound,

    grabbing: bool,
}

impl Mirror {
    pub fn new(name: String) -> Self {
        Mirror {
            name_id: name.clone(),

            anim_arm: Animation::new(format!("{}_arm_anim", name)),
            anim_x: Animation::new(format!("{}_x_anim", name)),
            anim_y: Animation::new(format!("{}_y_anim", name)),

            snd_auf: Sound::new(format!("snd_{}_open", name)),
            snd_zu: Sound::new(format!("snd_{}_close", name)),
            snd_move: Sound::new(format!("snd_{}_move", name)),
            snd_end: Sound::new(format!("snd_{}_end", name)),

            ..Default::default()
        }
    }

    pub fn toggle(&mut self, key: bool) {
        if key {
            self.pos = 1.0 - self.pos;
        }
    }

    pub fn grab(&mut self, hand_delta_x: f32, hand_delta_y: f32) {
        if self.grabbing {
            self.pos_x = self.pos_x + hand_delta_x * delta();
            if self.pos_x > 1.0 {
                self.pos_x = self.pos_x - 1.0;
            } else if self.pos_x < -1.0 {
                self.pos_x = self.pos_x + 1.0;
            }
            self.pos_y = self.pos_y + hand_delta_y * delta();
        }
    }

    pub fn tick(&mut self, move_r: bool, move_l: bool, move_o: bool, move_u: bool) {
        if self.broken {
            return;
        }

        if self.target && !self.target_last {
            self.snd_auf.update_target(SoundTarget::Start);
            self.snd_zu.update_target(SoundTarget::Stop);
        } else if !self.target && self.target_last {
            self.snd_auf.update_target(SoundTarget::Stop);
            self.snd_zu.update_target(SoundTarget::Start);
        } else {
            self.snd_auf.update_target(SoundTarget::Stop);
            self.snd_zu.update_target(SoundTarget::Stop);
        }

        self.target_last = self.target;

        if move_r {
            self.pos_x = self.pos_x + MIRROR_MOVE_FACTOR * delta();
            if self.pos_x >= MIRROR_R_MAX + MIRROR_VARIANZ {
                self.pos_x = MIRROR_R_MAX;
                self.snd_end.update_target(SoundTarget::Start);
            }
        }
        if move_l {
            self.pos_x = self.pos_x - MIRROR_MOVE_FACTOR * delta();
            if self.pos_x <= MIRROR_L_MAX - MIRROR_VARIANZ {
                self.pos_x = MIRROR_L_MAX;
                self.snd_end.update_target(SoundTarget::Start);
            }
        } else {
            self.pos_x = self.pos_x.min(MIRROR_R_MAX).max(MIRROR_L_MAX);
        }

        if move_o {
            self.pos_y = self.pos_y + MIRROR_MOVE_FACTOR * delta();
            if self.pos_y >= MIRROR_UP_MAX + MIRROR_VARIANZ {
                self.pos_y = MIRROR_UP_MAX;
                self.snd_end.update_target(SoundTarget::Start);
            }
        }
        if move_u {
            self.pos_y = self.pos_y - MIRROR_MOVE_FACTOR * delta();
            if self.pos_y <= MIRROR_DN_MAX - MIRROR_VARIANZ {
                self.pos_y = MIRROR_DN_MAX;
                self.snd_end.update_target(SoundTarget::Start);
            }
        } else {
            self.pos_y = self.pos_y.min(MIRROR_UP_MAX).max(MIRROR_DN_MAX);
        }

        if move_r || move_l || move_o || move_u {
            self.snd_move.update_target(SoundTarget::Start);
        } else {
            self.snd_move.update_target(SoundTarget::Stop);
        }

        if self.target {
            self.pos = (self.pos + MIRRORSPEED_OUT * delta()).min(1.0);
        } else {
            self.pos = (self.pos - MIRRORSPEED_IN * delta()).max(0.0);
        }

        self.anim_arm.update_pos(self.pos);
        self.anim_x.update_pos(self.pos_x);
        self.anim_y.update_pos(self.pos_y);
    }
}

impl OnButton for Mirror {
    fn on_button(&mut self, ev: &lotus_script::event::ButtonEvent) {
        if ev.id == format!("{}_grab", self.name_id) {
            self.grabbing = ev.value;
        } else if ev.id == format!("{}_toggle", self.name_id) {
            self.toggle(ev.value);
        }
    }
}
