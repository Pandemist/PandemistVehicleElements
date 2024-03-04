use lotus_script::event::{BroadcastValue, ReceiveMessageValue};

pub fn send_boradcast(_bus_id: String, _id: String, _value: BroadcastValue) {
    todo!()
}

pub fn send_message_to_child(_slot_index: i32, _id: String, _value: ReceiveMessageValue) {
    todo!()
}

pub fn send_message_to_parent(_slot_index: i32, _id: String, _value: ReceiveMessageValue) {
    todo!()
}

pub fn send_message_to_trigger(_id: String, _value: String, _sensor_index: i32) -> Option<String> {
    todo!()
}

pub fn snd_event(_id: String, _value: i32) {
    todo!()
}
