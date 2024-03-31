//use rand::Rng;

use lotus_script::delta;

use crate::{
    elements::std_elements::piecewise_linear_function::PiecewiseLinearFunction,
    mocks::{animation::Animation, sound::Sound, vehicle_door::VehicleDoor},
    structs::{
        internal_enums::{DoorState, DoorTarget, SoundTarget},
        traits::PassengerDoor,
    },
};

const FALTTUER_SCHLIESSZEIT: f32 = 3.0;
const FALTTUER_MOTOR_SPEED: f32 = 0.5;
const FALTTUER_MOTOR_DELAY: f32 = 2.0;
const FALTTUER_PI: f32 = 3.1415;

#[derive(Default, Debug)]
pub struct DueWagFalttuer {
    _name_id: String,

    wing_a_pos: f32,
    wing_b_pos: f32,

    wing_a_anim: Animation,
    wing_b_anim: Animation,

    wing_a_speed: f32,
    wing_b_speed: f32,

    state: DoorState,

    motor_rot: f32,
    motor_rot_last: f32,
    motor_pos: f32,
    motor_correct_delay: f32,
    motor_move: i32,

    lichtschranke_aktiv: bool,

    motor_endlage_auf: bool,
    motor_endlage_zu: bool,

    closing_timer: f32,
    speed_varianz: f32,

    fluegel_a_endlage_auf: bool,
    fluegel_a_endlage_zu: bool,
    fluegel_b_endlage_auf: bool,
    fluegel_b_endlage_zu: bool,

    kv_wing_dampfung: PiecewiseLinearFunction,
    wing_handforce: f32,
    wing_bump_factor_open_end: f32,
    wing_bump_factor_close_end: f32,

    snd_started: bool,
    snd_open: Sound,
    snd_close: Sound,
    snd_reverse: Sound,
    snd_hand: Sound,

    pass_door_1: VehicleDoor,
    pass_door_2: VehicleDoor,

    grabbing_aa: bool,
    grabbing_ab: bool,
    grabbing_ba: bool,
    grabbing_bb: bool,
}

impl DueWagFalttuer {
    pub fn new(name: String, door_index_1: usize, door_index_2: usize) -> Self {
        //let mut rng = rand::thread_rng();

        let mut tuer = DueWagFalttuer {
            _name_id: name.clone(),
            wing_a_anim: Animation::new(format!("{}_a_anim", name)),
            wing_b_anim: Animation::new(format!("{}_b_anim", name)),
            //	Speed_Varianz: rng.gen_range(0.8..=1.2),
            speed_varianz: 1.0,
            kv_wing_dampfung: PiecewiseLinearFunction::new(),
            wing_handforce: 0.1,
            wing_bump_factor_open_end: 0.15,
            wing_bump_factor_close_end: 0.1,
            motor_endlage_zu: true,

            snd_open: Sound::new(format!("snd_{}_open", name)),
            snd_close: Sound::new(format!("snd_{}_close", name)),
            snd_reverse: Sound::new(format!("snd_{}_reverse", name)),
            snd_hand: Sound::new(format!("snd_{}_hand", name)),

            pass_door_1: VehicleDoor::new(door_index_1, true, true),
            pass_door_2: VehicleDoor::new(door_index_2, true, true),

            fluegel_a_endlage_zu: true,
            fluegel_b_endlage_zu: true,
            ..Default::default()
        };

        tuer.kv_wing_dampfung.add_pair(0.0, 0.3);
        tuer.kv_wing_dampfung.add_pair(1.0, 0.3);

        tuer
    }

