use lotus_extra::rand::gen_f32;
use lotus_script::time::delta;

use crate::{
    api::{
        animation::Animation, electrical_supply::ApiThirdRailCollector, light::Light,
        mock_enums::ThirdRailState, simulation_settings::realisitc_electric_supply, sound::Sound,
    },
    management::enums::{
        general_enums::Side, state_enums::SwitchingState, target_enums::SwitchingTarget,
    },
};

#[derive(Debug)]
pub struct ThirdRailCollectorBuilder {
    id: usize,
    side: Side,

    spark_time: f32,
    spark_variance: f32,

    spark_on_connect: bool,
    spark_on_disconnect: bool,

    move_up_speed: f32,
    move_down_speed: f32,

    motor_relais: SwitchingState,
    motor_swiching_timer: f32,
    spark_timer: f32,
    third_rail_state_last: ThirdRailState,

    pos: f32,

    anim: Animation,

    motor_target: SwitchingTarget,
    /// Normalized voltage output (0.0 to 1.0)
    voltage_norm: f32,

    state: SwitchingState,

    api_thirdrailcollector: ApiThirdRailCollector,

    snd_on: Sound,
    snd_off: Sound,
    snd_anlauf: Sound,
    snd_ablauf: Sound,

    spark: Light,
}

impl ThirdRailCollectorBuilder {
    pub fn add_saprk(
        mut self,
        spark_light_name: Option<&str>,
        time: f32,
        variance: f32,
        on_connect: bool,
        on_disconnect: bool,
    ) -> Self {
        self.spark = Light::new(spark_light_name);
        self.spark_time = time;
        self.spark_variance = variance;
        self.spark_on_connect = on_connect;
        self.spark_on_disconnect = on_disconnect;
        self
    }

    pub fn move_up_speed(mut self, speed: f32) -> Self {
        self.move_up_speed = speed;
        self
    }

    pub fn move_down_speed(mut self, speed: f32) -> Self {
        self.move_down_speed = speed;
        self
    }

    pub fn snd_on(mut self, name: impl Into<String>) -> Self {
        self.snd_on = Sound::new_simple(Some(&name.into()));
        self
    }

    pub fn snd_off(mut self, name: impl Into<String>) -> Self {
        self.snd_off = Sound::new_simple(Some(&name.into()));
        self
    }

    pub fn snd_anlauf(mut self, name: impl Into<String>) -> Self {
        self.snd_anlauf = Sound::new_simple(Some(&name.into()));
        self
    }

    pub fn snd_ablauf(mut self, name: impl Into<String>) -> Self {
        self.snd_ablauf = Sound::new_simple(Some(&name.into()));
        self
    }

    pub fn init(mut self, applied: bool) -> Self {
        if applied {
            self.state = SwitchingState::On;
            self.pos = 1.0;
            self.anim.set(self.pos);
        }

        self
    }

    pub fn build(self) -> ThirdRailCollector {
        ThirdRailCollector {
            id: self.id,
            side: self.side,
            spark_time: self.spark_time,
            spark_variance: self.spark_variance,
            spark_on_connect: self.spark_on_connect,
            spark_on_disconnect: self.spark_on_disconnect,
            move_up_speed: self.move_up_speed,
            move_down_speed: self.move_down_speed,
            motor_relais: self.motor_relais,
            motor_swiching_timer: self.motor_swiching_timer,
            spark_timer: self.spark_timer,
            third_rail_state_last: self.third_rail_state_last,
            pos: self.pos,
            anim: self.anim,
            motor_target: self.motor_target,
            voltage_norm: self.voltage_norm,
            state: self.state,
            api_thirdrailcollector: self.api_thirdrailcollector,
            snd_on: self.snd_on,
            snd_off: self.snd_off,
            snd_anlauf: self.snd_anlauf,
            snd_ablauf: self.snd_ablauf,
            spark: self.spark,
        }
    }
}

//====================================================

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

    motor_relais: SwitchingState,
    motor_swiching_timer: f32,
    spark_timer: f32,
    third_rail_state_last: ThirdRailState,

    pos: f32,

    anim: Animation,

    pub motor_target: SwitchingTarget,
    /// Normalized voltage output (0.0 to 1.0)
    pub voltage_norm: f32,

    state: SwitchingState,

    api_thirdrailcollector: ApiThirdRailCollector,

    snd_on: Sound,
    snd_off: Sound,
    snd_anlauf: Sound,
    snd_ablauf: Sound,

    spark: Light,
}

