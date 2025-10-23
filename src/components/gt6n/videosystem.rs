use lotus_script::time::delta;

use crate::api::visible_flag::Visiblility;

const VIDEOSYSTEM_BLINK: f32 = 1.5;
const VIDEOSYSTEM_BLINK_HALF: f32 = VIDEOSYSTEM_BLINK / 2.0;

#[derive(Debug)]
pub struct VideoSystemGt6n {
    is_broken: bool,

    timer: f32,

    red: Visiblility,
    green: Visiblility,
}

impl VideoSystemGt6n {
    #[must_use]
    pub fn new(green_led_name: impl Into<String>, red_led_name: impl Into<String>) -> Self {
        Self {
            red: Visiblility::new(red_led_name.into()),
            green: Visiblility::new(green_led_name.into()),
            timer: 0.0,
            is_broken: true, // Standard to Lotus video images supported
        }
    }

    pub fn tick(&mut self, aktiv: bool, spannung: f32) {
        if aktiv {
            if self.is_broken {
                self.timer += delta();

                if self.timer > VIDEOSYSTEM_BLINK {
                    self.timer -= VIDEOSYSTEM_BLINK;
                }
                self.red
                    .set_visbility(self.timer > VIDEOSYSTEM_BLINK_HALF && spannung > 0.5);
            }
        } else {
            self.red.make_invisible();
            self.green.make_invisible();
            self.timer = 0.0;
        }
    }
}
