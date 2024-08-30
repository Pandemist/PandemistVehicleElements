use lotus_script::var::VariableType;

use super::mock_enums::CouplingState;

#[derive(Default, Debug)]
pub struct Coupler {
    id: usize,
}

impl Coupler {
    pub fn new(new_id: usize) -> Self {
        Coupler { id: new_id }
    }

    // Entspricht der Variable coupled_{a}
    pub fn coupled(&self) -> bool {
        bool::get(&format!("coupled_{}", self.id))
    }

    // Entspricht der Variable couplingState_{a}
    pub fn coupling_state(&self) -> CouplingState {
        match u8::get(&format!("couplingState_{}", self.id)) {
            2 => CouplingState::Coupled,
            1 => CouplingState::Ready,
            _ => CouplingState::Deactivated,
        }
    }

    // Entspricht der Variable couplingOffsetY_{a}
    pub fn coupling_y_offset(&self) -> f32 {
        f32::get(&format!("couplingOffsetY_{}", self.id))
    }

    // Entspricht der Variable couplingOffsetY_{a}
    pub fn set_coupling_y_offset(&mut self, value: f32) {
        value.set(&format!("couplingOffsetY_{}", self.id));
    }
}
