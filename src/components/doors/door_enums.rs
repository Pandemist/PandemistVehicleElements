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
