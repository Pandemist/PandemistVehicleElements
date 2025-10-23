use lotus_script::{
    prelude::{message_type, send_message, Message, MessageTarget},
    time::delta,
};
use serde::{Deserialize, Serialize};

use crate::{
    api::{key_event::KeyEventCab, light::Light},
    elements::tech::{buttons::PushButton, dekaden::DecadeSwitch},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MsgIndusi600mIndusiAcitve {
    pub side: KeyEventCab,
    pub value: bool,
}

message_type!(MsgIndusi600mIndusiAcitve, "Indusi600m", "IndusiAcitve");

//--------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MsgIndusi600mDeviceAcitve {
    pub side: KeyEventCab,
    pub value: bool,
}

message_type!(MsgIndusi600mDeviceAcitve, "Indusi600m", "DeviceAcitve");

//--------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MsgIndusi600mOverride {
    pub side: KeyEventCab,
    pub value: bool,
}

message_type!(MsgIndusi600mOverride, "Indusi600m", "Override");

//--------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MsgIndusi600mReset {
    pub side: KeyEventCab,
}

message_type!(MsgIndusi600mReset, "Indusi600m", "Reset");

//--------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MsgIndusi600mForcedBrake {
    pub side: KeyEventCab,
}

message_type!(MsgIndusi600mForcedBrake, "Indusi600m", "ForcedBrake");

//=====================================================

struct Indusi600mVehicleUnitSide {
    cab_side: KeyEventCab,
    sensor_id: Vec<u32>,
    active_signal_last: bool,
    active_last: bool,
}

impl Indusi600mVehicleUnitSide {
    pub fn new(sensor_id: Vec<u32>, cab_side: KeyEventCab) -> Option<Self> {
        if sensor_id.is_empty() {
            None
        } else {
            Some(Self {
                cab_side,
                sensor_id,
                active_signal_last: false,
                active_last: false,
            })
        }
    }

    pub fn tick(
        &mut self,
        active_signal: bool,
        triggered: bool,
        bypass_switch: bool,
        battery: bool,
    ) {
        if active_signal != self.active_signal_last {
            self.active_signal_last = active_signal;
            send_message(
                &(MsgIndusi600mDeviceAcitve {
                    value: active_signal,
                    side: self.cab_side,
                }),
                [MessageTarget::Broadcast {
                    across_couplings: false,
                    include_self: true,
                }],
            );
        }

        let indusi_active = active_signal && !triggered && !bypass_switch && battery;
        if indusi_active != self.active_last {
            self.active_last = indusi_active;

            send_message(
                &(MsgIndusi600mIndusiAcitve {
                    side: self.cab_side,
                    value: indusi_active,
                }),
                [MessageTarget::Broadcast {
                    across_couplings: false,
                    include_self: true,
                }],
            );
        }
    }

    fn trigger(&mut self) {
        send_message(
            &(MsgIndusi600mForcedBrake {
                side: self.cab_side,
            }),
            [MessageTarget::Broadcast {
                across_couplings: false,
                include_self: true,
            }],
        );
    }

    pub fn on_trigger(&mut self, sensor: u32, entering: bool) {
        for listen_sensor in &self.sensor_id {
            // TODO
        }
        // Send triggered to devices
    }
}

pub struct Indusi600mVehicleUnit {
    side_a: Option<Indusi600mVehicleUnitSide>,
    side_b: Option<Indusi600mVehicleUnitSide>,

    active_signal_a_last: bool,
    active_signal_b_last: bool,

    active_a_last: bool,
    active_b_last: bool,

    bypass_switch: bool,

    triggered: bool,

    overriden: i32,
}

impl Indusi600mVehicleUnit {
    pub fn new(side_a: Vec<u32>, side_b: Vec<u32>) -> Self {
        Self {
            side_a: Indusi600mVehicleUnitSide::new(side_a, KeyEventCab::ACab),
            side_b: Indusi600mVehicleUnitSide::new(side_b, KeyEventCab::BCab),

            active_signal_a_last: false,
            active_signal_b_last: false,

            active_a_last: false,
            active_b_last: false,

            bypass_switch: false,

            triggered: false,

            overriden: 0,
        }
    }

