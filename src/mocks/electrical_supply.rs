use crate::structs::enums::Side;
use lotus_script::var::VariableType;

use super::mock_enums::ThirdRailState;

#[derive(Debug)]
pub struct ApiPantograph {
    id: usize,
}

impl ApiPantograph {
    pub fn new(new_id: usize) -> Self {
        ApiPantograph { id: new_id }
    }

    // Entspricht der Variable panto_voltage_{a}
    pub fn voltage(&self) -> bool {
        bool::get(&format!("panto_voltage_{}", self.id))
    }

    // Entspricht der Variable panto_{a}
    pub fn panto(&self) -> f32 {
        f32::get(&format!("panto_{}", self.id))
    }
}

#[derive(Debug)]
pub struct ApiThirdRailCollector {
    id: usize,
    side: Side,
}

impl ApiThirdRailCollector {
    pub fn new(new_id: usize, new_side: Side) -> Self {
        ApiThirdRailCollector {
            id: new_id,
            side: new_side,
        }
    }

    // Position der Stromschiene am V_ThirdRailCollector_{b}_{L/R} (Nur Wertebereich -1, -0.5, 0/1)
    pub fn value(&self) -> ThirdRailState {
        match f32::get(&format!("V_ThirdRailCollector_{}_{}", self.id, self.side)) {
            -1.0 => ThirdRailState::Disconnnected,
            -0.5 => ThirdRailState::PartwiseConnected,
            1.0 => ThirdRailState::Connected,
            _ => ThirdRailState::Connected,
        }
    }

    // Wert ob V_ThirdRailCollector_{b}_{L/R} = 1 ist
    pub fn voltage(&self) -> bool {
        match f32::get(&format!("V_ThirdRailCollector_{}_{}", self.id, self.side)) {
            1.0 => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct ApiTrolleyPantograph {
    id: usize,
}

impl ApiTrolleyPantograph {
    pub fn new(new_id: usize) -> Self {
        ApiTrolleyPantograph { id: new_id }
    }

    // Entspricht der Variable panto_voltage_{a}
    pub fn voltage(&self) -> bool {
        bool::get(&format!("panto_voltage_{}", self.id))
    }

    // Entspricht der Variable panto_{a}
    pub fn panto(&self) -> f32 {
        f32::get(&format!("panto_{}", self.id))
    }

    // Entspricht der Variable trolley_angle_{a}_hori
    pub fn angle_hor(&self) -> f32 {
        f32::get(&format!("trolley_angle_{}_hori", self.id))
    }

    // Entspricht der Variable trolley_angle_{a}_hori
    pub fn set_angle_hor(&mut self, value: f32) {
        value.set(&format!("trolley_angle_{}_hori", self.id));
    }

    // Entspricht der Variable trolley_angle_{a}_vert
    pub fn angle_vert(&self) -> f32 {
        f32::get(&format!("trolley_angle_{}_vert", self.id))
    }

    // Entspricht der Variable trolley_angle_{a}_vert
    pub fn set_angle_vert(&mut self, value: f32) {
        value.set(&format!("trolley_angle_{}_vert", self.id));
    }

    // Entspricht der Variable trolley_free_{a}
    pub fn free(&self) -> bool {
        bool::get(&format!("trolley_free_{}", self.id))
    }

    // Entspricht der Variable trolley_online_{a}
    pub fn online(&self) -> bool {
        bool::get(&format!("trolley_online_{}", self.id))
    }
}
