#[derive(Debug, Clone, Copy)]
pub enum Steuerleitung {
    Batterie(bool),
    Spannung(f32),
}

impl PartialEq for Steuerleitung {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

pub struct Steuerleitungen {
    stl: Vec<Steuerleitung>,
}

impl Steuerleitungen {
    pub fn new() -> Self {
        Self { stl: Vec::new() }
    }

    pub fn set(&mut self, stl: Steuerleitung) {
        if let Some(pos) = self.stl.iter().position(|&x| x == stl) {
            self.stl[pos] = stl;
        } else {
            self.stl.push(stl);
        }
    }

    pub fn get(&mut self, stl: &Steuerleitung) -> Steuerleitung {
        match self.stl.iter().find(|&&x| x == *stl) {
            Some(&found) => found,
            None => *stl,
        }
    }
}
