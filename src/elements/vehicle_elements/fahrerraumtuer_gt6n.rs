use lotus_script::delta;

const HANDDOOR_REFLECT_CLOSE:f32 = 0.049;

#[derive(Debug, Default)]
pub struct HandDoor {
	pos: f32,
	pos_handle: f32,
	pos_riegel: f32,
	grabbing_a: bool,
	grabbing_b: bool,
	speed: f32,
	daempfung: f32,
}

impl HandDoor {
	pub fn new() -> Self {
		HandDoor{
			..Default::default()
		}
	}

	pub fn press_handle_a(&mut self, key: bool) {
		self.pos_handle = key as i32 as f32;
		self.grabbing_a = key;
	}
	
	pub fn press_handle_b(&mut self, key: bool) {
		self.pos_handle = key as i32 as f32;
		self.grabbing_b = key;
	}

	pub fn tick(&mut self, force: f32, physic_force: f32) {
		self.pos_riegel = (self.pos > 0.5 || (self.pos > 0.0 && (self.pos + 0.001) < HANDDOOR_REFLECT_CLOSE)) as i32 as f32;

		if (force > 0.0 && physic_force > 0.0) && self.pos < 0.01 && self.pos_handle != 1.0 {
			self.speed = 0.0;
			return;
		}

		if (self.grabbing_a || self.grabbing_b) && (self.pos_riegel != 1.0 && self.pos < 0.005) {
			self.pos = 0.0;
			return;
		}

		if self.grabbing_a {
			if self.pos_riegel < 0.5 {
				self.pos = (self.pos + force).max(0.0).min(1.0);
			}else{
				self.pos = (self.pos + force).max(HANDDOOR_REFLECT_CLOSE).min(1.0);
			}
			self.speed = force / delta();
		}else if self.grabbing_b {
			if self.pos_riegel < 0.5 {
				self.pos = (self.pos - force).max(0.0).min(1.0);
			}else{
				self.pos = (self.pos - force).max(HANDDOOR_REFLECT_CLOSE).min(1.0);
			}
			self.speed = force / delta();
		}else{
			if self.pos_riegel < 0.5 && self.pos >= HANDDOOR_REFLECT_CLOSE {
				self.pos = (self.pos + self.speed * delta()).max(HANDDOOR_REFLECT_CLOSE);
			}else{
				self.pos = self.pos + self.speed * delta();
			}
			
			self.speed = physic_force / delta();
		}

		if self.pos > 1.0 {
			self.pos = 1.0;
			self.speed = -0.2 * self.speed;
		}
		
		if self.pos < 0.005 {
			self.pos = 0.0;
			self.speed = 0.0;
		}

		if self.pos_riegel < 0.5 && self.pos < HANDDOOR_REFLECT_CLOSE && self.pos != 0.0 {
			self.pos = HANDDOOR_REFLECT_CLOSE + 0.001;
			self.speed = -0.2 * self.speed;
		}

		if self.pos < HANDDOOR_REFLECT_CLOSE && self.pos > 0.0 {
			self.speed = 0.0;
		}

		if self.speed != 0.0 {
			let new_speed = self.speed + (-self.speed.signum() * self.daempfung) * delta();

			if new_speed * self.speed < 0.0 {
				self.speed = 0.0;
			}else{
				self.speed = new_speed;
			}
		}
	}
}