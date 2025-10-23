use crate::api::{
    animation::Animation,
    general::mouse_move,
    key_event::{KeyEvent, KeyEventCab},
};

//======================================================================
// For GT6N microphone
//======================================================================

pub struct Slider3d {
    slide_x_pos: f32,
    slide_y_pos: f32,
    slide_z_pos: f32,

    mouse_factor: f32,

    pub key_grab: KeyEvent,
    pub key_pull: KeyEvent,

    pos_slide_x_anim: Animation,
    pos_slide_y_anim: Animation,
    pos_slide_z_anim: Animation,
}

impl Slider3d {
    pub fn new(
        cab_side: Option<KeyEventCab>,
        event_grab: &str,
        event_pull: &str,
        mouse_factor: f32,
        anim_x_name: impl Into<String>,
        anim_y_name: impl Into<String>,
        anim_z_name: impl Into<String>,
    ) -> Self {
        Self {
            slide_x_pos: 0.0,
            slide_y_pos: 0.0,
            slide_z_pos: 0.0,
            mouse_factor,
            key_grab: KeyEvent::new(Some(event_grab), cab_side),
            key_pull: KeyEvent::new(Some(event_pull), cab_side),
            pos_slide_x_anim: Animation::new(Some(&anim_x_name.into())),
            pos_slide_y_anim: Animation::new(Some(&anim_y_name.into())),
            pos_slide_z_anim: Animation::new(Some(&anim_z_name.into())),
        }
    }

    pub fn tick(&mut self) {
        let mouse_x = mouse_move().x * self.mouse_factor;
        let mouse_y = mouse_move().y * self.mouse_factor;

        if self.key_grab.is_pressed() {
            self.slide_z_pos = (self.slide_z_pos + mouse_x).clamp(0.0, 1.0);
            self.slide_x_pos = (self.slide_x_pos + mouse_y).clamp(0.0, 1.0);
        }
        if self.key_pull.is_pressed() {
            self.slide_y_pos = (self.slide_y_pos
                + ((mouse_y * self.slide_x_pos) - (mouse_x * (1.0 - self.slide_x_pos))))
                .clamp(0.0, 1.0);
        }

        self.pos_slide_x_anim.set(self.slide_x_pos);
        self.pos_slide_y_anim.set(self.slide_y_pos);
        self.pos_slide_z_anim.set(self.slide_z_pos);
    }
}
