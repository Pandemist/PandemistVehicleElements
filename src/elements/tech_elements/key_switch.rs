#[derive(Default, Debug)]
pub struct KeySwitch {
	a: i32,
	b: i32,
	pos: f32,
	value: i32,
	is_broken: bool,
	is_inserted: bool,
}

impl KeySwitch {
	pub fn new(min_val: i32, max_val: i32) -> Self {
		KeySwitch{
			a: min_val,
			b: max_val,
			..KeySwitch::default()
		}
	}

	pub fn plus(&mut self, key: bool) -> bool {
		if key && self.pos < self.b as f32 && self.is_inserted {
			self.pos = self.pos + 1.0;
			self.update_value();
			return true
		}
		false
	}
	
	pub fn minus(&mut self, key: bool) -> bool {
		if key && self.pos > self.a as f32 {
			if self.is_inserted {
				self.pos = self.pos - 1.0;
				self.update_value();
				return true
			}else{
				self.is_inserted = false;
				return true
			}
		} 
		false
	}

	pub fn plus_taster(&mut self, key: bool) -> bool {
		let result = key != (self.value != 0);

		if self.is_inserted {
			self.pos = 1.0;
		}else{
			self.pos = 0.0;
		}
		self.update_value();

		result
	}
	
	pub fn minus_taster(&mut self, key: bool) -> bool {
		let result = key != (self.value != 0);

		if self.is_inserted {
			self.pos = -1.0;
		}else{
			self.pos = 0.0;
		}
		self.update_value();

		result
	}
	
	pub fn toggle(&mut self, key: bool) -> bool {
		if self.is_inserted && key {
			self.pos = 1.0 - self.pos;
			self.update_value();
			return true
		}
		false
	}

	pub fn set(&mut self, new_val: i32) -> bool {
		let result = self.pos.round() as i32 != new_val;

		if result {
			self.pos = new_val as f32;
			self.is_inserted = if self.pos > self.a as f32 {
				true
			}else {
				false
			};
		}

		self.update_value();
		result
	}

	pub fn set_else(&mut self, new_val: i32, else_val: i32) {
		let val = if new_val == self.pos.round() as i32 {
			else_val
		}else{
			new_val
		};
		self.pos = val as f32;

		self.is_inserted = if self.pos > self.a as f32 {
			true
		}else {
			false
		};

		self.update_value();
	}

	fn update_value(&mut self) {
		if !self.is_broken {
			self.value = self.pos as i32;
		}
	}
}

#[test]
fn test_key_switch() {
	let mut sw = KeySwitch::new(-1, 1);
	sw.is_inserted = true;
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

	let mut sw = KeySwitch::new(-1, 1);
	sw.is_inserted = true;
	assert_eq!(sw.plus_taster(true), true);
	assert_eq!(sw.pos, 1.0);
	assert_eq!(sw.plus_taster(false), true);
	assert_eq!(sw.pos, 0.0);
	
	let mut sw = KeySwitch::new(-1, 1);
	sw.is_inserted = true;
	assert_eq!(sw.minus_taster(true), true);
	assert_eq!(sw.pos, -1.0);
	assert_eq!(sw.minus_taster(false), true);
	assert_eq!(sw.pos, 0.0);

	let mut sw = KeySwitch::new(-1, 1);
	assert_eq!(sw.toggle(true), false);
	
	sw.is_inserted = true;

	assert_eq!(sw.toggle(false), false);
	assert_eq!(sw.pos, 0.0);
	assert_eq!(sw.toggle(true), true);
	assert_eq!(sw.pos, 1.0);
	assert_eq!(sw.toggle(false), false);
	assert_eq!(sw.pos, 0.0);

	
	let mut sw = KeySwitch::new(-1, 1);
	sw.is_inserted = true;
	assert_eq!(sw.set(1), true);
	assert_eq!(sw.pos, 1.0);
	assert_eq!(sw.set(1), false);

	sw.set_else(1, -1);
	assert_eq!(sw.pos, -1.0);
}