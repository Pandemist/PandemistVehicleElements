use std::rc::Rc;

use lotus_script::var::Variable;

use crate::{
    api::{animation::Animation, bogie::ApiBogie, sound::Sound},
    elements::std::delay::Delay,
};

pub struct RailbrakesBuilder {
    const_force: f32,

    api_bogie: ApiBogie,
    animation_railbrake: Animation,
    delay_target: Delay<bool>,

    snd_railbrake_on: Sound,
    snd_railbrake_off: Sound,
    snd_friction_mapping: Rc<dyn Fn(f32) -> f32>,
    snd_friction_while_on: Variable<f32>,

    state: bool,
    state_last: bool,
}

impl RailbrakesBuilder {
    pub fn animation(mut self, name: impl Into<String>) -> Self {
        self.animation_railbrake = Animation::new(Some(&name.into()));
        self
    }

    pub fn snd_railbrake_on(mut self, name: impl Into<String>) -> Self {
        self.snd_railbrake_on = Sound::new_simple(Some(&name.into()));
        self
    }

    pub fn snd_railbrake_off(mut self, name: impl Into<String>) -> Self {
        self.snd_railbrake_off = Sound::new_simple(Some(&name.into()));
        self
    }

    pub fn snd_railbrake_friction(
        mut self,
        name: impl Into<String>,
        function: Rc<dyn Fn(f32) -> f32>,
    ) -> Self {
        self.snd_friction_mapping = function;
        self.snd_railbrake_off = Sound::new_simple(Some(&name.into()));
        self
    }

    pub fn build(self) -> Railbrakes {
        Railbrakes {
            const_force: self.const_force,
            api_bogie: self.api_bogie,
            animation_railbrake: self.animation_railbrake,
            delay_target: self.delay_target,
            snd_railbrake_on: self.snd_railbrake_on,
            snd_railbrake_off: self.snd_railbrake_off,
            snd_friction_mapping: self.snd_friction_mapping,
            snd_friction_while_on: self.snd_friction_while_on,
            state: self.state,
            state_last: self.state_last,
        }
    }
}

//===========================================================

pub struct Railbrakes {
    const_force: f32,

    api_bogie: ApiBogie,
    animation_railbrake: Animation,
    delay_target: Delay<bool>,

    snd_railbrake_on: Sound,
    snd_railbrake_off: Sound,
    snd_friction_mapping: Rc<dyn Fn(f32) -> f32>,
    snd_friction_while_on: Variable<f32>,

    pub state: bool,
    state_last: bool,
}

impl Railbrakes {
    pub fn builder(bogie_index: usize, brake_force: f32) -> RailbrakesBuilder {
        RailbrakesBuilder {
            const_force: brake_force,
            api_bogie: ApiBogie::new(bogie_index),
            animation_railbrake: Animation::new(None),
            delay_target: Delay::new(0.0, false),
            snd_railbrake_on: Sound::new_simple(None),
            snd_railbrake_off: Sound::new_simple(None),
            snd_friction_mapping: Rc::new(|x| x),
            snd_friction_while_on: Variable::new(""),
            state: false,
            state_last: false,
        }
    }

    pub fn tick(&mut self, target: bool, control_voltage: f32, brake_voltage: f32) {
        self.delay_target.tick(target);

        self.state = self.delay_target.output && control_voltage > 0.8;

        if !self.state_last && self.state {
            self.snd_railbrake_on.start();
        }

        if self.state_last && !self.state {
            self.snd_railbrake_off.start();
        }

        self.animation_railbrake.set(self.state as u8 as f32);

        //self.snd_railbrake
        //    .update_pitch((self.state as u8 as f32) * 1.5 - 0.5);

        self.snd_friction_while_on
            .set((self.snd_friction_mapping)(self.state as u8 as f32));

        self.api_bogie
            .railbrake_force((self.state as u8 as f32) * brake_voltage * self.const_force);

        self.state_last = self.state;
    }
}
