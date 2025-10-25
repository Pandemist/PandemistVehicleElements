use std::f32::consts::PI;

use lotus_extra::rand::gen_f32;
use lotus_script::time::delta;

use crate::{
    api::{
        animation::Animation,
        general::mouse_move,
        light::{BlinkRelais, Light},
        sound::Sound,
        vehicle_door::VehicleDoor,
    },
    management::enums::door_enums::{DoorState, DoorTarget},
};

const DOORWARN_INTERVAL_IN: f32 = 0.777;
const DOORWARN_INTERVAL_IN_HALF: f32 = DOORWARN_INTERVAL_IN / 2.0;

pub struct AegElectricDoorBuilder {
    id: usize,
    plug_radius: f32,
    shift: f32,
    friction: f32,
    open_start_speed: f32,
    open_end_speed: f32,
    open_start_end_change_pos: f32,
    close_start_speed: f32,
    close_end_speed: f32,
    close_start_end_change_pos: f32,
    traction_stiftness: f32,
    reflection_open: f32,
    reflection_close: f32,

    pos: f32,
    speed: f32,
    anim_x: Animation,
    anim_y: Animation,
    close_timer: f32,
    regular_open_time: f32,
    min_open_time: f32,

    grabbing_a: bool,
    grabbing_b: bool,

    mouse_factor: f32,

    is_series_1: bool,

    state: DoorState,

    target: i32,

    warn_relais: BlinkRelais,
    lm_warn_in: Light,

    emergency_door_unlock: bool,
    emergency_door_unlock_last: bool,

    open_flag: bool,

    closed_while_warning: bool,

    snd_open_start: Sound,
    snd_open_end: Sound,
    snd_close_start: Sound,
    snd_close_end: Sound,
    snd_door_close: Sound,

    snd_open_start_2: Sound,
    snd_open_end_2: Sound,
    snd_close_start_2: Sound,
    snd_close_end_2: Sound,
    snd_door_close_2: Sound,

    pass_door: VehicleDoor,

    snd_door_warn: Sound,
}

impl AegElectricDoorBuilder {
    pub fn plug_radius(mut self, plug_radius: f32) -> Self {
        self.plug_radius = plug_radius;
        self
    }

    pub fn shift(mut self, shift: f32) -> Self {
        self.shift = shift;
        self
    }

    pub fn friction(mut self, friction: f32) -> Self {
        self.friction = friction;
        self
    }

    pub fn open_start_speed(mut self, open_start_speed: f32) -> Self {
        self.open_start_speed = open_start_speed;
        self
    }

    pub fn open_end_speed(mut self, open_end_speed: f32) -> Self {
        self.open_end_speed = open_end_speed;
        self
    }

    pub fn open_start_end_change_pos(mut self, open_start_end_change_pos: f32) -> Self {
        self.open_start_end_change_pos = open_start_end_change_pos;
        self
    }

    pub fn close_start_speed(mut self, close_start_speed: f32) -> Self {
        self.close_start_speed = close_start_speed;
        self
    }

    pub fn close_end_speed(mut self, close_end_speed: f32) -> Self {
        self.close_end_speed = close_end_speed;
        self
    }

    pub fn close_start_end_change_pos(mut self, close_start_end_change_pos: f32) -> Self {
        self.close_start_end_change_pos = close_start_end_change_pos;
        self
    }

    pub fn traction_stiftness(mut self, traction_stiftness: f32) -> Self {
        self.traction_stiftness = traction_stiftness;
        self
    }

    pub fn modify_reflection(mut self, open: f32, close: f32) -> Self {
        self.reflection_open = open;
        self.reflection_close = close;
        self
    }

    pub fn mouse_factor(mut self, mouse_factor: f32) -> Self {
        self.mouse_factor = mouse_factor;
        self
    }

    pub fn set_1st_series(
        mut self,
        sound_open_start_name: impl Into<String>,
        sound_open_end_name: impl Into<String>,
        sound_close_start_name: impl Into<String>,
        sound_close_end_name: impl Into<String>,
        sound_door_close_name: impl Into<String>,
    ) -> Self {
        self.is_series_1 = true;
        self.snd_open_start = Sound::new_simple(Some(&sound_open_start_name.into()));
        self.snd_open_end = Sound::new_simple(Some(&sound_open_end_name.into()));
        self.snd_close_start = Sound::new_simple(Some(&sound_close_start_name.into()));
        self.snd_close_end = Sound::new_simple(Some(&sound_close_end_name.into()));
        self.snd_door_close = Sound::new_simple(Some(&sound_door_close_name.into()));

        self.reflection_open = gen_f32(0.03..=0.05);
        self.reflection_close = 0.05;
        self
    }

