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
