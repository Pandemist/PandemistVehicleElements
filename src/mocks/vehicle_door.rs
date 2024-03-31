#[derive(Default, Debug)]
pub struct VehicleDoor {
    _id: usize,
    entry_available: bool,
    exit_available: bool,
    open_last: bool,
    released_last: bool,
}

impl VehicleDoor {
    pub fn new(index: usize, entry_init: bool, exit_init: bool) -> Self {
        VehicleDoor {
            _id: index,
            entry_available: entry_init,
            exit_available: exit_init,
            ..Default::default()
        }
    }

    // Entspricht der Variable DoorOpen_#
    pub fn set_open(&mut self, open: bool) {
        todo!()
    }

    pub fn update_open(&mut self, open: bool) {
        if open != self.open_last {
            self.open_last = open;
            self.set_open(open);
        }
    }

    // Entspricht der Variable DoorEntryAvailable_#
    pub fn set_entry_available(&mut self, state: bool) {
        self.entry_available = state;
        todo!()
    }

    // Entspricht der Variable DoorExitAvailable_#
    pub fn set_exit_available(&mut self, state: bool) {
        self.exit_available = state;
        todo!()
    }

    // Entspricht der Variable DoorEntryReleased_#
    pub fn set_entry_released(&mut self, state: bool) {
        todo!()
    }

    // Entspricht der Variable DoorExitReleased_#
    pub fn set_exit_released(&mut self, state: bool) {
        todo!()
    }

    pub fn update_released(&mut self, state: bool) {
        if state != self.released_last {
            self.released_last = state;
            self.set_entry_released(state);
            self.set_exit_released(state);
        }
    }

    // Entspricht der Variable DoorReqIn_#
    pub fn request_in(&self) -> bool {
        todo!()
    }

    // Entspricht der Variable DoorReqOut_#
    pub fn request_out(&self) -> bool {
        todo!()
    }

    // Entspricht der Variable DoorOccupied_#
    pub fn occupied(&self) -> bool {
        todo!()
    }
}
