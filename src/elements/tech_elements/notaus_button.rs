use lotus_script::delta;

#[derive(Debug, Default)]
pub struct Notaus{
	pos: f32,
	rot: f32,
	target: bool,
	value: bool,
}

impl Notaus{
	pub fn press(&mut self) {
		self.target = true;
	}
	pub fn release(&mut self) {
		self.target = false;
	}
	pub fn tick(&mut self) {
		if self.target {
			if self.pos < 1.0 {
				self.pos = (self.pos + 20.0 * delta()).min(1.0);
			}
		}else{
			if self.pos >= 1.0 && self.rot < 1.0 {
				self.rot = self.rot + 2.0 * delta();
			}else{
				if self.pos > 0.0 {
					self.pos = (self.pos + 20.0 * delta()).max(0.0);
				}
				if self.rot > 0.0 {
					self.rot = (self.rot + 20.0 * delta()).max(0.0);
				}
			}			
		}
		self.value = self.pos > 0.5;
	}
}