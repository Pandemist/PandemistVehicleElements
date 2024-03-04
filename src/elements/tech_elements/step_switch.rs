#[derive(Default, Debug)]
pub struct StepSwitch {
	a: i32,
	b: i32,
	pos: f32,
	value: i32,
	is_broken: bool
}

impl StepSwitch {
	pub fn new(min_val: i32, max_val: i32) -> Self {
		StepSwitch{
			a: min_val,
			b: max_val,
			..StepSwitch::default()
		}
	}

	pub fn plus(&mut self, key: bool) -> bool {
		if key && self.pos < self.b as f32 {
			self.pos = self.pos + 1.0;
			self.update_value();
			return true
		}
		false
	}
	
	pub fn plus_spring(&mut self, key: bool) -> bool {
		if key && self.pos < self.b as f32 {
			self.pos = self.pos + 1.0;
			self.update_value();
			return true
		}else if !key && self.pos > 0.0 {
			self.pos = self.pos - 1.0;
			self.update_value();
			return true
		}
		false
	}
	
	pub fn minus(&mut self, key: bool) -> bool {
		if key && self.pos > self.a as f32 {
			self.pos = self.pos - 1.0;
			self.update_value();
			return true
		}
		false
	}
	
	pub fn minus_spring(&mut self, key: bool) -> bool {
		if key && self.pos > self.a as f32 {
			self.pos = self.pos - 1.0;
			self.update_value();
			return true
		}else if !key && self.pos < 0.0 {
			self.pos = self.pos + 1.0;
			self.update_value();
			return true
		}
		false
	}

	pub fn set(&mut self, new_val: i32) -> bool {
		if self.pos != new_val as f32 {
			self.pos = new_val as f32;
			self.update_value();
			return true
		}
		false
	}
	
	pub fn set_else(&mut self, new_val: i32, alt_val: i32) {
		if self.pos == new_val as f32 {
			self.pos = alt_val as f32;
			self.update_value();
		}else{
			self.pos = new_val as f32;
			self.update_value();
		}
	}

	fn update_value(&mut self) {
		if !self.is_broken {
			self.value = self.pos as i32;
		}
	}
}

#[test]
fn test_step_switch() {
	let mut sw = StepSwitch::new(-1, 1);
	assert_eq!(sw.plus(true), true);
	assert_eq!(sw.pos, 1.0);
	assert_eq!(sw.value, 1);
	assert_eq!(sw.plus(true), false);
	assert_eq!(sw.pos, 1.0);
	assert_eq!(sw.minus(true), true);
	assert_eq!(sw.pos, 0.0);
	assert_eq!(sw.value, 0);
	assert_eq!(sw.minus(false), false);
	assert_eq!(sw.pos, 0.0);

	let mut sw = StepSwitch::new(-1, 1);
	assert_eq!(sw.plus_spring(true), true);
	assert_eq!(sw.pos, 1.0);
	assert_eq!(sw.plus_spring(false), true);
	assert_eq!(sw.pos, 0.0);
	
	let mut sw = StepSwitch::new(-1, 1);
	assert_eq!(sw.minus_spring(true), true);
	assert_eq!(sw.pos, -1.0);
	assert_eq!(sw.minus_spring(false), true);
	assert_eq!(sw.pos, 0.0);
	
	let mut sw = StepSwitch::new(-1, 1);
	assert_eq!(sw.set(1), true);
	assert_eq!(sw.pos, 1.0);
	assert_eq!(sw.set(1), false);

	sw.set_else(1, -1);
	assert_eq!(sw.pos, -1.0);
}