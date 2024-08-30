use lotus_script::var::VariableType;

#[derive(Debug)]
pub struct Animation {
    name: String,
}

impl Animation {
    pub fn new(name: String) -> Self {
        Animation { name: name }
    }

    pub fn set(&mut self, pos: f32) {
        pos.set(&self.name);
    }
}