impl ThirdRailCollector {
    pub fn builder(
        animation_name: impl Into<String>,
        id: usize,
        side: Side,
    ) -> ThirdRailCollectorBuilder {
        ThirdRailCollectorBuilder {
            id,
            side,
            spark_time: 0.0,
            spark_variance: 0.0,
            spark_on_connect: false,
            spark_on_disconnect: false,
            move_up_speed: 1.0,
            move_down_speed: 1.0,
            motor_relais: SwitchingState::Off,
            motor_swiching_timer: 0.0,
            spark_timer: 0.0,
            third_rail_state_last: ThirdRailState::Disconnnected,
            pos: 0.0,
            anim: Animation::new(Some(&animation_name.into())),
            motor_target: SwitchingTarget::Neutral,
            voltage_norm: 0.0,
            state: SwitchingState::Off,
            api_thirdrailcollector: ApiThirdRailCollector::new(id, side),
            snd_on: Sound::new_simple(None),
            snd_off: Sound::new_simple(None),
            snd_anlauf: Sound::new_simple(None),
            snd_ablauf: Sound::new_simple(None),
            spark: Light::new(None),
        }
    }

    pub fn tick(&mut self, power_usage: bool, battery: bool, safeguard: bool) {
        let target_last = self.motor_target.into();

        // Target auf Motortarget übertragen
        match self.motor_target {
            SwitchingTarget::TurnOn(delay) => {
                self.motor_swiching_timer += delta();
                if self.motor_swiching_timer > delay {
                    self.motor_relais = SwitchingState::On;
                }
            }
            SwitchingTarget::TurnOff(delay) => {
                self.motor_swiching_timer += delta();
                if self.motor_swiching_timer > delay {
                    self.motor_relais = SwitchingState::Off;
                }
            }
            SwitchingTarget::Neutral => {
                self.motor_swiching_timer = 0.0;
            }
        }

        // Target zurücksetzen, wenn keine battery oder keine Sicherung
        if !battery || !safeguard {
            self.motor_relais = SwitchingState::Neutral;
        }

        // Target zurücksetzen, wenn schon in Endlage
        match self.motor_relais {
            SwitchingState::On => {
                if self.pos >= 1.0 {
                    self.motor_relais = SwitchingState::Neutral;
                }
            }
            SwitchingState::Off => {
                if self.pos <= 0.0 {
                    self.motor_relais = SwitchingState::Neutral;
                }
            }
            SwitchingState::Neutral => {}
        }

        // Motor bewegen
        match self.motor_relais {
            SwitchingState::On => {
                self.pos = (self.pos + self.move_up_speed * delta()).min(1.0);
            }
            SwitchingState::Off => {
                self.pos = (self.pos - self.move_down_speed * delta()).max(0.0);
            }
            SwitchingState::Neutral => {}
        }

        // Bewegeungssound abspielen
        if self.motor_relais != target_last {
            match self.motor_relais {
                SwitchingState::On => {
                    self.snd_on.start();
                    self.snd_off.start();
                }
                SwitchingState::Off => {
                    self.snd_on.start();
                    self.snd_off.stop();
                }
                SwitchingState::Neutral => {
                    self.snd_on.stop();
                    self.snd_off.stop();
                }
            }
        }

        // Play sound & start spark gap
        if self.state == SwitchingState::On
            && self.api_thirdrailcollector.value() == ThirdRailState::PartwiseConnected
            && (self.third_rail_state_last == ThirdRailState::Connected)
        {
            self.snd_anlauf.start();

            if self.spark_on_connect && power_usage {
                self.spark_timer = gen_f32(
                    (self.spark_time - self.spark_variance)
                        ..=(self.spark_time + self.spark_variance),
                );
            }
        }

        // Play sound & start spark gap
        if self.state == SwitchingState::On
            && self.api_thirdrailcollector.value() == ThirdRailState::Connected
            && (self.third_rail_state_last == ThirdRailState::PartwiseConnected)
        {
            self.snd_ablauf.start();

            if self.spark_on_disconnect && power_usage {
                self.spark_timer = gen_f32(
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
        .into();

        // Spark setzen
        if self.spark_timer > 0.0 {
            self.spark.set_brightness(1.0);
        } else {
            self.spark.set_brightness(0.0);
        }

        self.spark_timer = (self.spark_timer - delta()).max(0.0);

        self.anim.set(self.pos);
    }
}
