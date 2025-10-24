use std::collections::HashMap;

use lotus_script::{
    message::Coupling,
    prelude::{message_type, send_message, Message, MessageTarget},
};
use serde::{Deserialize, Serialize};

//===================================================================
// TrainBus coupling condition
//===================================================================

///
/// Information to the trainBus as to whether the electric coupling has been opened or closed
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EcouplerState {
    pub side: Coupling,
    pub value: bool,
}

message_type!(EcouplerState, "Std_TrainBus", "Ecoupler");

#[derive(Default, Debug)]
pub struct EcouplerSender {
    value_front_last: bool,
    value_rear_last: bool,
}

impl EcouplerSender {
    pub fn new() -> Self {
        Self {
            value_front_last: false,
            value_rear_last: false,
        }
    }

    pub fn update(&mut self, value: bool, side: Coupling) {
        match side {
            Coupling::Front => self.update_front(value),
            Coupling::Rear => self.update_rear(value),
        }
    }

    pub fn update_front(&mut self, value: bool) {
        if value != self.value_front_last {
            send_message(
                &(EcouplerState {
                    side: Coupling::Front,
                    value,
                }),
                [MessageTarget::Myself],
            );
            self.value_front_last = value;
        }
    }

    pub fn update_rear(&mut self, value: bool) {
        if value != self.value_rear_last {
            send_message(
                &(EcouplerState {
                    side: Coupling::Rear,
                    value,
                }),
                [MessageTarget::Myself],
            );
            self.value_rear_last = value;
        }
    }
}

//===================================================================
// TrainbusTelegram
//===================================================================

///
/// Communication of the TrainBus with each other (transmitted via the coupling) NOT intended for processing
/// by other structures. These are informed by blocking
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InternTelegram {
    pub master_pos: Option<u32>,
    pub vehicle_config: Vec<VehicleConfig>,
}

message_type!(InternTelegram, "Std_TrainBus_Intern", "TrainBusTelegram");

//===================================================================
// Trainbus Train Config
//===================================================================

///
/// Information on how the vehNubmber list looks on the train. Provided by the TrainBus in the carriage.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrainConfig {
    pub config_self: VehicleConfig,
    pub configs_front: Vec<VehicleConfig>,
    pub configs_rear: Vec<VehicleConfig>,
}

message_type!(TrainConfig, "Std_TrainBus", "TrainConfig");

//===================================================================
// TrainBus master position
//===================================================================

///
/// Information from the TrainBus as to whether the master is in the train. Communicated by the TrainBus in the carriage.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrainBusMaster {
    pub value: bool,
}

message_type!(TrainBusMaster, "Std_TrainBus", "MasterActiv");

//===================================================================
// Master TrainBus communication
//===================================================================

///
/// Information from the Ibis to the TrainBus as to whether it has taken over the position of the master (true)
/// or is switching back to the slave state (false)
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IbisState {
    pub is_master: bool,
}

message_type!(IbisState, "Std_TrainBus", "IbisState");

#[derive(Default, Debug)]
pub struct IbisStateSender {
    is_master_last: bool,
}

impl IbisStateSender {
    pub fn send(&mut self, is_master: bool) {
        if is_master != self.is_master_last {
            send_message(
                &(IbisState { is_master }),
                [MessageTarget::Broadcast {
                    across_couplings: false,
                    include_self: true,
                }],
            );
            self.is_master_last = is_master;
        }
    }
}

//===================================================================
// TrainBus Perifiery
//===================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PeripheryKind {
    MainIbis,                  // IBIS Master (MAS)
    TrainBusModul,             // Zugbus (ZB / ZBM)
    Redbox,                    // Kurzwegregister (Redbox) (KWR)
    InductivTransmissionModul, // Induktive Meldeübertragung (IMU)
    RadioModul,                // Funk-Modul (FUM)
    AnnouncementModul,         // Ansagengerät (ANS)
    DisplayGeneral,            // Anzeige (Allgemein) (ANZ)
    DisplayOuter,              // Anzeige (Aussen) (ANZ)
    DisplayInner,              // Anzeige (Innen) (ANZ)
    Validator,                 // Entwerter (ENTW)
    IbisTerminal,              // Ibis Terminal (TM)
    TrainProtection,           // Zugsicherung (INDUSI)
    TicketMachine,             // Fahrscheinautomat (VVG)
    Printer,                   // Drucker
    Iris,                      // IRIS
    VideoSystem,               // VIDEO
    Other {
        short_name: String,
        full_name: String,
    }, // Non-standard compliant devices
}

