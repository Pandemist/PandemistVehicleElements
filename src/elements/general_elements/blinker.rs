use lotus_script::delta;

use crate::mocks::light::Light;

#[derive(Default, Debug)]
pub struct JanusBlinker {
    name_id: String,

    interval_on: f32,
    interval_off: f32,

    timer_a: f32,
    timer_b: f32,

    is_on_r_a: bool,
    is_on_r_b: bool,
    is_on_l_a: bool,
    is_on_l_b: bool,

    is_on_warn_a: bool,
    is_on_warn_b: bool,

    lighted_a: bool,
    lighted_b: bool,
    lighted_r: bool,
    lighted_l: bool,

    light_r_a: Light,
    light_r_b: Light,
    light_l_a: Light,
    light_l_b: Light,
}

impl JanusBlinker {
    pub fn new(name: String, new_interval_on: f32, new_interval_off: f32) -> Self {
        JanusBlinker {
            name_id: name.clone(),
            interval_on: new_interval_on,
            interval_off: new_interval_off,
            light_r_a: Light::new(format!("light_{}_r_a", name)),
            light_r_b: Light::new(format!("light_{}_r_b", name)),
            light_l_a: Light::new(format!("light_{}_l_a", name)),
            light_l_b: Light::new(format!("light_{}_l_b", name)),
            ..Default::default()
        }
    }

    pub fn tick(&mut self, spannung: f32) {
        if self.is_on_r_a || self.is_on_l_a || self.is_on_warn_a {
            self.timer_a = self.timer_a + delta();

            if self.timer_a > self.interval_on && self.lighted_a {
                self.lighted_a = !self.lighted_a;
                self.timer_a = self.timer_a - self.interval_on;
            }

            if self.timer_a > self.interval_off && !self.lighted_a {
                self.lighted_a = !self.lighted_a;
                self.timer_a = self.timer_a - self.interval_off;
            }
        } else {
            self.lighted_a = false;
            self.timer_a = self.interval_off;
        }

        if self.is_on_r_b || self.is_on_l_b || self.is_on_warn_b {
            self.timer_b = self.timer_b + delta();

            if self.timer_b > self.interval_on && self.lighted_b {
                self.lighted_b = !self.lighted_b;
                self.timer_b = self.timer_b - self.interval_on;
            }

            if self.timer_b > self.interval_off && !self.lighted_b {
                self.lighted_b = !self.lighted_b;
                self.timer_b = self.timer_b - self.interval_off;
            }
        } else {
            self.lighted_b = false;
            self.timer_b = self.interval_off;
        }

        self.lighted_r = (self.lighted_a && (self.is_on_r_a || self.is_on_warn_a))
            || (self.lighted_b && (self.is_on_r_b || self.is_on_warn_b));
        self.lighted_r = (self.lighted_a && (self.is_on_l_a || self.is_on_warn_a))
            || (self.lighted_b && (self.is_on_l_b || self.is_on_warn_b));

        self.light_r_a
            .update_light_level((self.lighted_r as i32 as f32) * spannung);
        self.light_r_b
            .update_light_level((!self.lighted_r as i32 as f32) * spannung);
        self.light_l_a
            .update_light_level((self.lighted_l as i32 as f32) * spannung);
        self.light_l_b
            .update_light_level((!self.lighted_l as i32 as f32) * spannung);
    }
}

#[derive(Default, Debug)]
pub struct Blinker {
    name_id: String,
    interval_on: f32,
    interval_off: f32,

    timer: f32,

    is_on_r: bool,
    is_on_l: bool,
    is_on_warn: bool,

    lighted: bool,

    light_r_a: Light,
    light_r_b: Light,
    light_l_a: Light,
    light_l_b: Light,
}

impl Blinker {
    pub fn new(name: String, new_interval_on: f32, new_interval_off: f32) -> Self {
        Blinker {
            name_id: name.clone(),
            interval_on: new_interval_on,
            interval_off: new_interval_off,
            light_r_a: Light::new(format!("light_{}_r_a", name)),
            light_r_b: Light::new(format!("light_{}_r_b", name)),
            light_l_a: Light::new(format!("light_{}_l_a", name)),
            light_l_b: Light::new(format!("light_{}_l_b", name)),
            ..Default::default()
        }
    }

    pub fn tick(&mut self, spannung: f32) {
        if self.is_on_r || self.is_on_l || self.is_on_warn {
            self.timer = self.timer + delta();
            if self.timer > self.interval_on && self.lighted {
                self.lighted = !self.lighted;
                self.timer = self.timer - self.interval_on;
            }
            if self.timer > self.interval_off && !self.lighted {
                self.lighted = !self.lighted;
                self.timer = self.timer - self.interval_off;
            }
        } else {
            self.timer = self.interval_off;
            self.lighted = false;
        }

        self.light_r_a.update_light_level(
            ((self.lighted && (self.is_on_r || self.is_on_warn)) as i32 as f32) * spannung,
        );
        self.light_r_b.update_light_level(
            ((!self.lighted && (self.is_on_r || self.is_on_warn)) as i32 as f32) * spannung,
        );
        self.light_l_a.update_light_level(
            ((self.lighted && (self.is_on_l || self.is_on_warn)) as i32 as f32) * spannung,
        );
        self.light_l_b.update_light_level(
            ((!self.lighted && (self.is_on_l || self.is_on_warn)) as i32 as f32) * spannung,
        );
    }
}
