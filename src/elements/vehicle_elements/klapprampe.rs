use std::f32::consts::PI;

use lotus_script::delta;

use crate::elements::tech_elements::{slider::{AntiGravitySlider, InertionSlider}, warnrelais::Blinkrelais};

const KLAPPRAMPE_DOORWARN_INTERVAL:f32 = 1.0;
const KLAPPRAMPE_DOORWARN_INTERVAL_HALF:f32 = KLAPPRAMPE_DOORWARN_INTERVAL / 2.0;

#[derive(Debug, Default)]
pub struct Klapprampe{
	lockpin: bool,

	rampe: InertionSlider,

	slider_pos: f32,
	slider_speed: f32,
	slider_new_speed: f32,
	slider_grabbing: bool,
	slider_daempfer: f32,
	slider_bumpfactor: f32,

	abdeckung: AntiGravitySlider,

	rampe_bahnsteig: InertionSlider,

	warnrelais: Blinkrelais,

//	min_slider_out: f32,

	can_open: bool,
	can_close: bool,
}

impl Klapprampe{
	pub fn new() -> Self {
		Klapprampe{
		//	min_slider_out: 0.15,
			rampe: InertionSlider::new(10.0, 0.1, 0.25, 0.0, 0.0, 202.7),
			abdeckung: AntiGravitySlider::new(0.2, 0.5, 2.4),
			rampe_bahnsteig: InertionSlider::new(10.0, -1.0, 0.25, 0.0, 0.0, 183.7),
			warnrelais: Blinkrelais::new(KLAPPRAMPE_DOORWARN_INTERVAL, KLAPPRAMPE_DOORWARN_INTERVAL_HALF, 0.1),
			..Default::default()
		}
	}

	pub fn tick(&mut self, mouse_x:f32, mouse_y:f32, door_open_lock: bool) {
		self.can_open = door_open_lock;
		self.can_close = self.abdeckung.pos <= 0.01 && self.rampe_bahnsteig.pos <= 0.01;

		if self.lockpin {
			self.warnrelais.tick();
		}else{
			self.warnrelais.reset();
		}

		let abdeckung_upper_bump = 1.0;

		self.abdeckung.b = abdeckung_upper_bump;
		self.abdeckung.tick(mouse_y * (self.lockpin as i32 as f32));

		if self.abdeckung.pos > 0.3333 {
			if self.slider_grabbing {
				self.slider_pos = self.slider_pos - mouse_x;
				self.slider_speed = mouse_x / delta();
			}else{
				self.slider_pos = self.slider_pos - self.slider_speed * delta();
			}

			if self.slider_pos < 0.0 {
				self.slider_pos = 0.0;
				self.slider_speed = -self.slider_bumpfactor * self.slider_speed;
			}
			
			if self.slider_pos > 1.0 {
				self.slider_pos = 1.0;
				self.slider_speed = -self.slider_bumpfactor * self.slider_speed;
			}

			if self.slider_speed != 0.0 {
				self.slider_new_speed = self.slider_speed + (-self.slider_speed.signum() * self.slider_daempfer) * delta();

				if self.slider_new_speed * self.slider_speed < 0.0 {
					self.slider_speed = 0.0;
				}else{
					self.slider_speed = self.slider_new_speed;
				}
			}
		}

		self.rampe.force = -600.0 * (self.rampe.pos * PI / 180.0).cos();
		self.rampe.tick(-mouse_y / 5.0);

		self.rampe_bahnsteig.force = -600.0 * (self.rampe_bahnsteig.pos * PI / 180.0).cos();
		self.rampe_bahnsteig.tick((-mouse_y / 5.0) * (self.lockpin as i32 as f32));
	}
}