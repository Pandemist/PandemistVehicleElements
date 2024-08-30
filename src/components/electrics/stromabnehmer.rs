use lotus_script::delta;

use crate::{
    elements::std::{helper::gen_f32_range, piecewise_linear_function::PiecewiseLinearFunction},
    mocks::{
        animation::Animation,
        electrical_supply::{ApiPantograph, ApiThirdRailCollector},
        light::Light,
        mock_enums::{ThirdRailState, VehicleInitState},
        settings::{init_ready_state, realisitc_electric_supply},
        sound::{Sound, SoundTarget},
    },
    structs::enums::{Side, SwitchingState, SwitchingTarget},
};

#[derive(Debug)]
pub struct Pantograph {
    id: usize,

    move_up_speed: f32,
    move_down_speed: f32,

    height_curve: PiecewiseLinearFunction,

    motor_target: SwitchingState,
    motor_swiching_timer: f32,
    current_wire_height: f32,
    current_wire_max_anim: f32,
    motor_pos: f32,

    panto_pos: f32,

    anim: Animation,

    voltage_norm: f32,

    state: SwitchingState,

    api_panto: ApiPantograph,

    snd_up: Sound,
    snd_down: Sound,
}

impl Pantograph {
    pub fn new(id: usize, curve: PiecewiseLinearFunction, move_up: f32, move_down: f32) -> Self {
        let mut panto = Pantograph {
            id: id,
            move_up_speed: move_up,
            move_down_speed: move_down,
            height_curve: curve,
            current_wire_height: 10.0,
            api_panto: ApiPantograph::new(id),
            anim: Animation::new(format!("pantograph_{}_anim", id)),
            snd_up: Sound::new(format!("snd_pantograph_{}_up", id)),
            snd_down: Sound::new(format!("snd_pantograph_{}_down", id)),
            motor_target: SwitchingState::Neutral,
            motor_swiching_timer: 0.0,
            current_wire_max_anim: 0.0,
            motor_pos: 0.0,
            panto_pos: 0.0,
            voltage_norm: 0.0,
            state: SwitchingState::Neutral,
        };

        match init_ready_state() {
            VehicleInitState::ColdAndDark => {
                panto.motor_pos = 0.0;
                panto.state = SwitchingState::Off;
            }
            _ => {
                panto.motor_pos = 1.0;
                panto.state = SwitchingState::On;
            }
        }

        panto
    }

    pub fn tick(
        &mut self,
        target: SwitchingTarget,
        cranc: SwitchingTarget,
        battery: bool,
        safeguard: bool,
    ) {
        let target_last = match target {
            SwitchingTarget::Einlegen(_) => SwitchingState::On,
            SwitchingTarget::Auslegen(_) => SwitchingState::Off,
            SwitchingTarget::Neutral => SwitchingState::Neutral,
        };

        if self.api_panto.panto() < 10.0 && self.api_panto.panto() > 1.0 {
            self.current_wire_height = self.api_panto.panto();
        }

        self.current_wire_max_anim = self.height_curve.get_value(self.current_wire_height);

        match target {
            SwitchingTarget::Einlegen(delay) => {
                self.motor_swiching_timer = self.motor_swiching_timer + delta();
                if self.motor_swiching_timer > delay {
                    self.motor_target = SwitchingState::On;
                }
            }
            SwitchingTarget::Auslegen(delay) => {
                self.motor_swiching_timer = self.motor_swiching_timer + delta();
                if self.motor_swiching_timer > delay {
                    self.motor_target = SwitchingState::Off;
                }
            }
            _ => {
                self.motor_swiching_timer = 0.0;
            }
        }

        if !battery || !safeguard {
            self.motor_target = SwitchingState::Neutral;
        }

        match self.motor_target {
            SwitchingState::On => {
                if self.panto_pos >= 1.0 {
                    self.motor_target = SwitchingState::Neutral;
                }
            }
            SwitchingState::Off => {
                if self.panto_pos <= 0.0 {
                    self.motor_target = SwitchingState::Neutral;
                }
            }
            _ => {}
        }

        if self.motor_target == SwitchingState::Neutral {
            match cranc {
                SwitchingTarget::Einlegen(factor) => {
                    self.motor_pos = (self.motor_pos + factor * delta()).min(1.0);
                }
                SwitchingTarget::Auslegen(factor) => {
                    self.motor_pos = (self.motor_pos - factor * delta()).max(0.0);
                }
                SwitchingTarget::Neutral => {}
            }
        }

        match self.motor_target {
            SwitchingState::On => {
                self.motor_pos = (self.motor_pos + self.move_up_speed * delta()).min(1.0);
            }
            SwitchingState::Off => {
                self.motor_pos = (self.motor_pos - self.move_down_speed * delta()).max(0.0);
            }
            SwitchingState::Neutral => {}
        }

        if self.motor_target != target_last {
            match self.motor_target {
                SwitchingState::On => {
                    self.snd_up.update_target(SoundTarget::Start);
                    self.snd_down.update_target(SoundTarget::Stop);
                }
                SwitchingState::Off => {
                    self.snd_up.update_target(SoundTarget::Stop);
                    self.snd_down.update_target(SoundTarget::Start);
                }
                SwitchingState::Neutral => {
                    self.snd_up.update_target(SoundTarget::Stop);
                    self.snd_down.update_target(SoundTarget::Stop);
                }
            }
        }

        if self.motor_pos >= self.current_wire_max_anim && self.motor_pos > 0.95 {
            self.state = SwitchingState::On;
        } else if self.motor_pos < 0.05 {
            self.state = SwitchingState::Off;
            self.current_wire_height = 10.0;
        } else {
            self.state = SwitchingState::Neutral;
        }

        self.voltage_norm = (self.state == SwitchingState::On
            && (self.api_panto.voltage() || !realisitc_electric_supply()))
            as i32 as f32;

        self.panto_pos = self.motor_pos;
        self.panto_pos = self.panto_pos.min(self.current_wire_height);
        self.anim.set(self.panto_pos);
    }
}