    pub fn handmove(&mut self, move_delta: f32) {
        if self.grabbing_aa || self.grabbing_ab {
            if self.grabbing_aa {
                self.wing_a_pos = (self.wing_a_pos + move_delta).min(1.0).max(0.0);
                self.wing_a_speed = move_delta * self.wing_handforce / delta();
            } else {
                self.wing_a_pos = (self.wing_a_pos + move_delta).min(1.0).max(0.0);
                self.wing_a_speed = -move_delta * self.wing_handforce / delta();
            }
        } else {
            self.wing_a_pos = self.wing_a_pos + self.wing_a_speed * delta();

            if self.wing_a_pos > 1.0 {
                self.wing_a_pos = 1.0;
                self.wing_a_speed = -self.wing_bump_factor_open_end * self.wing_a_speed;
            }

            if self.wing_a_pos < 0.0 {
                self.wing_a_pos = 0.0;
                self.wing_a_speed = -self.wing_bump_factor_close_end * self.wing_a_speed;
            }

            if self.wing_a_speed != 0.0 {
                let new_speed_a = self.wing_a_speed
                    + (-self.wing_a_speed.signum()
                        * self.kv_wing_dampfung.get_value(self.wing_a_pos))
                        * delta();

                if new_speed_a * self.wing_a_speed < 0.0 {
                    self.wing_a_speed = 0.0;
                } else {
                    self.wing_a_speed = new_speed_a;
                }
            }
        }

        if self.grabbing_ba || self.grabbing_bb {
            if self.grabbing_ba {
                self.wing_b_pos = (self.wing_b_pos + move_delta).min(1.0).max(0.0);
                self.wing_b_speed = move_delta * self.wing_handforce / delta();
            } else {
                self.wing_b_pos = (self.wing_b_pos + move_delta).min(1.0).max(0.0);
                self.wing_b_speed = -move_delta * self.wing_handforce / delta();
            }
        } else {
            self.wing_b_pos = self.wing_b_pos + self.wing_b_speed * delta();

            if self.wing_b_pos > 1.0 {
                self.wing_b_pos = 1.0;
                self.wing_b_speed = -self.wing_bump_factor_open_end * self.wing_b_speed;
            }

            if self.wing_b_pos < 0.0 {
                self.wing_b_pos = 0.0;
                self.wing_b_speed = -self.wing_bump_factor_close_end * self.wing_b_speed;
            }

            if self.wing_b_speed != 0.0 {
                let new_speed_b = self.wing_b_speed
                    + (-self.wing_b_speed.signum()
                        * self.kv_wing_dampfung.get_value(self.wing_b_pos))
                        * delta();

                if new_speed_b * self.wing_b_speed < 0.0 {
                    self.wing_b_speed = 0.0;
                } else {
                    self.wing_b_speed = new_speed_b;
                }
            }
        }
    }

