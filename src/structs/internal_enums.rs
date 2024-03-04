// Zustand der Kupplung
#[derive(Debug, Clone)]
pub enum CouplingState {
    Deactivated,
    Ready,
    Coupled,
}

// Startzustand des Fahrzeugs
#[derive(Debug, Clone)]
pub enum VehicleInitState {
    ColdAndDark,
    Setuped,
    ReadyToDrive,
}

// Zustand an der Stromschiene
#[derive(Debug, Clone, PartialEq)]
pub enum ThirdRailState {
    Disconnnected,
    PartwiseConnected,
    Connected,
}

#[derive(Debug, Clone)]
pub enum ThirdRailSide {
    Left,
    Right,
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

// Startposition des Spielers
#[derive(Debug, Clone)]
pub enum PlayerInitPos {
    FrontCab,
    BackCab,
    NotHere,
}

//Gleisqualität
#[derive(Debug, Clone)]
pub enum Railquality {
    Even,
    Uneven,
    EvenWithCenterPiece,
    UnevenWithCenterPiece,
    Flat,
    VeryEven,
    DisortedEven,
    DisortedUneven,
}

// Oberflächentyp
#[derive(Debug, Clone)]
pub enum Surfacetype {
    Ballast,
    Road,
    Grass,
}

// Soundansteuerung
#[derive(Debug)]
pub enum SoundTarget {
    Start,
    Stop,
    Loop,
    Pause,
    Idle,
}

impl Default for SoundTarget {
    fn default() -> Self {
        SoundTarget::Idle
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DoorTarget {
    Zwangssschliessen,
    Zu,
    Freigabe,
    Oeffnen,
}

impl Default for DoorTarget {
    fn default() -> Self {
        DoorTarget::Zu
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DoorSide {
    None,
    Left,
    Right,
    Both,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DoorState {
    Closed,
    Other,
    Open,
}

impl Default for DoorState {
    fn default() -> Self {
        DoorState::Other
    }
}
