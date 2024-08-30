use lotus_script::delta;

use crate::{
    mocks::{
        animation::Animation,
        generell::mouse_move,
        key_event::{KeyEvent, KeyEventCab},
        sound::{Sound, SoundTarget},
    },
    structs::enums::{SimpleState, SwitchingTarget},
};

#[derive(Debug)]
pub struct WagenautomatElectric {
    id: usize,

    state: SimpleState,
    switching_timer: f32,

    target_last: SwitchingTarget,

    einlegen_start: Sound,
    einlegen: Sound,
    auslegen: Sound,
}

impl WagenautomatElectric {
    pub fn new(id: usize, init_state: SimpleState) -> Self {
        Self {
            id: id,
            state: init_state,
            einlegen_start: Sound::new(format!("snd_wagenautomat_{}_start", id)),
            einlegen: Sound::new(format!("snd_wagenautomat_{}_on", id)),
            auslegen: Sound::new(format!("snd_wagenautomat_{}_off", id)),
            switching_timer: 0.0,
            target_last: SwitchingTarget::Neutral,
        }
    }

    pub fn tick(&mut self, target: SwitchingTarget, inlay_block: bool) {
        match target {
            SwitchingTarget::Einlegen(delay) => {
                if (self.target_last != target) && !inlay_block {
                    self.einlegen_start.update_target(SoundTarget::Start);
                }
                self.switching_timer += delta();
                if self.switching_timer > delay {
                    if self.state == SimpleState::Off && !inlay_block {
                        self.einlegen.update_target(SoundTarget::Start);
                        self.state = SimpleState::On;
                    }
                }
            }
            SwitchingTarget::Auslegen(delay) => {
                self.switching_timer += delta();
                if self.switching_timer > delay {
                    if self.state == SimpleState::On {
                        self.auslegen.update_target(SoundTarget::Start);
                        self.state = SimpleState::Off;
                    }
                }
            }
            _ => {
                self.switching_timer = 0.0;
            }
        }
        self.target_last = target;
    }

    pub fn auslegen(&mut self) {
        if self.state == SimpleState::On {
            self.auslegen.update_target(SoundTarget::Start);
            self.state = SimpleState::Off;
        }
    }
}

#[derive(Debug)]
pub struct WagenautomatManual {
    id: usize,

    state: SimpleState,

    slider: f32,

    key_grab: KeyEvent,

    slider_anim: Animation,
    state_anim: Animation,

    einlegen_start: Sound,
    einlegen: Sound,
    auslegen: Sound,
}

impl WagenautomatManual {
    pub fn new(id: usize, cab_side: KeyEventCab, init_state: SimpleState) -> Self {
        Self {
            id: id,
            state: init_state,
            slider_anim: Animation::new(format!("wagenautomat_{}_slider", id)),
            state_anim: Animation::new(format!("wagenautomat_{}_state", id)),

            key_grab: KeyEvent::new(format!("wagenautomat_{}_grab", id), cab_side),

            einlegen_start: Sound::new(format!("snd_wagenautomat_{}_start", id)),
            einlegen: Sound::new(format!("snd_wagenautomat_{}_on", id)),
            auslegen: Sound::new(format!("snd_wagenautomat_{}_off", id)),
            slider: 0.0,
        }
    }

    pub fn tick(&mut self, inlay_block: bool, remote_target: SwitchingTarget) {
        let slider_min = inlay_block.into();

        let slider_last = self.slider;

        // Automat bewegen
        if self.key_grab.is_pressed() {
            self.slider = (self.slider + (mouse_move().x * 350.0))
                .max(1.0)
                .min(slider_min);
        }
        self.slider_anim.set(self.slider);

        // Remote Target auswerten
        let bool_remote_target = match remote_target {
            SwitchingTarget::Einlegen(_) => true,
            _ => false,
        };

        // Automat händisch einlegen
        if (self.slider <= 0.1 && slider_last > 0.1) || bool_remote_target {
            if self.state == SimpleState::Off {
                self.einlegen.update_target(SoundTarget::Start);
                self.state = SimpleState::On;
            }
        }

        // Automat händisch auslegen
        if self.slider > 0.1 && slider_last <= 0.1 {
            self.auslegen.update_target(SoundTarget::Start);
        }

        let state_anim_target = match self.state {
            SimpleState::On => 1.0,
            SimpleState::Off => 0.0,
        };
        self.state_anim.set(state_anim_target);
    }

    pub fn auslegen(&mut self) {
        if self.state == SimpleState::On {
            self.auslegen.update_target(SoundTarget::Start);
            self.state = SimpleState::Off;
        }
    }
}
