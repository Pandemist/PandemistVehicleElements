use lotus_script::delta;

#[derive(Debug, Default)]
pub struct Kurbel{
	pos: f32,
	speed: f32,

	a: f32,
	b: f32,

	plus: bool,
	minus: bool,
}

impl Kurbel{
	pub fn new(new_speed: f32) -> Self {
		Kurbel{
			speed: new_speed,
			..Default::default()
		}
	}

	pub fn tick(&mut self) {
		if self.plus {
			self.pos = (self.pos + self.speed * delta()).min(self.b);
		}
		if self.minus {
			self.pos = (self.pos - self.speed * delta()).max(self.a);
		}
	}
}