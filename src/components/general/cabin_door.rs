//! # Cabin Door Module
//!
//! This module provides interactive door systems for train cabin simulations.
//! It includes two main door types:
//! - `HandDoorWithLever`: A more complex door with handle and bolt mechanism
//!
//! Both door types support physics-based movement, mouse interaction, sound effects,
//! and animations.

use std::rc::Rc;

use lotus_script::time::delta;

use crate::api::{
    animation::Animation,
    general::mouse_move,
    key_event::{KeyEvent, KeyEventCab},
    sound::Sound,
};

#[derive(PartialEq, Eq)]
pub enum HandDoorLockingMode {
    Latch, // Falle (Andere)
    Bolt,  // Bolzen (GT6N)
}

pub struct HandDoorWithLeverBuilder {
    mode: HandDoorLockingMode,

    reflect_close: f32,

    cab_side: Option<KeyEventCab>,
    pub pos: f32,
    speed: f32,
    friction: f32,
    mouse_factor: f32,
    pub door_key_value: bool,

    snd_open_end: Sound,
    snd_open_end_vol_curve: Rc<dyn Fn(f32) -> f32>,

    snd_close_snap_end: Sound,
    snd_close_snap_end_vol_curve: Rc<dyn Fn(f32) -> f32>,

    snd_close_bounce_end: Sound,
    snd_close_bounce_end_vol_curve: Rc<dyn Fn(f32) -> f32>,

    snd_close_throw_end: Sound,
    snd_close_throw_end_vol_curve: Rc<dyn Fn(f32) -> f32>,

    snd_handle_press: Sound,
    snd_handle_release: Sound,

    door_anim: Animation,
    bolt_atch_anim: Animation,
    handle_anim: Animation,

    key_grab_a: KeyEvent,
    key_grab_b: KeyEvent,
    key_handle_a: KeyEvent,
    key_handle_b: KeyEvent,
}

impl HandDoorWithLeverBuilder {
    pub fn set_bolt_mode(mut self) -> Self {
        self.mode = HandDoorLockingMode::Bolt;
        self
    }

    pub fn reflect_close(mut self, reflect_close: f32) -> Self {
        self.reflect_close = reflect_close;
        self
    }

    pub fn friction(mut self, friction: f32) -> Self {
        self.friction = friction;
        self
    }

    pub fn mouse_factor(mut self, mouse_factor: f32) -> Self {
        self.mouse_factor = mouse_factor;
        self
    }

    pub fn snd_open_end(
        mut self,
        snd_open_end_name: impl Into<String>,
        volume_name: Option<&str>,
        volume_func: Option<Rc<dyn Fn(f32) -> f32>>,
    ) -> Self {
        self.snd_open_end = Sound::new(Some(&snd_open_end_name.into()), volume_name, None);
        self.snd_open_end_vol_curve = match volume_func {
            Some(curve) => curve,
            None => Rc::new(|x| x),
        };
        self
    }

    pub fn snd_close_snap_end(
        mut self,
        snd_close_snap_end_name: impl Into<String>,
        volume_name: Option<&str>,
        volume_func: Option<Rc<dyn Fn(f32) -> f32>>,
    ) -> Self {
        self.snd_close_snap_end =
            Sound::new(Some(&snd_close_snap_end_name.into()), volume_name, None);
        self.snd_close_snap_end_vol_curve = match volume_func {
            Some(curve) => curve,
            None => Rc::new(|x| x),
        };
        self
    }

    pub fn snd_close_bounce_end(
        mut self,
        snd_close_bounce_end_name: impl Into<String>,
        volume_name: Option<&str>,
        volume_func: Option<Rc<dyn Fn(f32) -> f32>>,
    ) -> Self {
        self.snd_close_bounce_end =
            Sound::new(Some(&snd_close_bounce_end_name.into()), volume_name, None);
        self.snd_close_bounce_end_vol_curve = match volume_func {
            Some(curve) => curve,
            None => Rc::new(|x| x),
        };
        self
    }

    pub fn snd_close_throw_end(
        mut self,
        snd_close_throw_end_name: impl Into<String>,
        volume_name: Option<&str>,
        volume_func: Option<Rc<dyn Fn(f32) -> f32>>,
    ) -> Self {
        self.snd_close_throw_end =
            Sound::new(Some(&snd_close_throw_end_name.into()), volume_name, None);
        self.snd_close_throw_end_vol_curve = match volume_func {
            Some(curve) => curve,
            None => Rc::new(|x| x),
        };
        self
    }

    pub fn snd_handle_press(mut self, snd_handle_press_name: impl Into<String>) -> Self {
        self.snd_handle_press = Sound::new_simple(Some(&snd_handle_press_name.into()));
        self
    }

    pub fn snd_handle_release(mut self, snd_handle_release_name: impl Into<String>) -> Self {
        self.snd_handle_release = Sound::new_simple(Some(&snd_handle_release_name.into()));
        self
    }

