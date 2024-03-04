#[derive(Default, Debug)]
pub struct PushButton {
    pos: f32,
    value: i32,
    is_broken: bool
}

impl PushButton {
    pub fn press(&mut self, key: bool) {
        if key {
            self.pos = 1.0;
        }else{
            self.pos = 0.0;
        }
        self.update_value();
    }
    
	fn update_value(&mut self) {
		if !self.is_broken {
			self.value = self.pos as i32;
		}
	}
}

#[test]
fn test_push_button() {
    let mut btn = PushButton::default();
    btn.press(true);
    assert_eq!(btn.pos, 1.0);
    assert_eq!(btn.value, 1);
    btn.press(false);
    assert_eq!(btn.pos, 0.0);
    assert_eq!(btn.value, 0);
}