    pub fn tick(
        &mut self,
        active_signal_a: bool,
        active_signal_b: bool,
        bypass_switch: bool,
        battery: bool,
    ) {
        self.bypass_switch = bypass_switch;

        // Wenn beide Eingangssignale aktiv sind, wird eine Zwangsbremsung ausgelöst. Sicherheitsmaßnahme
        if !self.triggered && active_signal_a && active_signal_b {
            self.trigger();
        }

        if let Some(side) = &mut self.side_a {
            side.tick(
                active_signal_a && battery,
                self.triggered,
                bypass_switch,
                battery,
            );
        }
        if let Some(side) = &mut self.side_b {
            side.tick(
                active_signal_b && battery,
                self.triggered,
                bypass_switch,
                battery,
            );
        }

        if !battery {
            self.overriden = 0;
        }
    }

    fn trigger(&mut self) {
        if !self.triggered {
            self.triggered = true;
            if let Some(side) = &mut self.side_a {
                side.trigger();
            }
            if let Some(side) = &mut self.side_b {
                side.trigger();
            }
        }
    }

    pub fn is_triggered(&mut self) -> bool {
        self.triggered && !self.bypass_switch
    }

    pub fn on_message(&mut self, msg: Message) {
        msg.handle::<MsgIndusi600mOverride>(|m| {
            if m.value {
                self.overriden += 1;
            } else {
                self.overriden = (self.overriden - 1).max(0);
            }
            Ok(())
        })
        .expect("MsgIndusi600mOverride: message handle failed");

        msg.handle::<MsgIndusi600mReset>(|m| {
            self.triggered = false;
            Ok(())
        })
        .expect("MsgIndusi600mReset: message handle failed");
    }

    pub fn on_trigger(&mut self, sensor: u32, entering: bool) {
        if let Some(side) = &mut self.side_a {
            side.on_trigger(sensor, entering);
        }
        if let Some(side) = &mut self.side_b {
            side.on_trigger(sensor, entering);
        }
        // Send triggered to devices
    }
}

//--------------------------

pub struct Indusi600mDeviceDecade {
    cab_side: KeyEventCab,

    device_active: bool,
    indusi_active: bool,

    btn_rueckstellen: PushButton,
    btn_freigabe: PushButton,

    rueckstellung_dekade_10: DecadeSwitch,
    rueckstellung_dekade_01: DecadeSwitch,
    freigabe_dekade_10: DecadeSwitch,
    freigabe_dekade_01: DecadeSwitch,

    lm_reuckstellung_btn: Light,
    lm_freigabe_btn: Light,
    lm_ziffern: Light,
    lm_betriebskontrolle: Light,

    triggered: bool,
    triggered_last: bool,

    trigger_sperr_time: f32,
    trigger_sperr_timer: f32,

    override_time: f32,
    override_timer: f32,

    was_pressed_while_not_triggered: bool,
}