    pub fn build(self) -> HandDoorWithLever {
        HandDoorWithLever {
            mode: self.mode,
            reflect_close: self.reflect_close,
            cab_side: self.cab_side,
            pos: self.pos,
            speed: self.speed,
            friction: self.friction,
            mouse_factor: self.mouse_factor,
            door_key_value: self.door_key_value,
            snd_open_end: self.snd_open_end,
            snd_open_end_vol_curve: self.snd_open_end_vol_curve,
            snd_close_snap_end: self.snd_close_snap_end,
            snd_close_snap_end_vol_curve: self.snd_close_snap_end_vol_curve,
            snd_close_bounce_end: self.snd_close_bounce_end,
            snd_close_bounce_end_vol_curve: self.snd_close_bounce_end_vol_curve,
            snd_close_throw_end: self.snd_close_throw_end,
            snd_close_throw_end_vol_curve: self.snd_close_throw_end_vol_curve,
            snd_handle_press: self.snd_handle_press,
            snd_handle_release: self.snd_handle_release,
            door_anim: self.door_anim,
            bolt_atch_anim: self.bolt_atch_anim,
            handle_anim: self.handle_anim,
            key_grab_a: self.key_grab_a,
            key_grab_b: self.key_grab_b,
            key_handle_a: self.key_handle_a,
            key_handle_b: self.key_handle_b,
            snd_closed_played: true,
        }
    }
}

pub struct HandDoorWithLever {
    mode: HandDoorLockingMode,

    reflect_close: f32,

    cab_side: Option<KeyEventCab>,
    pub pos: f32,
    pub speed: f32,
    friction: f32,
    mouse_factor: f32,
    pub door_key_value: bool,

    snd_open_end: Sound,
    snd_open_end_vol_curve: Rc<dyn Fn(f32) -> f32>,

    snd_close_snap_end: Sound,
    snd_close_snap_end_vol_curve: Rc<dyn Fn(f32) -> f32>,

    snd_close_bounce_end: Sound,
    snd_close_bounce_end_vol_curve: Rc<dyn Fn(f32) -> f32>,

    snd_close_throw_end: Sound,
    snd_close_throw_end_vol_curve: Rc<dyn Fn(f32) -> f32>,

    snd_handle_press: Sound,
    snd_handle_release: Sound,

    door_anim: Animation,
    bolt_atch_anim: Animation,
    handle_anim: Animation,

    key_grab_a: KeyEvent,
    key_grab_b: KeyEvent,
    pub key_handle_a: KeyEvent,
    pub key_handle_b: KeyEvent,

    snd_closed_played: bool,
}

impl HandDoorWithLever {
    #[expect(clippy::too_many_arguments)]
    pub fn builder(
        anim_name: impl Into<String>,
        anim_bolt_name: impl Into<String>,
        anim_handle_name: impl Into<String>,
        event_grab_a_name: &str,
        event_grab_b_name: &str,
        event_handle_a_name: &str,
        event_handle_b_name: &str,
        cab_side: Option<KeyEventCab>,
    ) -> HandDoorWithLeverBuilder {
        HandDoorWithLeverBuilder {
            mode: HandDoorLockingMode::Latch,

            reflect_close: 0.049,

            cab_side,
            pos: 0.0,
            speed: 0.0,
            friction: 0.0,
            mouse_factor: 0.0,
            door_key_value: false,

            snd_open_end: Sound::new_simple(None),
            snd_open_end_vol_curve: Rc::new(|x| x),

            snd_close_snap_end: Sound::new_simple(None),
            snd_close_snap_end_vol_curve: Rc::new(|x| x),

            snd_close_bounce_end: Sound::new_simple(None),
            snd_close_bounce_end_vol_curve: Rc::new(|x| x),

            snd_close_throw_end: Sound::new_simple(None),
            snd_close_throw_end_vol_curve: Rc::new(|x| x),

            snd_handle_press: Sound::new_simple(None),
            snd_handle_release: Sound::new_simple(None),

            door_anim: Animation::new(Some(&anim_name.into())),
            bolt_atch_anim: Animation::new(Some(&anim_bolt_name.into())),
            handle_anim: Animation::new(Some(&anim_handle_name.into())),

            key_grab_a: KeyEvent::new(Some(event_grab_a_name), cab_side),
            key_grab_b: KeyEvent::new(Some(event_grab_b_name), cab_side),
            key_handle_a: KeyEvent::new(Some(event_handle_a_name), cab_side),
            key_handle_b: KeyEvent::new(Some(event_handle_b_name), cab_side),
        }
    }

