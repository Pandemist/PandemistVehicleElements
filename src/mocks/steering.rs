use lotus_script::var::VariableType;

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

    pub fn steering(&self) -> f32 {
        f32::get("Steering")
    }

    pub fn set_steering(&mut self, value: f32) {
        value.set("Steering");
    }
}