    pub fn set_2nd_series(
        mut self,
        sound_open_start_name: impl Into<String>,
        sound_open_end_name: impl Into<String>,
        sound_close_start_name: impl Into<String>,
        sound_close_end_name: impl Into<String>,
        sound_door_close_name: impl Into<String>,
    ) -> Self {
        self.is_series_1 = false;
        self.snd_open_start_2 = Sound::new_simple(Some(&sound_open_start_name.into()));
        self.snd_open_end_2 = Sound::new_simple(Some(&sound_open_end_name.into()));
        self.snd_close_start_2 = Sound::new_simple(Some(&sound_close_start_name.into()));
        self.snd_close_end_2 = Sound::new_simple(Some(&sound_close_end_name.into()));
        self.snd_door_close_2 = Sound::new_simple(Some(&sound_door_close_name.into()));
        self.reflection_open = gen_f32(0.05..=0.07);
        self.reflection_close = 0.07;
        self
    }

    pub fn modify_warn_relais(mut self, interval: f32, on_time: f32, reset_time: f32) -> Self {
        self.warn_relais = BlinkRelais::new(interval, on_time, reset_time);
        self
    }

    pub fn regular_open_time(mut self, regular_open_time: f32) -> Self {
        self.regular_open_time = regular_open_time;
        self
    }

    pub fn min_open_time(mut self, min_open_time: f32) -> Self {
        self.min_open_time = min_open_time;
        self
    }

    pub fn add_warning(
        mut self,
        light_name: impl Into<String>,
        sound_name: impl Into<String>,
    ) -> Self {
        self.snd_door_warn = Sound::new_simple(Some(&sound_name.into()));
        self.lm_warn_in = Light::new(Some(&light_name.into()));
        self
    }

    pub fn build(self) -> AegElectricDoor {
        AegElectricDoor {
            id: self.id,
            plug_radius: self.plug_radius,
            shift: self.shift,
            friction: self.friction,
            open_start_speed: self.open_start_speed,
            open_end_speed: self.open_end_speed,
            open_start_end_change_pos: self.open_start_end_change_pos,
            close_start_speed: self.close_start_speed,
            close_end_speed: self.close_end_speed,
            close_start_end_change_pos: self.close_start_end_change_pos,
            traction_stiftness: self.traction_stiftness,
            reflection_open: self.reflection_open,
            reflection_close: self.reflection_close,
            pos: self.pos,
            speed: self.speed,
            anim_x: self.anim_x,
            anim_y: self.anim_y,
            close_timer: self.close_timer,
            regular_open_time: self.regular_open_time,
            min_open_time: self.min_open_time,
            grabbing_a: self.grabbing_a,
            grabbing_b: self.grabbing_b,
            mouse_factor: self.mouse_factor,
            is_series_1: self.is_series_1,
            state: self.state,
            target: self.target,
            warn_relais: self.warn_relais,
            lm_warn_in: self.lm_warn_in,
            emergency_door_unlock: self.emergency_door_unlock,
            emergency_door_unlock_last: self.emergency_door_unlock_last,
            open_flag: self.open_flag,
            closed_while_warning: self.closed_while_warning,
            snd_open_start: self.snd_open_start,
            snd_open_end: self.snd_open_end,
            snd_close_start: self.snd_close_start,
            snd_close_end: self.snd_close_end,
            snd_door_close: self.snd_door_close,
            snd_open_start_2: self.snd_open_start_2,
            snd_open_end_2: self.snd_open_end_2,
            snd_close_start_2: self.snd_close_start_2,
            snd_close_end_2: self.snd_close_end_2,
            snd_door_close_2: self.snd_door_close_2,
            pass_door: self.pass_door,
            snd_door_warn: self.snd_door_warn,
        }
    }
}

#[derive(Debug)]
pub struct AegElectricDoor {
    id: usize,
    plug_radius: f32,
    shift: f32,
    friction: f32,
    open_start_speed: f32,
    open_end_speed: f32,
    open_start_end_change_pos: f32,
    close_start_speed: f32,
    close_end_speed: f32,
    close_start_end_change_pos: f32,
    traction_stiftness: f32,
    reflection_open: f32,
    reflection_close: f32,

    pub pos: f32,
    speed: f32,
    anim_x: Animation,
    anim_y: Animation,
    close_timer: f32,
    regular_open_time: f32,
    min_open_time: f32,
    grabbing_a: bool,
    grabbing_b: bool,

    mouse_factor: f32,

    is_series_1: bool,