impl Indusi600mDeviceDecade {
    #[expect(clippy::too_many_arguments)]
    pub fn new(
        btn_rueckstellen_anim_name: impl Into<String>,
        btn_freigabe_anim_name: impl Into<String>,

        btn_rueckstellen_event_name: impl Into<String>,
        btn_freigabe_event_name: impl Into<String>,

        rueckstellung_dekade_10_name: impl Into<String>,
        rueckstellung_dekade_01_name: impl Into<String>,
        freigabe_dekade_10_name: impl Into<String>,
        freigabe_dekade_01_name: impl Into<String>,

        lm_reuckstellung_btn_name: impl Into<String>,
        lm_freigabe_btn_name: impl Into<String>,
        lm_ziffern_name: impl Into<String>,
        lm_betriebskontrolle_name: impl Into<String>,

        snd_btn_press_name: impl Into<String>,
        snd_btn_release_name: impl Into<String>,

        cab_side: KeyEventCab,
        init_value_rueckstellung: i32,
        init_value_freigabe: i32,
    ) -> Self {
        let snd_btn_press = snd_btn_press_name.into();
        let snd_btn_release = snd_btn_release_name.into();

        Self {
            cab_side,

            device_active: false,
            indusi_active: false,

            btn_rueckstellen: PushButton::builder(
                btn_rueckstellen_anim_name.into(),
                btn_rueckstellen_event_name.into(),
                Some(cab_side),
            )
            .snd_press(snd_btn_press.clone())
            .snd_release(snd_btn_release.clone())
            .build(),

            btn_freigabe: PushButton::builder(
                btn_freigabe_anim_name.into(),
                btn_freigabe_event_name.into(),
                Some(cab_side),
            )
            .snd_press(snd_btn_press.clone())
            .snd_release(snd_btn_release.clone())
            .build(),

            rueckstellung_dekade_10: DecadeSwitch::builder(
                10,
                rueckstellung_dekade_10_name.into(),
                Some(cab_side),
            )
            .rotation_speed(1.5)
            .init_value((init_value_rueckstellung / 10) as f32)
            .build(),
            rueckstellung_dekade_01: DecadeSwitch::builder(
                10,
                rueckstellung_dekade_01_name.into(),
                Some(cab_side),
            )
            .rotation_speed(1.5)
            .init_value((init_value_rueckstellung % 10) as f32)
            .build(),

            freigabe_dekade_10: DecadeSwitch::builder(
                10,
                freigabe_dekade_10_name.into(),
                Some(cab_side),
            )
            .rotation_speed(1.5)
            .init_value((init_value_freigabe / 10) as f32)
            .build(),
            freigabe_dekade_01: DecadeSwitch::builder(
                10,
                freigabe_dekade_01_name.into(),
                Some(cab_side),
            )
            .rotation_speed(1.5)
            .init_value((init_value_freigabe % 10) as f32)
            .build(),

            lm_reuckstellung_btn: Light::new(Some(&lm_reuckstellung_btn_name.into())),
            lm_freigabe_btn: Light::new(Some(&lm_freigabe_btn_name.into())),
            lm_ziffern: Light::new(Some(&lm_ziffern_name.into())),
            lm_betriebskontrolle: Light::new(Some(&lm_betriebskontrolle_name.into())),

            triggered: false,
            triggered_last: false,

            trigger_sperr_time: 10.0,
            trigger_sperr_timer: -1.0,

            override_time: 20.0,
            override_timer: -1.0,

            was_pressed_while_not_triggered: false,
        }
    }

    pub fn tick(&mut self, rueckstellen_allowed: bool, override_allowed: bool, voltage: f32) {
        // Beleuchtung
        self.lm_betriebskontrolle
            .set_brightness((self.device_active && self.indusi_active) as u8 as f32 * voltage);
        self.lm_freigabe_btn.set_brightness(
            (self.device_active && self.override_timer > 0.0) as u8 as f32 * voltage,
        );
        self.lm_reuckstellung_btn
            .set_brightness((self.device_active && self.triggered) as u8 as f32 * voltage);

        self.lm_ziffern.set_brightness(
            (self.device_active && self.was_pressed_while_not_triggered) as u8 as f32 * voltage,
        );

        self.btn_rueckstellen.tick();
        self.btn_freigabe.tick();

        // Batterie aus
        if voltage < 0.5 {
            self.trigger_sperr_timer = -1.0;
            self.override_timer = -1.0;
        }

        if !self.device_active {
            self.was_pressed_while_not_triggered = false;
            return;
        }

        // Ab hier nur ausführen, wenn das Gerät aktiv geschaltet ist

        // Ziffern Licht
        if self.btn_rueckstellen.is_just_pressed() && !self.triggered && voltage > 0.5 {
            self.was_pressed_while_not_triggered = true;
        }
        self.was_pressed_while_not_triggered =
            self.was_pressed_while_not_triggered && self.btn_rueckstellen.is_pressed();

        // Rückstellung
        if self.trigger_sperr_timer > 0.0 {
            self.trigger_sperr_timer -= delta();
        }

        let add_target = if self.triggered && !self.triggered_last {
            0.5
        } else if self.triggered
            && self.trigger_sperr_timer < 0.0
            && self.btn_rueckstellen.value(rueckstellen_allowed)
        {
            self.triggered = false;
            send_message(
                &(MsgIndusi600mReset {
                    side: self.cab_side,
                }),
                [MessageTarget::Broadcast {
                    across_couplings: false,
                    include_self: true,
                }],
            );
            0.5
        } else {
            0.0
        };
        self.triggered_last = self.triggered;

        self.rueckstellung_dekade_10
            .tick(self.rueckstellung_dekade_01.tick(add_target) as f32);

        // Freigabe
        let timer_last = self.override_timer;

        if self.override_timer > 0.0 && voltage > 0.5 {
            self.override_timer -= delta();
        }

        let add_target = if timer_last > 0.0 && self.override_timer <= 0.0 {
            send_message(
                &(MsgIndusi600mOverride {
                    side: self.cab_side,
                    value: false,
                }),
                [MessageTarget::Broadcast {
                    across_couplings: false,
                    include_self: true,
                }],
            );
            0.5
        } else if !self.triggered
            && self.override_timer < 0.0
            && self.btn_freigabe.value(override_allowed)
        {
            self.override_timer = self.override_time;
            send_message(
                &(MsgIndusi600mOverride {
                    side: self.cab_side,
                    value: true,
                }),
                [MessageTarget::Broadcast {
                    across_couplings: false,
                    include_self: true,
                }],
            );
            0.5
        } else {
            0.0
        };

        self.freigabe_dekade_10
            .tick(self.freigabe_dekade_01.tick(add_target) as f32);
    }

