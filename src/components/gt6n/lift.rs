use lotus_extra::{math::PiecewiseLinearFunction, vehicle::CockpitSide};
use lotus_script::time::delta;

use crate::api::{
    animation::{Animation, MappedAnimation},
    light::{BlinkRelais, Light},
    sound::{Sound, SoundTarget},
};

const LIFT_HIGHT_SO: f32 = 1.0;
const LIFT_HIGHT_PLATFORM: f32 = 0.3;
const LOCKING_CHANGE_TIME: f32 = 2.0;
const LIFT_RAMPE_SPEED: f32 = 1.0 / 6.0;
const LIFT_ABROLLSCHUTZ_SPEED: f32 = 1.0 / 1.5;
const LIFT_HIGHT_SPEED: f32 = 1.0 / 3.0;
const LIFT_DOORWARN_INTERVAL: f32 = 1.0;
const LIFT_DOORWARN_INTERVAL_HALF: f32 = LIFT_DOORWARN_INTERVAL / 2.0;

#[derive(Debug)]
#[expect(clippy::struct_excessive_bools)]
pub struct Hublift {
    height_pos: f32,
    height_anim: Animation,
    ramp_pos: f32,
    ramp_anim: Animation,
    roll_protection_pos: f32,
    roll_protection_anim: Animation,
    rampflap_anim: MappedAnimation,
    ramptransition_anim: MappedAnimation,

    paus_timer: f32,

    locking: f32,

    pub fuse_control: bool,
    pub fuse_power: bool,
    pub fuse_control_signal: bool,

    action_ready: bool,

    prepared: bool,

    snd_warn: Sound,
    snd_a_down: Sound,
    snd_a_up: Sound,
    snd_a_up_end: Sound,
    snd_b_down: Sound,
    snd_b_up: Sound,
    snd_b_up_end: Sound,
    snd_barrier_down: Sound,
    snd_barrier_up: Sound,

    pub warnrelais: BlinkRelais,
    l_lift_warning: Light,

    pub in_use: bool,
}

impl Hublift {
    #[must_use]
    pub fn new(cab_side: CockpitSide) -> Self {
        Self {
            height_pos: 0.0,
            height_anim: Animation::new(Some(&format!(
                "AV_{}_Hublift_Hight_Pos",
                String::from(cab_side)
            ))),
            ramp_pos: 0.0,
            ramp_anim: Animation::new(Some(&format!(
                "AV_{}_Hublift_Rampen_Pos",
                String::from(cab_side)
            ))),
            roll_protection_pos: 0.0,
            roll_protection_anim: Animation::new(Some(&format!(
                "AV_{}_Hublift_Abrollschutz_Pos",
                String::from(cab_side)
            ))),
            rampflap_anim: MappedAnimation::new(
                Some(&format!(
                    "AV_{}_Hublift_Rampenklappe_Pos",
                    String::from(cab_side)
                )),
                Some(PiecewiseLinearFunction::new(vec![
                    (0.0, 0.0),
                    (0.038, 0.0),
                    (0.060, 0.090),
                    (0.090, 0.190),
                    (0.122, 0.316),
                    (0.156, 0.424),
                    (0.182, 0.468),
                    (0.208, 0.512),
                    (0.252, 0.584),
                    (0.296, 0.652),
                    (0.320, 0.688),
                    (0.364, 0.752),
                    (0.388, 0.786),
                    (0.412, 0.822),
                    (0.430, 0.844),
                    (0.448, 0.854),
                    (1.0, 0.854),
                ])),
            ),
            ramptransition_anim: MappedAnimation::new(
                Some(&format!(
                    "AV_{}_Hublift_Rampenuebergang_Pos",
                    String::from(cab_side)
                )),
                Some(PiecewiseLinearFunction::new(vec![
                    (0.0, 0.0),
                    (0.02, 0.1),
                    (0.038, 0.2),
                    (0.058, 0.3),
                    (0.078, 0.4),
                    (0.098, 0.5),
                    (0.120, 0.6),
                    (0.138, 0.7),
                    (0.156, 0.8),
                    (0.174, 0.9),
                    (0.194, 1.0),
                    (0.212, 1.116),
                    (0.3, 1.605),
                ])),
            ),

            paus_timer: 0.0,
            locking: 0.0,
            fuse_control: false,
            fuse_power: false,
            fuse_control_signal: false,
            action_ready: false,
            prepared: false,

            snd_warn: Sound::new(
                Some(&format!("Snd_{}_Hublift_Warn", String::from(cab_side))),
                None,
                None,
            ),
            snd_a_down: Sound::new(
                Some(&format!("Snd_{}_Hublift_Lift_A_Dn", String::from(cab_side))),
                None,
                None,
            ),
            snd_a_up: Sound::new(
                Some(&format!("Snd_{}_Hublift_Lift_A_Up", String::from(cab_side))),
                None,
                None,
            ),
            snd_a_up_end: Sound::new(
                Some(&format!(
                    "Snd_{}_Hublift_Lift_A_Up_End",
                    String::from(cab_side)
                )),
                None,
                None,
            ),
            snd_b_down: Sound::new(
                Some(&format!("Snd_{}_Hublift_Lift_B_Dn", String::from(cab_side))),
                None,
                None,
            ),
            snd_b_up: Sound::new(
                Some(&format!("Snd_{}_Hublift_Lift_B_Up", String::from(cab_side))),
                None,
                None,
            ),
            snd_b_up_end: Sound::new(
                Some(&format!(
                    "Snd_{}_Hublift_Lift_B_Up_End",
                    String::from(cab_side)
                )),
                None,
                None,
            ),
            snd_barrier_down: Sound::new(
                Some(&format!(
                    "Snd_{}_Hublift_Lift_Barrier_Dn",
                    String::from(cab_side)
                )),
                None,
                None,
            ),
            snd_barrier_up: Sound::new(
                Some(&format!(
                    "Snd_{}_Hublift_Lift_Barrier_Up",
                    String::from(cab_side)
                )),
                None,
                None,
            ),

            warnrelais: BlinkRelais::new(LIFT_DOORWARN_INTERVAL, LIFT_DOORWARN_INTERVAL_HALF, 0.1),

            l_lift_warning: Light::new(Some(&format!(
                "L_{}_Hubliftwarnung",
                String::from(cab_side)
            ))),

            in_use: false,
        }
    }

