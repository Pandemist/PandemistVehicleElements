use lotus_script::time::delta;

use crate::api::{
    animation::Animation,
    key_event::{KeyEvent, KeyEventCab},
    light::Light,
    variable::{get_var, set_var},
    visible_flag::Visiblility,
};

const KOMBIBTN_PRESS_TIME: f32 = 0.5;

// INDOOR

pub struct SimpleInBtn {
    key_toggle: KeyEvent,

    ai_door_btns: Vec<usize>,
}

impl SimpleInBtn {
    pub fn new(
        cab_side: Option<KeyEventCab>,
        event_toggle_name: &str,
        ai_doors: Vec<usize>,
    ) -> Self {
        Self {
            key_toggle: KeyEvent::new(Some(event_toggle_name), cab_side),
            ai_door_btns: ai_doors,
        }
    }

    pub fn tick(&mut self) -> bool {
        let mut value = self.key_toggle.is_pressed();

        for ai_door in &self.ai_door_btns {
            let var_name = format!("DoorReqIn_{ai_door}");
            value = value || get_var::<bool>(&var_name);
            set_var(&var_name, false);
        }
        value
    }
}

//------------------------

pub struct DuoSpecialInBtn {
    buggy: bool,
    wheelchair: bool,

    key_kombi: KeyEvent,
}

impl DuoSpecialInBtn {
    pub fn new(cab_side: Option<KeyEventCab>, event_kombi_name: &str) -> Self {
        Self {
            buggy: false,
            wheelchair: false,
            key_kombi: KeyEvent::new(Some(event_kombi_name), cab_side),
        }
    }

    pub fn tick(&mut self, allowed: bool, leader: bool) {
        if self.key_kombi.is_pressed() {
            if leader {
                self.wheelchair = allowed;
            } else {
                self.buggy = allowed;
            }
        }
    }

    pub fn value_wheelchair(&mut self, allowed: bool) -> bool {
        if !allowed {
            self.reset_wheelchair();
        }

        self.wheelchair
    }

    pub fn value_buggy(&mut self, allowed: bool) -> bool {
        if !allowed {
            self.reset_buggy();
        }

        self.buggy
    }

    pub fn reset_wheelchair(&mut self) {
        self.wheelchair = false;
    }
    pub fn reset_buggy(&mut self) {
        self.buggy = false;
    }
}

//------------------------

pub struct SimpleWithStateInPushBtn {
    pub btn_anim: Animation,

    pub key_toggle: KeyEvent,

    btn_request_light: Light,

    ai_door_btns: Vec<usize>,
}

impl SimpleWithStateInPushBtn {
    pub fn new(
        cab_side: Option<KeyEventCab>,
        anim_name: impl Into<String>,
        event_toggle_name: &str,
        light_request_name: &str,
        ai_doors: Vec<usize>,
    ) -> Self {
        Self {
            btn_anim: Animation::new(Some(&anim_name.into())),
            key_toggle: KeyEvent::new(Some(event_toggle_name), cab_side),
            btn_request_light: Light::new(Some(light_request_name)),
            ai_door_btns: ai_doors,
        }
    }

    pub fn tick(&mut self, request_light: bool) -> bool {
        let mut value = self.key_toggle.is_pressed();

        if value {
            self.btn_anim.set(1.0);
        } else {
            self.btn_anim.set(0.0);
        }

        for ai_door in &self.ai_door_btns {
            let var_name = format!("DoorReqIn_{ai_door}");
            value = value || get_var::<bool>(&var_name);
            set_var(&var_name, false);
        }

        self.btn_request_light
            .set_brightness(request_light as u8 as f32);

        value
    }
}

//------------------------

pub struct DuwagDreieckIn2PushBtn {
    btn1_anim: Animation,
    btn2_anim: Animation,

    key1_toggle: KeyEvent,
    key2_toggle: KeyEvent,

    btn_light: Light,

    ai_door_btns: Vec<usize>,
}

impl DuwagDreieckIn2PushBtn {
    pub fn new(
        cab_side: Option<KeyEventCab>,
        anim1_name: impl Into<String>,
        anim2_name: impl Into<String>,
        event1_toggle_name: &str,
        event2_toggle_name: &str,
        light_name: &str,
        ai_doors: Vec<usize>,
    ) -> Self {
        Self {
            btn1_anim: Animation::new(Some(&anim1_name.into())),
            btn2_anim: Animation::new(Some(&anim2_name.into())),
            key1_toggle: KeyEvent::new(Some(event1_toggle_name), cab_side),
            key2_toggle: KeyEvent::new(Some(event2_toggle_name), cab_side),
            btn_light: Light::new(Some(light_name)),
            ai_door_btns: ai_doors,
        }
    }

