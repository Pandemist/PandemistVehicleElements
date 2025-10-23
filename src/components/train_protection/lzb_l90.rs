use lotus_script::{prelude::Message, time::delta};

use crate::{
    api::{key_event::KeyEventCab, light::Light},
    elements::tech::{buttons::PushButton, dekaden::DecadeSwitch},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum LzbMode {
    #[default]
    Off,
    Standby,
    Startup,
    Shutdown,
    StehendeUebernahme,
    StehendeUebergabe,
    FliegendeUebernahme,
    FliegendeUebergabe,
    AbgabeErfolgtWarteAufAus,
    InLzbStrecke,
    InLzbStreckeMitBremsen,
    AtStop,
    AbfertigungWarteAufFahrweg,
    AbfertigungBereit,
    Nothalt,
    ZugconfigConfirm,
}

//=====================================================

//=====================================================

pub struct LzbL90VehicleUnit {
    pub on_lzb_run: bool,

    pub display_target_speed: f32,
    pub sanding_blocked: bool,
}

impl LzbL90VehicleUnit {
    pub fn new() -> Self {
        Self {
            on_lzb_run: false,
            display_target_speed: 0.0,
            sanding_blocked: false,
        }
    }

    pub fn tick(&mut self) {}

    pub fn on_message(&mut self, _msg: Message) {}
}

impl Default for LzbL90VehicleUnit {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LzbL90Device {
    btn_si_ankuppeln: PushButton,
    btn_pruef_ankupeln: PushButton,
    btn_bu_stoerung: PushButton,
    btn_r2_c2: PushButton,
    btn_ein: PushButton,
    btn_bremsankuendigung: PushButton,
    btn_lfz_defekt: PushButton,
    btn_nothalt: PushButton,
    btn_alf: PushButton,
    btn_ssf: PushButton,
    btn_anz_test: PushButton,
    btn_r3_c4: PushButton,
    btn_start: PushButton,

    decade_10000: DecadeSwitch,
    decade_1000: DecadeSwitch,
    decade_100: DecadeSwitch,
    decade_10: DecadeSwitch,
    decade_1: DecadeSwitch,

    lm_si_ankuppeln: Light,
    lm_pruef_ankupeln: Light,
    lm_bu_stoerung: Light,
    lm_r2_c2: Light,
    lm_ein: Light,
    lm_bremsankuendigung: Light,
    lm_lfz_defekt: Light,
    lm_nothalt: Light,
    lm_alf: Light,
    lm_ssf: Light,
    lm_anz_test: Light,
    lm_r3_c4: Light,
    lm_start: Light,

    // Zustände
    mode: LzbMode,
    test_timer: f32,
}

impl LzbL90Device {
    #[expect(clippy::too_many_arguments)]
    pub fn new(
        btn_si_ankuppeln_name: impl Into<String>,
        btn_pruef_ankupeln_name: impl Into<String>,
        btn_bu_stoerung_name: impl Into<String>,
        btn_r2_c2_name: impl Into<String>,
        btn_ein_name: impl Into<String>,
        btn_bremsankuendigung_name: impl Into<String>,
        btn_lfz_defekt_name: impl Into<String>,
        btn_nothalt_name: impl Into<String>,
        btn_alf_name: impl Into<String>,
        btn_ssf_name: impl Into<String>,
        btn_anz_test_name: impl Into<String>,
        btn_r3_c4_name: impl Into<String>,
        btn_start_name: impl Into<String>,

        decade_10000_name: impl Into<String>,
        decade_1000_name: impl Into<String>,
        decade_100_name: impl Into<String>,
        decade_10_name: impl Into<String>,
        decade_1_name: impl Into<String>,

        lm_si_ankuppeln_name: impl Into<String>,
        lm_pruef_ankupeln_name: impl Into<String>,
        lm_bu_stoerung_name: impl Into<String>,
        lm_r2_c2_name: impl Into<String>,
        lm_ein_name: impl Into<String>,
        lm_bremsankuendigung_name: impl Into<String>,
        lm_lfz_defekt_name: impl Into<String>,
        lm_nothalt_name: impl Into<String>,
        lm_alf_name: impl Into<String>,
        lm_ssf_name: impl Into<String>,
        lm_anz_test_name: impl Into<String>,
        lm_r3_c4_name: impl Into<String>,
        lm_start_name: impl Into<String>,

        btn_si_ankuppeln_event_name: impl Into<String>,
        btn_pruef_ankupeln_event_name: impl Into<String>,
        btn_bu_stoerung_event_name: impl Into<String>,
        btn_r2_c2_event_name: impl Into<String>,
        btn_ein_event_name: impl Into<String>,
        btn_bremsankuendigung_event_name: impl Into<String>,
        btn_lfz_defekt_event_name: impl Into<String>,
        btn_nothalt_event_name: impl Into<String>,
        btn_alf_event_name: impl Into<String>,
        btn_ssf_event_name: impl Into<String>,
        btn_anz_test_event_name: impl Into<String>,
        btn_r3_c4_event_name: impl Into<String>,
        btn_start_event_name: impl Into<String>,

        snd_btn_name: impl Into<String>,

        cab_side: KeyEventCab,
    ) -> Self {
        let snd_btn_name: String = snd_btn_name.into();

        Self {
            btn_si_ankuppeln: PushButton::builder(
                btn_si_ankuppeln_name.into(),
                btn_si_ankuppeln_event_name.into(),
                Some(cab_side),
            )
            .snd_press(snd_btn_name.clone())
            .build(),
            btn_pruef_ankupeln: PushButton::builder(
                btn_pruef_ankupeln_name.into(),
                btn_pruef_ankupeln_event_name.into(),
                Some(cab_side),
            )
            .snd_press(snd_btn_name.clone())
            .build(),
            btn_bu_stoerung: PushButton::builder(
                btn_bu_stoerung_name.into(),
                btn_bu_stoerung_event_name.into(),
                Some(cab_side),
            )
            .snd_press(snd_btn_name.clone())
            .build(),
            btn_r2_c2: PushButton::builder(
                btn_r2_c2_name.into(),
                btn_r2_c2_event_name.into(),
                Some(cab_side),
            )
            .snd_press(snd_btn_name.clone())
            .build(),
            btn_ein: PushButton::builder(
                btn_ein_name.into(),
                btn_ein_event_name.into(),
                Some(cab_side),
            )
            .snd_press(snd_btn_name.clone())
            .build(),
            btn_bremsankuendigung: PushButton::builder(
                btn_bremsankuendigung_name.into(),
                btn_bremsankuendigung_event_name.into(),
                Some(cab_side),
            )
            .snd_press(snd_btn_name.clone())
            .build(),
            btn_lfz_defekt: PushButton::builder(
                btn_lfz_defekt_name.into(),
                btn_lfz_defekt_event_name.into(),
                Some(cab_side),
            )
            .snd_press(snd_btn_name.clone())
            .build(),
            btn_nothalt: PushButton::builder(
                btn_nothalt_name.into(),
                btn_nothalt_event_name.into(),
                Some(cab_side),
            )
            .snd_press(snd_btn_name.clone())
            .build(),
            btn_alf: PushButton::builder(
                btn_alf_name.into(),
                btn_alf_event_name.into(),
                Some(cab_side),
            )
            .snd_press(snd_btn_name.clone())
            .build(),
            btn_ssf: PushButton::builder(
                btn_ssf_name.into(),
                btn_ssf_event_name.into(),
                Some(cab_side),
            )
            .snd_press(snd_btn_name.clone())
            .build(),
            btn_anz_test: PushButton::builder(
                btn_anz_test_name.into(),
                btn_anz_test_event_name.into(),
                Some(cab_side),
            )
            .snd_press(snd_btn_name.clone())
            .build(),
            btn_r3_c4: PushButton::builder(
                btn_r3_c4_name.into(),
                btn_r3_c4_event_name.into(),
                Some(cab_side),
            )
            .snd_press(snd_btn_name.clone())
            .build(),
            btn_start: PushButton::builder(
                btn_start_name.into(),
                btn_start_event_name.into(),
                Some(cab_side),
            )
            .snd_press(snd_btn_name.clone())
            .build(),
            decade_10000: DecadeSwitch::builder(10, decade_10000_name.into(), Some(cab_side))
                .rotation_speed(0.5)
                .build(),
            decade_1000: DecadeSwitch::builder(10, decade_1000_name.into(), Some(cab_side))
                .rotation_speed(0.5)
                .build(),
            decade_100: DecadeSwitch::builder(10, decade_100_name.into(), Some(cab_side))
                .rotation_speed(0.5)
                .build(),
            decade_10: DecadeSwitch::builder(10, decade_10_name.into(), Some(cab_side))
                .rotation_speed(0.5)
                .build(),
            decade_1: DecadeSwitch::builder(10, decade_1_name.into(), Some(cab_side))
                .rotation_speed(0.5)
                .build(),

            lm_si_ankuppeln: Light::new(Some(&lm_si_ankuppeln_name.into())),
            lm_pruef_ankupeln: Light::new(Some(&lm_pruef_ankupeln_name.into())),
            lm_bu_stoerung: Light::new(Some(&lm_bu_stoerung_name.into())),
            lm_r2_c2: Light::new(Some(&lm_r2_c2_name.into())),
            lm_ein: Light::new(Some(&lm_ein_name.into())),
            lm_bremsankuendigung: Light::new(Some(&lm_bremsankuendigung_name.into())),
            lm_lfz_defekt: Light::new(Some(&lm_lfz_defekt_name.into())),
            lm_nothalt: Light::new(Some(&lm_nothalt_name.into())),
            lm_alf: Light::new(Some(&lm_alf_name.into())),
            lm_ssf: Light::new(Some(&lm_ssf_name.into())),
            lm_anz_test: Light::new(Some(&lm_anz_test_name.into())),
            lm_r3_c4: Light::new(Some(&lm_r3_c4_name.into())),
            lm_start: Light::new(Some(&lm_start_name.into())),

            mode: LzbMode::default(),
            test_timer: 0.0,
        }
    }

    pub fn tick(&mut self, activ: bool, spannung: f32) {
        self.btn_si_ankuppeln.tick();
        self.btn_pruef_ankupeln.tick();
        self.btn_bu_stoerung.tick();
        self.btn_r2_c2.tick();
        self.btn_ein.tick();
        self.btn_bremsankuendigung.tick();
        self.btn_lfz_defekt.tick();
        self.btn_nothalt.tick();
        self.btn_alf.tick();
        self.btn_ssf.tick();
        self.btn_anz_test.tick();
        self.btn_r3_c4.tick();
        self.btn_start.tick();

        self.decade_10000.tick(0.0);
        self.decade_1000.tick(0.0);
        self.decade_100.tick(0.0);
        self.decade_10.tick(0.0);
        self.decade_1.tick(0.0);

        if self.btn_anz_test.is_just_pressed() && self.test_timer < 0.0 {
            self.test_timer = 10.0;
        }

        if self.test_timer >= 0.0 {
            self.test_timer -= delta();
        }

        if spannung < 0.1 || !activ {
            self.test_timer = -1.0;
        }

        self.lm_si_ankuppeln.set_brightness(
            spannung * (self.test_timer > 0.06 && self.test_timer < 10.0) as u8 as f32,
        );
        self.lm_pruef_ankupeln.set_brightness(
            spannung * (self.test_timer > 0.44 && self.test_timer < 9.62) as u8 as f32,
        );
        self.lm_bu_stoerung.set_brightness(
            spannung * (self.test_timer > 0.82 && self.test_timer < 9.24) as u8 as f32,
        );
        self.lm_r2_c2.set_brightness(
            spannung * (self.test_timer > 1.2 && self.test_timer < 8.86) as u8 as f32,
        );
        self.lm_ein.set_brightness(
            spannung * (self.test_timer > 2.34 && self.test_timer < 7.72) as u8 as f32,
        );
        self.lm_bremsankuendigung.set_brightness(
            spannung * (self.test_timer > 2.72 && self.test_timer < 7.34) as u8 as f32,
        );
        self.lm_lfz_defekt.set_brightness(
            spannung * (self.test_timer > 3.86 && self.test_timer < 6.2) as u8 as f32,
        );
        self.lm_nothalt.set_brightness(
            spannung * (self.test_timer > 4.24 && self.test_timer < 5.82) as u8 as f32,
        );
        self.lm_alf.set_brightness(
            spannung * (self.test_timer > 1.58 && self.test_timer < 8.48) as u8 as f32,
        );
        self.lm_ssf.set_brightness(
            spannung * (self.test_timer > 1.96 && self.test_timer < 8.1) as u8 as f32,
        );
        self.lm_anz_test.set_brightness(
            spannung * (self.test_timer > 3.1 && self.test_timer < 6.96) as u8 as f32,
        );
        self.lm_r3_c4.set_brightness(
            spannung * (self.test_timer > 3.48 && self.test_timer < 6.58) as u8 as f32,
        );
        self.lm_start.set_brightness(
            spannung * (self.test_timer > 4.62 && self.test_timer < 5.44) as u8 as f32,
        );
    }

    pub fn on_message(&mut self, _msg: Message) {}
}
