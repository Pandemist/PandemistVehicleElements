use lotus_extra::messages::handle_message;
use lotus_script::{
    message,
    prelude::{message_type, send_message, MessageTarget},
    time::delta,
    vehicle::TrainConfigurationChanged,
};
use serde::{Deserialize, Serialize};

// Zwischeninfo: Hauptluftleitung für 15m Wagen etwa 10 Liter

// All units are in SI units, pressures absolute in Pascal, temperatures in Kelvin, volumes in cubic meters, mass flows in kilograms per second.

const R: f32 = 287.0;
const T: f32 = 293.15;
pub const RT_AIR: f32 = R * T;
const GAMMA: f32 = 1.4;
const PI_CRITICAL: f32 = 0.528;
const P_EPS: f32 = 1.0;
pub const P_STD: f32 = 101_300.0;

// ---------------------------------------------------------------------------
// Tank
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct Tank {
    volume: f32,
    pub pressure: f32,
    pub additional_mass_flow: f32,
}

impl Tank {
    pub fn new(volume: f32) -> Self {
        Self {
            volume,
            pressure: P_STD,
            additional_mass_flow: 0.0,
        }
    }
}

// ---------------------------------------------------------------------------
// Connection / Valve
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub enum ValveAutomatic {
    Manual,
    /// Air cannot flow from tank 1 to tank 0
    CheckValve,
    /// Valve will close if tank 1 pressure is higher than nominal value OR of tank 0.
    Regulator {
        nominal_pressure: f32,
        /// Tolerance: the regulation pressure increases with increasing pressure in tank 0.
        /// Value is per Pa. Typical value: 0.02
        increase_tolerance: f32,
    },
}

#[derive(Clone)]
pub struct Connection {
    tank_indices: [usize; 2],
    /// Opening area in m²
    opening: f32,
    automatic: ValveAutomatic,
    pub valve_open: f32,
}

impl Connection {
    pub fn new(tank_indices: [usize; 2], opening: f32, automatic: ValveAutomatic) -> Self {
        Self {
            tank_indices,
            opening,
            automatic,
            valve_open: 0.0,
        }
    }
}

// ---------------------------------------------------------------------------
// Compressor
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct Compressor {
    tank_index_source: Option<usize>,
    tank_index_target: Option<usize>,
    /// Volume flow in m³/s
    volume_flow: f32,
    /// Normalised speed [0.0 … 1.0]
    speed: f32,
}

impl Compressor {
    pub fn new(
        tank_index_source: Option<usize>,
        tank_index_target: Option<usize>,
        volume_flow: f32,
    ) -> Self {
        Self {
            tank_index_source,
            tank_index_target,
            volume_flow,
            speed: 0.0,
        }
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }
}

// ---------------------------------------------------------------------------
// Coupling connection
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct CouplingConnection {
    tank_index: usize,
    opening: f32,
    pub open: f32,
}

impl CouplingConnection {
    pub fn new(tank_index: usize, opening: f32) -> Self {
        Self {
            tank_index,
            opening,
            open: 1.0,
        }
    }
}

// ---------------------------------------------------------------------------
// Coupling
// ---------------------------------------------------------------------------

#[derive(Copy, Clone, Default, PartialEq)]
enum WhoIsMaster {
    #[default]
    Unknown,
    Me,
    Other,
}

#[derive(Clone, Default)]
pub struct Coupling {
    connections: Vec<CouplingConnection>,
    my_entity_id: Option<u64>,
    master: WhoIsMaster,
    pressure_opening_other_side: Option<(f32, f32)>,
}

impl Coupling {
    pub fn reset(&mut self, my_entity_id: u64, this_is_coupling: message::Coupling) {
        self.my_entity_id = Some(my_entity_id);
        self.master = WhoIsMaster::Unknown;
        self.pressure_opening_other_side = None;

        send_message(
            &PneumaticCouplingMessage::Reset {
                entity: my_entity_id,
            },
            MessageTarget::AcrossCoupling {
                coupling: this_is_coupling,
                cascade: false,
            },
        );
    }

