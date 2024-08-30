use lotus_script::var::VariableType;

#[derive(Debug)]
pub struct Visiblility {
    name_id: String,
}

impl Visiblility {
    pub fn new(name: String) -> Self {
        Visiblility { name_id: name }
    }

    pub fn check(&mut self) -> bool {
        bool::get(&self.name_id)
    }

    pub fn make_visible(&mut self) {
        true.set(&self.name_id);
    }

    pub fn set_visbility(&mut self, value: bool) {
        value.set(&self.name_id);
    }

    pub fn make_invisible(&mut self) {
        false.set(&self.name_id);
    }
}
