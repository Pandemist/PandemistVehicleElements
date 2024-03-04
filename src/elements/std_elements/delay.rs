use lotus_script::delta;

#[derive(Debug, Default)]
pub struct DelayBool{
	delay: f32,
	input: bool,
	pub output: bool,
	timer: f32,
}

impl DelayBool{
	pub fn new() -> Self {
		DelayBool{
			..Default::default()
		}
	}

	pub fn tick(&mut self, new_input: bool) {
		if self.input != new_input {
			self.timer = self.delay;
			self.input = new_input;
		}
		if self.timer < 0.0 {
			self.output = self.input;
		}else{
			self.timer = self.timer - delta();
		}
	}
}