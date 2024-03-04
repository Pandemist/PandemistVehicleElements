use glam::Vec3;

#[derive(Default, Debug)]

pub struct Light {
    _name_id: String,
}

impl Light {
    pub fn new(name: String) -> Self {
        Light {
            _name_id: name,
            ..Default::default()
        }
    }

    // Setzt die Helligkeit einer Lichtquelle
    pub fn update_light_level(&mut self, new_level: f32) {
        todo!()
    }

    // Setzt die Farbe einer Lichtquelle
    pub fn update_color(&mut self, new_color: Vec3) {
        todo!()
    }
}
