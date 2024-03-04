use lotus_script::delta;

const SPRECHSTELLE_TIME:f32 = 1.0;
const SPRECHSTELLE_TIME_HALF:f32 = SPRECHSTELLE_TIME / 2.0;

#[derive(Debug, Default)]
pub struct SprechstelleGt6n {
	id: i32,

	aktiv: bool,
	aktiv_last: bool,
	confirmed: bool,

	blink_timer: f32,

	lm_rot: bool,
	lm_grn: bool,
	lm_glb: bool,
	snd_talk: i8,
}

impl SprechstelleGt6n {
	pub fn new(new_id: i32) -> Self {
		SprechstelleGt6n {
			id: new_id,
			..Default::default()
		}
	}

	pub fn tick(&mut self, current_activ: i32) {
		if current_activ != self.id {
			self.confirmed = false;
		}

		let other = current_activ >= 0 && current_activ != self.id;
		let waiting = current_activ == self.id && !self.confirmed;

		self.aktiv_last = self.aktiv;
		self.aktiv = waiting || self.confirmed;

		if !self.aktiv_last && self.aktiv {
			self.snd_talk = 1;
		}

		if !waiting {
			self.blink_timer = 0.0;
		}

		self.blink_timer += delta();
		
		if self.blink_timer > SPRECHSTELLE_TIME {
			self.blink_timer -= SPRECHSTELLE_TIME;
		}

		self.lm_rot = other;
		self.lm_grn = self.confirmed;
		self.lm_glb = waiting && (self.blink_timer > SPRECHSTELLE_TIME_HALF);
	}

	pub fn confirm(&mut self, current_activ: i32) {
		if current_activ == self.id {
			self.confirmed = true;
		}
	}
}