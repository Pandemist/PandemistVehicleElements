use lotus_script::delta;


#[derive(Debug, Default)]
pub struct Blinkrelais {
	interval: f32,
	on_time: f32,
	timer: f32,
	pub is_on: bool,
	reset_time: f32,
}

impl Blinkrelais{
	pub fn new(new_interval: f32, new_on_time: f32, new_reset_time: f32) -> Self {
		Blinkrelais{
			interval: new_interval,
			on_time: new_on_time,
			reset_time: new_reset_time,
			..Default::default()
		}
	}

	pub fn tick(&mut self) -> i32 {
		self.timer = self.timer + delta();

		if self.timer > self.interval {
			self.timer = self.timer - self.interval;
		}

		let new_on = self.timer < self.on_time;

		let result = if new_on && !self.is_on {
			1
		}else if !new_on && self.is_on  {
			-1
		}else{
			0
		};
		self.is_on = new_on;

		result
	}

	pub fn reset(&mut self) {
		self.timer = self.reset_time;
		self.is_on = false;
	}
}