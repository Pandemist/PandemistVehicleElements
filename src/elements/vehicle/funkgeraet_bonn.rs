use lotus_script::{content::ContentId, delta};

use crate::{
    elements::{
        std::helper::{b_to_f, get_random_element},
        tech::{buttons::PushButton, dekaden::ButtonDekadenschalter, slider::SliderY},
    },
    mocks::{
        key_event::KeyEventCab,
        light::Light,
        sound::{IndipendentSound, Sound, SoundTarget},
    },
};

#[derive(Debug)]
pub struct FunkgeraetBonn {
    timer_swb: f32,
    timer_kbe: f32,
    timer_kvb: f32,
    timer_drani: f32,

    ch_swb: Vec<(ContentId, bool)>,
    ch_hgk: Vec<(ContentId, bool)>,
    ch_kvb: Vec<(ContentId, bool)>,
    ch_dra: Vec<(ContentId, bool)>,
}

impl FunkgeraetBonn {
    pub fn new(cab: KeyEventCab) -> Self {
        let mut s = Self {
            timer_swb: 0.0,
            timer_kbe: 0.0,
            timer_kvb: 0.0,
            timer_drani: 0.0,

            ch_swb: Vec::new(),
            ch_hgk: Vec::new(),
            ch_kvb: Vec::new(),
            ch_dra: Vec::new(),
        };

        s.ch_swb.push((
            ContentId {
                user_id: 5749281,
                sub_id: 9400,
                version: 0.0,
            },
            true,
        ));
        s.ch_swb.push((
            ContentId {
                user_id: 5749281,
                sub_id: 9401,
                version: 0.0,
            },
            true,
        ));
        s.ch_swb.push((
            ContentId {
                user_id: 5749281,
                sub_id: 9405,
                version: 0.0,
            },
            true,
        ));
        s.ch_swb.push((
            ContentId {
                user_id: 5749281,
                sub_id: 9406,
                version: 0.0,
            },
            true,
        ));
        s.ch_swb.push((
            ContentId {
                user_id: 5749281,
                sub_id: 9407,
                version: 0.0,
            },
            true,
        ));
        s.ch_swb.push((
            ContentId {
                user_id: 5749281,
                sub_id: 9409,
                version: 0.0,
            },
            true,
        ));
        s.ch_swb.push((
            ContentId {
                user_id: 5749281,
                sub_id: 9412,
                version: 0.0,
            },
            true,
        ));
        s.ch_swb.push((
            ContentId {
                user_id: 5749281,
                sub_id: 9413,
                version: 0.0,
            },
            true,
        ));
        s.ch_swb.push((
            ContentId {
                user_id: 5749281,
                sub_id: 9414,
                version: 0.0,
            },
            true,
        ));
        s.ch_swb.push((
            ContentId {
                user_id: 5749281,
                sub_id: 9415,
                version: 0.0,
            },
            true,
        ));
        s.ch_swb.push((
            ContentId {
                user_id: 5749281,
                sub_id: 9516,
                version: 0.0,
            },
            true,
        ));

        s.ch_kvb.push((
            ContentId {
                user_id: 5749281,
                sub_id: 9402,
                version: 0.0,
            },
            true,
        ));
        s.ch_kvb.push((
            ContentId {
                user_id: 5749281,
                sub_id: 9408,
                version: 0.0,
            },
            true,
        ));
        s.ch_kvb.push((
            ContentId {
                user_id: 5749281,
                sub_id: 9410,
                version: 0.0,
            },
            true,
        ));

        s.ch_hgk.push((
            ContentId {
                user_id: 5749281,
                sub_id: 9402,
                version: 0.0,
            },
            true,
        ));
        s.ch_hgk.push((
            ContentId {
                user_id: 5749281,
                sub_id: 9408,
                version: 0.0,
            },
            true,
        ));
        s.ch_hgk.push((
            ContentId {
                user_id: 5749281,
                sub_id: 9410,
                version: 0.0,
            },
            true,
        ));

        s.ch_dra.push((
            ContentId {
                user_id: 5749281,
                sub_id: 9403,
                version: 0.0,
            },
            false,
        ));
        s.ch_dra.push((
            ContentId {
                user_id: 5749281,
                sub_id: 9404,
                version: 0.0,
            },
            false,
        ));
        s.ch_dra.push((
            ContentId {
                user_id: 5749281,
                sub_id: 9411,
                version: 0.0,
            },
            false,
        ));

        s
    }

    pub fn tick(&mut self, spannung: f32, battery: bool) {
        if self.timer_swb > 0.0 {
            self.timer_swb = self.timer_swb - delta();
        } else {
            let index = get_random_element(&self.ch_swb);
        }
    }
}

#[derive(Debug)]
pub struct FunkbediengeraetBonn {
    activ_channel: i32,
    is_hoeren: bool,

    funk_kennung: String,

    btn_send: PushButton,
    btn_swb: PushButton,
    btn_kvb: PushButton,
    btn_kbe: PushButton,
    btn_dra: PushButton,
    btn_notruf: PushButton,
    btn_anruf: PushButton,
    btn_belgt: PushButton,
    btn_hoeren: PushButton,

    volume_slider: SliderY,

    lm_swb: Light,
    lm_kvb: Light,
    lm_kbe: Light,
    lm_drani: Light,
    lm_hoeren: Light,

