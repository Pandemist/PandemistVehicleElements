use crate::api::{
    key_event::KeyEvent, light::BlinkRelais, sound::Sound, visible_flag::Visiblility,
};

const INTERCOM_TIME: f32 = 1.0;
const INTERCOM_TIME_HALF: f32 = INTERCOM_TIME / 2.0;

#[derive(Debug)]
pub struct IntercomGt6n {
    id: usize,

    active: bool,
    active_last: bool,
    confirmed: bool,

    flash_timer: BlinkRelais,

    lm_red: Visiblility,
    lm_green: Visiblility,
    lm_yellow: Visiblility,

    key_activate: KeyEvent,

    snd_talk: Sound,
}

impl IntercomGt6n {
    #[must_use]
    pub fn new(
        id: usize,
        red_light_name: impl Into<String>,
        green_light_name: impl Into<String>,
        yellow_light_name: impl Into<String>,
        event_name: impl Into<String>,
        sound_activated_name: impl Into<String>,
    ) -> Self {
        IntercomGt6n {
            id,
            active: false,
            active_last: false,
            confirmed: false,

            flash_timer: BlinkRelais::new(INTERCOM_TIME, INTERCOM_TIME_HALF, 0.0),

            lm_red: Visiblility::new(red_light_name.into()),
            lm_green: Visiblility::new(green_light_name.into()),
            lm_yellow: Visiblility::new(yellow_light_name.into()),

            key_activate: KeyEvent::new(Some(&event_name.into()), None),

            snd_talk: Sound::new_simple(Some(&sound_activated_name.into())),
        }
    }

    pub fn tick(&mut self, current_activ: Option<&usize>) {
        let mut other = false;
        let mut waiting = false;
        if let Some(curr) = current_activ {
            if *curr != self.id {
                self.confirmed = false;
            }

            other = *curr > 0 && *curr != self.id;
            waiting = *curr == self.id && !self.confirmed;
        }

        self.active_last = self.active;
        self.active = waiting || self.confirmed;

        if !self.active_last && self.active {
            self.snd_talk.start();
        }

        if !waiting {
            self.flash_timer.reset();
        } else {
            self.flash_timer.tick();
        }

        self.lm_red.set_visbility(other);
        self.lm_green.set_visbility(self.confirmed);
        self.lm_yellow
            .set_visbility(waiting && self.flash_timer.is_on);
    }

    pub fn pressed(&mut self, allowed: bool) -> bool {
        self.key_activate.is_just_pressed() && allowed
    }

    pub fn confirm(&mut self, current_activ: Option<usize>) {
        if let Some(curr) = current_activ {
            if curr == self.id {
                self.confirmed = true;
            }
        }
    }
}
