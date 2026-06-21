use lotus_script::time::delta;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Timer {
    pub time: f32,
    finished: bool,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            time: -1.0,
            finished: false,
        }
    }

    pub fn idle(&mut self) -> bool {
        self.time < 0.0 && !self.finished
    }

    pub fn finished(&mut self) -> bool {
        self.finished
    }

    pub fn running(&mut self) -> bool {
        self.time >= 0.0
    }

    pub fn start(&mut self, time: f32) {
        self.time = time;
        self.finished = false;
    }

    pub fn reset(&mut self) {
        self.time = -1.0;
        self.finished = false;
    }

    pub fn tick(&mut self) -> bool {
        if self.time >= 0.0 {
            self.time -= delta();
            self.finished = self.time < 0.0;
            self.time < 0.0
        } else {
            false
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

//==========================================================

#[derive(Debug)]
pub struct TimerWithValue<T: Clone> {
    pub time: f32,
    finished: bool,
    value: T,
}

impl<T: Clone> TimerWithValue<T> {
    pub fn new(value: T) -> Self {
        Self {
            time: -1.0,
            finished: false,
            value,
        }
    }

    pub fn idle(&mut self) -> bool {
        self.time < 0.0 && !self.finished
    }

    pub fn finished(&mut self) -> bool {
        self.finished
    }

    pub fn running(&mut self) -> bool {
        self.time >= 0.0
    }

    pub fn start(&mut self, time: f32, value: T) {
        self.time = time;
        self.value = value;
        self.finished = false;
    }

    pub fn reset(&mut self) {
        self.time = -1.0;
        self.finished = false;
    }

    pub fn tick(&mut self) -> bool {
        if self.time >= 0.0 {
            self.time -= delta();
            self.finished = self.time < 0.0;
            self.time < 0.0
        } else {
            false
        }
    }

    pub fn value(&self) -> T {
        self.value.clone()
    }
}
