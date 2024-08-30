use lotus_script::delta;

use crate::mocks::{
    key_event::{KeyEvent, KeyEventCab},
    sound::{Sound, SoundTarget},
    visible_flag::Visiblility,
};

const SPRECHSTELLE_TIME: f32 = 1.0;
const SPRECHSTELLE_TIME_HALF: f32 = SPRECHSTELLE_TIME / 2.0;

#[derive(Debug)]
pub struct SprechstelleGt6n {
    id: usize,

    aktiv: bool,
    aktiv_last: bool,
    confirmed: bool,

    blink_timer: f32,

    lm_rot: Visiblility,
    lm_grn: Visiblility,
    lm_glb: Visiblility,

    key_activate: KeyEvent,

    snd_talk: Sound,
}

impl SprechstelleGt6n {
    pub fn new(id: usize) -> Self {
        SprechstelleGt6n {
            id: id,
            aktiv: false,
            aktiv_last: false,
            confirmed: false,

            blink_timer: 0.0,

            lm_rot: Visiblility::new(format!("lm_sprechstelle_{}_rot", id)),
            lm_grn: Visiblility::new(format!("lm_sprechstelle_{}_grn", id)),
            lm_glb: Visiblility::new(format!("lm_sprechstelle_{}_glb", id)),

            key_activate: KeyEvent::new(
                format!("sprechstelle_{}_press", id),
                KeyEventCab::Indifferent,
            ),

            snd_talk: Sound::new(format!("snd_sprechstelle_{}_talk", id)),
        }
    }

    pub fn tick(&mut self, current_activ: Option<&usize>) {
        let mut other = false;
        let mut waiting = false;
        match current_activ {
            Some(curr) => {
                if *curr != self.id {
                    self.confirmed = false;
                }

                other = *curr > 0 && *curr != self.id;
                waiting = *curr == self.id && !self.confirmed;
            }
            None => {}
        }

        self.aktiv_last = self.aktiv;
        self.aktiv = waiting || self.confirmed;

        if !self.aktiv_last && self.aktiv {
            self.snd_talk.update_target(SoundTarget::Start);
        }

        if !waiting {
            self.blink_timer = 0.0;
        }

        self.blink_timer += delta();

        if self.blink_timer > SPRECHSTELLE_TIME {
            self.blink_timer -= SPRECHSTELLE_TIME;
        }

        self.lm_rot.set_visbility(other.into());
        self.lm_grn.set_visbility(self.confirmed.into());
        self.lm_glb
            .set_visbility((waiting && (self.blink_timer > SPRECHSTELLE_TIME_HALF)).into());
    }

    pub fn confirm(&mut self, current_activ: Option<usize>) {
        match current_activ {
            Some(curr) => {
                if curr == self.id {
                    self.confirmed = true;
                }
            }
            None => {}
        }
    }
}

#[derive(Debug)]
pub struct SprechstellenControllerGT6N {
    name: String,
    sprechstellen: Vec<SprechstelleGt6n>,
    current_activ: Option<usize>,
    call_queue_high: Vec<usize>,
    call_queue_low: Vec<usize>,
}

impl SprechstellenControllerGT6N {
    pub fn new(name: &str, anz_sprechstellen: usize) -> Self {
        let mut controller = Self {
            name: name.to_string(),
            sprechstellen: vec![],
            call_queue_high: vec![],
            call_queue_low: vec![],
            current_activ: None,
        };
        for i in 0..anz_sprechstellen {
            controller.sprechstellen.push(SprechstelleGt6n::new(i));
        }
        controller
    }

    // Todo: test ob Sperchstelle schon in der queue
    // Testen ob die Sprechstelle entfernt werden darf (nicht, wenn notbremse noch aktiv ist)

    pub fn quit_current(&mut self) {}

    fn clear_all(&mut self) {
        self.call_queue_high.clear();
        self.call_queue_low.clear();
    }

    pub fn tick(&mut self) {
        // bestimmte Current
        let current_activ = if !self.call_queue_high.is_empty() {
            self.call_queue_high.last()
        } else if !self.call_queue_low.is_empty() {
            self.call_queue_low.last()
        } else {
            None
        };

        for sprechstelle in self.sprechstellen.iter_mut() {
            sprechstelle.tick(current_activ);
        }
    }
}
