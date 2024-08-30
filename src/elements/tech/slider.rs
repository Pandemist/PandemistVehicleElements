use lotus_script::delta;

use crate::mocks::{
    animation::Animation,
    generell::mouse_move,
    key_event::{KeyEvent, KeyEventCab},
};

#[derive(Debug)]
pub struct SliderX {
    name: String,
    pub pos: f32,

    pub min: f32,
    pub max: f32,

    mouse_factor: f32,

    key_grab: KeyEvent,
    pos_anim: Animation,
}

impl SliderX {
    pub fn new(name: &str, cab_side: KeyEventCab, mouse_factor: f32) -> Self {
        SliderX {
            name: name.to_string(),
            pos: 0.0,
            min: 0.0,
            max: 1.0,
            mouse_factor: mouse_factor,
            key_grab: KeyEvent::new(format!("{}_grab", name), cab_side),
            pos_anim: Animation::new(format!("{}_anim", name)),
        }
    }

    pub fn set_pos(&mut self, new_pos: f32) {
        self.pos = new_pos;
        self.pos_anim.set(self.pos);
    }

    pub fn tick(&mut self) {
        let hand_delta = mouse_move().x * self.mouse_factor * delta();
        if self.key_grab.is_pressed() {
            if self.min > self.pos {
                self.pos = self
                    .min
                    .max(self.pos)
                    .max(self.max.min(self.pos + hand_delta));
            } else if self.max < self.pos {
                self.pos = self
                    .min
                    .max((self.max.min(self.pos)).min(self.pos + hand_delta));
            } else {
                self.pos = self.min.max(self.max.min(self.pos + hand_delta));
            }
        }
        self.pos_anim.set(self.pos);
    }
}

#[derive(Debug)]
pub struct SliderY {
    name: String,
    pub pos: f32,

    pub min: f32,
    pub max: f32,

    mouse_factor: f32,

    key_grab: KeyEvent,
    pos_anim: Animation,
}

impl SliderY {
    pub fn new(name: &str, cab_side: KeyEventCab, mouse_factor: f32) -> Self {
        SliderY {
            name: name.to_string(),
            pos: 0.0,
            min: 0.0,
            max: 1.0,
            mouse_factor: mouse_factor,
            key_grab: KeyEvent::new(format!("{}_grab", name), cab_side),
            pos_anim: Animation::new(format!("{}_anim", name)),
        }
    }

    pub fn set_pos(&mut self, new_pos: f32) {
        self.pos = new_pos;
        self.pos_anim.set(self.pos);
    }

    pub fn tick(&mut self) {
        let hand_delta = mouse_move().y * self.mouse_factor * delta();
        if self.key_grab.is_pressed() {
            if self.min > self.pos {
                self.pos = self
                    .min
                    .max(self.pos)
                    .max(self.max.min(self.pos + hand_delta));
            } else if self.max < self.pos {
                self.pos = self
                    .min
                    .max((self.max.min(self.pos)).min(self.pos + hand_delta));
            } else {
                self.pos = self.min.max(self.max.min(self.pos + hand_delta));
            }
        }
        self.pos_anim.set(self.pos);
    }
}

#[derive(Debug)]
pub struct GravitySlider {
    name: String,
    pos: f32,
    speed: f32,

    bumb_factor: f32,
    force: f32,

    mouse_factor: f32,

    pub min: f32,
    pub max: f32,

    key_grab: KeyEvent,
    pos_anim: Animation,
}

impl GravitySlider {
    pub fn new(
        name: &str,
        cab_side: KeyEventCab,
        bumb_factor: f32,
        force: f32,
        mouse_factor: f32,
    ) -> Self {
        GravitySlider {
            name: name.to_string(),
            pos: 0.0,
            speed: 0.0,
            bumb_factor: bumb_factor,
            force: force,
            mouse_factor: mouse_factor,
            min: 0.0,
            max: 1.0,
            key_grab: KeyEvent::new(format!("{}_grab", name), cab_side),
            pos_anim: Animation::new(format!("{}_anim", name)),
        }
    }

    pub fn set_pos(&mut self, new_pos: f32) {
        self.pos = new_pos;
        self.speed = 0.0;
        self.pos_anim.set(self.pos);
    }