    pub state: DoorState,

    target: i32,

    warn_relais: BlinkRelais,
    lm_warn_in: Light,

    emergency_door_unlock: bool,
    emergency_door_unlock_last: bool,

    open_flag: bool,

    closed_while_warning: bool,

    snd_open_start: Sound,
    snd_open_end: Sound,
    snd_close_start: Sound,
    snd_close_end: Sound,
    snd_door_close: Sound,

    snd_open_start_2: Sound,
    snd_open_end_2: Sound,
    snd_close_start_2: Sound,
    snd_close_end_2: Sound,
    snd_door_close_2: Sound,

    pass_door: VehicleDoor,

    snd_door_warn: Sound,
}

impl AegElectricDoor {
    pub fn builder(
        id: usize,
        animation_x_name: impl Into<String>,
        animation_y_name: impl Into<String>,
    ) -> AegElectricDoorBuilder {
        AegElectricDoorBuilder {
            id,
            plug_radius: 0.06,
            shift: 0.58,
            friction: 0.05,
            open_start_speed: gen_f32(0.58..=0.65),
            open_end_speed: 0.3,
            open_start_end_change_pos: 0.6,
            close_start_speed: gen_f32(0.45..=0.5),
            close_end_speed: 0.1,
            close_start_end_change_pos: 0.2,
            traction_stiftness: 4.0,
            reflection_open: 0.0,
            reflection_close: 0.0,
            pos: 0.0,
            speed: 0.0,
            anim_x: Animation::new(Some(&animation_x_name.into())),
            anim_y: Animation::new(Some(&animation_y_name.into())),
            close_timer: 0.0,
            regular_open_time: 6.0,
            min_open_time: 2.0,
            grabbing_a: false,
            grabbing_b: false,
            mouse_factor: 1.0,
            is_series_1: false,
            state: DoorState::default(),
            target: 0,
            warn_relais: BlinkRelais::new(DOORWARN_INTERVAL_IN, DOORWARN_INTERVAL_IN_HALF, 0.12),
            lm_warn_in: Light::new(None),
            emergency_door_unlock: false,
            emergency_door_unlock_last: false,
            open_flag: false,
            closed_while_warning: false,
            snd_open_start: Sound::new_simple(None),
            snd_open_end: Sound::new_simple(None),
            snd_close_start: Sound::new_simple(None),
            snd_close_end: Sound::new_simple(None),
            snd_door_close: Sound::new_simple(None),
            snd_open_start_2: Sound::new_simple(None),
            snd_open_end_2: Sound::new_simple(None),
            snd_close_start_2: Sound::new_simple(None),
            snd_close_end_2: Sound::new_simple(None),
            snd_door_close_2: Sound::new_simple(None),
            pass_door: VehicleDoor::new(id, true, true),
            snd_door_warn: Sound::new_simple(None),
        }
    }

    fn move_door(&mut self, a: f32) {
        let mut new_speed = self.speed + delta() * a;
        if new_speed * self.speed < 0.0 {
            new_speed = 0.0;
        }
        self.speed = new_speed;

        let mut new_pos = self.pos + self.speed * delta();

        if (new_pos < 0.1 && self.pos >= 0.1) && self.is_series_1 {
            self.snd_door_close.start();
        }
        if (new_pos < 0.01 && self.pos >= 0.01) && self.is_series_1 {
            self.snd_close_end.start();
        }
        if (new_pos < 0.08 && self.pos >= 0.08) && !self.is_series_1 {
            self.snd_close_end_2.start();
        }

        if new_pos > 1.0 {
            new_pos = 1.0;
            new_speed = -self.speed + self.reflection_open;
            if (new_speed * self.speed) > 0.0 {
                new_speed = 0.0;
            }
            self.speed = new_speed;
            if self.is_series_1 {
                self.snd_open_end.start();
            } else {
                self.snd_open_end_2.start();
            }
        } else if new_pos < 0.0 {
            new_pos = 0.0;
            new_speed = -self.speed - self.reflection_close;
            if (new_speed * self.speed) > 0.0 {
                new_speed = 0.0;
            }
            self.speed = new_speed;
        }

        self.pos = new_pos;

        if self.pos < 0.1 {
            self.anim_x
                .set((self.pos * 5.0 * PI).sin() * self.plug_radius);
            self.anim_y
                .set((1.0 - (self.pos * 5.0 * PI).cos()) * self.plug_radius);
        } else {
            self.anim_x.set(self.plug_radius);
            self.anim_y
                .set((self.pos - 0.1) / 0.9 * self.shift + self.plug_radius);
        }
    }

