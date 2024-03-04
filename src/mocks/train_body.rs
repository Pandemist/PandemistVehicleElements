#[derive(Default, Debug)]
pub struct Trainbody {
    _name_id: String,
}

impl Trainbody {
    pub fn new(name: String) -> Self {
        Trainbody {
            _name_id: name,
            ..Default::default()
        }
    }

    // Ausgabe der Z-Stellungsvariable eines Wagenkasten
    pub fn z_position(&self) -> f32 {
        todo!()
    }

    // Setzten der Z-Stellung eines Wagenkastens
    // Was genau Z-Stellung ist und wie das Funktioniert weiß ich nicht (da nicht dokumentiert)
    // Generell fände ich es ganz gut, wenn man man aber irgendwie den Winkel in Grad bekommen könnte, wie 2 Wagenkästen zueinander verdreht sind.
    // Für Beispielsweise Schlingerdämpfer auf Fahrzeugzeugdächern (siehe GT6N)
    pub fn set_z_position(&mut self, new_pos: f32) {
        todo!()
    }
}