impl PeripheryKind {
    pub fn is_same_kind(&self, other: &PeripheryKind) -> bool {
        match (self, other) {
            (
                PeripheryKind::Other { short_name: s1, .. },
                PeripheryKind::Other { short_name: s2, .. },
            ) => s1 == s2,

            (a, b) => std::mem::discriminant(a) == std::mem::discriminant(b),
        }
    }
}

//---------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PeripheryFault {
    Defect,     // Defekt
    Disrupted,  // Gestört
    NoAnswer,   // Antwortet nicht
    BatteryLow, // voltage schwach
    Ok,         // Ok
    Undefined {
        short_text: String,
        long_text: String,
    }, // Non-standard message
}

//===================================================================

///
/// Registration of peripheral devices on the TrainBus (only used at the start of the simulation)
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PeripheryRegister {
    pub id: u32,
    pub kind: PeripheryKind,
}

message_type!(PeripheryRegister, "Std_TrainBus", "PeripheryRegister");

//---------------------------------------------

///
/// The periphery transmits information about its functional status.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PeripheryFaultReport {
    pub id: u32,
    pub kind: PeripheryFault,
}

message_type!(PeripheryFaultReport, "Std_TrainBus", "PeripheryFaultReport");

//---------------------------------------------

///
/// The periphery transmits information about its functional status.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IbisFaultReport {
    pub car: u32,
    pub coupling: Option<Coupling>,

    pub kind: PeripheryKind,
    pub counter: u32,

    pub state: PeripheryFault,
}

message_type!(IbisFaultReport, "Std_TrainBus", "PeripheryReport");

//---------------------------------------------

///
/// The train bus sends error messages about the train bus in the direction of the master.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InternFaultReport {
    pub car: u32,

    pub kind: PeripheryKind,
    pub counter: u32,

    pub state: PeripheryFault,
}

message_type!(InternFaultReport, "Std_TrainBus_Intern", "FaultReport");

//===================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VehicleConfig {
    pub number: String,
    pub periphery: Vec<PeripheryElement>,
}

impl VehicleConfig {
    pub fn count_kind(&self, new_element: &PeripheryKind) -> usize {
        self.periphery
            .iter()
            .filter(|elem| new_element.is_same_kind(&elem.kind))
            .count()
    }

