use lotus_script::content::ContentId;

#[derive(Debug)]
pub struct Rollerblind {
    name: String,

    user_id: i32,
    base_sub_id: i32,

    tex_sub_id_last: i32,

    upper_tex: ContentId,
    lower_tex: ContentId,
    preload_tex: ContentId,
    shift: f32,
}

impl Rollerblind {
    pub fn new(name: &str, user_id: i32, base_sub_id: i32) -> Self {
        Self {
            name: name.to_string(),

            user_id: user_id,
            base_sub_id: base_sub_id,

            tex_sub_id_last: -1,

            upper_tex: ContentId {
                user_id: 0,
                sub_id: 0,
                version: 0.0,
            },
            lower_tex: ContentId {
                user_id: 0,
                sub_id: 0,
                version: 0.0,
            },
            preload_tex: ContentId {
                user_id: 0,
                sub_id: 0,
                version: 0.0,
            },
            shift: 0.0,
        }
    }

    pub fn tick(&mut self, display_id: f32) {
        let tex_sub_id = self.base_sub_id + (display_id as i32);
        self.shift = display_id.rem_euclid(1.0);

        if tex_sub_id != self.tex_sub_id_last {
            self.upper_tex = ContentId {
                user_id: self.user_id,
                sub_id: tex_sub_id,
                version: 0.0,
            };
            self.lower_tex = ContentId {
                user_id: self.user_id,
                sub_id: (tex_sub_id + 1).min(tex_sub_id),
                version: 0.0,
            };
            self.preload_tex = ContentId {
                user_id: self.user_id,
                sub_id: (tex_sub_id + 2).min(tex_sub_id),
                version: 0.0,
            };
        }

        self.tex_sub_id_last = tex_sub_id;
    }
}
