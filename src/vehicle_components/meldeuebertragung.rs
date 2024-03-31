use std::collections::HashMap;

use crate::mocks::communication::send_message_to_trigger;

#[derive(Default, Debug)]
pub struct Meldeuebertragung {
    sensor_map: HashMap<i32, MueEntry>,
    iws_defect: bool,
}

#[derive(Default, Debug)]
pub struct MueEntry {
    aktiv: bool,
    id: String,
    value: String,
}

impl Meldeuebertragung {
    pub fn new() -> Self {
        Meldeuebertragung {
            sensor_map: HashMap::new(),
            ..Default::default()
        }
    }

    pub fn update_sensor_value(
        &mut self,
        sensor_id: i32,
        sensor_activ: bool,
        id: String,
        sensor_val: String,
    ) {
        self.sensor_map.insert(
            sensor_id,
            MueEntry {
                aktiv: sensor_activ,
                id: id,
                value: sensor_val,
            },
        );
    }

    pub fn trigger_handler(&mut self, id: &str, sensor_index: i32) {
        if !self.iws_defect {
            let map_entry = self.sensor_map.get(&sensor_index);

            match map_entry {
                Some(v) => {
                    if v.aktiv {
                        handle_reply(send_message_to_trigger(
                            v.id.clone(),
                            v.value.clone(),
                            sensor_index,
                        ));
                    }
                }
                None => {}
            }
        }
    }
}

fn handle_reply(reply: Option<String>) {
    match reply {
        Some(r) => {
            for pair in r.split(',') {
                let key_value: Vec<&str> = pair.split("=").collect();

                if key_value.len() >= 1 {
                    let key = key_value[0].trim().to_string();
                    let value = if key_value.len() >= 2 {
                        Some(key_value[1].trim().to_string())
                    } else {
                        None
                    };

                    //trigger_event(InternalEvents::IMU(key, value));
                }
            }
        }
        None => {}
    }
}