    pub fn tick(&mut self, light: bool) -> bool {
        let mut value = self.key1_toggle.is_pressed() || self.key2_toggle.is_pressed();

        if self.key1_toggle.is_pressed() {
            self.btn1_anim.set(1.0);
        } else {
            self.btn1_anim.set(0.0);
        }
        if self.key2_toggle.is_pressed() {
            self.btn2_anim.set(1.0);
        } else {
            self.btn2_anim.set(0.0);
        }

        for ai_door in &self.ai_door_btns {
            let var_name = format!("DoorReqIn_{ai_door}");
            value = value || get_var::<bool>(&var_name);
            set_var(&var_name, false);
        }

        self.btn_light.set_brightness(light as u8 as f32);

        value
    }
}

// OUTDOOR

pub struct SimpleOutBtn {
    value: bool,

    key_toggle: KeyEvent,

    ai_door_btns: Vec<usize>,
}

impl SimpleOutBtn {
    pub fn new(
        cab_side: Option<KeyEventCab>,
        event_toggle_name: &str,
        ai_doors: Vec<usize>,
    ) -> Self {
        Self {
            value: false,
            key_toggle: KeyEvent::new(Some(event_toggle_name), cab_side),
            ai_door_btns: ai_doors,
        }
    }

    pub fn tick(&mut self) -> bool {
        let mut value = self.key_toggle.is_pressed();

        for ai_door in &self.ai_door_btns {
            let var_name = format!("DoorReqOut_{ai_door}");
            value = value || get_var::<bool>(&var_name);
            set_var(&var_name, false);
        }

        value
    }
}

pub struct SimpleOutPushBtn {
    btn_anim: Animation,

    key_toggle: KeyEvent,

    ai_door_btns: Vec<usize>,
}

impl SimpleOutPushBtn {
    pub fn new(
        cab_side: Option<KeyEventCab>,
        anim_name: impl Into<String>,
        event_toggle_name: &str,
        ai_doors: Vec<usize>,
    ) -> Self {
        Self {
            btn_anim: Animation::new(Some(&anim_name.into())),
            key_toggle: KeyEvent::new(Some(event_toggle_name), cab_side),
            ai_door_btns: ai_doors,
        }
    }

    pub fn tick(&mut self) -> bool {
        if self.key_toggle.is_pressed() {
            self.btn_anim.set(1.0);
        } else {
            self.btn_anim.set(0.0);
        }

        self.key_toggle.is_pressed()
    }
}

//------------------------

pub struct LightedOutPushBtn {
    btn_anim: Animation,

    key_toggle: KeyEvent,

    btn_light: Light,

    ai_door_btns: Vec<usize>,
}

impl LightedOutPushBtn {
    pub fn new(
        cab_side: Option<KeyEventCab>,
        anim_name: impl Into<String>,
        event_toggle_name: &str,
        light_name: &str,
        ai_doors: Vec<usize>,
    ) -> Self {
        Self {
            btn_anim: Animation::new(Some(&anim_name.into())),
            key_toggle: KeyEvent::new(Some(event_toggle_name), cab_side),
            btn_light: Light::new(Some(light_name)),
            ai_door_btns: ai_doors,
        }
    }

    pub fn tick(&mut self, light: bool) -> bool {
        let mut value = self.key_toggle.is_pressed();
        self.btn_light.set_brightness(light as u8 as f32);

        if self.key_toggle.is_pressed() {
            self.btn_anim.set(1.0);
        } else {
            self.btn_anim.set(0.0);
        }

        for ai_door in &self.ai_door_btns {
            let var_name = format!("DoorReqOut_{ai_door}");
            value = value || get_var::<bool>(&var_name);
            set_var(&var_name, false);
        }

        value
    }
}

//------------------------

pub struct RedGreenOutBtn {
    value: bool,
    pressed: bool,
    push_timer: f32,

    key_toggle: KeyEvent,

    green_vis: Visiblility,
    red_vis: Visiblility,

    ai_door_btns: Vec<usize>,
}

