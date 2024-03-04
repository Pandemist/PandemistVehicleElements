#[derive(Default, Debug)]
pub struct VehicleDoor {
    _name_id: String,
    entry_available: bool,
    exit_available: bool,
}

impl VehicleDoor {
    pub fn new(name: String, entry_init: bool, exit_init: bool) -> Self {
        VehicleDoor {
            _name_id: name,
            entry_available: entry_init,
            exit_available: exit_init,
            ..Default::default()
        }
    }

    // Entspricht der Variable DoorOpen_#
    pub fn set_open(&mut self, open: bool) {
        todo!()
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
