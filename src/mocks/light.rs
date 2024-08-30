use glam::Vec3;
use lotus_script::var::VariableType;

#[derive(Debug)]

pub struct Light {
    name_id: String,
}

impl Light {
    pub fn new(name: String) -> Self {
        Light { name_id: name }
    }

    // Setzt die Helligkeit einer Lichtquelle
    pub fn update_light_level(&self, new_level: f32) {
        new_level.set(&self.name_id);
    }

    // Setzt die Farbe einer Lichtquelle
    pub fn update_color(&self, new_color: Vec3) {
        new_color.x.set(&format!("{}_r", self.name_id));
        new_color.y.set(&format!("{}_g", self.name_id));
        new_color.z.set(&format!("{}_b", self.name_id));
    }
}
