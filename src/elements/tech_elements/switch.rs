#[derive(Default, Debug)]
pub struct Switch {
	pos: f32,
	value: bool,
	is_broken: bool
}

impl Switch {
	pub fn toggle(&mut self) {
		self.pos = 1.0 - self.pos;
		self.update_value();
	}

	pub fn on(&mut self) -> bool {
		if !self.value {
			self.pos = 1.0;
			self.update_value();
			return true
		}
		false
	}

	pub fn off(&mut self) -> bool {
		if self.value {
			self.pos = 0.0;
			self.update_value();
			return true
		}
		false
	}

	fn update_value(&mut self) {
		if !self.is_broken {
			self.value = self.pos > 0.0;
		}
	}
}

#[test]
fn test_switch() {
	let mut sw = Switch::default();
	sw.toggle();
	assert_eq!(sw.pos, 1.0);
	assert_eq!(sw.value, true);

	sw.toggle();
	assert_eq!(sw.pos, 0.0);
	assert_eq!(sw.value, false);

	let mut sw = Switch::default();
	assert_eq!(sw.on(), true);
	assert_eq!(sw.pos, 1.0);
	assert_eq!(sw.value, true);
	assert_eq!(sw.on(), false);
	
	assert_eq!(sw.off(), true);
	assert_eq!(sw.pos, 0.0);
	assert_eq!(sw.value, false);
	assert_eq!(sw.off(), false);
}