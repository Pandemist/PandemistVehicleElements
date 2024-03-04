use lotus_script::delta;

#[derive(Debug, Default)]
pub struct Slider{
	pub pos: f32,

	pub a: f32,
	pub b: f32,

	pub grabbing: bool,
}

impl Slider{
	pub fn new() -> Self {
		Slider{
			a: 0.0,
			b: 1.0,
			..Default::default()
		}
	}

	pub fn tick(&mut self, hand_delta: f32) {
		if self.grabbing {
			if self.a > self.pos {
				self.pos = self.pos.max(self.a).max((self.pos + hand_delta).min(self.b));
			}else if self.b < self.pos {
				self.pos = self.pos.min(self.b).min(self.pos + hand_delta).max(self.a);
			}else{
				self.pos = (self.pos + hand_delta).min(self.b).max(self.a);
			}
		}
	}

}

#[derive(Debug, Default)]
pub struct GravitySlider{
	pub pos: f32,
	speed: f32,

	//daempfung: f32,
	bumb_factor: f32,
	force: f32,
	
	pub a: f32,
	pub b: f32,

	pub grabbing: bool,
}

impl GravitySlider{
	pub fn new(_new_daempfung: f32, new_bumb_factor: f32, new_force: f32) -> Self {
		GravitySlider{
			//daempfung: new_daempfung,
			bumb_factor: new_bumb_factor,
			force: new_force,
			a: 0.0,
			b: 1.0,
			..Default::default()
		}
	}

	pub fn tick(&mut self, hand_delta: f32) {
		if self.grabbing {
			self.pos = (self.pos + hand_delta).max(self.a).min(self.b);
			self.speed = 0.0;
		}else{
			if self.pos < (self.b - 0.01) {
				self.pos = self.pos - self.speed * delta();
			}

			if self.pos < self.a {
				self.pos = self.a;
				if self.bumb_factor > 0.0 {
					self.speed = -self.bumb_factor * self.speed;
				}else{
					self.speed = 0.0;
				}
			}
		}
		self.speed = self.speed + self.force * delta();
	}
}

#[derive(Debug, Default)]
pub struct AntiGravitySlider{
	pub pos: f32,
	speed: f32,

	//daempfung: f32,
	bumb_factor: f32,
	force: f32,

	pub a: f32,
	pub b: f32,

	pub grabbing: bool,
}

impl AntiGravitySlider{
	pub fn new(_new_daempfung: f32, new_bumb_factor: f32, new_force: f32) -> Self {
		AntiGravitySlider{
			//daempfung: new_daempfung,
			bumb_factor: new_bumb_factor,
			force: new_force,
			a: 0.0,
			b: 1.0,
			..Default::default()
		}
	}

	pub fn tick(&mut self, hand_delta: f32) {
		if self.grabbing {
			self.pos = (self.pos + hand_delta).max(self.b).min(self.a);
			self.speed = 0.0;
		}else{
			if self.pos >= (self.a + 0.01) {
				self.pos = self.pos - self.speed * delta();
			}

			if self.pos > self.b {
				self.pos = self.b;
				if self.bumb_factor > 0.0 {
					self.speed = -self.bumb_factor * self.speed;
				}else{
					self.speed = 0.0;
				}
			}
		}
		self.speed = self.speed + self.force * delta();
	}
}

#[derive(Debug, Default)]
pub struct InertionSlider{
	pub pos: f32,
	speed: f32,

	pub a: f32,
	pub b: f32,

	daempfung: f32,
	lower_bumb_factor: f32,
	higher_bumb_factor: f32,
	pub force: f32,

	pub grabbing: bool,
}

impl InertionSlider{
	pub fn new(new_daempfung: f32, new_lower_bumb_factor: f32, new_higher_bumb_factor:f32, new_force: f32, new_a: f32, new_b:f32) -> Self {
		InertionSlider{
			a: new_a,
			b: new_b,

			daempfung: new_daempfung,
			lower_bumb_factor: new_lower_bumb_factor,
			higher_bumb_factor: new_higher_bumb_factor,
			force: new_force,
			..Default::default()
		}
	}

