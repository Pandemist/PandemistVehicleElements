use std::fmt;

#[derive(Debug, Clone)]
pub enum Side {
    Left,
    Right,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Side::Left => write!(f, "L"),
            Side::Right => write!(f, "R"),
        }
    }
}

// Ansteuerung von elektirschen Anlagen
#[derive(Debug, Clone, PartialEq)]
pub enum SwitchingTarget {
    Einlegen(f32),
    Auslegen(f32),
    Neutral,
}

impl Default for SwitchingTarget {
    fn default() -> Self {
        SwitchingTarget::Neutral
    }
}

// Ansteuertarget
#[derive(Debug, Clone, PartialEq)]
pub enum SwitchingState {
    On,
    Off,
    Neutral,
}

impl Default for SwitchingState {
    fn default() -> Self {
        SwitchingState::Neutral
    }
}

// Ansteuertarget
#[derive(Debug, Clone, PartialEq)]
pub enum SimpleState {
    On,
    Off,
}

impl Default for SimpleState {
    fn default() -> Self {
        SimpleState::Off
    }
}