    pub fn tick(&mut self) {
        let hand_delta = mouse_move().x * self.mouse_factor * delta();
        if self.key_grab.is_pressed() {
            self.pos = (self.pos + hand_delta).max(self.min).min(self.max);
            self.speed = 0.0;
        } else {
            if self.pos < (self.max - 0.01) {
                self.pos = self.pos - self.speed * delta();
            }

            if self.pos < self.min {
                self.pos = self.min;
                if self.bumb_factor > 0.0 {
                    self.speed = -self.bumb_factor * self.speed;
                } else {
                    self.speed = 0.0;
                }
            }
        }
        self.speed = self.speed + self.force * delta();
        self.pos_anim.set(self.pos);
    }
}

#[derive(Debug)]
pub struct AntiGravitySlider {
    name: String,
    pub pos: f32,
    speed: f32,

    bumb_factor: f32,
    force: f32,

    mouse_factor: f32,

    pub min: f32,
    pub max: f32,

    key_grab: KeyEvent,
    pos_anim: Animation,
}

impl AntiGravitySlider {
    pub fn new(
        name: &str,
        cab_side: KeyEventCab,
        bumb_factor: f32,
        force: f32,
        mouse_factor: f32,
    ) -> Self {
        AntiGravitySlider {
            name: name.to_string(),
            pos: 0.0,
            speed: 0.0,
            bumb_factor: bumb_factor,
            force: force,
            mouse_factor: mouse_factor,
            min: 0.0,
            max: 1.0,
            key_grab: KeyEvent::new(format!("{}_grab", name), cab_side),
            pos_anim: Animation::new(format!("{}_anim", name)),
        }
    }

    pub fn set_pos(&mut self, new_pos: f32) {
        self.pos = new_pos;
        self.speed = 0.0;
        self.pos_anim.set(self.pos);
    }

    pub fn tick(&mut self) {
        let hand_delta = mouse_move().x * self.mouse_factor * delta();
        if self.key_grab.is_pressed() {
            self.pos = (self.pos + hand_delta * delta())
                .max(self.max)
                .min(self.min);
            self.speed = 0.0;
        } else {
            if self.pos >= (self.min + 0.01) {
                self.pos = self.pos - self.speed * delta();
            }

            if self.pos > self.max {
                self.pos = self.max;
                if self.bumb_factor > 0.0 {
                    self.speed = -self.bumb_factor * self.speed;
                } else {
                    self.speed = 0.0;
                }
            }
        }
        self.speed = self.speed + self.force * delta();
        self.pos_anim.set(self.pos);
    }
}

#[derive(Debug)]
pub struct InertionSlider {
    name: String,
    pub pos: f32,
    speed: f32,

    pub min: f32,
    pub max: f32,

    daempfung: f32,
    lower_bumb_factor: f32,
    higher_bumb_factor: f32,
    pub force: f32,

    key_grab: KeyEvent,
    pos_anim: Animation,
}

impl InertionSlider {
    pub fn new(
        name: &str,
        cab_side: KeyEventCab,
        daempfung: f32,
        lower_bumb_factor: f32,
        higher_bumb_factor: f32,
        force: f32,
        min: f32,
        max: f32,
    ) -> Self {
        InertionSlider {
            name: name.to_string(),
            pos: 0.0,
            speed: 0.0,

            min: min,
            max: max,

            daempfung: daempfung,
            lower_bumb_factor: lower_bumb_factor,
            higher_bumb_factor: higher_bumb_factor,
            force: force,

            key_grab: KeyEvent::new(format!("{}_grab", name), cab_side),
            pos_anim: Animation::new(format!("{}_anim", name)),
        }
    }

    pub fn set_pos(&mut self, new_pos: f32) {
        self.pos = new_pos;
        self.speed = 0.0;
        self.pos_anim.set(self.pos);
    }

    pub fn tick(&mut self, hand_delta: f32) -> f32 {
        let mut result = 0.0;
        if self.key_grab.is_pressed() {
            self.pos = self.pos + hand_delta;
            self.speed = hand_delta / delta();
        } else {
            self.pos = self.pos + self.speed * delta();
        }

        if self.pos < self.min {
            self.pos = self.min;
            result = self.speed;
            if self.lower_bumb_factor > 0.0 {
                self.speed = -self.lower_bumb_factor * self.speed;
            } else {
                self.speed = 0.0;
            }
        }

        if self.pos > self.max {
            self.pos = self.max;
            result = self.speed;
            if self.higher_bumb_factor > 0.0 {
                self.speed = -self.higher_bumb_factor * self.speed;
            } else {
                self.speed = 0.0;
            }
        }

        self.speed = self.speed + self.force * delta();

        if self.speed != 0.0 {
            let new_speed = self.speed + (-self.speed.signum() * self.daempfung) * delta();

            self.speed = if new_speed * self.speed < 0.0 {
                0.0
            } else {
                new_speed
            };
        }

        result
    }
}

