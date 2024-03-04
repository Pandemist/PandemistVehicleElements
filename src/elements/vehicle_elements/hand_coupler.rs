use lotus_script::delta;

const KUPPLUNG_REFLECT_CLOSE:f32 = 0.95;

#[derive(Debug, Default)]
pub struct HandCoupler {
	pos: f32,
	hebel_pos: f32,
	grabbing: bool,
	close_cab: bool,
	speed: f32,
	daempfung: f32,
}

impl HandCoupler {
	pub fn new() -> Self {
		HandCoupler{
			..Default::default()
		}
	}

	pub fn pull_lever(&mut self, key: bool) {
		self.hebel_pos = key as i32 as f32;
		self.grabbing = key;
	}

	pub fn tick(&mut self, force: f32) {
		if force > 0.0 && self.hebel_pos > 0.5 && self.pos > 0.99 {
			self.speed = 0.0;
			return;
		}

		if self.grabbing && (self.hebel_pos < 0.5 && self.pos > (KUPPLUNG_REFLECT_CLOSE + 0.001)) {
			self.pos = 1.0;
			return;
		}

		let pos_last = self.pos;

		if self.grabbing {
			if self.hebel_pos > 0.5 {
				self.pos = (self.pos + delta()).max(0.0).min(1.0);
			}else{
				self.pos = (self.pos + force).max(0.0).min(KUPPLUNG_REFLECT_CLOSE);
			}

			self.speed = force / delta();
		}

		if self.hebel_pos < 0.5 && self.pos > KUPPLUNG_REFLECT_CLOSE && self.pos != 1.0 {
			self.pos = KUPPLUNG_REFLECT_CLOSE - 0.001;
			self.speed = -0.2 * self.speed;
		}

		if self.hebel_pos > 0.5 && self.pos >= 1.0 {
			self.pos = 1.0;
			self.speed = 0.0;
		}

		if pos_last > 0.0 && self.pos <= 0.0 {
			self.pos = 0.0;
			self.speed = 0.0;
			self.close_cab = true;
		}

		if self.speed.abs() > 0.0 {
			let new_speed = self.speed + (-self.speed.signum() * self.daempfung) * delta();

			if new_speed * self.speed < 0.0 {
				self.speed = 0.0;
			}else{
				self.speed = new_speed;
			}
		}
	}
}