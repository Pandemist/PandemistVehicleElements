use lotus_script::delta;

use crate::mocks::light::Light;

#[derive(Debug, Default)]
pub struct Blinkrelais {
    interval: f32,
    on_time: f32,
    timer: f32,
    pub is_on: bool,
    reset_time: f32,
}

impl Blinkrelais {
    pub fn new(new_interval: f32, new_on_time: f32, new_reset_time: f32) -> Self {
        Blinkrelais {
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
        } else if !new_on && self.is_on {
            -1
        } else {
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

#[derive(Debug)]
pub struct JanusBlinker {
    name: String,
    interval_on: f32,
    interval_off: f32,
    timer_a: f32,
    timer_b: f32,
    is_on_r_a: bool,
    is_on_r_b: bool,
    is_on_l_a: bool,
    is_on_l_b: bool,
    lighted_a: bool,
    lighted_b: bool,
    is_on_warn_a: bool,
    is_on_warn_b: bool,
    light_r_a: Light,
    light_r_b: Light,
    light_l_a: Light,
    light_l_b: Light,
}

impl JanusBlinker {
    pub fn new(name: &str, interval_on: f32, interval_off: f32) -> Self {
        Self {
            name: name.to_string(),
            interval_on: interval_on,
            interval_off: interval_off,
            timer_a: 0.0,
            timer_b: 0.0,
            is_on_r_a: false,
            is_on_r_b: false,
            is_on_l_a: false,
            is_on_l_b: false,
            is_on_warn_a: false,
            is_on_warn_b: false,
            lighted_a: false,
            lighted_b: false,
            light_r_a: Light::new(format!("light_{}_r_a", name)),
            light_r_b: Light::new(format!("light_{}_r_b", name)),
            light_l_a: Light::new(format!("light_{}_l_a", name)),
            light_l_b: Light::new(format!("light_{}_l_b", name)),
        }
    }

    pub fn tick(&mut self, spannung: f32) {
        if self.is_on_r_a || self.is_on_l_a || self.is_on_warn_a {
            self.timer_a += delta();

            if (self.timer_a > self.interval_on) && self.lighted_a {
                self.lighted_a = !self.lighted_a;
                self.timer_a -= self.interval_on;
            }
            if (self.timer_a > self.interval_off) && !self.lighted_a {
                self.lighted_a = !self.lighted_a;
                self.timer_a -= self.interval_off;
            }
        } else {
            self.lighted_a = false;
            self.timer_a = self.interval_off;
        }

        if self.is_on_r_b || self.is_on_l_b || self.is_on_warn_b {
            self.timer_b += delta();

            if (self.timer_b > self.interval_on) && self.lighted_b {
                self.lighted_b = !self.lighted_b;
                self.timer_b -= self.interval_on;
            }
            if (self.timer_b > self.interval_off) && !self.lighted_b {
                self.lighted_b = !self.lighted_b;
                self.timer_b -= self.interval_off;
            }
        } else {
            self.lighted_b = false;
            self.timer_b = self.interval_off;
        }

        let lighted_r = (self.lighted_a && (self.is_on_r_a || self.is_on_warn_a))
            || (self.lighted_b && (self.is_on_r_b || self.is_on_warn_b));
        let lighted_l = (self.lighted_a && (self.is_on_l_a || self.is_on_warn_a))
            || (self.lighted_b && (self.is_on_l_b || self.is_on_warn_b));

        self.light_r_a
            .update_light_level((lighted_r as u8 as f32) * spannung);
        self.light_r_b
            .update_light_level((!lighted_r as u8 as f32) * spannung);
        self.light_l_a
            .update_light_level((lighted_l as u8 as f32) * spannung);
        self.light_l_b
            .update_light_level((!lighted_l as u8 as f32) * spannung);
    }
}

#[derive(Debug)]
pub struct Blinker {
    name: String,
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
    pub fn new(name: &str, interval_on: f32, interval_off: f32) -> Self {
        Self {
            name: name.to_string(),
            interval_on: interval_on,
            interval_off: interval_off,
            timer: 0.0,
            is_on_r: false,
            is_on_l: false,
            is_on_warn: false,
            lighted: false,
            light_r_a: Light::new(format!("light_{}_r_a", name)),
            light_r_b: Light::new(format!("light_{}_r_b", name)),
            light_l_a: Light::new(format!("light_{}_l_a", name)),
            light_l_b: Light::new(format!("light_{}_l_b", name)),
        }
    }

    pub fn tick(&mut self, spannung: f32) {
        if self.is_on_r || self.is_on_l || self.is_on_warn {
            self.timer += delta();

            if (self.timer > self.interval_on) && self.lighted {
                self.lighted = !self.lighted;
                self.timer -= self.interval_on;
            }
            if (self.timer > self.interval_off) && !self.lighted {
                self.lighted = !self.lighted;
                self.timer -= self.interval_off;
            }
        } else {
            self.lighted = false;
            self.timer = self.interval_off;
        }

        let lighted_r_a = self.lighted && (self.is_on_r || self.is_on_warn);
        let lighted_r_b = !self.lighted && (self.is_on_r || self.is_on_warn);
        let lighted_l_a = self.lighted && (self.is_on_l || self.is_on_warn);
        let lighted_l_b = !self.lighted && (self.is_on_l || self.is_on_warn);

        self.light_r_a
            .update_light_level((lighted_r_a as u8 as f32) * spannung);
        self.light_r_b
            .update_light_level((lighted_r_b as u8 as f32) * spannung);
        self.light_l_a
            .update_light_level((lighted_l_a as u8 as f32) * spannung);
        self.light_l_b
            .update_light_level((lighted_l_b as u8 as f32) * spannung);
    }
}