#[derive(Debug)]
pub struct LockableSlider {
    name: String,
    pub pos: f32,
    speed: f32,

    has_critical_zone: bool,
    critical_uppper: f32,
    critical_lower: f32,

    daempfung: f32,
    lower_bumb_factor: f32,
    higher_bumb_factor: f32,

    is_locked: bool,

    is_critical_blocked: bool,
    is_blocking_critical: bool,

    key_grab: KeyEvent,
    pos_anim: Animation,
}

impl LockableSlider {
    pub fn new(
        name: &str,
        cab_side: KeyEventCab,
        has_critical_zone: bool,
        critical_uppper: f32,
        critical_lower: f32,
        daempfung: f32,
        lower_bumb_factor: f32,
        higher_bumb_factor: f32,
    ) -> Self {
        LockableSlider {
            name: name.to_string(),
            pos: 0.0,
            speed: 0.0,
            has_critical_zone: has_critical_zone,
            critical_uppper: critical_uppper,
            critical_lower: critical_lower,
            daempfung: daempfung,
            lower_bumb_factor: lower_bumb_factor,
            higher_bumb_factor: higher_bumb_factor,
            is_locked: false,
            is_critical_blocked: false,
            is_blocking_critical: false,
            key_grab: KeyEvent::new(format!("{}_grab", name), cab_side),
            pos_anim: Animation::new(format!("{}_anim", name)),
        }
    }

    pub fn tick(&mut self, hand_delta: f32) {
        if self.is_locked && self.pos < 0.005 {
            self.pos = 0.0;
            return;
        }

        let lower_bumb_pos = if self.is_locked && self.pos > 0.05 {
            0.05
        } else {
            0.0
        };
        if self.key_grab.is_pressed() {
            if (self.is_locked && self.pos != 0.0) || !self.is_locked {
                if self.has_critical_zone && self.is_critical_blocked {
                    if self.pos >= self.critical_uppper {
                        self.pos = (self.pos + hand_delta).min(1.0).max(self.critical_uppper);
                    }
                    if self.pos <= self.critical_lower {
                        self.pos = (self.pos + hand_delta).max(0.0).min(self.critical_lower);
                    }
                } else {
                    self.pos = (self.pos + hand_delta).min(1.0).max(lower_bumb_pos);
                }
                self.speed = hand_delta / 5.0 / delta();
            }
        } else {
            self.pos = self.pos + self.speed * delta();
        }

        if self.has_critical_zone && self.is_critical_blocked {
            if (self.pos - self.critical_uppper).abs() < (self.pos - self.critical_lower).abs() {
                self.pos = self.critical_uppper;
                self.speed = if self.higher_bumb_factor > 0.0 {
                    -self.higher_bumb_factor * 0.5 * self.speed
                } else {
                    0.0
                };
            } else {
                self.pos = self.critical_lower;
                self.speed = if self.lower_bumb_factor > 0.0 {
                    -self.lower_bumb_factor * 0.5 * self.speed
                } else {
                    0.0
                };
            }
        }

        if self.pos < lower_bumb_pos {
            self.pos = lower_bumb_pos;
            self.speed = if self.lower_bumb_factor > 0.0 {
                -self.lower_bumb_factor * self.speed
            } else {
                0.0
            };
        }

        if self.pos > 1.0 {
            self.pos = 1.0;
            self.speed = if self.higher_bumb_factor > 0.0 {
                -self.higher_bumb_factor * self.speed
            } else {
                0.0
            };
        }

        self.is_blocking_critical =
            self.pos < self.critical_uppper && self.pos > self.critical_lower;

        if self.speed != 0.0 {
            let new_speed = self.speed + (-self.speed.signum() * self.daempfung) * delta();

            self.speed = if new_speed * self.speed < 0.0 {
                0.0
            } else {
                new_speed
            };
        }
        self.pos_anim.set(self.pos);
    }
}
