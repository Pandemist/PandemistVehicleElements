use crate::mocks::coupler::CouplerLine;

#[derive(Default, Debug)]
pub struct OrSteuerleitung {
    name_id: String,

    couple: CouplerLine<bool>,

    pub value: bool,
}

impl OrSteuerleitung {
    pub fn new(name: String) -> Self {
        OrSteuerleitung {
            name_id: name.clone(),
            couple: CouplerLine::new(name),
            ..OrSteuerleitung::default()
        }
    }

    pub fn tick(&mut self, new_value: bool) {
        self.value = self.value || new_value;

        if self.couple.front().is_some() {
            self.value = self.value || self.couple.front().unwrap()
        }

        if self.couple.back().is_some() {
            self.value = self.value || self.couple.back().unwrap()
        }
    }
}

#[derive(Default, Debug)]
pub struct AndSteuerleitung {
    name_id: String,

    couple: CouplerLine<bool>,

    pub value: bool,
}

impl AndSteuerleitung {
    pub fn new(name: String) -> Self {
        AndSteuerleitung {
            name_id: name.clone(),
            couple: CouplerLine::new(name),
            ..AndSteuerleitung::default()
        }
    }

    pub fn tick(&mut self, new_value: bool) {
        self.value = self.value && new_value;

        if self.couple.front().is_some() {
            self.value = self.value && self.couple.front().unwrap()
        }

        if self.couple.back().is_some() {
            self.value = self.value && self.couple.back().unwrap()
        }
    }
}
