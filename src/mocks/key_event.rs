use std::fmt;

use lotus_script::action::state;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEventCab {
    None,
    ACab,
    BCab,
    Indifferent,
}

impl Default for KeyEventCab {
    fn default() -> Self {
        KeyEventCab::None
    }
}

impl fmt::Display for KeyEventCab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            KeyEventCab::None => write!(f, "X"),
            KeyEventCab::ACab => write!(f, "0"),
            KeyEventCab::BCab => write!(f, "1"),
            KeyEventCab::Indifferent => write!(f, "X"),
        }
    }
}

#[derive(Debug)]
pub struct KeyEvent {
    name: String,
    cab_side: KeyEventCab,
    injection: bool,
    injection_last: bool,
}

impl KeyEvent {
    pub fn new(name: String, cab_side: KeyEventCab) -> Self {
        Self {
            name: name,
            cab_side: cab_side,
            injection: false,
            injection_last: false,
        }
    }

    fn matching_cab(&self) -> bool {
        match state(&self.name).cockpit_index {
            Some(index) => match self.cab_side {
                KeyEventCab::None => true,
                KeyEventCab::ACab => index == 0,
                KeyEventCab::BCab => index == 1,
                KeyEventCab::Indifferent => true,
            },
            None => true,
        }
    }

    pub fn set_injection(&mut self, value: bool) {
        self.injection = value;
    }

    pub fn is_just_pressed(&mut self) -> bool {
        let result = (state(&self.name).kind.is_just_pressed() && self.matching_cab())
            || (self.injection && !self.injection_last);
        self.injection_last = self.injection;
        result
    }
    pub fn is_just_released(&mut self) -> bool {
        let result = (state(&self.name).kind.is_just_released() && self.matching_cab())
            || (!self.injection && self.injection_last);
        self.injection_last = self.injection;
        result
    }
    pub fn is_pressed(&self) -> bool {
        (state(&self.name).kind.is_pressed() && self.matching_cab()) || self.injection
    }
    pub fn is_released(&self) -> bool {
        (state(&self.name).kind.is_released() && self.matching_cab()) && !self.injection
    }
}
