use lotus_script::delta;

use crate::structs::enums::SwitchingTarget;

#[derive(Default, Debug)]
pub struct Kleinspannung {
    const_batteriespannung_normal_v: f32,
    const_batteriespannung_max_v: f32,
    const_batteriespannung_min_v: f32,
    const_batteriespannung_loss_vs: f32,
    const_batteriespannung_load_vs: f32,

    batteriespannung_abs: f32,
    batteriespannung_abs_last: f32,

    batterie_switching_timer: f32,

    batterie_mainswitch: bool,
    batterie_mainswitch_last: bool,

    kleinspannung_abs: f32,
    kleinspannung_norm: f32,
    kleinspannung_norm_last: f32,
    dauerspannung_abs: f32,
    dauerspannung_norm: f32,
}

impl Kleinspannung {
    pub fn new(
        const_batteriespannung_normal_v: f32,
        const_batteriespannung_max_v: f32,
        const_batteriespannung_min_v: f32,
        const_batteriespannung_loss_vs: f32,
        const_batteriespannung_load_vs: f32,
        voltage: f32,
        mainswitch: bool,
    ) -> Self {
        let mut kl = Kleinspannung {
            const_batteriespannung_normal_v: const_batteriespannung_normal_v,
            const_batteriespannung_max_v: const_batteriespannung_max_v,
            const_batteriespannung_min_v: const_batteriespannung_min_v,
            const_batteriespannung_loss_vs: const_batteriespannung_loss_vs,
            const_batteriespannung_load_vs: const_batteriespannung_load_vs,

            batteriespannung_abs: voltage,

            batterie_mainswitch: mainswitch,
            ..Default::default()
        };
        kl.kleinspannung_abs = kl.batteriespannung_abs * ((kl.batterie_mainswitch as i32) as f32);
        kl.kleinspannung_norm = kl.kleinspannung_abs / kl.const_batteriespannung_normal_v;

        kl
    }

    pub fn tick(&mut self, umformerspannung: f32, batterie_target: SwitchingTarget) {
        // Batteriehauptschalter mit verzögerung einschalten oder ausschalten
        match batterie_target {
            SwitchingTarget::Einlegen(delay) => {
                self.batterie_switching_timer = self.batterie_switching_timer + delta();
                if self.batterie_switching_timer > delay {
                    self.batterie_mainswitch = true;
                }
            }
            SwitchingTarget::Auslegen(delay) => {
                self.batterie_switching_timer = self.batterie_switching_timer + delta();
                if self.batterie_switching_timer > delay {
                    self.batterie_mainswitch = false;
                }
            }
            _ => {
                self.batterie_switching_timer = 0.0;
            }
        }

        // Batterieschütz abfallen lassen, wenn die Batteriespannung den Mindeswert unterscheitet
        if self.batteriespannung_abs < self.const_batteriespannung_min_v {
            self.batterie_mainswitch = false;
        }

        if self.batterie_mainswitch {
            self.batteriespannung_abs = (self.batteriespannung_abs
                - self.const_batteriespannung_loss_vs * delta())
            .max(0.0);
            self.batteriespannung_abs = self.const_batteriespannung_max_v.min(
                self.batteriespannung_abs
                    + self.const_batteriespannung_load_vs * umformerspannung * delta(),
            );
        }

        self.kleinspannung_abs =
            self.batteriespannung_abs * ((self.batterie_mainswitch as i32) as f32);
        self.kleinspannung_norm = self.kleinspannung_abs / self.const_batteriespannung_normal_v;

        self.dauerspannung_abs = self.batteriespannung_abs;
        self.dauerspannung_norm = self.dauerspannung_abs / self.const_batteriespannung_normal_v;

        if self.batterie_mainswitch != self.batterie_mainswitch_last {
            todo!()
        }

        if (self.kleinspannung_norm - self.kleinspannung_norm_last).abs() > 0.05 {
            todo!()
        }

        self.batteriespannung_abs_last = self.batteriespannung_abs;
        self.batterie_mainswitch_last = self.batterie_mainswitch;
    }
}
