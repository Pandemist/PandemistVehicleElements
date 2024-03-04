use lotus_script::delta;

#[derive(Debug, Default)]
pub struct HandPin {
	pos_x: f32,
	pos_y: f32,
	target_l: bool,
	target_r: bool,
	target_o: bool,
	target_u: bool,
	is_right: bool,
	is_left: bool,
	is_top: bool,
	is_bottom: bool,
	selected_side: bool,
	grabbing: bool,
}

impl HandPin {
	pub fn new() -> Self {
		HandPin{
			..Default::default()
		}
	}

	pub fn toggle_side(&mut self) {
		self.selected_side = !self.selected_side;
	}

	pub fn tick(&mut self, delta_x: f32, delta_y:f32) {
		if self.grabbing {
			self.pos_x = (self.pos_x + delta_x * delta()).min(1.0).max(-1.0);
			self.pos_y = (self.pos_y + delta_y * delta()).min(1.0).max(-1.0);
		}else{
			if self.target_l {
				self.pos_x = self.pos_x - 2.0 * delta();
			}else if self.target_r {
				self.pos_x = self.pos_x + 2.0 * delta();
			}else if self.target_o {
				self.pos_y = self.pos_y + 2.0 * delta();
			}else if self.target_u {
				self.pos_y = self.pos_y - 2.0 * delta();
			}else{
				self.pos_x = self.pos_x - (self.pos_x / 2.0);
				self.pos_y = self.pos_y - (self.pos_y / 2.0);
			}

			if self.pos_x.abs() < 0.025 {
				self.pos_x = 0.0;
			}
			if self.pos_y.abs() < 0.025 {
				self.pos_y = 0.0;
			}
		}

		self.is_right = self.pos_x > 0.8;
		self.is_left = self.pos_x < -0.8;
		self.is_top = self.pos_y > 0.8;
		self.is_bottom = self.pos_y < -0.8;
	}
}

#[derive(Debug, Default)]
pub struct HandPinAnalog {
	pos_x: f32,
	pos_y: f32,
	target_l: bool,
	target_r: bool,
	target_o: bool,
	target_u: bool,
	is_right: bool,
	is_left: bool,
	is_top: bool,
	is_bottom: bool,
	selected_side: bool,
	grabbing: bool,
}

impl HandPinAnalog {
	pub fn new() -> Self {
		HandPinAnalog{
			..Default::default()
		}
	}

	pub fn toggle_side(&mut self) {
		self.selected_side = !self.selected_side;
	}

	pub fn tick(&mut self, delta_x: f32, delta_y:f32) {
		if self.grabbing {
			if self.pos_x < 0.025 && self.pos_x > -0.025 {
				self.pos_x = (self.pos_x + delta_x * delta()).min(1.0).max(-1.0);
			}
			if self.pos_y < 0.025 && self.pos_y > -0.025 {
				self.pos_y = (self.pos_y + delta_y * delta()).min(1.0).max(-1.0);
			}
		}else{
			if self.target_l {
				self.pos_x = self.pos_x - 2.0 * delta();
			}else if self.target_r {
				self.pos_x = self.pos_x + 2.0 * delta();
			}else if self.target_o {
				self.pos_y = self.pos_y + 2.0 * delta();
			}else if self.target_u {
				self.pos_y = self.pos_y - 2.0 * delta();
			}else{
				self.pos_x = self.pos_x - (self.pos_x / 2.0);
				self.pos_y = self.pos_y - (self.pos_y / 2.0);
			}

			if self.pos_x.abs() < 0.025 {
				self.pos_x = 0.0;
			}
			if self.pos_y.abs() < 0.025 {
				self.pos_y = 0.0;
			}
		}

		self.is_right = self.pos_x > 0.8;
		self.is_left = self.pos_x < -0.8;
		self.is_top = self.pos_y > 0.8;
		self.is_bottom = self.pos_y < -0.8;
	}
}