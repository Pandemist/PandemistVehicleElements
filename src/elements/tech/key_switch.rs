use crate::mocks::{
    animation::Animation,
    key_event::{KeyEvent, KeyEventCab},
    sound::{Sound, SoundTarget},
    variable::Variable,
    visible_flag::Visiblility,
};

#[derive(Debug)]
pub struct KeyDepot {
    key_inventory: Variable<bool>,
}

impl KeyDepot {
    pub fn new(key_depot: String) -> Self {
        Self {
            key_inventory: Variable::new(key_depot),
        }
    }

    pub fn testfor_key(&self) -> bool {
        self.key_inventory.get()
    }

    pub fn put_in(&self) {
        self.key_inventory.set(&true);
    }

    pub fn take_out(&self) {
        self.key_inventory.set(&false);
    }

    pub fn test_and_take_out(&self) -> bool {
        if self.testfor_key() {
            self.take_out();
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct KeySwitch {
    name: String,
    key_depot: KeyDepot,
    max: i32,
    min: i32,
    pos: f32,
    value: i32,

    min_pullout: bool,
    max_pullout: bool,
    pullout_values: Vec<i32>,

    min_spring: bool,
    max_spring: bool,

    key_anim: Animation,
    key_visibility: Visiblility,

    key_plus: KeyEvent,
    key_minus: KeyEvent,
    key_toggle: KeyEvent,

    snd_plus: Sound,
    snd_minus: Sound,
    snd_insert: Sound,
    snd_takeout: Sound,
}

impl KeySwitch {
    pub fn new(
        name: &str,
        cab_side: KeyEventCab,
        key_depot: String,
        sound_id: String,
        min: i32,
        max: i32,
        min_pullout: bool,
        max_pullout: bool,
        pullout_values: Vec<i32>,
        min_spring: bool,
        max_spring: bool,
    ) -> Self {
        Self {
            name: name.to_string(),
            key_depot: KeyDepot::new(key_depot),
            max: max,
            min: min,
            pos: 0.0,
            value: 0,

            min_pullout: min_pullout,
            max_pullout: max_pullout,
            pullout_values: pullout_values,

            min_spring: min_spring,
            max_spring: max_spring,

            key_anim: Animation::new(format!("{}_anim", name)),

            key_visibility: Visiblility::new(format!("{}_vis", name)),

            key_plus: KeyEvent::new(format!("{}_plus", name), cab_side),
            key_minus: KeyEvent::new(format!("{}_minus", name), cab_side),
            key_toggle: KeyEvent::new(format!("{}_toggle", name), cab_side),

            snd_plus: Sound::new(format!("{}_plus", sound_id)),
            snd_minus: Sound::new(format!("{}_minus", sound_id)),
            snd_insert: Sound::new(format!("{}_insert", sound_id)),
            snd_takeout: Sound::new(format!("{}_takeout", sound_id)),
        }
    }

    pub fn init(&mut self, new_pos: i32) {
        if self.key_depot.test_and_take_out() {
            self.key_visibility.make_visible();

            if self.min <= new_pos && new_pos >= self.max {
                self.pos = new_pos as f32;
                self.update();
            }
        }
    }

    pub fn tick(&mut self) {
        if self.key_visibility.check() {
            if self.key_plus.is_just_pressed() {
                if self.value < self.max {
                    self.pos += 1.0;
                    self.snd_plus.update_target(SoundTarget::Start);
                    self.update();
                } else if self.value == self.max && self.max_pullout {
                    self.key_visibility.make_invisible();
                    self.key_depot.put_in();
                    self.snd_takeout.update_target(SoundTarget::Start);
                }
            }

            if self.key_plus.is_just_released() && self.max_spring {
                if self.value == self.max {
                    self.pos -= 1.0;
                    self.snd_minus.update_target(SoundTarget::Start);
                    self.update();
                }
            }

            if self.key_minus.is_just_pressed() {
                if self.value > self.min {
                    self.pos -= 1.0;
                    self.snd_minus.update_target(SoundTarget::Start);
                    self.update();
                } else if self.value == self.min && self.min_pullout {
                    self.key_visibility.make_invisible();
                    self.key_depot.put_in();
                    self.snd_takeout.update_target(SoundTarget::Start);
                }
            }

            if self.key_minus.is_just_released() && self.min_spring {
                if self.value == self.min {
                    self.pos += 1.0;
                    self.snd_plus.update_target(SoundTarget::Start);
                    self.update();
                }
            }
        }

        if self.key_toggle.is_just_pressed() {
            if self.key_visibility.check() {
                if self.pullout_values.contains(&self.value) {
                    self.key_visibility.make_invisible();
                    self.key_depot.put_in();
                    self.snd_takeout.update_target(SoundTarget::Start);
                }
            } else {
                if self.key_depot.test_and_take_out() {
                    self.key_visibility.make_visible();
                    self.snd_insert.update_target(SoundTarget::Start)
                }
            }
        }
    }

    fn update(&mut self) {
        self.value = self.pos as i32;
        self.key_anim.set(self.pos);
    }
}