impl RedGreenOutBtn {
    pub fn new(
        cab_side: Option<KeyEventCab>,
        event_toggle_name: &str,
        green_vis_name: &str,
        red_vis_name: &str,
        ai_doors: Vec<usize>,
    ) -> Self {
        Self {
            value: false,
            pressed: false,
            push_timer: 0.0,
            key_toggle: KeyEvent::new(Some(event_toggle_name), cab_side),
            green_vis: Visiblility::new(green_vis_name),
            red_vis: Visiblility::new(red_vis_name),
            ai_door_btns: ai_doors,
        }
    }

    pub fn tick(&mut self, green_light: bool, red_light: bool) -> bool {
        let mut value = self.key_toggle.is_pressed();

        self.green_vis.set_visbility(green_light);
        self.red_vis
            .set_visbility(red_light || (self.push_timer > 0.0));

        if self.key_toggle.is_pressed() {
            self.push_timer = KOMBIBTN_PRESS_TIME;
        } else {
            self.push_timer = (self.push_timer - delta()).max(0.0);
        }

        for ai_door in &self.ai_door_btns {
            let var_name = format!("DoorReqOut_{ai_door}");
            value = value || get_var::<bool>(&var_name);
            set_var(&var_name, false);
        }

        value
    }
}

// Universal

pub struct RedGreenBtn {
    value: bool,
    push_timer: f32,

    key: KeyEvent,

    green_vis: Visiblility,
    red_vis: Visiblility,
}

impl RedGreenBtn {
    pub fn new(
        cab_side: Option<KeyEventCab>,
        event_name: &str,
        green_vis_name: &str,
        red_vis_name: &str,
    ) -> Self {
        Self {
            value: false,
            push_timer: 0.0,
            key: KeyEvent::new(Some(event_name), cab_side),
            green_vis: Visiblility::new(green_vis_name),
            red_vis: Visiblility::new(red_vis_name),
        }
    }

    pub fn tick(&mut self, green_light: bool, red_light: bool) {
        self.green_vis.set_visbility(green_light);
        self.red_vis
            .set_visbility(red_light || (self.push_timer > 0.0));

        if self.key.is_pressed() {
            self.push_timer = KOMBIBTN_PRESS_TIME;
            self.value = true;
        } else {
            self.push_timer = (self.push_timer - delta()).max(0.0);
        }

        if self.key.is_released() {
            self.value = false;
        }
    }

    pub fn value(&mut self, allowed: bool) -> bool {
        self.value && allowed
    }
}

//------------------------

pub struct RedGreenDuoBtn {
    buggy: bool,
    wheelchair: bool,
    push_timer: f32,

    key_buggy: KeyEvent,
    key_wheelchair: KeyEvent,

    green_vis: Visiblility,
    red_vis: Visiblility,
}

impl RedGreenDuoBtn {
    pub fn new(
        cab_side: Option<KeyEventCab>,
        event_buggy_name: &str,
        event_wheelchair_name: &str,
        green_vis_name: &str,
        red_vis_name: &str,
    ) -> Self {
        Self {
            buggy: false,
            wheelchair: false,
            push_timer: 0.0,
            key_buggy: KeyEvent::new(Some(event_buggy_name), cab_side),
            key_wheelchair: KeyEvent::new(Some(event_wheelchair_name), cab_side),
            green_vis: Visiblility::new(green_vis_name),
            red_vis: Visiblility::new(red_vis_name),
        }
    }

    pub fn tick(&mut self, green_light: bool, red_light: bool) {
        self.green_vis.set_visbility(green_light);
        self.red_vis
            .set_visbility(red_light || (self.push_timer > 0.0));

        if self.key_buggy.is_pressed() {
            self.push_timer = KOMBIBTN_PRESS_TIME;
            self.buggy = true;
        } else if self.key_wheelchair.is_pressed() {
            self.push_timer = KOMBIBTN_PRESS_TIME;
            self.wheelchair = true;
        } else {
            self.push_timer = (self.push_timer - delta()).max(0.0);
        }

        if self.key_buggy.is_released() {
            self.buggy = false;
        }
        if self.key_wheelchair.is_released() {
            self.wheelchair = false;
        }
    }

    pub fn value_wheelchair(&mut self) -> bool {
        self.wheelchair
    }

    pub fn value_buggy(&mut self) -> bool {
        self.buggy
    }
}

//------------------------
