#[derive(Debug)]
pub struct Entwertersteuergeraet {
    id: usize,
}

impl Entwertersteuergeraet {
    pub fn new(id: usize) -> Self {
        Self { id: id }
    }

    pub fn tick(&mut self) {}
}