#[derive(Debug)]
pub struct ThirdRailCollector {
    id: usize,
    side: Side,

    spark_time: f32,
    spark_variance: f32,

    spark_on_connect: bool,
    spark_on_disconnect: bool,

    move_up_speed: f32,
    move_down_speed: f32,

    motor_target: SwitchingState,
    motor_swiching_timer: f32,
    spark_timer: f32,
    third_rail_state_last: ThirdRailState,

    pos: f32,

    anim: Animation,

    voltage_norm: f32,

    state: SwitchingState,

    api_thirdrailcollector: ApiThirdRailCollector,

    snd_on: Sound,
    snd_off: Sound,
    snd_anlauf: Sound,
    snd_ablauf: Sound,

    spark: Light,
}

impl ThirdRailCollector {
    pub fn new(
        id: usize,
        side: Side,
        spark_time: f32,
        spark_variance: f32,
        spark_on_connect: bool,
        spark_on_disconnect: bool,
        move_up: f32,
        move_down: f32,
    ) -> Self {
        let mut panto = ThirdRailCollector {
            id: id,
            side: side.clone(),

            spark_time: spark_time,
            spark_variance: spark_variance,

            spark_on_connect: spark_on_connect,
            spark_on_disconnect: spark_on_disconnect,

            move_up_speed: move_up,
            move_down_speed: move_down,

            motor_target: SwitchingState::Neutral,
            motor_swiching_timer: 0.0,
            spark_timer: 0.0,
            third_rail_state_last: ThirdRailState::Disconnnected,

            pos: 0.0,

            anim: Animation::new(format!("stromabnehmer_{}_anim", id)),

            voltage_norm: 0.0,

            state: SwitchingState::Neutral,

            api_thirdrailcollector: ApiThirdRailCollector::new(id, side),

            snd_on: Sound::new(format!("snd_stromabnehmer_{}_on", id)),
            snd_off: Sound::new(format!("snd_stromabnehmer_{}_off", id)),
            snd_anlauf: Sound::new(format!("snd_stromabnehmer_{}_anlauf", id)),
            snd_ablauf: Sound::new(format!("snd_stromabnehmer_{}_ablauf", id)),

            spark: Light::new(format!("light_stromabnehmer_{}", id)),
        };

        match init_ready_state() {
            VehicleInitState::ColdAndDark => {
                panto.state = SwitchingState::Off;
                panto.pos = 0.0;
            }
            _ => {
                panto.state = SwitchingState::On;
                panto.pos = 1.0;
            }
        }

        panto
    }