    pub fn tick(&mut self, physic_force: f32) {
        let force = mouse_move().x * self.mouse_factor;
        let grabbing_a =
            self.key_grab_a.is_pressed() || self.key_handle_a.is_pressed() || self.door_key_value;
        let grabbing_b = self.key_handle_b.is_pressed() || self.key_grab_b.is_pressed();
        let grabbing = grabbing_a || grabbing_b;
        let handle =
            self.key_handle_a.is_pressed() || self.key_handle_b.is_pressed() || self.door_key_value;

        if self.key_handle_a.is_just_pressed() || self.key_handle_b.is_just_pressed() {
            self.snd_handle_press.start();
        }
        if self.key_handle_a.is_just_released() || self.key_handle_b.is_just_released() {
            self.snd_handle_release.start();
        }

        let pos_bolt_latch =
            if self.mode == HandDoorLockingMode::Bolt || self.mode == HandDoorLockingMode::Latch {
                handle || self.pos > 0.1 && self.pos < self.reflect_close
            } else {
                handle
            };

        /*if force > 0.0 && physic_force > 0.0 {
            self.speed = 0.0;
            return;
        }*/

        let grab_vz = if grabbing_a {
            1.0
        } else if grabbing_b {
            -1.0
        } else {
            0.0
        };

        let pos_last = self.pos;

        if grabbing {
            self.pos = if pos_bolt_latch {
                (self.pos + grab_vz * force).clamp(0.0, 1.0)
            } else if self.pos > self.reflect_close {
                if self.mode == HandDoorLockingMode::Bolt {
                    (self.pos + grab_vz * force).clamp(self.reflect_close, 1.0)
                } else {
                    (self.pos + grab_vz * force).clamp(0.0, 1.0)
                }
            } else if self.mode == HandDoorLockingMode::Latch && self.pos > 0.0 {
                if (grab_vz * force) > 0.0 {
                    self.pos + grab_vz * force * 0.8 * delta()
                } else {
                    self.pos + grab_vz * force * delta()
                }
            } else {
                self.pos
            };

            self.speed = force / delta();
        } else {
            // Während gerade auf höhe der Falle
            self.pos = if !pos_bolt_latch && self.pos < self.reflect_close {
                if self.mode == HandDoorLockingMode::Bolt {
                    self.pos
                } else if self.mode == HandDoorLockingMode::Latch {
                    if self.pos > 0.0 {
                        self.pos + self.speed.min(0.0) * delta()
                    } else {
                        0.0
                    }
                } else {
                    self.pos
                }
            } else if self.pos >= self.reflect_close {
                if self.mode == HandDoorLockingMode::Bolt {
                    (self.pos + self.speed * delta()).max(self.reflect_close)
                } else if self.mode == HandDoorLockingMode::Latch {
                    self.pos + self.speed * delta()
                } else {
                    self.pos
                }
            } else {
                self.pos
            };

            self.speed += physic_force / delta();
        }

        if self.mode == HandDoorLockingMode::Bolt {
            // Sound for bouncing
            if self.pos <= self.reflect_close && pos_last > self.reflect_close {
                self.snd_close_bounce_end
                    .update_volume((self.snd_close_bounce_end_vol_curve)(self.speed));
                self.snd_close_bounce_end.start();
                self.pos = self.reflect_close;
                self.speed *= -0.2;
            }
        }

        if self.mode == HandDoorLockingMode::Latch {
            // Sound for throw
            if self.pos < self.reflect_close && pos_last >= self.reflect_close && !grabbing {
                self.snd_close_throw_end
                    .update_volume((self.snd_close_throw_end_vol_curve)(self.speed));
                self.snd_close_throw_end.start();
            }
        }

        // Sound for snapping
        if self.pos < 0.0 {
            self.snd_close_snap_end
                .update_volume((self.snd_close_snap_end_vol_curve)(self.speed));
            self.snd_close_snap_end.start();
            self.pos = 0.0;
            self.speed = 0.0;
        }

        // Door bounces off the outer end
        if self.pos > 1.0 {
            self.snd_open_end
                .update_volume((self.snd_open_end_vol_curve)(self.speed));
            self.snd_open_end.start();
            self.pos = 1.0;
            self.speed *= -0.2;
        }

        if self.mode == HandDoorLockingMode::Bolt {
            // Bolt is pressed in and is 'pushed' straight
            if self.pos < self.reflect_close && self.pos > 0.0 && !handle {
                self.speed = 0.0;
            }
        }

        // Wenn keine bewegung
        if pos_last == self.pos {
            //self.speed = 0.0;
        }

        // Calc new Speed
        if self.speed.abs() > 0.0001 {
            let new_speed = self.speed + (-self.speed.signum() * self.friction) * delta();

            self.speed = if new_speed * self.speed < 0.0 {
                0.0
            } else {
                new_speed
            };
        }

        self.door_anim.set(self.pos);
        self.bolt_atch_anim.set(pos_bolt_latch.into());
        self.handle_anim.set(handle.into());
    }
}
