#[derive(Default, Debug)]
pub struct Steering {
    _name_id: String,
}

impl Steering {
    pub fn new(name: String) -> Self {
        Steering {
            _name_id: name,
            ..Default::default()
        }
    }

    // Entspricht der Variable Steering
    pub fn steering(&self) -> f32 {
        todo!()
    }

    // Entspricht der Variable Steering
    pub fn set_steering(&mut self, value: f32) {
        todo!()
    }
}