    pub fn tick(
        &mut self,
        target: SwitchingTarget,
        power_usage: bool,
        batterie: bool,
        safeguard: bool,
    ) {
        let target_last = match target {
            SwitchingTarget::Einlegen(_) => SwitchingState::On,
            SwitchingTarget::Auslegen(_) => SwitchingState::Off,
            SwitchingTarget::Neutral => SwitchingState::Neutral,
        };

        // Target auf Motortarget übertragen
        match target {
            SwitchingTarget::Einlegen(delay) => {
                self.motor_swiching_timer = self.motor_swiching_timer + delta();
                if self.motor_swiching_timer > delay {
                    self.motor_target = SwitchingState::On;
                }
            }
            SwitchingTarget::Auslegen(delay) => {
                self.motor_swiching_timer = self.motor_swiching_timer + delta();
                if self.motor_swiching_timer > delay {
                    self.motor_target = SwitchingState::Off;
                }
            }
            _ => {
                self.motor_swiching_timer = 0.0;
            }
        }

        // Target zurücksetzen, wenn keine Batterie oder keine Sicherung
        if !batterie || !safeguard {
            self.motor_target = SwitchingState::Neutral;
        }

        // Target zurücksetzen, wenn schon in Endlage
        match self.motor_target {
            SwitchingState::On => {
                if self.pos >= 1.0 {
                    self.motor_target = SwitchingState::Neutral;
                }
            }
            SwitchingState::Off => {
                if self.pos <= 0.0 {
                    self.motor_target = SwitchingState::Neutral;
                }
            }
            _ => {}
        }

        // Motor bewegen
        match self.motor_target {
            SwitchingState::On => {
                self.pos = (self.pos + self.move_up_speed * delta()).min(1.0);
            }
            SwitchingState::Off => {
                self.pos = (self.pos - self.move_down_speed * delta()).max(0.0);
            }
            SwitchingState::Neutral => {}
        }

        // Bewegeungssound abspielen
        if self.motor_target != target_last {
            match self.motor_target {
                SwitchingState::On => {
                    self.snd_on.update_target(SoundTarget::Start);
                    self.snd_off.update_target(SoundTarget::Stop);
                }
                SwitchingState::Off => {
                    self.snd_on.update_target(SoundTarget::Stop);
                    self.snd_off.update_target(SoundTarget::Start);
                }
                SwitchingState::Neutral => {
                    self.snd_on.update_target(SoundTarget::Stop);
                    self.snd_off.update_target(SoundTarget::Stop);
                }
            }
        }

        // Stromschienenanlauf Sound abspielen & Abreißfunke starten
        if self.state == SwitchingState::On
            && self.api_thirdrailcollector.value() == ThirdRailState::PartwiseConnected
            && (self.third_rail_state_last == ThirdRailState::Connected)
        {
            self.snd_anlauf.update_target(SoundTarget::Start);

            if self.spark_on_connect && power_usage {
                self.spark_timer = gen_f32_range(
                    (self.spark_time - self.spark_variance)
                        ..=(self.spark_time + self.spark_variance),
                );
            }
        }

        // Stromschienenauflauf Sound abspielen & Abreißfunke starten
        if self.state == SwitchingState::On
            && self.api_thirdrailcollector.value() == ThirdRailState::Connected
            && (self.third_rail_state_last == ThirdRailState::PartwiseConnected)
        {
            self.snd_ablauf.update_target(SoundTarget::Start);

            if self.spark_on_disconnect && power_usage {
                self.spark_timer = gen_f32_range(
                    (self.spark_time - self.spark_variance)
                        ..=(self.spark_time + self.spark_variance),
                );
            }
        }
        self.third_rail_state_last = self.api_thirdrailcollector.value();

        // State setzen
        if self.pos > 0.95 {
            self.state = SwitchingState::On;
        } else if self.pos < 0.05 {
            self.state = SwitchingState::Off;
        } else {
            self.state = SwitchingState::Neutral;
        }

        // Voltage aktualisieren
        self.voltage_norm = (self.state == SwitchingState::On
            && (self.api_thirdrailcollector.voltage() || !realisitc_electric_supply()))
            as i32 as f32;

        // Spark setzen
        if self.spark_timer > 0.0 {
            self.spark.update_light_level(1.0);
        } else {
            self.spark.update_light_level(0.0);
        }

        self.spark_timer = (self.spark_timer - delta()).max(0.0);

        self.anim.set(self.pos);
    }
}