    pub fn on_message(&mut self, msg: Message) {
        msg.handle::<MsgIndusi600mForcedBrake>(|m| {
            if self.cab_side == m.side {
                self.trigger_sperr_timer = self.trigger_sperr_time;
                self.triggered = true;
            }
            Ok(())
        })
        .expect("MsgIndusi600mForcedBrake: message handle failed");

        msg.handle::<MsgIndusi600mDeviceAcitve>(|m| {
            if self.cab_side == m.side {
                self.device_active = m.value;
            }
            Ok(())
        })
        .expect("MsgIndusi600mDeviceAcitve: message handle failed");

        msg.handle::<MsgIndusi600mIndusiAcitve>(|m| {
            self.indusi_active = m.value;
            Ok(())
        })
        .expect("MsgIndusi600mIndusiAcitve: message handle failed");
    }
}

//--------------------------

pub struct Indusi600mDeviceLcd {
    cab_side: KeyEventCab,

    device_active: bool,
    indusi_active: bool,

    btn_rueckstellen: PushButton,
    btn_freigabe: PushButton,

    rueckstellung_value: i32,
    freigabe_value: i32,

    display_rueckstellung: String,
    display_freigabe: String,

    lm_reuckstellung_btn: Light,
    lm_freigabe_btn: Light,
    lm_betriebskontrolle: Light,

    triggered: bool,
    triggered_last: bool,

    trigger_sperr_time: f32,
    trigger_sperr_timer: f32,

    override_time: f32,
    override_timer: f32,
}

impl Indusi600mDeviceLcd {
    #[expect(clippy::too_many_arguments)]
    pub fn new(
        btn_rueckstellen_anim_name: impl Into<String>,
        btn_freigabe_anim_name: impl Into<String>,

        btn_rueckstellen_event_name: impl Into<String>,
        btn_freigabe_event_name: impl Into<String>,

        snd_btn_press_name: impl Into<String>,
        snd_btn_release_name: impl Into<String>,

        lm_reuckstellung_btn_name: impl Into<String>,
        lm_freigabe_btn_name: impl Into<String>,
        lm_betriebskontrolle_name: impl Into<String>,

        cab_side: KeyEventCab,
        init_value_rueckstellung: i32,
        init_value_freigabe: i32,
    ) -> Self {
        let snd_btn_press = snd_btn_press_name.into();
        let snd_btn_release = snd_btn_release_name.into();

        Self {
            cab_side,

            device_active: false,
            indusi_active: false,

            btn_rueckstellen: PushButton::builder(
                btn_rueckstellen_anim_name.into(),
                btn_rueckstellen_event_name.into(),
                Some(cab_side),
            )
            .snd_press(snd_btn_press.clone())
            .snd_release(snd_btn_release.clone())
            .build(),

            btn_freigabe: PushButton::builder(
                btn_freigabe_anim_name.into(),
                btn_freigabe_event_name.into(),
                Some(cab_side),
            )
            .snd_press(snd_btn_press.clone())
            .snd_release(snd_btn_release.clone())
            .build(),

            rueckstellung_value: init_value_rueckstellung,
            freigabe_value: init_value_freigabe,

            display_rueckstellung: "".to_string(),
            display_freigabe: "".to_string(),

            lm_reuckstellung_btn: Light::new(Some(&lm_reuckstellung_btn_name.into())),
            lm_freigabe_btn: Light::new(Some(&lm_freigabe_btn_name.into())),
            lm_betriebskontrolle: Light::new(Some(&lm_betriebskontrolle_name.into())),

            triggered: false,
            triggered_last: false,

            trigger_sperr_time: 10.0,
            trigger_sperr_timer: -1.0,

            override_time: 20.0,
            override_timer: -1.0,
        }
    }

