#[derive(Debug, Default)]
pub struct Pedal{
	pos: f32,
	val: bool,
	is_broken: bool
}

impl Pedal{
	pub fn press(&mut self, key: bool) {
		self.pos = if key {
			1.0
		}else{
			0.0
		};
		if self.pos > 0.5 {
			self.update_value();
		}
	}

	pub fn toggle(&mut self) {
		self.pos = 1.0 - self.pos;
		self.update_value();
	}

	pub fn on(&mut self) -> bool {
		if !self.val {
			self.pos = 1.0;
			self.update_value();
			return true
		}
		false
	}

	pub fn off(&mut self) -> bool {
		if self.val {
			self.pos = 0.0;
			self.update_value();
			return true
		}
		false
	}

	fn update_value(&mut self) {
		if !self.is_broken {
			self.val = self.pos > 0.5;
		}
	}
}

#[test]
fn test_pedal() {
	let mut sw = Pedal::default();
	sw.press(true);
	assert_eq!(sw.pos, 1.0);
	assert_eq!(sw.val, true);
	sw.press(false);
	assert_eq!(sw.pos, 0.0);
	assert_eq!(sw.val, false);

	let mut sw = Pedal::default();
	sw.toggle();
	assert_eq!(sw.pos, 1.0);
	assert_eq!(sw.val, true);

	sw.toggle();
	assert_eq!(sw.pos, 0.0);
	assert_eq!(sw.val, false);

	let mut sw = Pedal::default();
	assert_eq!(sw.on(), true);
	assert_eq!(sw.pos, 1.0);
	assert_eq!(sw.val, true);
	assert_eq!(sw.on(), false);
	
	assert_eq!(sw.off(), true);
	assert_eq!(sw.pos, 0.0);
	assert_eq!(sw.val, false);
	assert_eq!(sw.off(), false);
}