    dekade_1000: ButtonDekadenschalter,
    dekade_100: ButtonDekadenschalter,
    dekade_10: ButtonDekadenschalter,
    dekade_1: ButtonDekadenschalter,

    snd_send: Sound,
    snd_funkspruch: IndipendentSound,
}

impl FunkbediengeraetBonn {
    pub fn new(cab: KeyEventCab) -> Self {
        let mut s = Self {
            activ_channel: 0,
            is_hoeren: false,
            funk_kennung: "0000".to_string(),

            btn_send: PushButton::new(
                &format!("btn_funkgeraet_bonn_send_{}", cab),
                cab,
                "snd_funkgeraet_bonn_btn",
            ),
            btn_swb: PushButton::new(
                &format!("btn_funkgeraet_bonn_ch_swb_{}", cab),
                cab,
                "snd_funkgeraet_bonn_btn",
            ),
            btn_kvb: PushButton::new(
                &format!("btn_funkgeraet_bonn_ch_kvb_{}", cab),
                cab,
                "snd_funkgeraet_bonn_btn",
            ),
            btn_kbe: PushButton::new(
                &format!("btn_funkgeraet_bonn_ch_kbe_{}", cab),
                cab,
                "snd_funkgeraet_bonn_btn",
            ),
            btn_dra: PushButton::new(
                &format!("btn_funkgeraet_bonn_ch_dra_{}", cab),
                cab,
                "snd_funkgeraet_bonn_btn",
            ),
            btn_notruf: PushButton::new(
                &format!("btn_funkgeraet_bonn_notruf_{}", cab),
                cab,
                "snd_funkgeraet_bonn_btn",
            ),
            btn_anruf: PushButton::new(
                &format!("btn_funkgeraet_bonn_anruf_{}", cab),
                cab,
                "snd_funkgeraet_bonn_btn",
            ),
            btn_belgt: PushButton::new(
                &format!("btn_funkgeraet_bonn_belegt_{}", cab),
                cab,
                "snd_funkgeraet_bonn_btn",
            ),
            btn_hoeren: PushButton::new(
                &format!("btn_funkgeraet_bonn_hoeren_{}", cab),
                cab,
                "snd_funkgeraet_bonn_btn",
            ),
            volume_slider: SliderY::new(
                &format!("funkgeraet_bonn_vol_slider_{}", cab),
                cab,
                1.0 / 200.0,
            ),

            lm_swb: Light::new(format!("lm_funkgeraet_bonn_ch_swb_{}", cab)),
            lm_kvb: Light::new(format!("lm_funkgeraet_bonn_ch_kvb_{}", cab)),
            lm_kbe: Light::new(format!("lm_funkgeraet_bonn_ch_kbe_{}", cab)),
            lm_drani: Light::new(format!("lm_funkgeraet_bonn_ch_dran_{}", cab)),
            lm_hoeren: Light::new(format!("lm_funkgeraet_bonn_ch_hoeren_{}", cab)),

            dekade_1000: ButtonDekadenschalter::new(&format!("dekade_1000_{}", cab), cab, 3.5, 10),
            dekade_100: ButtonDekadenschalter::new(&format!("dekade_100_{}", cab), cab, 3.5, 10),
            dekade_10: ButtonDekadenschalter::new(&format!("dekade_10_{}", cab), cab, 3.5, 10),
            dekade_1: ButtonDekadenschalter::new(&format!("dekade_1_{}", cab), cab, 3.5, 10),

            snd_send: Sound::new(format!("snd_funkgeraet_bonn_send_{}", cab)),
            snd_funkspruch: IndipendentSound::new(format!(
                "snd_funkgeraet_bonn_funkspruch_{}",
                cab
            )),
        };

        s.volume_slider.pos = 1.0;

        s
    }

    pub fn tick(&mut self, spannung: f32, battery: bool, activ: bool) {
        self.lm_swb
            .update_light_level(b_to_f(self.activ_channel == 1) * spannung);
        self.lm_kbe
            .update_light_level(b_to_f(self.activ_channel == 2) * spannung);
        self.lm_kvb
            .update_light_level(b_to_f(self.activ_channel == 3) * spannung);
        self.lm_drani
            .update_light_level(b_to_f(self.activ_channel == 4) * spannung);
        self.lm_hoeren
            .update_light_level(b_to_f(self.is_hoeren) * spannung);

        self.btn_send.tick();
        self.btn_swb.tick();
        self.btn_kvb.tick();
        self.btn_kbe.tick();
        self.btn_dra.tick();
        self.btn_notruf.tick();
        self.btn_anruf.tick();
        self.btn_belgt.tick();
        self.btn_hoeren.tick();

        self.dekade_1000.tick();
        self.dekade_100.tick();
        self.dekade_10.tick();
        self.dekade_1.tick();

        self.volume_slider.tick();

        if self.btn_send.is_just_pressed() {
            self.snd_send.update_target(SoundTarget::Start);
        }

        if self.btn_hoeren.is_just_pressed() && battery {
            self.is_hoeren = !self.is_hoeren;
        }

        self.snd_funkspruch
            .update_volume(self.volume_slider.pos * b_to_f(activ));
        self.snd_send
            .update_volume(self.volume_slider.pos * b_to_f(activ));

        self.funk_kennung = format!(
            "{}{}{}{}",
            self.dekade_1000.value,
            self.dekade_100.value,
            self.dekade_10.value,
            self.dekade_1.value
        );
    }
}
