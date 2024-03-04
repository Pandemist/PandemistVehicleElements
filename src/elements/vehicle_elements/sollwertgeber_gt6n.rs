use lotus_script::delta;

const THROTTLE_LEVER_SPEED:f32 = 1.0;
const THROTTLE_LEVER_SPEED_HIGH:f32 = 5.0;
const THROTTLE_LEVER_SPEED_VERYHIGH:f32 = 20.0;

#[derive(Debug, PartialEq)]
pub enum ThrottleMode {
	EmBrake,
	Brake,
	Neutral,
	Throttle,
}

impl Default for ThrottleMode {
    fn default() -> Self {
        ThrottleMode::Neutral
    }
}

#[derive(Debug, Default)]
pub struct SollwertgeberGt6n {
	speed: f32,
	target: f32,
	pos: f32,
	mode: ThrottleMode,
	raste: i32,
	raste_new: i32,

	snd_notch_neutral: i8,
	snd_notch_end: i8,
	snd_notch_other: i8,
}

impl SollwertgeberGt6n {
	pub fn new() -> Self {
		SollwertgeberGt6n {
			..Default::default()
		}
	}

	pub fn tick(&mut self) {
		self.pos = (self.pos + self.speed * delta()).min(1.0).max(0.0);

		if (self.speed > 0.0 && self.pos >= self.target) || (self.speed < 0.0 && self.pos <= self.target) {
			self.pos = self.target;
			self.speed = 0.0;
		}

		self.raste_new = if self.pos < -0.95 {
			0
		}else if self.pos < -0.87 {
			1
		}else if self.pos < -0.87 {
			2
		}else if self.pos < -0.05 {
			3
		}else if self.pos < 0.05 {
			4
		}else if self.pos < 0.13 {
			5
		}else if self.pos < 0.97 {
			6
		}else{
			7
		};

		if self.raste_new != self.raste {
			if self.raste == 4 {
				self.snd_notch_neutral = 1;
			}else if (self.raste == 6 && self.raste_new == 7) ||
					(self.raste == 2 && self.raste_new == 1) {
				self.snd_notch_end = 1;
			}else if !((self.raste == 6 && self.raste_new == 7) ||
					(self.raste == 2 && self.raste_new == 1)) {
				self.snd_notch_other = 1;
			}
		}
		self.raste = self.raste_new;
	}

	pub fn axis_input(&mut self, cab_is_vr: bool, new_value: f32) {
		self.speed = 0.0;
		self.target = new_value;

		if cab_is_vr {
			self.pos = 0.0;
			self.raste = 4;
		}else{
			self.pos = new_value;
			self.mode = if self.raste == 0 || new_value < -0.99 {
				ThrottleMode::EmBrake
			}else if self.raste <= 3 {
				ThrottleMode::Brake
			}else if self.raste == 4 {
				ThrottleMode::Neutral
			}else {
				ThrottleMode::Throttle
			};
		}
	}

	pub fn release(&mut self) {
		match self.mode {
			ThrottleMode::EmBrake => {}
			ThrottleMode::Neutral => {}
			_ => {
				if self.pos > 0.1 || self.pos < -0.1 {
					self.target = self.pos;
					self.speed = 0.0;
				}else if self.target > 0.0 {
					self.target = 0.1;
				}else{
					self.target = -0.1;
				}
			}
		}
	}

	pub fn throttle(&mut self) {
		match self.mode {
			ThrottleMode::Throttle => {
				self.speed = THROTTLE_LEVER_SPEED;
				self.target = 1.0;
			}
			ThrottleMode::Neutral => {
				self.speed = THROTTLE_LEVER_SPEED;
				self.target = 1.0;
				self.mode = ThrottleMode::Throttle;
			}
			_ => {
				if self.pos < -0.15 {
					self.speed = THROTTLE_LEVER_SPEED;
					self.target = -0.1;
					if self.pos < -0.9 {
						self.pos = -0.9;
					}
					self.mode = ThrottleMode::Brake;
				}else{
					self.speed = -THROTTLE_LEVER_SPEED_HIGH;
					self.target = 0.0;
					self.mode = ThrottleMode::Neutral;
				}
			}
		}
	}

	pub fn neutral(&mut self) {
		if self.pos > 0.0 {
			self.speed = -THROTTLE_LEVER_SPEED_HIGH;
		}else{
			self.speed = THROTTLE_LEVER_SPEED_HIGH;
		}
		self.target = 0.0;
		self.mode = ThrottleMode::Neutral;
	}

	pub fn brake(&mut self) {
		match self.mode {
			ThrottleMode::Throttle => {
				if self.pos > 0.15 {
					self.speed = -THROTTLE_LEVER_SPEED;
					self.target = 0.1;
				}else{
					self.speed = -THROTTLE_LEVER_SPEED_HIGH;
					self.target = 0.0;
					self.mode = ThrottleMode::Neutral;
				}
			}
			ThrottleMode::Neutral => {
				self.speed = -THROTTLE_LEVER_SPEED;
				self.target = -0.9;
				self.mode = ThrottleMode::Brake;
			}
			ThrottleMode::Brake => {
				if self.pos > -0.9 {
					self.speed = -THROTTLE_LEVER_SPEED;
					self.target = -0.9;
				}
			}
			_ => {}
		}
	}

	pub fn max_brake(&mut self) {
		self.speed = -THROTTLE_LEVER_SPEED_VERYHIGH;
		self.target = -1.0;
		self.mode = ThrottleMode::EmBrake;
	}
}