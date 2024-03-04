use lotus_script::event::ButtonEvent;

use crate::{
    elements::tech_elements::slider::GravitySlider,
    mocks::{animation::Animation, generell::mouse_move, passenger_seat::PassengerSeat},
    structs::traits::OnButton,
};

#[derive(Debug, Default)]
pub struct Klappsitz {
    name_id: String,

    seat: GravitySlider,

    anim: Animation,

    fg_sitz: PassengerSeat,
}

impl Klappsitz {
    pub fn new(name: String, seat_id: String) -> Self {
        Klappsitz {
            name_id: name.clone(),
            seat: GravitySlider::new(0.2, 0.05, 2.4),
            anim: Animation::new(format!("{}_anim", name)),
            fg_sitz: PassengerSeat::new(format!("{}_seat", seat_id)),
        }
    }

    pub fn tick(&mut self) {
        self.seat.tick(mouse_move().x);
        self.fg_sitz.set_valid(self.seat.pos < 0.025);
        self.anim.update_pos(self.seat.pos);
    }
}

impl OnButton for Klappsitz {
    fn on_button(&mut self, ev: &ButtonEvent) {
        if ev.id == format!("{}_grab", self.name_id) {
            if self.fg_sitz.is_free() {
                self.seat.grabbing = true;
            } else {
                self.seat.grabbing = false;
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct DoppelKlappsitz {
    name_id: String,

    seat: GravitySlider,

    anim: Animation,

    fg_sitz_1: PassengerSeat,
    fg_sitz_2: PassengerSeat,
}

impl DoppelKlappsitz {
    pub fn new(name: String, seat_id: String) -> Self {
        DoppelKlappsitz {
            name_id: name.clone(),
            seat: GravitySlider::new(0.2, 0.05, 2.4),
            anim: Animation::new(format!("{}_anim", name)),
            fg_sitz_1: PassengerSeat::new(format!("{}_1_seat", seat_id)),
            fg_sitz_2: PassengerSeat::new(format!("{}_2_seat", seat_id)),
        }
    }

    pub fn tick(&mut self) {
        self.seat.tick(mouse_move().x);
        self.fg_sitz_1.set_valid(self.seat.pos < 0.025);
        self.fg_sitz_2.set_valid(self.seat.pos < 0.025);
        self.anim.update_pos(self.seat.pos);
    }
}

impl OnButton for DoppelKlappsitz {
    fn on_button(&mut self, ev: &ButtonEvent) {
        if ev.id == format!("{}_grab", self.name_id) {
            if self.fg_sitz_1.is_free() && self.fg_sitz_2.is_free() {
                self.seat.grabbing = true;
            } else {
                self.seat.grabbing = false;
            }
        }
    }
}
