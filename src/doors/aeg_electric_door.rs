use std::f32::consts::PI;

use lotus_script::delta;
use rand::Rng;

use crate::{
    elements::tech_elements::warnrelais::Blinkrelais,
    mocks::{animation::Animation, sound::Sound},
    structs::{
        internal_enums::{DoorState, DoorTarget, SoundTarget},
        traits::PassengerDoor,
    },
};

const CLOSE_TIME: f32 = 6.0;
const CLOSE_TIME_ZW: f32 = 2.0;
const DOORWARN_INTERVAL_IN: f32 = 0.777;
const DOORWARN_INTERVAL_IN_HALF: f32 = DOORWARN_INTERVAL_IN / 2.0;

#[derive(Default, Debug)]
pub struct AegElectricDoor {
    name_id: String,

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

    anim_x: Animation,
    anim_y: Animation,

    pos: f32,
    speed: f32,
    pos_x: f32,
    pos_y: f32,
    close_timer: f32,
    grabbing_a: bool,
    grabbing_b: bool,

    is_serie1: bool,

    state: DoorState,

    target: i32,
    energy: bool,
    sicherung: bool,

    warn_relais: Blinkrelais,

    kiwa_rolli: bool,

    lm_warn_in: f32,

    notentriegelung: bool,
    notentriegelung_last: bool,

    door_mode_last: DoorTarget,
    door_1_last: bool,

    new_mode: DoorTarget,
    auf_flag: bool,

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

    snd_door_warn: Sound,
    _snd_door_warn_pitch: f32,
}

impl AegElectricDoor {
    pub fn new(name: String, new_serie_1: bool) -> Self {
        let mut rng = rand::thread_rng();

        let mut door = AegElectricDoor {
            name_id: name.clone(),
            plug_radius: 0.06,
            shift: 0.58,
            friction: 0.05,
            open_start_speed: rng.gen_range(0.58..=0.65),
            open_end_speed: 0.3,
            open_start_end_change_pos: 0.6,
            close_start_speed: rng.gen_range(0.45..=0.5),
            close_end_speed: 0.1,
            close_start_end_change_pos: 0.2,
            traction_stiftness: 4.0,
            is_serie1: new_serie_1,
            warn_relais: Blinkrelais::new(DOORWARN_INTERVAL_IN, DOORWARN_INTERVAL_IN_HALF, 0.12),
            _snd_door_warn_pitch: 1.0,

            anim_x: Animation::new(format!("{}_x_anim", name)),
            anim_y: Animation::new(format!("{}_y_anim", name)),

            snd_open_start: Sound::new(format!("snd_{}_open_start", name)),
            snd_open_end: Sound::new(format!("snd_{}_open_end", name)),
            snd_close_start: Sound::new(format!("snd_{}_close_start", name)),
            snd_close_end: Sound::new(format!("snd_{}_close_end", name)),
            snd_door_close: Sound::new(format!("snd_{}_close", name)),

            snd_open_start_2: Sound::new(format!("snd_{}_open_start_2", name)),
            snd_open_end_2: Sound::new(format!("snd_{}_open_end_2", name)),
            snd_close_start_2: Sound::new(format!("snd_{}_close_Start_2", name)),
            snd_close_end_2: Sound::new(format!("snd_{}_close_end_2", name)),
            snd_door_close_2: Sound::new(format!("snd_{}_close_2", name)),

            ..Default::default()
        };

        if new_serie_1 {
            door.reflection_open = rng.gen_range(0.03..=0.05);
            door.reflection_close = 0.05;
        } else {
            door.reflection_open = rng.gen_range(0.05..=0.07);
            door.reflection_close = 0.07;
        }

        door
    }

    fn move_door(&mut self, a: f32) {
        let mut new_speed = self.speed + delta() * a;
        if new_speed * self.speed < 0.0 {
            new_speed = 0.0;
        }
        self.speed = new_speed;

        let mut new_pos = self.pos + self.speed * delta();

        if new_pos < 0.1 && self.pos >= 0.1 && self.is_serie1 {
            self.snd_door_close.update_target(SoundTarget::Start);
        }
        if new_pos < 0.01 && self.pos >= 0.01 && self.is_serie1 {
            self.snd_close_end.update_target(SoundTarget::Start);
        }
        if new_pos < 0.08 && self.pos >= 0.08 && !self.is_serie1 {
            self.snd_close_end_2.update_target(SoundTarget::Start);
        }

        if new_pos > 1.0 {
            new_pos = 1.0;
            new_speed = -self.speed + self.reflection_open;
            if new_speed * self.speed > 0.0 {
                new_speed = 0.0;
            }
            self.speed = new_speed;
            if self.is_serie1 {
                self.snd_open_end.update_target(SoundTarget::Start);
            } else {
                self.snd_open_end_2.update_target(SoundTarget::Start);
            }
        } else if new_pos < 0.0 {
            new_pos = 0.0;
            new_speed = -self.speed + self.reflection_close;
            if new_speed * self.speed > 0.0 {
                new_speed = 0.0;
            }
            self.speed = new_speed;
        }

        self.pos = new_pos;

        if self.pos < 0.1 {
            self.pos_x = (self.pos * 5.0 * PI).sin() * self.plug_radius;
            self.pos_y = 1.0 - (self.pos * 5.0 * PI).cos() * self.plug_radius;
        } else {
            self.pos_x = self.plug_radius;
            self.pos_y = (self.pos - 0.1) / 0.9 * self.shift + self.plug_radius;
        }

        self.anim_x.update_pos(self.pos_x);
        self.anim_y.update_pos(self.pos_y);
    }

