use crate::structs::internal_enums::CouplingState;

#[derive(Default, Debug)]
pub struct Coupler {
    name_id: String,
}

impl Coupler {
    pub fn new(name: String) -> Self {
        Coupler {
            name_id: name,
            ..Default::default()
        }
    }

    // Entspricht der Variable coupled_{a}
    pub fn coupled(&self) -> bool {
        todo!()
    }

    // Entspricht der Variable couplingState_{a}
    pub fn coupling_state(&self) -> CouplingState {
        todo!()
    }

    // Entspricht der Variable couplingOffsetY_{a}
    pub fn coupling_y_offset(&self) -> f32 {
        todo!()
    }

    // Entspricht der Variable couplingOffsetY_{a}
    pub fn set_coupling_y_offset(&mut self, value: f32) {
        todo!()
    }
}

#[derive(Default, Debug)]
pub struct CouplerLine<T> {
    name_id: String,

    value: T,
}

impl<T: Default> CouplerLine<T> {
    pub fn new(name: String) -> Self {
        CouplerLine {
            name_id: name,
            ..CouplerLine::default()
        }
    }

    pub fn front(&self) -> Option<T> {
        todo!()
    }

    pub fn back(&self) -> Option<T> {
        todo!()
    }
}
