#[derive(Default, Debug)]
pub struct Bogie {
    _name_id: String,
    _index: u32,
}

impl Bogie {
    pub fn new(name: String, new_index: u32) -> Self {
        Bogie {
            _name_id: name,
            _index: new_index,
            ..Default::default()
        }
    }

    // Entspricht der Variable F_RailBrake_Bogie_N_{b}
    pub fn railbrake_force(&mut self, force: f32) {
        todo!()
    }
}
