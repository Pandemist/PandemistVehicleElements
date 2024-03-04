use crate::structs::internal_enums::{ThirdRailSide, ThirdRailState};

#[derive(Default, Debug)]
pub struct ApiPantograph {
    _name_id: String,
}

impl ApiPantograph {
    pub fn new(name: String) -> Self {
        ApiPantograph {
            _name_id: name,
            ..Default::default()
        }
    }

    // Entspricht der Variable panto_voltage_{a}
    pub fn voltage(&self) -> bool {
        todo!()
    }

    // Entspricht der Variable panto_{a}
    pub fn panto(&self) -> f32 {
        todo!()
    }
}

#[derive(Debug)]
pub struct ApiThirdRailCollector {
    _name_id: String,
    _side: ThirdRailSide,
}

impl ApiThirdRailCollector {
    pub fn new(name: String, new_side: ThirdRailSide) -> Self {
        ApiThirdRailCollector {
            _name_id: name,
            _side: new_side,
        }
    }

    // Position der Stromschiene am V_ThirdRailCollector_{b}_{L/R} (Nur Wertebereich -1, -0.5, 0/1)
    pub fn value(&self) -> ThirdRailState {
        todo!()
    }

    // Wert ob V_ThirdRailCollector_{b}_{L/R} = 1 ist
    pub fn voltage(&self) -> bool {
        todo!()
    }
}

#[derive(Default, Debug)]
pub struct ApiTrolleyPantograph {
    _name_id: String,
}

impl ApiTrolleyPantograph {
    pub fn new(name: String) -> Self {
        ApiTrolleyPantograph {
            _name_id: name,
            ..Default::default()
        }
    }

    // Entspricht der Variable panto_voltage_{a}
    pub fn voltage(&self) -> bool {
        todo!()
    }

    // Entspricht der Variable panto_{a}
    pub fn panto(&self) -> f32 {
        todo!()
    }

    // Entspricht der Variable trolley_angle_{a}_hori
    pub fn angle_hor(&self) -> f32 {
        todo!()
    }

    // Entspricht der Variable trolley_angle_{a}_hori
    pub fn set_angle_hor(&mut self, value: f32) {
        todo!()
    }

    // Entspricht der Variable trolley_angle_{a}_vert
    pub fn angle_vert(&self) -> f32 {
        todo!()
    }

    // Entspricht der Variable trolley_angle_{a}_vert
    pub fn set_angle_vert(&mut self, value: f32) {
        todo!()
    }

    // Entspricht der Variable trolley_free_{a}
    pub fn free(&self) -> bool {
        todo!()
    }

    // Entspricht der Variable trolley_online_{a}
    pub fn online(&self) -> bool {
        todo!()
    }
}