    pub fn tick(&mut self, rueckstellen_allowed: bool, override_allowed: bool, voltage: f32) {
        // Beleuchtung
        self.lm_betriebskontrolle
            .set_brightness((self.device_active && self.indusi_active) as u8 as f32 * voltage);
        self.lm_freigabe_btn.set_brightness(
            (self.device_active && self.override_timer > 0.0) as u8 as f32 * voltage,
        );
        self.lm_reuckstellung_btn
            .set_brightness((self.device_active && self.triggered) as u8 as f32 * voltage);

        if self.device_active && voltage > 0.5 {
            self.display_rueckstellung = format!("{}", self.rueckstellung_value);
            self.display_freigabe = format!("{}", self.freigabe_value);
        } else {
            self.display_rueckstellung = "".to_string();
            self.display_freigabe = "".to_string();
        }

        self.btn_rueckstellen.tick();
        self.btn_freigabe.tick();

        // Batterie aus
        if voltage < 0.5 {
            self.trigger_sperr_timer = -1.0;
            self.override_timer = -1.0;
        }

        if !self.device_active {
            return;
        }

        // Ab hier nur ausführen, wenn das Gerät aktiv geschaltet ist

        // Rückstellung
        if self.trigger_sperr_timer > 0.0 {
            self.trigger_sperr_timer -= delta();
        }

        if self.triggered && !self.triggered_last {
            //    0.5
        } else if self.triggered
            && self.trigger_sperr_timer < 0.0
            && self.btn_rueckstellen.value(rueckstellen_allowed)
        {
            self.triggered = false;
            send_message(
                &(MsgIndusi600mReset {
                    side: self.cab_side,
                }),
                [MessageTarget::Broadcast {
                    across_couplings: false,
                    include_self: true,
                }],
            );
            //    0.5
            self.rueckstellung_value += 1;
        }
        self.triggered_last = self.triggered;

        // Freigabe
        let timer_last = self.override_timer;

        if self.override_timer > 0.0 && voltage > 0.5 {
            self.override_timer -= delta();
        }

        if timer_last > 0.0 && self.override_timer <= 0.0 {
            send_message(
                &(MsgIndusi600mOverride {
                    side: self.cab_side,
                    value: false,
                }),
                [MessageTarget::Broadcast {
                    across_couplings: false,
                    include_self: true,
                }],
            );
            //0.5
            self.freigabe_value += 1;
        } else if !self.triggered
            && self.override_timer < 0.0
            && self.btn_freigabe.value(override_allowed)
        {
            self.override_timer = self.override_time;
            send_message(
                &(MsgIndusi600mOverride {
                    side: self.cab_side,
                    value: true,
                }),
                [MessageTarget::Broadcast {
                    across_couplings: false,
                    include_self: true,
                }],
            );
            //0.5
        }
    }

    pub fn on_message(&mut self, msg: Message) {
        msg.handle::<MsgIndusi600mForcedBrake>(|m| {
            if self.cab_side == m.side {
                self.trigger_sperr_timer = self.trigger_sperr_time;
                self.triggered = true;
            }
            Ok(())
        })
        .expect("MsgIndusi600mForcedBrake: message handle failed");

        msg.handle::<MsgIndusi600mDeviceAcitve>(|m| {
            if self.cab_side == m.side {
                self.device_active = m.value;
            }
            Ok(())
        })
        .expect("MsgIndusi600mDeviceAcitve: message handle failed");

        msg.handle::<MsgIndusi600mIndusiAcitve>(|m| {
            self.indusi_active = m.value;
            Ok(())
        })
        .expect("MsgIndusi600mIndusiAcitve: message handle failed");
    }
}