	pub fn tick(&mut self, hand_delta: f32) -> f32 {
		let mut result = 0.0;
		if self.grabbing {
			self.pos = self.pos + hand_delta;
			self.speed = hand_delta / delta();
		}else{
			self.pos = self.pos + self.speed * delta();
		}

		if self.pos < self.a {
			self.pos = self.a;
			result = self.speed;
			if self.lower_bumb_factor > 0.0 {
				self.speed = -self.lower_bumb_factor * self.speed;
			}else{
				self.speed = 0.0;
			}
		}
		
		if self.pos > self.b {
			self.pos = self.b;
			result = self.speed;
			if self.higher_bumb_factor > 0.0 {
				self.speed = -self.higher_bumb_factor * self.speed;
			}else{
				self.speed = 0.0;
			}
		}

		self.speed = self.speed + self.force * delta();

		if self.speed != 0.0 {
			let new_speed = self.speed + (-self.speed.signum() * self.daempfung) * delta();

			self.speed = if new_speed * self.speed < 0.0 {
				0.0
			}else{
				new_speed
			};
		}

		result
	}
}

#[derive(Debug, Default)]
pub struct LockableSlider{
	has_critical_zone: bool,
	critical_uppper: f32,
	critical_lower: f32,

	daempfung: f32,
	lower_bumb_factor: f32,
	higher_bumb_factor: f32,

	is_locked: bool,

	pos: f32,
	speed: f32,
	is_critical_blocked: bool,
	is_blocking_critical: bool,

	grabbing: bool,
}

impl LockableSlider{
	pub fn new() -> Self{
		LockableSlider{
			..Default::default()
		}
	}

	pub fn tick(&mut self, hand_delta: f32) {
		if self.is_locked && self.pos < 0.005 {
			self.pos = 0.0;
			return;
		}

		let lower_bumb_pos = if self.is_locked && self.pos > 0.05 {
			0.05
		}else{
			0.0
		};

		if self.grabbing {
			if (self.is_locked && self.pos != 0.0) || !self.is_locked {
				if self.has_critical_zone && self.is_critical_blocked {
					if self.pos >= self.critical_uppper {
						self.pos = (self.pos + hand_delta).min(1.0).max(self.critical_uppper);
					}
					if self.pos <= self.critical_lower {
						self.pos = (self.pos + hand_delta).max(0.0).min(self.critical_lower);
					}
				}else{
					self.pos = (self.pos + hand_delta).min(1.0).max(lower_bumb_pos);
				}
				self.speed = hand_delta / 5.0 / delta();
			}
		}else{
			self.pos = self.pos + self.speed * delta();
		}

		if self.has_critical_zone && self.is_critical_blocked {
			if (self.pos - self.critical_uppper).abs() < (self.pos - self.critical_lower).abs() {
				self.pos = self.critical_uppper;
				self.speed = if self.higher_bumb_factor > 0.0 {
					-self.higher_bumb_factor * 0.5 * self.speed
				}else{
					0.0
				};
			}else{
				self.pos = self.critical_lower;
				self.speed = if self.lower_bumb_factor > 0.0 {
					-self.lower_bumb_factor * 0.5 * self.speed
				}else{
					0.0
				};
			}
		}

		if self.pos < lower_bumb_pos {
			self.pos = lower_bumb_pos;
			self.speed = if self.lower_bumb_factor > 0.0 {
				-self.lower_bumb_factor * self.speed
			}else{
				0.0
			};
		}

		if self.pos > 1.0 {
			self.pos = 1.0;
			self.speed = if self.higher_bumb_factor > 0.0 {
				-self.higher_bumb_factor * self.speed
			}else{
				0.0
			};
		}

		self.is_blocking_critical = self.pos < self.critical_uppper && self.pos > self.critical_lower;

		if self.speed != 0.0 {
			let new_speed = self.speed + (-self.speed.signum() * self.daempfung) * delta();

			self.speed = if new_speed * self.speed < 0.0 {
				0.0
			}else{
				new_speed
			};
		}
	}
}