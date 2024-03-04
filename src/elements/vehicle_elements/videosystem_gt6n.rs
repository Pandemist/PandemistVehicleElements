use lotus_script::delta;

const VIDEOSYSTEM_BLINK:f32 = 1.5;
const VIDEOSYSTEM_BLINK_HALF:f32 = VIDEOSYSTEM_BLINK / 2.0;

#[derive(Debug, Default)]
pub struct VideoSystem{
	is_broken: bool,

	timer: f32,

	red: bool,
	grn: bool,
}

impl VideoSystem{
	pub fn new() -> Self {
		VideoSystem{
			is_broken: true,
			..Default::default()
		}
	}

	pub fn tick(&mut self, aktiv: bool) {
		if aktiv {
			if self.is_broken {
				self.timer = self.timer + delta();

				if self.timer > VIDEOSYSTEM_BLINK {
					self.timer = self.timer - VIDEOSYSTEM_BLINK;
				}
				self.red = self.timer > VIDEOSYSTEM_BLINK_HALF;
			}
		}else{
			self.red = false;
			self.grn = false;
			self.timer = 0.0;
		}
	}
}