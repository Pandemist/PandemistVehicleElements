#[derive(Default, Debug)]
pub struct Animation {
    _name_id: String,
}

impl Animation {
    pub fn new(name: String) -> Self {
        Animation {
            _name_id: name,
            ..Default::default()
        }
    }

    // setzte die Position einer Variable
    pub fn update_pos(&mut self, new_pos: f32) {
        todo!()
    }
}