    pub fn find_by_id(&mut self, id: u32) -> Option<PeripheryElement> {
        self.periphery.iter().find(|e| e.id == id).cloned()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PeripheryElement {
    id: u32,

    kind: PeripheryKind,
    counter: u32,
}

//===================================================================
// TrainBus
//===================================================================

#[derive(Debug)]
pub struct TrainBusManager {
    my_adress_map: HashMap<u32, u32>,
    my_vehicle_config: VehicleConfig,
    //my_perifery_list: Vec<PeripheryElement>,
    my_perifery_faults: HashMap<u32, PeripheryFault>,

    //veh_number: String,
    veh_config_list_received: (Vec<VehicleConfig>, Vec<VehicleConfig>),
    master_pos_received: (Option<u32>, Option<u32>),

    veh_config_list_last_send: (Vec<VehicleConfig>, Vec<VehicleConfig>),
    master_pos_last_send: (Option<u32>, Option<u32>),

    veh_config_list_last_local: (Vec<VehicleConfig>, Vec<VehicleConfig>),
    veh_config_self_last_local: VehicleConfig,
    master_pos_last_local: bool,
    am_i_master: bool,

    train_bus_error: bool,

    is_coupled: (bool, bool),
}

impl TrainBusManager {
    pub fn new(veh_number: String, adress_map: Option<HashMap<u32, u32>>) -> Self {
        Self {
            my_adress_map: adress_map.unwrap_or_default(),
            my_vehicle_config: VehicleConfig {
                number: veh_number,
                periphery: Vec::new(),
            },
            //my_perifery_list: Vec::new(),
            my_perifery_faults: HashMap::new(),

            //veh_number,
            veh_config_list_received: (Vec::new(), Vec::new()),
            master_pos_received: (None, None),

            veh_config_list_last_send: (Vec::new(), Vec::new()),
            master_pos_last_send: (None, None),

            veh_config_list_last_local: (Vec::new(), Vec::new()),
            veh_config_self_last_local: VehicleConfig {
                number: "".to_string(),
                periphery: Vec::new(),
            },
            master_pos_last_local: false,
            am_i_master: false,

            train_bus_error: false,

            is_coupled: (false, false),
        }
    }

    fn find_adress(&mut self, id: &u32, kind: &PeripheryKind) -> u32 {
        // Check whether an address is stored for the ID in my_address_map
        if let Some(&adress) = self.my_adress_map.get(id) {
            // Find the index of the conflicting element
            let conflict_index = self.my_vehicle_config.periphery.iter().position(|elem| {
                kind.is_same_kind(&elem.kind) && elem.counter == adress && elem.id != *id
            });

            if let Some(idx) = conflict_index {
                // Conflict found - find a new address for the conflicting element
                let mut new_adress = 1;
                loop {
                    let is_taken =
                        self.my_vehicle_config.periphery.iter().any(|elem| {
                            kind.is_same_kind(&elem.kind) && elem.counter == new_adress
                        });

                    if !is_taken {
                        // Assign the new address to the conflicting element
                        self.my_vehicle_config.periphery[idx].counter = new_adress;
                        break;
                    }
                    new_adress += 1;
                }
            }

            // Return the address from my_address_map
            return adress;
        }

        // No address in my_address_map - find the first available address
        let mut adress = 1;
        loop {
            let is_taken = self
                .my_vehicle_config
                .periphery
                .iter()
                .any(|elem| kind.is_same_kind(&elem.kind) && elem.counter == adress);

            if !is_taken {
                return adress;
            }
            adress += 1;
        }
    }

    fn update(&mut self) {
        let mut new_veh_config_front = Vec::new();
        new_veh_config_front.extend(self.veh_config_list_received.1.clone());
        new_veh_config_front.push(self.my_vehicle_config.clone());

        let mut new_veh_config_rear = Vec::new();
        new_veh_config_rear.extend(self.veh_config_list_received.0.clone());
        new_veh_config_rear.push(self.my_vehicle_config.clone());

        let new_master_pos_front = if self.am_i_master {
            Some(1)
        } else {
            self.master_pos_received.1.map(|s| s + 1)
        };
        let new_master_pos_rear = if self.am_i_master {
            Some(1)
        } else {
            self.master_pos_received.0.map(|s| s + 1)
        };

        if (new_veh_config_front != self.veh_config_list_last_send.0
            || new_master_pos_front != self.master_pos_last_send.0)
            && self.is_coupled.0
        {
            self.veh_config_list_last_send.0 = new_veh_config_front.clone();
            self.master_pos_last_send.0 = new_master_pos_front;

            send_message(
                &InternTelegram {
                    master_pos: new_master_pos_front,
                    vehicle_config: new_veh_config_front,
                },
                [MessageTarget::AcrossCoupling {
                    coupling: Coupling::Front,
                    cascade: false,
                }],
            );
        }

        if (new_veh_config_rear != self.veh_config_list_last_send.1
            || new_master_pos_rear != self.master_pos_last_send.1)
            && self.is_coupled.1
        {
            self.veh_config_list_last_send.1 = new_veh_config_rear.clone();
            self.master_pos_last_send.1 = new_master_pos_rear;

            send_message(
                &InternTelegram {
                    master_pos: new_master_pos_rear,
                    vehicle_config: new_veh_config_rear,
                },
                [MessageTarget::AcrossCoupling {
                    coupling: Coupling::Rear,
                    cascade: false,
                }],
            );
        }

        if self.is_master_there() && !self.master_pos_last_local {
            let mut entries: Vec<_> = self.my_perifery_faults.iter().collect();
            entries.sort_by_key(|(&k, _)| k);
            for (key, fault) in entries {
                if let Some(pe) = self.my_vehicle_config.find_by_id(*key) {
                    self.send_fault_to_master(1, pe.kind.clone(), pe.counter, fault.clone());
                }
            }
        }

        self.send_to_local();
        //    self.update_train_bus_error();
    }

    fn send_to_local(&mut self) {
        // Has the VehNumberList changed? Determine value and propagate on change

        let new_veh_config_list = (
            self.veh_config_list_received
                .0
                .iter()
                .rev()
                .cloned()
                .collect(),
            self.veh_config_list_received
                .1
                .iter()
                .rev()
                .cloned()
                .collect(),
        );

        if new_veh_config_list != self.veh_config_list_last_local
            || self.veh_config_self_last_local != self.my_vehicle_config
        {
            self.veh_config_list_last_local = new_veh_config_list.clone();
            self.veh_config_self_last_local = self.my_vehicle_config.clone();

            send_message(
                &(TrainConfig {
                    config_self: self.my_vehicle_config.clone(),
                    configs_front: new_veh_config_list.0,
                    configs_rear: new_veh_config_list.1,
                }),
                [MessageTarget::Broadcast {
                    across_couplings: false,
                    include_self: true,
                }],
            );
        }

        // Is a master active? Determine value and propagate on change
        let new_is_a_master = if self.am_i_master {
            Some(0)
        } else if self.master_pos_received.0.is_some() {
            let value = self.master_pos_received.0.unwrap();
            //Some((6_u32.saturating_sub(value)) + 1)
            self.master_pos_received.0
        } else if self.master_pos_received.1.is_some() {
            self.master_pos_received.1
        } else {
            None
        };

        if self.is_master_there() != self.master_pos_last_local {
            send_message(
                &(TrainBusMaster {
                    value: self.is_master_there(),
                }),
                [MessageTarget::Broadcast {
                    across_couplings: false,
                    include_self: true,
                }],
            );
        }

        self.master_pos_last_local = self.is_master_there();
    }

    pub fn on_message(&mut self, msg: Message) {
        msg.handle::<InternTelegram>(|m| {
            if let Some(coupler) = msg.source().coupling {
                match coupler {
                    Coupling::Front => {
                        self.veh_config_list_received.0 = m.vehicle_config;
                        self.master_pos_received.0 = m.master_pos;
                        self.update();
                    }
                    Coupling::Rear => {
                        self.veh_config_list_received.1 = m.vehicle_config;
                        self.master_pos_received.1 = m.master_pos;
                        self.update();
                    }
                }
            }

            Ok(())
        })
        .expect("InternTelegram: message handle failed");

        msg.handle::<IbisState>(|m| {
            self.am_i_master = m.is_master;
            self.update();
            Ok(())
        })
        .expect("IbisState: message handle failed");

        msg.handle::<EcouplerState>(|m| {
            match m.side {
                Coupling::Front => {
                    if m.value != self.is_coupled.0 {
                        self.is_coupled.0 = m.value;
                        self.update_bus(m.side, m.value);
                        if !m.value {
                            self.veh_config_list_received.0 = Vec::new();
                            self.master_pos_received.0 = None;
                        }
                        self.update();
                    }
                }
                Coupling::Rear => {
                    if m.value != self.is_coupled.1 {
                        self.is_coupled.1 = m.value;
                        self.update_bus(m.side, m.value);
                        if !m.value {
                            self.veh_config_list_received.1 = Vec::new();
                            self.master_pos_received.1 = None;
                        }
                        self.update();
                    }
                }
            }
            Ok(())
        })
        .expect("EcouplerState: message handle failed");

        msg.handle::<PeripheryRegister>(|m| {
            let id = m.id;
            let kind = m.kind;

            // find counter state
            // let counter = self.my_vehicle_config.count_kind(&kind) + 1;

            let counter = self.find_adress(&id, &kind);

            let p = PeripheryElement { id, kind, counter };

            self.my_vehicle_config.periphery.push(p);
            self.update();
            Ok(())
        })
        .expect("PeripheryRegister: message handle failed");

        msg.handle::<PeripheryFaultReport>(|m| {
            if let Some(pe) = self.my_vehicle_config.find_by_id(m.id) {
                self.my_perifery_faults.insert(m.id, m.kind.clone());

                // Send directly to master
                if self.is_master_there() {
                    self.send_fault_to_master(1, pe.kind.clone(), pe.counter, m.kind);
                }
            }

            Ok(())
        })
        .expect("PeripheryFaultReport: message handle failed");

        msg.handle::<InternFaultReport>(|m| {
            if let Some(side) = msg.source().coupling {
                if self.am_i_master {
                    send_message(
                        &IbisFaultReport {
                            car: m.car + 1,
                            coupling: msg.source().coupling,
                            kind: m.kind,
                            counter: m.counter,
                            state: m.state,
                        },
                        MessageTarget::Broadcast {
                            across_couplings: false,
                            include_self: true,
                        },
                    );
                } else {
                    match side {
                        Coupling::Front => {
                            send_message(
                                &InternFaultReport {
                                    car: m.car + 1,
                                    kind: m.kind,
                                    counter: m.counter,
                                    state: m.state,
                                },
                                MessageTarget::AcrossCoupling {
                                    coupling: Coupling::Rear,
                                    cascade: false,
                                },
                            );
                        }
                        Coupling::Rear => {
                            send_message(
                                &InternFaultReport {
                                    car: m.car + 1,
                                    kind: m.kind,
                                    counter: m.counter,
                                    state: m.state,
                                },
                                MessageTarget::AcrossCoupling {
                                    coupling: Coupling::Front,
                                    cascade: false,
                                },
                            );
                        }
                    }
                }
            }

            Ok(())
        })
        .expect("InternFaultReport: message handle failed");
    }

    fn send_fault_to_master(
        &self,
        car: u32,
        kind: PeripheryKind,
        counter: u32,
        state: PeripheryFault,
    ) {
        if self.am_i_master {
            send_message(
                &IbisFaultReport {
                    car,
                    coupling: None,
                    kind,
                    counter,
                    state,
                },
                MessageTarget::Broadcast {
                    across_couplings: false,
                    include_self: true,
                },
            );
        } else if self.master_pos_received.0.is_some() {
            send_message(
                &InternFaultReport {
                    car,
                    kind,
                    counter,
                    state,
                },
                MessageTarget::AcrossCoupling {
                    coupling: Coupling::Front,
                    cascade: false,
                },
            );
        } else if self.master_pos_received.1.is_some() {
            send_message(
                &InternFaultReport {
                    car,
                    kind,
                    counter,
                    state,
                },
                MessageTarget::AcrossCoupling {
                    coupling: Coupling::Rear,
                    cascade: false,
                },
            );
        }
    }

    fn update_bus(&mut self, side: Coupling, state: bool) {
        if state {
            side.open_bus("TrainBus");
        } else {
            side.close_bus("TrainBus");
        }
    }

    fn is_master_there(&self) -> bool {
        [
            self.am_i_master,
            self.master_pos_received.0.is_some(),
            self.master_pos_received.1.is_some(),
        ]
        .iter()
        .filter(|&&b| b)
        .count()
            > 0
    }

    /*fn update_train_bus_error(&mut self) {
        let new_train_bus_error = [
            self.am_i_master,
            self.master_pos_received.0.is_some(),
            self.master_pos_received.1.is_some(),
        ]
        .iter()
        .filter(|&&b| b)
        .count()
            >= 1;

        if new_train_bus_error != self.train_bus_error {
            self.train_bus_error = new_train_bus_error;
            // TODO: ZUGBUS Fehler
        }
    }*/
}

//===================================================================
// TrainBus - Perifery Interface
//===================================================================

#[derive(Debug)]
pub struct TrainBusPeriferie {
    kind: PeripheryKind,
    slot_index: u32,

    state: PeripheryFault,
}

impl TrainBusPeriferie {
    pub fn new(perifierie_kind: PeripheryKind, slot_index: i32) -> Self {
        send_message(
            &PeripheryRegister {
                id: slot_index as u32,
                kind: perifierie_kind.clone(),
            },
            MessageTarget::Broadcast {
                across_couplings: false,
                include_self: true,
            },
        );

        Self {
            kind: perifierie_kind,
            slot_index: slot_index as u32,
            state: PeripheryFault::Ok,
        }
    }

    pub fn set_defect(&mut self, fault: PeripheryFault) {
        if fault != self.state {
            send_message(
                &PeripheryFaultReport {
                    id: self.slot_index,
                    kind: fault,
                },
                MessageTarget::Broadcast {
                    across_couplings: false,
                    include_self: true,
                },
            );
        }
    }
}
