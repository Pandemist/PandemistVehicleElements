#[derive(Default, Debug)]
pub struct Oberleitungsspannung {
    const_oberleistungsspannung_v: f32,

    oberleitungsspannung: bool,
    oberleitungsspannung_abs: f32,
    oberleitungsspannung_norm: f32,

    fahrspannung: bool,
    fahrspannung_abs: f32,
    fahrspannung_norm: f32,
}

impl Oberleitungsspannung {
    pub fn new(
            
        init_const_oberleistungsspannung_v: f32,
    ) -> Self {
        Oberleitungsspannung{
            const_oberleistungsspannung_v: init_const_oberleistungsspannung_v,
            ..Default::default()
        }
    }

    pub fn tick(&mut self, wagenautomat: bool, fahrdrahtspannung: bool) {
        self.oberleitungsspannung = fahrdrahtspannung;
        self.oberleitungsspannung_abs = self.const_oberleistungsspannung_v * (self.oberleitungsspannung as i32 as f32);
        self.oberleitungsspannung_norm = self.oberleitungsspannung_abs / self.const_oberleistungsspannung_v;

        self.fahrspannung = fahrdrahtspannung && wagenautomat;
        self.fahrspannung_abs = self.const_oberleistungsspannung_v * (self.fahrspannung as i32 as f32);
        self.fahrspannung_norm = self.fahrspannung_abs / self.const_oberleistungsspannung_v;
    }
}