use lotus_script::time::delta;

use crate::{api::animation::Animation, elements::tech::dekaden::DecadeSwitch};

pub struct Odometer {
    frac: f32,
    full_km: u64,

    anim_frac: Animation,
    decades: Vec<DecadeSwitch>,
}

impl Odometer {
    pub fn new(init_km: f64, animation_names: Vec<&str>) -> Self {
        let mut decades = vec![];

        let mut init_values = init_km.trunc() as u64;

        for name in animation_names.iter().skip(1) {
            let init = if init_values > 0 {
                let v = init_values % 10;
                init_values /= 10;
                v as f32
            } else {
                0.0
            };

            decades.push(
                DecadeSwitch::builder(10, *name, None)
                    .rotation_speed(0.5)
                    .init_value(init)
                    .build(),
            );
        }

        Self {
            frac: (init_km - init_km.floor()) as f32,
            full_km: init_km.trunc() as u64,

            anim_frac: Animation::new(animation_names.first().copied()),
            decades,
        }
    }

    pub fn tick(&mut self, mps: f32) {
        if self.frac > 1.0 {
            self.frac -= 1.0;
            self.full_km += 1;
        };

        let frac_last = self.frac;
        self.frac += mps.abs() * delta() / 1000.0;

        let mut transfer = (self.frac.clamp(0.9, 1.0) - frac_last.clamp(0.9, 1.0)) * 10.0;

        self.decades.iter_mut().for_each(|decade| {
            transfer = decade.tick(transfer);
        });

        self.anim_frac.set(self.frac * 10.0);
    }

    pub fn get_km_h(&mut self) -> f64 {
        self.full_km as f64 + self.frac as f64
    }

    fn calc_digit(&self, km: i32, pre: f32) -> f32 {
        let mut result = (km % 10) as f32 / 10.0;

        if pre > 0.9 {
            result += pre - 0.9;
        }

        result
    }
}
