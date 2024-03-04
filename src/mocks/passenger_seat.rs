#[derive(Default, Debug)]
pub struct PassengerSeat {
    _name_id: String,
}

impl PassengerSeat {
    pub fn new(name: String) -> Self {
        PassengerSeat {
            ..Default::default()
        }
    }

    // Setzt die Verfügbarkeit eines Sitzplatzes (Kann ein FG sich dort hinsetzten) NEUES FEATURE
    pub fn set_valid(&mut self, state: bool) {
        todo!()
    }

    // Fragt ab ob ein Sitzplatz frei ist (False, sobald ein FG ab einsteigen bis aufstehen einen Sitz beansprucht) NEUES FEATURE
    pub fn is_free(&self) -> bool {
        todo!()
    }
}