    pub fn tick(
        &mut self,
        request: bool,
        occupied: bool,
        stuermodus: bool,
        ansteuerung: DoorTarget,
        direkt_ansteuerung: bool,
        bat_spannung_norm: f32,
    ) {
        if bat_spannung_norm < 0.5 {
            return;
        }

        if self.motor_move == 0
            && (direkt_ansteuerung
                || (ansteuerung == DoorTarget::Oeffnen && !self.motor_endlage_auf)
                || (ansteuerung == DoorTarget::Freigabe && request && !self.motor_endlage_auf))
        {
            self.motor_move = 1;
        }

        if ansteuerung == DoorTarget::Oeffnen
            || (ansteuerung == DoorTarget::Freigabe && request)
            || (self.motor_endlage_auf && self.closing_timer <= 0.0)
        {
            self.closing_timer = FALTTUER_SCHLIESSZEIT;
        }

        if self.closing_timer > 0.0 && self.motor_endlage_auf {
            self.closing_timer -= delta();
        }

        if ansteuerung == DoorTarget::Zwangssschliessen {
            self.closing_timer = -1.0;
        }

        // Tür zu laufen lassen, wenn der Timer abgelaufen ist
        if self.motor_move == 0
            && ansteuerung >= DoorTarget::Freigabe
            && self.closing_timer <= 0.0
            && !stuermodus
            && !self.motor_endlage_zu
        {
            self.motor_move = 1;
        }

        if self.motor_move == 0
            && ((self.motor_endlage_auf
                && (!self.fluegel_a_endlage_auf || !self.fluegel_b_endlage_auf))
                || (self.motor_endlage_zu
                    && (!self.fluegel_a_endlage_zu || !self.fluegel_b_endlage_zu)))
            && !stuermodus
            && self.motor_correct_delay <= 0.0
        {
            self.motor_correct_delay = FALTTUER_MOTOR_DELAY;
        }

        if self.motor_correct_delay > 0.0 {
            self.motor_correct_delay -= delta();
            if self.motor_correct_delay <= 0.0 {
                self.motor_move = 1;
            }
        }

        // Tür reversieren, wenn sie gerade zu läuft (Motor_Move und Rot ungerade)
        if self.motor_move == 1
            && (occupied && self.lichtschranke_aktiv)
            && !stuermodus
            && (self.motor_rot.trunc() % 2.0 != 0.0)
        {
            self.motor_move = -1;
            self.snd_close.update_target(SoundTarget::Stop);
        }

        // Motor bewegen (Auf)
        if self.motor_move > 0 {
            if !self.snd_started && self.motor_endlage_auf {
                self.snd_started = true;
                self.snd_close.update_target(SoundTarget::Start);
            }

            if !self.snd_started && self.motor_endlage_zu {
                self.snd_started = true;
                self.snd_open.update_target(SoundTarget::Start);
            }

            self.motor_rot_last = self.motor_rot;
            self.motor_rot += self.speed_varianz * FALTTUER_MOTOR_SPEED * delta();
            self.motor_endlage_auf = false;
            self.motor_endlage_zu = false;
        }

        // Motor bewegen (Reversieren)
        if self.motor_move < 0 {
            self.motor_rot_last = self.motor_rot;
            self.motor_rot -= self.speed_varianz * FALTTUER_MOTOR_SPEED * delta();
            self.motor_endlage_auf = false;
            self.motor_endlage_zu = false;
        }

        // Endlage festgestellt
        if self.motor_rot.trunc() != self.motor_rot_last.trunc() {
            self.motor_move = 0;

            self.motor_rot = self.motor_rot.round();

            self.snd_started = false;

            self.motor_endlage_auf = self.motor_rot.trunc() % 2.0 != 0.0;
            self.motor_endlage_zu = self.motor_rot.trunc() % 2.0 == 0.0;
        }

        // Motor_Pos_last := Motor_Pos;
        let motor_pos_last = self.motor_pos;
        self.motor_pos = (1.0 - (FALTTUER_PI * self.motor_rot).cos()) / 2.0;

        // Türflügel an den Motor anpassen
        let motor_pos_delta = self.motor_pos - motor_pos_last;

        if (self.wing_a_pos - motor_pos_last).abs() < 0.02 {
            self.wing_a_pos += motor_pos_delta;
        } else if (self.wing_a_pos - motor_pos_last).abs() > 0.1 {
            self.wing_a_pos += motor_pos_delta * 0.2;
        } else {
            self.wing_a_pos += motor_pos_delta * 0.75;
        }

        if (self.wing_b_pos - motor_pos_last).abs() < 0.02 {
            self.wing_b_pos += motor_pos_delta;
        } else if (self.wing_b_pos - motor_pos_last).abs() > 0.1 {
            self.wing_b_pos += motor_pos_delta * 0.2;
        } else {
            self.wing_b_pos += motor_pos_delta * 0.75;
        }

        self.wing_a_pos = self.wing_a_pos.min(1.0).max(0.0);
        self.wing_b_pos = self.wing_b_pos.min(1.0).max(0.0);

        if self.fluegel_a_endlage_auf && self.fluegel_b_endlage_auf {
            self.lichtschranke_aktiv = true;
        } else if self.motor_pos < 0.4 {
            self.lichtschranke_aktiv = false;
        }

        self.fluegel_a_endlage_auf = self.wing_a_pos > 0.975;
        self.fluegel_a_endlage_zu = self.wing_a_pos < 0.025;

        self.fluegel_b_endlage_auf = self.wing_b_pos > 0.975;
        self.fluegel_b_endlage_zu = self.wing_b_pos < 0.025;

        self.wing_a_anim.update_pos(self.wing_a_pos);
        self.wing_b_anim.update_pos(self.wing_b_pos);

        self.state = if self.fluegel_a_endlage_zu && self.fluegel_b_endlage_zu {
            DoorState::Closed
        } else if self.fluegel_a_endlage_auf && self.fluegel_b_endlage_auf {
            DoorState::Open
        } else {
            DoorState::Other
        };

        self.pass_door_1.update_open(self.state == DoorState::Open);
        self.pass_door_2.update_open(self.state == DoorState::Open);

        self.pass_door_1
            .update_released(ansteuerung == DoorTarget::Freigabe);
        self.pass_door_2
            .update_released(ansteuerung == DoorTarget::Freigabe);
    }
}

impl PassengerDoor for DueWagFalttuer {
    fn state(&self) -> &DoorState {
        &self.state
    }

    fn closed(&self) -> bool {
        self.state == DoorState::Closed
    }
}