    #[expect(clippy::too_many_lines)]
    pub fn tick(
        &mut self,
        mut target: i32,
        target_level: i32,
        allowed: bool,
        _notablegen: bool,
        _spannung: f32,
    ) {
        let height_pos_last = self.height_pos;
        let ramp_pos_last = self.ramp_pos;
        let roll_protection_pos_last = self.roll_protection_pos;
        let locking_last = self.locking;

        if allowed {
            if target == 0 {
                self.action_ready = false;
            }

            if self.action_ready {
                target = 0;
            }

            /*set_var("AA_Lift_Target", target);
            set_var("AA_Lift_Level", target_level);
            set_var("AA_Lift_Pause", self.paus_timer);
            set_var("AA_Lift_InUse", self.in_use);
            set_var("AA_Lift_Arretierung", self.locking);
            set_var("AA_Lift_Vorbereitet", self.prepared);*/

            self.paus_timer = (self.paus_timer * delta()).max(0.0);

            // Pause timer <= 0
            if self.paus_timer <= 0.0 {
                // Lowering
                if target < 0 && target_level > 1 {
                    self.locking = (self.locking + delta()).min(LOCKING_CHANGE_TIME);

                    // Lower to platform level
                    if target_level == 2 {
                        // Only work when the lock is released
                        if self.locking >= LOCKING_CHANGE_TIME {
                            // Preparation completed
                            if locking_last != LOCKING_CHANGE_TIME
                                && self.locking == LOCKING_CHANGE_TIME
                            {
                                self.action_ready = true;
                            }

                            // Lowering
                            self.height_pos = (self.height_pos + LIFT_HIGHT_SPEED * delta())
                                .min(LIFT_HIGHT_PLATFORM);

                            if self.height_pos == LIFT_HIGHT_PLATFORM {
                                self.action_ready = true;
                            }
                        }
                    }
                    // Lower to road level
                    else if target_level == 3 {
                        if self.prepared {
                            // Lower ramp
                            self.height_pos =
                                (self.height_pos + LIFT_HIGHT_SPEED * delta()).min(LIFT_HIGHT_SO);

                            // Lower roll guard
                            if self.height_pos >= LIFT_HIGHT_SO {
                                self.roll_protection_pos = (self.roll_protection_pos
                                    - LIFT_ABROLLSCHUTZ_SPEED * delta())
                                .max(0.0);
                            }

                            // Completion message
                            if self.height_pos >= LIFT_HIGHT_SO && self.roll_protection_pos == 0.0 {
                                self.action_ready = true;
                            }
                        } else {
                            // Extend ramp
                            self.ramp_pos = (self.ramp_pos + LIFT_RAMPE_SPEED * delta()).min(1.0);

                            // Set up roll-off protection
                            if self.ramp_pos >= 1.0 {
                                self.roll_protection_pos = (self.roll_protection_pos
                                    + LIFT_ABROLLSCHUTZ_SPEED * delta())
                                .min(1.0);
                            }

                            // Completion message
                            if self.ramp_pos >= 1.0
                                && self.roll_protection_pos >= 1.0
                                && self.locking >= LOCKING_CHANGE_TIME
                            {
                                self.prepared = true;
                                self.action_ready = true;
                            }
                        }
                    }
                }
                // Lifting
                else if target > 0 {
                    // Retract if above
                    if self.height_pos <= 0.0 {
                        self.prepared = false;

                        // Lower the roll-off protection again
                        self.roll_protection_pos =
                            (self.roll_protection_pos - LIFT_ABROLLSCHUTZ_SPEED * delta()).max(0.0);

                        // Retract ramp
                        if self.roll_protection_pos == 0.0 {
                            self.ramp_pos = (self.ramp_pos - LIFT_RAMPE_SPEED * delta()).max(0.0);
                        }

                        // Short pause when the ramp has reached the end position
                        if self.ramp_pos <= 0.0 && ramp_pos_last > 0.0 {
                            self.paus_timer = 0.5;
                        }

                        // Lock the lock again
                        if self.height_pos <= 0.0 && self.ramp_pos == 0.0 {
                            self.locking = (self.locking - delta()).max(0.0);
                        }

                        // Completion message
                        if self.locking <= 0.0 {
                            self.action_ready = true;
                        }
                    }
                    // Lift up
                    else {
                        // At street level
                        if self.ramp_pos > 0.0 {
                            // Lifting roll-off protection
                            self.roll_protection_pos = (self.roll_protection_pos
                                + LIFT_ABROLLSCHUTZ_SPEED * delta())
                            .min(1.0);

                            // Lift again when roll-off protection is set up
                            if self.roll_protection_pos >= 1.0 {
                                self.height_pos =
                                    (self.height_pos - LIFT_HIGHT_SPEED * delta()).max(0.0);
                            }
                        }
                        // Lift again
                        else {
                            self.height_pos =
                                (self.height_pos - LIFT_HIGHT_SPEED * delta()).max(0.0);
                        }

                        // Completion message
                        if self.height_pos == 0.0 {
                            self.action_ready = true;
                        }
                    }
                }
            }
        }

        if target != 0 && target_level > 0 && !self.action_ready {
            self.snd_warn.start();
        } else {
            self.snd_warn.stop();
        }

        self.snd_a_down.update_target(
            if (self.height_pos < height_pos_last) || (self.locking < locking_last) {
                SoundTarget::Start
            } else {
                SoundTarget::Stop
            },
        );
        self.snd_a_up.update_target(
            if (self.height_pos > height_pos_last) || (self.locking > locking_last) {
                SoundTarget::Start
            } else {
                SoundTarget::Stop
            },
        );
        if self.height_pos >= 1.0 && height_pos_last < 1.0 {
            self.snd_a_up_end.start();
        }
        self.snd_b_down
            .update_target(if self.ramp_pos < ramp_pos_last {
                SoundTarget::Start
            } else {
                SoundTarget::Stop
            });
        self.snd_b_up
            .update_target(if self.ramp_pos > ramp_pos_last {
                SoundTarget::Start
            } else {
                SoundTarget::Stop
            });
        if self.ramp_pos >= 1.0 && ramp_pos_last < 1.0 {
            self.snd_b_up_end.start();
        }
        if self.roll_protection_pos < 1.0 && roll_protection_pos_last >= 1.0 {
            self.snd_barrier_down.start();
        }
        if self.roll_protection_pos > 0.0 && roll_protection_pos_last <= 0.0 {
            self.snd_barrier_up.start();
        }

        self.height_anim.set(self.height_pos);
        self.ramp_anim.set(self.ramp_pos);
        self.roll_protection_anim.set(self.roll_protection_pos);
        self.rampflap_anim.set(self.ramp_pos);
        self.ramptransition_anim.set(self.height_pos);

        self.in_use = self.locking > 0.0;

        if self.in_use {
            self.warnrelais.tick();
        } else {
            self.warnrelais.reset();
        }
        self.l_lift_warning
            .set_brightness(self.warnrelais.is_on as u8 as f32);
    }
}
