use lotus_script::delta;

use crate::{
    elements::{general::rollerblind::Rollerblind, tech::slider::GravitySlider},
    mocks::{
        key_event::{KeyEvent, KeyEventCab},
        sound::Sound,
    },
};

#[derive(Debug)]
pub struct KursschildZiffer {
    name: String,
    speed: f32,
    anz_pos: i32,

    display_id: f32,

    blind: Rollerblind,

    snd_drehen: Sound,

    key_plus: KeyEvent,
    key_minus: KeyEvent,
}

impl KursschildZiffer {
    pub fn new(
        name: &str,
        sound_id: &str,
        cab: KeyEventCab,
        speed: f32,
        user_id: i32,
        base_sub_id: i32,
        anz_pos: i32,
    ) -> Self {
        Self {
            name: name.to_string(),
            speed: speed,
            anz_pos: anz_pos,

            display_id: 0.0,

            blind: Rollerblind::new(&format!("tex_{}", name), user_id, base_sub_id),

            snd_drehen: Sound::new(format!("snd_{}_drehen", sound_id)),
            key_plus: KeyEvent::new(format!("{}_plus", name), cab),
            key_minus: KeyEvent::new(format!("{}_minus", name), cab),
        }
    }

    pub fn set(&mut self, id: i32) {
        self.display_id = id as f32;
    }

    pub fn tick(&mut self) {
        if self.key_plus.is_pressed() {
            self.display_id = (self.display_id + (self.speed * delta())).min(self.anz_pos as f32);
        }
        if self.key_minus.is_pressed() {
            self.display_id = (self.display_id - (self.speed * delta())).max(0.0);
        }

        self.blind.tick(self.display_id);
    }
}

#[derive(Debug)]
pub struct Kursschild {
    id: usize,
    cab: KeyEventCab,
    klappe: GravitySlider,

    ziffer_10: KursschildZiffer,
    ziffer_01: KursschildZiffer,
}

impl Kursschild {
    pub fn new(id: usize, cab: KeyEventCab) -> Self {
        Self {
            id: id,
            cab: cab,
            klappe: GravitySlider::new("kursschild_klappe", cab, 0.2, 2.4, 0.02),
            ziffer_10: KursschildZiffer::new(
                &format!("kursschild_10_{}", cab),
                &format!("kursschild_{}", cab),
                cab,
                1.5,
                5749281,
                10020,
                10,
            ),
            ziffer_01: KursschildZiffer::new(
                &format!("kursschild_01_{}", cab),
                &format!("kursschild_{}", cab),
                cab,
                1.5,
                5749281,
                10020,
                10,
            ),
        }
    }

    pub fn tick(&mut self) {
        self.klappe.tick();

        self.ziffer_10.tick();
        self.ziffer_01.tick();
    }
}
