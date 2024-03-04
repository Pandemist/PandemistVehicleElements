#[derive(Debug, Default)]
pub struct PushHoldButton {
    pos: f32,
    value: bool,
}

impl PushHoldButton {
    pub fn set(&mut self) {
        self.value = true;
        self.pos = 0.75;
    }

    pub fn set_state(&mut self, state: i32) {
        if state == 0 {
            self.pos = 0.0;
        } else if state == 1 {
            self.value = !self.value;
            self.pos = 1.0;
        } else if state == 2 {
            self.pos = 0.75;
        }
    }

    pub fn press(&mut self, key: bool) {
        self.pos = if key { 1.0 } else { 0.0 };
        self.value = self.pos > 0.5;
    }

    pub fn press_release_press(&mut self, key: bool) {
        if !self.value && !key {
            self.value = true;
        }
        self.pos = if key { 1.0 } else { 0.75 };
    }

    pub fn press_release_release(&mut self, key: bool) {
        if self.value && !key {
            self.value = false;
        }
        self.pos = if key { 1.0 } else { 0.0 };
    }

    pub fn press_release(&mut self, key: bool) {
        if key {
            self.value = !self.value;
        }
        self.pos = if key {
            1.0
        } else {
            if self.value {
                0.75
            } else {
                0.0
            }
        };
    }
}
