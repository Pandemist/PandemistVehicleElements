use glam::Vec2;
use lotus_script::delta;

use crate::{
    mocks::{
        animation::Animation,
        generell::mouse_move,
        key_event::{KeyEvent, KeyEventCab},
        sound::{Sound, SoundTarget},
    },
    structs::{enums::SimpleState, structs::FourDirections},
};

#[derive(Debug)]
pub struct Spiegel {
    name: String,

    open_speed: f32,
    close_speed: f32,

    pos_arm: f32,
    pos_x: f32,
    pos_y: f32,

    target_last: SimpleState,

    mouse_factor: Vec2,
    mirror_speed: Vec2,

    mirror_x_border: Vec2,
    mirror_y_border: Vec2,

    mirror_x_variance: Vec2,
    mirror_y_variance: Vec2,

    arm_anim: Animation,
    pos_x_anim: Animation,
    pos_y_anim: Animation,

    key_grab: KeyEvent,
    key_arm: KeyEvent,

    snd_open: Sound,
    snd_close: Sound,
    snd_move: Sound,
    snd_move_end: Sound,
}

impl Spiegel {
    pub fn new(
        name: &str,
        cab_side: KeyEventCab,
        open_speed: f32,
        close_speed: f32,
        mouse_factor: Vec2,
        mirror_speed: Vec2,
        mirror_x_border: Vec2,
        mirror_y_border: Vec2,
        mirror_x_variance: Vec2,
        mirror_y_variance: Vec2,
    ) -> Self {
        Self {
            name: name.to_string(),
            open_speed: open_speed,
            close_speed: close_speed,
            pos_arm: 0.0,
            pos_x: 0.0,
            pos_y: 0.0,
            target_last: SimpleState::Off,
            mouse_factor: mouse_factor,
            mirror_speed: mirror_speed,
            mirror_x_border: mirror_x_border,
            mirror_y_border: mirror_y_border,
            mirror_x_variance: mirror_x_variance,
            mirror_y_variance: mirror_y_variance,
            arm_anim: Animation::new(format!("{}_arm_anim", name)),
            pos_x_anim: Animation::new(format!("{}_x_anim", name)),
            pos_y_anim: Animation::new(format!("{}_y_anim", name)),
            key_grab: KeyEvent::new(format!("{}_grab", name), cab_side),
            key_arm: KeyEvent::new(format!("{}_arm", name), cab_side),
            snd_open: Sound::new(format!("snd_{}_open", name)),
            snd_close: Sound::new(format!("snd_{}_close", name)),
            snd_move: Sound::new(format!("snd_{}_move", name)),
            snd_move_end: Sound::new(format!("snd_{}_move_end", name)),
        }
    }

    pub fn tick(
        &mut self,
        arm_target: Option<SimpleState>,
        mirror_target: Option<FourDirections>,
        spannung_norm: f32,
    ) {
        // Arm händisch bewegen
        if self.key_arm.is_just_pressed() {
            self.pos_arm = if self.pos_arm > 0.5 { 0.0 } else { 1.0 };
        }
        // Arm elektisch bewegen
        if spannung_norm > 0.25 {
            match arm_target {
                Some(target) => {
                    match target {
                        SimpleState::On => {
                            if self.target_last == SimpleState::Off {
                                self.snd_open.update_target(SoundTarget::Start)
                            }
                            self.pos_arm = (self.pos_arm + self.open_speed * delta()).min(1.0)
                        }
                        SimpleState::Off => {
                            if self.target_last == SimpleState::Off {
                                self.snd_close.update_target(SoundTarget::Start)
                            }
                            self.pos_arm = (self.pos_arm + self.close_speed * delta()).max(0.0)
                        }
                    }
                    self.target_last = target;
                }
                None => {}
            }
        }
        self.arm_anim.set(self.pos_arm);

        // Spiegel her Hand bewegen
        if self.key_grab.is_pressed() {
            self.pos_x = (self.pos_x + (mouse_move().x * self.mouse_factor.x) * delta())
                .rem_euclid(360.0)
                .min(self.mirror_x_border.x)
                .max(self.mirror_x_border.y);
            self.pos_y = (self.pos_y + (mouse_move().y * self.mouse_factor.y) * delta())
                .rem_euclid(360.0)
                .min(self.mirror_y_border.x)
                .max(self.mirror_y_border.y);
        }

        if spannung_norm > 0.25 {
            match mirror_target {
                Some(target) => {
                    if target.up {
                        self.pos_y = (self.pos_y + self.mirror_speed.y * delta()).rem_euclid(360.0);
                        if self.pos_y >= self.mirror_y_border.x + self.mirror_y_variance.x {
                            self.pos_y = self.pos_y.min(self.mirror_x_border.x);
                            self.snd_move_end.update_target(SoundTarget::Start);
                        }
                    } else if target.down {
                        self.pos_y = (self.pos_y - self.mirror_speed.y * delta()).rem_euclid(360.0);
                        if self.pos_y >= self.mirror_y_border.x + self.mirror_y_variance.y {
                            self.pos_y = self.pos_y.min(self.mirror_y_border.y);
                            self.snd_move_end.update_target(SoundTarget::Start);
                        }
                    } else {
                        self.pos_y = self
                            .pos_y
                            .rem_euclid(360.0)
                            .min(self.mirror_y_border.x)
                            .max(self.mirror_y_border.y);
                    }
                    if target.left {
                        self.pos_x = (self.pos_x - self.mirror_speed.x * delta()).rem_euclid(360.0);
                        if self.pos_x >= self.mirror_x_border.x + self.mirror_x_variance.x {
                            self.pos_x = self.pos_x.min(self.mirror_x_border.x);
                            self.snd_move_end.update_target(SoundTarget::Start);
                        }
                    } else if target.right {
                        self.pos_x = (self.pos_x + self.mirror_speed.x * delta()).rem_euclid(360.0);
                        if self.pos_x >= self.mirror_x_border.x + self.mirror_x_variance.y {
                            self.pos_x = self.pos_x.min(self.mirror_x_border.y);
                            self.snd_move_end.update_target(SoundTarget::Start);
                        }
                    } else {
                        self.pos_x = self
                            .pos_x
                            .rem_euclid(360.0)
                            .min(self.mirror_x_border.x)
                            .max(self.mirror_x_border.y);
                    }

                    let snd_move_target = if target.is_one() {
                        SoundTarget::Start
                    } else {
                        SoundTarget::Stop
                    };
                    self.snd_move.update_target(snd_move_target);
                }
                None => {
                    self.snd_move.update_target(SoundTarget::Stop);
                }
            }
        } else {
            self.snd_move.update_target(SoundTarget::Stop);
        }

        self.pos_x_anim.set(self.pos_x);
        self.pos_y_anim.set(self.pos_y);
    }
}
