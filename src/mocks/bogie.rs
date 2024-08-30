use lotus_script::var::VariableType;

#[derive(Debug)]
pub struct Bogie {
    id: usize,
}

impl Bogie {
    pub fn new(new_id: usize) -> Self {
        Bogie { id: new_id }
    }

    pub fn railbrake_force(&mut self, force: f32) {
        force.set(&format!("F_RailBrake_Bogie_N_{}", self.id));
    }
}
