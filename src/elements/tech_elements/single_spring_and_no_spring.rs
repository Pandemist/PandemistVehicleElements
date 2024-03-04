#[derive(Debug, Default)]
pub struct SingleSpringAndNoSpring {
    pos: f32,
    value: i32,
    is_broken: bool,
}

impl SingleSpringAndNoSpring {
    pub fn plus(&mut self, key: bool) -> bool {
        let mut result = false;
        if key && self.value < 1 {
            self.pos = self.pos + 1.0;
            result = true;
        }
        self.update_value();
        result
    }

    pub fn minus(&mut self, key: bool) -> bool {
        let mut result = false;
        if key {
            if self.pos > -1.0 {
                self.pos = self.pos - 1.0;
                result = true;
            }
        } else {
            if self.pos == -1.0 {
                self.pos = 0.0;
                result = true;
            }
        }

        self.update_value();
        result
    }

    pub fn set_minus(&mut self, key: bool) -> bool {
        let mut result = false;
        if key {
            if self.pos > -1.0 {
                self.pos = -1.0;
                result = true;
            }
        } else {
            if self.pos == -1.0 {
                self.pos = 0.0;
                result = true;
            }
        }

        self.update_value();
        result
    }

    pub fn set(&mut self, new_value: i32) -> bool {
        let result = self.pos != new_value as f32;
        if result {
            self.pos = new_value as f32;
            self.update_value();
        }
        result
    }

    fn update_value(&mut self) {
        if !self.is_broken {
            self.value = self.pos as i32;
        }
    }
}
