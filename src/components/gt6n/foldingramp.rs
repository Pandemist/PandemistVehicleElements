use std::f32::consts::PI;

use lotus_script::time::delta;

use crate::{
    api::{
        animation::Animation,
        general::mouse_move,
        key_event::{KeyEvent, KeyEventCab},
        light::{BlinkRelais, Light},
    },
    elements::tech::{slider::Slider, switches::Switch},
};

const RAMP_DOORWARN_INTERVAL: f32 = 1.0;
const RAMP_DOORWARN_INTERVAL_HALF: f32 = RAMP_DOORWARN_INTERVAL / 2.0;

pub struct Foldingramp {
    lockpin: Switch,

    ramp: Slider,

    slider_event: KeyEvent,
    slider_pos: f32,
    slider_anim: Animation,
    slider_speed: f32,
    slider_friction: f32,
    slider_bump_factor: f32,

    coverage: Slider,

    ramp_platform: Slider,

    pub warnrelais: BlinkRelais,
    l_warning: Light,

    pub in_use: bool,
}

impl Foldingramp {
    #[must_use]
    pub fn new(name: &str, cab_side: KeyEventCab) -> Self {
        Foldingramp {
            lockpin: Switch::builder(
                format!("AV_{}_Klapprampe_Lockpin", String::from(cab_side)),
                Some(cab_side),
            )
            .event_toggle("Klapprampe_Lockpin")
            .build(),

            ramp: Slider::builder()
                .animation(format!("AV_{}_Klapprampe_Rampe", String::from(cab_side)))
                .key_event("Klapprampe_Rampe", Some(cab_side))
                .axis_x()
                .friction(10.0)
                .mouse_factor(-1.0 / 5.0)
                .lower_bumb_factor(0.1)
                .upper_bump_factor(0.25)
                .min(0.0)
                .max(202.7)
                .build(),

            coverage: Slider::builder()
                .animation(format!(
                    "AV_{}_Klapprampe_Abdeckung",
                    String::from(cab_side)
                ))
                .key_event("Klapprampe_Abdeckung", Some(cab_side))
                .axis_x()
                .upper_bump_factor(0.5)
                .force(2.4)
                .mouse_factor(1.0 / 250.0)
                .build(),

            ramp_platform: Slider::builder()
                .animation(format!(
                    "AV_{}_Klapprampe_Bahnsteigrampe",
                    String::from(cab_side)
                ))
                .key_event("Klapprampe_Bahnsteigrampe", Some(cab_side))
                .axis_x()
                .friction(10.0)
                .mouse_factor(-1.0 / 5.0)
                .lower_bumb_factor(0.0)
                .upper_bump_factor(0.25)
                .min(0.0)
                .max(283.7)
                .build(),

            warnrelais: BlinkRelais::new(RAMP_DOORWARN_INTERVAL, RAMP_DOORWARN_INTERVAL_HALF, 0.1),
            l_warning: Light::new(Some(&format!(
                "L_{}_Hubliftwarnung",
                String::from(cab_side)
            ))),

            slider_event: KeyEvent::new(Some("Klapprampe_Slider"), Some(cab_side)),
            slider_pos: 0.0,
            slider_anim: Animation::new(Some(&format!(
                "AV_{}_Klapprampe_Slider",
                String::from(cab_side)
            ))),
            slider_speed: 0.0,
            slider_friction: 0.0,
            slider_bump_factor: 0.0,

            in_use: false,
        }
    }

    pub fn tick(&mut self, allowed: bool) {
        let closed = (self.coverage.pos <= 0.01) && (self.ramp_platform.pos <= 0.01);

        if closed {
            self.lockpin.tick();
        }

        let can_use = allowed && self.lockpin.value(true);

        if can_use {
            self.warnrelais.tick();
        } else {
            self.warnrelais.reset();
        }
        self.l_warning
            .set_brightness(self.warnrelais.is_on as u8 as f32);

        let coverage_upper_bump = 1.0;

        self.coverage.max = coverage_upper_bump;
        if can_use {
            self.coverage.tick();
        }

        if self.coverage.pos > 0.3333 {
            if self.slider_event.is_pressed() {
                self.slider_pos -= mouse_move().x;
                self.slider_speed = mouse_move().x / delta();
            } else {
                self.slider_pos -= self.slider_speed * delta();
            }

            if self.slider_pos < 0.0 {
                self.slider_pos = 0.0;
                self.slider_speed *= -self.slider_bump_factor;
            }

            if self.slider_pos > 1.0 {
                self.slider_pos = 1.0;
                self.slider_speed *= -self.slider_bump_factor;
            }

            self.slider_anim.set(self.slider_pos);

            if self.slider_speed != 0.0 {
                let new_speed = self.slider_speed
                    + (-self.slider_speed.signum() * self.slider_friction) * delta();

                if new_speed * self.slider_speed < 0.0 {
                    self.slider_speed = 0.0;
                } else {
                    self.slider_speed = new_speed;
                }
            }
        }

        self.ramp.force = -600.0 * (self.ramp.pos * PI / 180.0).cos();
        self.ramp.tick();

        self.ramp_platform.force = -600.0 * (self.ramp_platform.pos * PI / 180.0).cos();
        if can_use {
            self.ramp_platform.tick();
        }

        self.in_use = self.lockpin.value(true);
    }
}
