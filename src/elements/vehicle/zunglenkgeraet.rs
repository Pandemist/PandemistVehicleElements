#[derive(Debug)]
pub struct Zuglenkgeraet {
    id: usize,
}

impl Zuglenkgeraet {
    pub fn new(id: usize) -> Self {
        Self { id: id }
    }

    pub fn tick(&mut self) {}
}
