#[derive(Debug, Default)]
pub struct FourDirections {
    pub up: bool,
    pub down: bool,
    pub right: bool,
    pub left: bool,
}

impl FourDirections {
    pub fn new(up: bool, down: bool, right: bool, left: bool) -> Self {
        Self {
            up: up,
            down: down,
            right: right,
            left: left,
        }
    }

    pub fn is_one(&self) -> bool {
        self.up || self.down || self.right || self.left
    }
}