    pub fn warn_tick(&mut self, target: bool, spannung: f32) {
        if target && self.state == DoorState::Closed {
            self.closed_while_warning = true;
        }
        if !target {
            self.closed_while_warning = false;
        }

        if target
            && self.sicherung
            && self.energy
            && !self.notentriegelung
            && !self.closed_while_warning
        {
            if self.warn_relais.tick() == 1 {
                self.snd_door_warn.update_target(SoundTarget::Loop);
            }
        } else {
            self.warn_relais.reset();
            self.snd_door_warn.update_target(SoundTarget::Stop);
            self.lm_warn_in = 0.0;
        }

        self.lm_warn_in = self.warn_relais.is_on as i32 as f32 * spannung;
    }

    pub fn tick(
        &mut self,
        modus: DoorTarget,
        door_1_btn: bool,
        new_notentriegelung: bool,
        haltewunsch: bool,
        kiwa_rolli: bool,
        lichtschranke_frei: bool,
        hand_delta_x: f32,
    ) {
        self.notentriegelung = new_notentriegelung;

        if kiwa_rolli && (modus == DoorTarget::Freigabe || modus == DoorTarget::Oeffnen) {
            self.kiwa_rolli = true;
        } else if !kiwa_rolli {
            self.kiwa_rolli = false;
        }

        let target_last = self.target;
        if self.energy && self.sicherung && !self.notentriegelung {
            if self.door_mode_last != modus || (door_1_btn && !self.door_1_last) {
                if door_1_btn {
                    self.auf_flag = false;
                }

                if modus == DoorTarget::Oeffnen {
                    self.new_mode = DoorTarget::Oeffnen;
                    self.auf_flag = self.state != DoorState::Open;
                } else if modus == DoorTarget::Freigabe {
                    if door_1_btn {
                        if self.state == DoorState::Open || self.target > 0 {
                            self.new_mode = DoorTarget::Zwangssschliessen;
                        } else if self.state == DoorState::Closed || self.target < 0 {
                            self.new_mode = DoorTarget::Oeffnen;
                        }
                    } else {
                        self.new_mode = DoorTarget::Freigabe;
                    }
                } else if modus == DoorTarget::Zu {
                    if door_1_btn {
                        if self.state == DoorState::Open || self.target > 0 {
                            self.new_mode = DoorTarget::Zwangssschliessen;
                            self.auf_flag = false;
                        } else if self.state == DoorState::Closed || self.target < 0 {
                            self.new_mode = DoorTarget::Oeffnen;
                        }
                    } else {
                        self.new_mode = DoorTarget::Zu;
                    }
                }

                if (self.new_mode == DoorTarget::Freigabe && (self.kiwa_rolli || haltewunsch))
                    || self.new_mode == DoorTarget::Oeffnen
                {
                    self.target = 1;
                    self.auf_flag = self.state != DoorState::Open;
                } else if self.new_mode == DoorTarget::Zu {
                    self.target = 0;
                } else if self.new_mode == DoorTarget::Zwangssschliessen {
                    self.target = -1;
                }

                self.close_timer = 0.0;
            } else if self.target == 0 {
                if !self.auf_flag {
                    if self.new_mode == DoorTarget::Freigabe
                        && self.state == DoorState::Open
                        && lichtschranke_frei
                    {
                        self.close_timer += 1.0;
                        if self.close_timer > CLOSE_TIME {
                            self.target = -1;
                        }
                    } else if self.new_mode == DoorTarget::Zu && self.state == DoorState::Open {
                        self.close_timer += 1.0;
                        if self.close_timer > CLOSE_TIME_ZW {
                            self.target = -1;
                        }
                    } else if self.new_mode == DoorTarget::Zu && self.state == DoorState::Other {
                        self.target = -1;
                        self.close_timer = 0.0;
                    } else if self.new_mode == DoorTarget::Zwangssschliessen {
                        self.target = -1;
                        self.close_timer = 0.0;
                    } else {
                        self.close_timer = 0.0;
                    }
                }

                if (self.new_mode == DoorTarget::Freigabe && (self.kiwa_rolli || haltewunsch))
                    || self.new_mode == DoorTarget::Oeffnen
                {
                    self.close_timer = 0.0;
                    self.target = 1;
                    self.auf_flag = self.state != DoorState::Open;
                }

                if self.notentriegelung_last {
                    self.target = -1;
                }
            } else if self.target == -1 {
                if (self.new_mode == DoorTarget::Freigabe && (!lichtschranke_frei || haltewunsch))
                    || (self.new_mode == DoorTarget::Zu && !lichtschranke_frei)
                {
                    self.close_timer = 0.0;
                    self.target = 1;
                    self.auf_flag = self.state != DoorState::Open;
                }
            } else {
                self.close_timer = 0.0;
            }

            if self.kiwa_rolli && self.state == DoorState::Open {
                self.target = self.target.max(0);
                self.close_timer = 0.0;
            }

            if self.state == DoorState::Closed
                && self.new_mode == DoorTarget::Zwangssschliessen
                && modus == DoorTarget::Freigabe
            {
                self.new_mode = modus.clone();
            }

            self.notentriegelung_last = self.notentriegelung;
        } else {
            self.target = 0;
            self.close_timer = 0.0;
        }

        if target_last != self.target {
            if self.is_serie1 {
                self.snd_open_start.update_target(SoundTarget::Stop);
                self.snd_open_end.update_target(SoundTarget::Stop);
                self.snd_close_start.update_target(SoundTarget::Stop);
                self.snd_door_close.update_target(SoundTarget::Stop);
                self.snd_close_end.update_target(SoundTarget::Stop);
            } else {
                self.snd_open_start_2.update_target(SoundTarget::Stop);
                self.snd_open_end_2.update_target(SoundTarget::Stop);
                self.snd_close_start_2.update_target(SoundTarget::Stop);
                self.snd_door_close_2.update_target(SoundTarget::Stop);
                self.snd_close_end_2.update_target(SoundTarget::Stop);
            }
        }

        if self.notentriegelung || !(self.energy && self.sicherung && self.pos > 0.01) {
            if self.grabbing_a {
                self.pos = (self.pos - hand_delta_x * delta()).min(1.0).max(0.0);
            } else if self.grabbing_b {
                self.pos = (self.pos + hand_delta_x * delta()).min(1.0).max(0.0);
            }
        }

        if (self.speed == 0.0)
            && ((self.target > 0 && self.pos >= 1.0) || (self.target < 0 && self.pos <= 0.0))
        {
            self.target = 0;
            self.speed = 0.0;
        }

        if !self.energy && !self.sicherung {
            let a = if self.speed > 0.0 {
                -self.friction
            } else if self.speed < 0.0 {
                self.friction
            } else {
                0.0
            };
            self.move_door(a);
        }

        if self.target > 0 || self.auf_flag {
            if self.pos < 0.01 && self.speed <= 0.0 {
                if self.is_serie1 {
                    self.snd_open_start.update_target(SoundTarget::Start);
                } else {
                    self.snd_open_start_2.update_target(SoundTarget::Start);
                }
            }

            let v_soll = if self.pos < self.open_start_end_change_pos {
                self.open_start_speed
            } else {
                self.open_end_speed
            };

            self.move_door(v_soll - self.speed * self.traction_stiftness);
        }

        if self.target < 0 {
            if self.pos > 0.99 && self.speed >= 0.0 {
                if self.is_serie1 {
                    self.snd_close_start.update_target(SoundTarget::Start);
                } else {
                    self.snd_close_start_2.update_target(SoundTarget::Start);
                }
            }

            let v_soll = if self.pos > self.close_start_end_change_pos {
                -self.close_start_speed
            } else {
                -self.close_end_speed
            };

            self.move_door(v_soll - self.speed * self.traction_stiftness);
        }

        if self.pos == 1.0 {
            self.state = DoorState::Open;
            self.auf_flag = false;
        } else if self.pos < 0.005 {
            self.state = DoorState::Closed;
        } else {
            self.state = DoorState::Other;
        }

        self.door_mode_last = modus;
        self.door_1_last = door_1_btn;
    }
}

impl PassengerDoor for AegElectricDoor {
    fn state(&self) -> &DoorState {
        &self.state
    }

    fn closed(&self) -> bool {
        self.state == DoorState::Closed
    }
}
