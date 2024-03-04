use lotus_script::delta;

const KNUEPPEL_SPEED:f32 = 7.5;
const KNUEPPEL_SPEED_MIDI:f32 = 25.0;
const KNUEPPEL_SPEED_FAST:f32 = 40.0;
const KNUEPPEL_PAUSE:f32 = 0.075;

#[derive(Debug, PartialEq)]
enum MovementMode {
	None,
	Min,
	Up,
	Down,
}

impl Default for MovementMode {
    fn default() -> Self {
        MovementMode::None
    }
}


#[derive(Debug, Default)]
pub struct Knueppel {
	const_max_raste: i8,
	const_min_raste: i8,

	const_max_serie: i8,
	const_max_parallel: i8,

	knochen_pos: i8,

	pos: f32,
	moving_pos: f32,
	raste: i8,
	raste_last: i8,
	raste_target: i8,

	move_speed: f32,
	move_type: MovementMode,
	pause_timer: f32,

	controller: bool,
}

impl Knueppel {
	fn get_pos_from_raste(&self, raste: i8) -> f32 {
		if raste > 0 {
			(raste / self.const_max_raste) as f32
		}else if raste < 0 {
			(raste.abs() / self.const_min_raste) as f32
		}else{
			0.0
		}
	}

	fn get_raste_from_pos(&self, pos: f32) -> i8 {
		if pos > 0.05 {
			(pos * (self.const_max_raste as f32)).trunc() as i8
		}else if pos < 0.05 {
			(pos * (self.const_max_raste as f32)).trunc() as i8
		}else{
			0
		}
	}

	pub fn new(
		fahrrasten: i8,
		bremsrasten: i8,
		dauerstufe_serie: i8,
		dauerstufe_parallel: i8,
	) -> Self {
		Knueppel{
			const_max_raste: fahrrasten,
			const_min_raste: bremsrasten,

			const_max_serie: dauerstufe_serie,
			const_max_parallel: dauerstufe_parallel,
			..Default::default()
		}
	}

	pub fn tick(&mut self) {
		self.pause_timer = (self.pause_timer - delta()).max(0.0);

		if self.knochen_pos != 0 {

			if (self.knochen_pos == -2) || (self.knochen_pos == -1) {
				self.raste_target = self.raste_target.min(3);
			}

			if (self.knochen_pos == 4)
				|| (self.knochen_pos == 3)
				|| (self.knochen_pos == -3)
				|| (self.knochen_pos == -4) {
				self.raste_target = self.raste_target.min(self.const_max_serie);
			}

			if !self.controller {
				
				if ((self.pause_timer <= 0.0)
					&& ((self.move_type == MovementMode::Down) || (self.move_type == MovementMode::Up)))
					|| ((self.move_type == MovementMode::None) || (self.move_type == MovementMode::Min)) {
					self.moving_pos = (self.moving_pos + self.move_speed * delta()).max(self.const_min_raste as f32).min(self.const_max_raste as f32);

					if ((self.move_speed > 0.0) && (self.moving_pos >= self.raste_target as f32)) ||
						((self.move_speed < 0.0) && (self.moving_pos <= self.raste_target as f32)) {
						self.moving_pos = self.raste_target as f32;
						self.move_speed = 0.0;
					}

					self.raste = self.raste_target;

					if self.raste != self.raste_last {
						self.pause_timer = KNUEPPEL_PAUSE;
						self.raste_last = self.raste;
					}

				}
			}else{
				self.raste = self.moving_pos.trunc() as i8;
				self.raste_last = self.raste;
			}
		}

		self.pos = self.get_pos_from_raste(self.moving_pos.round() as i8);
	}

	pub fn axis_input(&mut self, new_value: f32) {
		self.controller = true;

		self.moving_pos = self.get_raste_from_pos(new_value) as f32;
		self.raste = self.moving_pos.trunc() as i8;
		self.raste_target = self.raste;
	}

	pub fn throttle(&mut self) {
		self.controller = false;

		if self.raste < 0 {
			self.raste_target = 0;
		}else if self.raste < self.const_max_serie {
			self.raste_target =  self.const_max_serie;
		}else if self.raste < self.const_max_parallel {
			self.raste_target =  self.const_max_parallel;
		}else {
			self.raste_target =  self.const_max_raste;
		}

		self.move_type = MovementMode::Up;
		self.move_speed = KNUEPPEL_SPEED;
	}

	pub fn neutral(&mut self) {
		self.controller = false;

		self.raste_target = 0;
		self.move_type = MovementMode::None;

		self.move_speed = (self.raste_target - self.raste).signum() as f32 * KNUEPPEL_SPEED_MIDI;
	}

	pub fn brake(&mut self) {
		self.controller = false;

		self.raste_target = self.const_min_raste;
		self.move_type = MovementMode::Down;
		self.move_speed = -KNUEPPEL_SPEED;
	}
	
	pub fn last_brake(&mut self) {
		self.controller = false;

		self.raste_target = self.const_min_raste;
		self.move_type = MovementMode::Min;
		self.move_speed = -KNUEPPEL_SPEED_FAST;
	}

	pub fn release(&mut self) {
		self.controller = false;

		if self.move_type == MovementMode::Up || self.move_type == MovementMode::Down {
			self.moving_pos = self.moving_pos.round();
			self.raste = self.moving_pos.round() as i8;
			self.raste_target = self.raste;
			self.move_type = MovementMode::Min;
			self.move_speed = 0.0;
			self.pause_timer = 0.0;
		}
	}
}