    pub fn set_master_slave(&mut self, other: u64) {
        let my_entity_id = self.my_entity_id.unwrap();

        if my_entity_id > other {
            self.master = WhoIsMaster::Me;
        } else {
            self.master = WhoIsMaster::Other;
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum PneumaticCouplingMessage {
    Reset {
        entity: u64,
    },
    EntityId(u64),
    /// Positive if stream flows from master vehicle to slave vehicle
    MDot(Vec<f32>),
    PressureAndOpening(Vec<(f32, f32)>),
}

message_type!(PneumaticCouplingMessage, "Pneumatic", "Coupling");

// ---------------------------------------------------------------------------
// PneumaticSystem  (owns all state directly)
// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct PneumaticSystem {
    tanks: Vec<Tank>,
    connections: Vec<Connection>,
    compressors: Vec<Compressor>,
    couplings: [Coupling; 2],
}

impl PneumaticSystem {
    // ------------------------------------------------------------------
    // Public accessors (mirror of the old BB accessor methods)
    // ------------------------------------------------------------------

    pub fn tank_pressure(&self, index: usize) -> f32 {
        self.tanks[index].pressure
    }

    pub fn set_tank_pressure(&mut self, index: usize, pressure: f32) {
        self.tanks[index].pressure = pressure;
    }

    pub fn set_valve_open(&mut self, connection_index: usize, valve_open: f32) {
        self.connections[connection_index].valve_open = valve_open;
    }

    pub fn compressor_speed(&self, index: usize) -> f32 {
        self.compressors[index].speed()
    }

    pub fn set_compressor_speed(&mut self, index: usize, speed: f32) {
        self.compressors[index].set_speed(speed);
    }

    // ------------------------------------------------------------------
    // Builder helpers
    // ------------------------------------------------------------------

    pub fn add_tank_get_index(&mut self, volume: f32) -> usize {
        let index = self.tanks.len();
        self.tanks.push(Tank::new(volume));
        index
    }

    pub fn add_connection_get_index(
        &mut self,
        tank_indices: [usize; 2],
        opening: f32,
        automatic: ValveAutomatic,
    ) -> usize {
        let index = self.connections.len();
        self.connections
            .push(Connection::new(tank_indices, opening, automatic));
        index
    }

    pub fn add_compressor_get_index(
        &mut self,
        tank_index_source: Option<usize>,
        tank_index_target: Option<usize>,
        volume_flow: f32,
    ) -> usize {
        let index = self.compressors.len();
        self.compressors.push(Compressor::new(
            tank_index_source,
            tank_index_target,
            volume_flow,
        ));
        index
    }

    pub fn add_coupling_get_index(
        &mut self,
        coupling: message::Coupling,
        opening: f32,
        tank_index: usize,
    ) -> usize {
        let coupling_index: usize = coupling.into();
        let index = self.couplings[coupling_index].connections.len();
        self.couplings[coupling_index]
            .connections
            .push(CouplingConnection::new(tank_index, opening));
        index
    }

    pub fn set_valve_type(&mut self, index: usize, new_type: ValveAutomatic) {
        self.connections[index].automatic = new_type;
    }

    pub fn tank_count(&self) -> usize {
        self.tanks.len()
    }

    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }

    pub fn compressor_count(&self) -> usize {
        self.compressors.len()
    }

    // ------------------------------------------------------------------
    // Simulation tick  (previously ModuleTick::tick)
    // ------------------------------------------------------------------

    pub fn tick(&mut self) {
        // Extract volumes upfront so the closure below does not need to borrow
        // `self.tanks` at the same time as the mutable iterators further down.
        let volumes: Vec<f32> = self.tanks.iter().map(|t| t.volume).collect();

        let mut tank_dp_dt: Vec<f32> = vec![0.0; self.tanks.len()];

        // Helper: accumulate dp/dt for a tank given a mass-flow contribution.
        // Uses the pre-extracted `volumes` slice instead of `self.tanks`.
        let mut change_by_mdot = |index: usize, mdot: f32| {
            tank_dp_dt[index] += mdot_to_pdot(mdot, volumes[index]);
        };

        // Collect additional_mass_flow from each tank and reset it.
        // Done with index-based access to avoid a simultaneous mutable borrow
        // of `self.tanks` and the closure's immutable borrow of `volumes`.
        for index in 0..self.tanks.len() {
            let amf = self.tanks[index].additional_mass_flow;
            self.tanks[index].additional_mass_flow = 0.0;
            change_by_mdot(index, amf);
        }

        for connection in self.connections.iter_mut() {
            match connection.automatic {
                ValveAutomatic::Manual => {}
                ValveAutomatic::CheckValve => {
                    connection.valve_open = (self.tanks[connection.tank_indices[0]].pressure
                        > self.tanks[connection.tank_indices[1]].pressure)
                        .if_else(1.0, 0.0);
                }
                ValveAutomatic::Regulator {
                    nominal_pressure,
                    increase_tolerance,
                } => {
                    let pressure_0 = self.tanks[connection.tank_indices[0]].pressure;
                    let pressure_limit = nominal_pressure + increase_tolerance * pressure_0;
                    connection.valve_open = (pressure_0.min(pressure_limit)
                        > self.tanks[connection.tank_indices[1]].pressure)
                        .if_else(1.0, 0.0);
                }
            }

            let valve = connection.valve_open;
            let p0 = self.tanks[connection.tank_indices[0]].pressure;
            let p1 = self.tanks[connection.tank_indices[1]].pressure;

            let mdot = p_to_mdot(p0, p1, connection.opening * valve);

            change_by_mdot(connection.tank_indices[0], -mdot);
            change_by_mdot(connection.tank_indices[1], mdot);
        }

        for compressor in self.compressors.iter() {
            let compressor_speed = compressor.speed();

            if compressor_speed < 0.01 {
                continue;
            }

            let mdot = compressor_speed
                * compressor.volume_flow
                * compressor
                    .tank_index_source
                    .map(|index| self.tanks[index].pressure)
                    .unwrap_or(P_STD)
                / RT_AIR;

            if let Some(tank_target_index) = compressor.tank_index_target {
                change_by_mdot(tank_target_index, mdot);
            }

            if let Some(tank_source_index) = compressor.tank_index_source {
                change_by_mdot(tank_source_index, -mdot);
            }
        }

        for (tank, dp_dt) in self.tanks.iter_mut().zip(tank_dp_dt.iter()) {
            let mut dp = dp_dt * delta();

            dp = dp.clamp(-0.05 * tank.pressure, 0.05 * tank.pressure);

            tank.pressure = (tank.pressure + dp).max(P_STD);
        }

        for side in [0, 1] {
            let coupling = &self.couplings[side];

            if coupling.master == WhoIsMaster::Me {
                let mut mdot_vec = Vec::new();

                for coupling_connection in coupling.connections.iter() {
                    if let Some((other_pressure, other_opening)) =
                        self.couplings[side].pressure_opening_other_side
                    {
                        let opening = (coupling_connection.opening * coupling_connection.open)
                            .min(other_opening);

                        let p_tank = self.tanks[coupling_connection.tank_index].pressure;
                        let mdot = p_to_mdot(p_tank, other_pressure, opening);

                        self.tanks[coupling_connection.tank_index].additional_mass_flow -= mdot;
                        mdot_vec.push(mdot);
                    }
                }

                send_message(
                    &PneumaticCouplingMessage::MDot(mdot_vec),
                    MessageTarget::AcrossCoupling {
                        coupling: side.into(),
                        cascade: false,
                    },
                );
            } else if coupling.master == WhoIsMaster::Other {
                let mut p_vec = Vec::new();

                for coupling_connection in coupling.connections.iter() {
                    let p_tank = self.tanks[coupling_connection.tank_index].pressure;
                    let opening = coupling_connection.open;
                    p_vec.push((p_tank, opening));
                }

                send_message(
                    &PneumaticCouplingMessage::PressureAndOpening(p_vec),
                    MessageTarget::AcrossCoupling {
                        coupling: side.into(),
                        cascade: false,
                    },
                );
            }
        }
    }

    // ------------------------------------------------------------------
    // Message handler  (previously ModuleOnMessage::on_message)
    // ------------------------------------------------------------------

    pub fn on_message(&mut self, msg: &lotus_script::message::Message) -> bool {
        let config_handled = handle_message(msg, |c: TrainConfigurationChanged| {
            self.couplings[0].reset(c.entity_id, message::Coupling::Front);
            self.couplings[1].reset(c.entity_id, message::Coupling::Rear);
            true
        });

        let coupling_handled = handle_message(msg, |c: PneumaticCouplingMessage| {
            let coupling = msg.source().coupling.unwrap();
            let coupling_index: usize = coupling.into();

            match c {
                PneumaticCouplingMessage::Reset { entity } => {
                    self.couplings[coupling_index].set_master_slave(entity);
                }
                PneumaticCouplingMessage::EntityId(entity_id) => {
                    self.couplings[coupling_index].my_entity_id = Some(entity_id);
                }
                PneumaticCouplingMessage::MDot(mdot) => {
                    self.set_mdot(coupling_index, mdot);
                }
                PneumaticCouplingMessage::PressureAndOpening(pressure_and_opening) => {
                    self.set_pressure_and_opening(coupling_index, pressure_and_opening);
                }
            }
            true
        });

        config_handled || coupling_handled
    }

    // ------------------------------------------------------------------
    // Private helpers
    // ------------------------------------------------------------------

    fn set_mdot(&mut self, coupling_index: usize, mdot: Vec<f32>) {
        for (coupling_connection, mdot) in
            self.couplings[coupling_index].connections.iter().zip(mdot)
        {
            let tank_index = coupling_connection.tank_index;
            self.tanks[tank_index].additional_mass_flow += mdot;
        }
    }

    fn set_pressure_and_opening(
        &mut self,
        coupling_index: usize,
        pressures_and_opening: Vec<(f32, f32)>,
    ) {
        for pressure_and_opening in pressures_and_opening {
            self.couplings[coupling_index].pressure_opening_other_side = Some(pressure_and_opening);
        }
    }
}

// ---------------------------------------------------------------------------
// Physics helpers
// ---------------------------------------------------------------------------

fn mdot_to_pdot(mdot: f32, volume: f32) -> f32 {
    mdot * RT_AIR / volume
}

/// If air flows from p_a to p_b, mdot is positive.
fn p_to_mdot(p_a: f32, p_b: f32, opening: f32) -> f32 {
    if (p_a - p_b).abs() < P_EPS {
        return 0.0;
    }

    let p_high = p_a.max(p_b);
    let p_low = p_a.min(p_b);
    let pi = p_low / p_high;

    opening
        * p_high
        * if pi > PI_CRITICAL {
            (2.0 / RT_AIR).sqrt() * (pi.powf(2.0 / GAMMA) - pi.powf((GAMMA + 1.0) / GAMMA)).sqrt()
        } else {
            (GAMMA / RT_AIR * (2.0 / (GAMMA + 1.0)).powf((GAMMA + 1.0) / (GAMMA - 1.0))).sqrt()
        }
        * if p_a > p_b { 1.0 } else { -1.0 }
}

//---------------------------------

pub trait IfElse<T> {
    fn if_else(self, on_true: T, on_false: T) -> T;
}

impl<T> IfElse<T> for bool {
    fn if_else(self, on_true: T, on_false: T) -> T {
        if self {
            on_true
        } else {
            on_false
        }
    }
}