    pub fn warn_tick(&mut self, power: bool, target: bool, spannung: f32) {
        if target && self.state == DoorState::Closed {
            self.closed_while_warning = true;
        }
        if !target {
            self.closed_while_warning = false;
        }

        if target && power && !self.emergency_door_unlock && !self.closed_while_warning {
            if self.warn_relais.tick() == 1 {
                self.snd_door_warn.start();
            }
        } else {
            self.warn_relais.reset();
            self.snd_door_warn.stop();
            self.lm_warn_in.set_brightness(0.0);
        }

        self.lm_warn_in
            .set_brightness((self.warn_relais.is_on as u8 as f32) * spannung);
    }

    pub fn tick(
        &mut self,
        power: bool,
        door_target: DoorTarget,
        door_1_btn: bool,
        emergency_door_unlock: bool,
        haltewunsch: bool,
    ) {
        self.emergency_door_unlock = emergency_door_unlock;

        let lichtschranke_frei = !self.pass_door.occupied();

        // Ansteuerung
        //----------------------------------------------

        let target_last = self.target;

        if power && !self.emergency_door_unlock {
            // Öffnen
            if door_target == DoorTarget::Open
                || (door_target == DoorTarget::Release && haltewunsch)
            {
                self.target = 1;
                self.open_flag = true;
            }

            // Bei Freigabe wieder zu laufen lassen
            if door_target == DoorTarget::Release
                && self.state == DoorState::Open
                && lichtschranke_frei
            {
                self.close_timer += delta();
                if self.close_timer > self.regular_open_time {
                    self.target = -1;
                }
            } else if door_target == DoorTarget::Close && self.state == DoorState::Open {
                self.close_timer += delta();
                if self.close_timer > self.min_open_time {
                    self.target = -1;
                }
            } else {
                self.close_timer = 0.0;
            }

            // Reversieren bei Lichtschranke
            if door_target == DoorTarget::Release
                && self.state == DoorState::Other
                && self.target < 0
                && !lichtschranke_frei
            {
                self.target = 1;
                self.open_flag = true;
            }

            // Direkt schließen
            if door_target == DoorTarget::FastClose {
                self.target = -1;
            }

            // Direkt schließen
            /*if door_target == DoorTarget::ForceClose && !self.open_flag {
                self.target = -1;
                self.open_flag = false;
            }*/

            self.emergency_door_unlock_last = self.emergency_door_unlock;
        } else {
            self.target = 0;
            self.close_timer = 0.0;
        }

        //format!("{:?}", self.target).set(&format!("AA_RealTarget_{}", self.id));

        //----------------------------------------------

        /*if force_open && (door_target == DoorTarget::Release || door_target == DoorTarget::Open) {
            self.force_open = true;
        } else if !force_open {
            self.force_open = false;
        }

        let target_last = self.target;
        if power && !self.emergency_door_unlock {
            if self.door_mode_last != door_target || (door_1_btn && !self.door_1_last) {
                if door_1_btn {
                    self.auf_flag = false;
                }

                if door_target == DoorTarget::Open {
                    self.new_mode = DoorTarget::Open;
                    self.auf_flag = self.state != DoorState::Open;
                } else if door_target == DoorTarget::Release {
                    if door_1_btn {
                        if self.state == DoorState::Open || self.target > 0 {
                            self.new_mode = DoorTarget::ForceClose;
                        } else if self.state == DoorState::Closed || self.target < 0 {
                            self.new_mode = DoorTarget::Open;
                        }
                    } else {
                        self.new_mode = DoorTarget::Release;
                    }
                } else if door_target == DoorTarget::Close {
                    if door_1_btn {
                        if self.state == DoorState::Open || self.target > 0 {
                            self.new_mode = DoorTarget::ForceClose;
                            self.auf_flag = false;
                        } else if self.state == DoorState::Closed || self.target < 0 {
                            self.new_mode = DoorTarget::Open;
                        }
                    } else {
                        self.new_mode = DoorTarget::Close;
                    }
                }

                if (self.new_mode == DoorTarget::Release && (self.force_open || haltewunsch))
                    || self.new_mode == DoorTarget::Open
                {
                    self.target = 1;
                    self.auf_flag = self.state != DoorState::Open;
                } else if self.new_mode == DoorTarget::Close {
                    self.target = 0;
                } else if self.new_mode == DoorTarget::ForceClose {
                    self.target = -1;
                }

                self.close_timer = 0.0;
            } else if self.target == 0 {
                if !self.auf_flag {
                    if self.new_mode == DoorTarget::Release
                        && self.state == DoorState::Open
                        && lichtschranke_frei
                    {
                        self.close_timer += 1.0;
                        if self.close_timer > CLOSE_TIME {
                            self.target = -1;
                        }
                    } else if self.new_mode == DoorTarget::Close {
                        if self.state == DoorState::Open {
                            self.close_timer += 1.0;
                            if self.close_timer > CLOSE_TIME_ZW {
                                self.target = -1;
                            }
                        } else if self.state == DoorState::Other {
                            self.target = -1;
                            self.close_timer = 0.0;
                        } else {
                            self.close_timer = 0.0;
                        }
                    } else if self.new_mode == DoorTarget::ForceClose {
                        self.target = -1;
                        self.close_timer = 0.0;
                    } else {
                        self.close_timer = 0.0;
                    }
                }

                if (self.new_mode == DoorTarget::Release && (self.force_open || haltewunsch))
                    || self.new_mode == DoorTarget::Open
                {
                    self.close_timer = 0.0;
                    self.target = 1;
                    self.auf_flag = self.state != DoorState::Open;
                }

                if self.emergency_door_unlock_last {
                    self.target = -1;
                }
            } else if self.target == -1 {
                if (self.new_mode == DoorTarget::Release && (!lichtschranke_frei || haltewunsch))
                    || (self.new_mode == DoorTarget::Close && !lichtschranke_frei)
                {
                    self.close_timer = 0.0;
                    self.target = 1;
                    self.auf_flag = self.state != DoorState::Open;
                }
            } else {
                self.close_timer = 0.0;
            }

            if self.force_open && self.state == DoorState::Open {
                self.target = self.target.max(0);
                self.close_timer = 0.0;
            }

            if self.state == DoorState::Closed
                && self.new_mode == DoorTarget::ForceClose
                && door_target == DoorTarget::Release
            {
                self.new_mode = door_target;
            }

            self.emergency_door_unlock_last = self.emergency_door_unlock;
        } else {
            self.target = 0;
            self.close_timer = 0.0;
        }*/

        //----------------------------------------------

        if target_last != self.target {
            if self.is_series_1 {
                self.snd_open_start.stop();
                self.snd_open_end.stop();
                self.snd_close_start.stop();
                self.snd_door_close.stop();
                self.snd_close_end.stop();
            } else {
                self.snd_open_start_2.stop();
                self.snd_open_end_2.stop();
                self.snd_close_start_2.stop();
                self.snd_door_close_2.stop();
                self.snd_close_end_2.stop();
            }
        }

        let mouse_delta_x = mouse_move().x * self.mouse_factor;

        if self.emergency_door_unlock || !(power && self.pos > 0.01) {
            if self.grabbing_a {
                self.pos = (self.pos - mouse_delta_x * delta()).clamp(0.0, 1.0);
            } else if self.grabbing_b {
                self.pos = (self.pos + mouse_delta_x * delta()).clamp(0.0, 1.0);
            }
        }

        if (self.speed == 0.0)
            && ((self.target > 0 && self.pos >= 1.0) || (self.target < 0 && self.pos <= 0.0))
        {
            self.target = 0;
            self.speed = 0.0;
        }

        if !power {
            let a = if self.speed > 0.0 {
                -self.friction
            } else if self.speed < 0.0 {
                self.friction
            } else {
                0.0
            };
            self.move_door(a);
        }

        if self.target > 0 {
            if self.pos < 0.01 && self.speed <= 0.0 {
                if self.is_series_1 {
                    self.snd_open_start.start();
                } else {
                    self.snd_open_start_2.start();
                }
            }

            let v_soll = if self.pos < self.open_start_end_change_pos {
                self.open_start_speed
            } else {
                self.open_end_speed
            };

            self.move_door((v_soll - self.speed) * self.traction_stiftness);
        }

        if self.target < 0 {
            if self.pos > 0.99 && self.speed >= 0.0 {
                if self.is_series_1 {
                    self.snd_close_start.start();
                } else {
                    self.snd_close_start_2.start();
                }
            }

            let v_soll = if self.pos > self.close_start_end_change_pos {
                -self.close_start_speed
            } else {
                -self.close_end_speed
            };

            self.move_door((v_soll - self.speed) * self.traction_stiftness);
        }

        if self.pos == 1.0 {
            self.state = DoorState::Open;
            self.open_flag = false;
        } else if self.pos < 0.005 {
            self.state = DoorState::Closed;
        } else {
            self.state = DoorState::Other;
        }

        self.pass_door.update_open(self.pos > 0.75);
        self.pass_door
            .update_released(door_target >= DoorTarget::Release);
    }
}
