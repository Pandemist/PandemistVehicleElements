#[derive(Default, Debug)]
pub struct Texture {
    _name_id: String,
}

impl Texture {
    pub fn new(name: String) -> Self {
        Texture {
            _name_id: name,
            ..Default::default()
        }
    }

    // Setzt den Blendfaktor eines Materials
    pub fn set_blending(&mut self, new_value: f32) {
        todo!()
    }

    // Setzt die Textur eines Materials
    pub fn set_texture(&mut self, user_id: u32, sub_id: u32) {
        todo!()
    }
}
