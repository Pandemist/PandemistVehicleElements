use crate::structs::internal_enums::{Railquality, Surfacetype};

#[derive(Default, Debug)]
pub struct RailAxis {
    _name_id: String,
    axis_index: usize,
    bogie_index: usize,
}

impl RailAxis {
    pub fn new(name: String, axis_id: usize, bogie_id: usize) -> Self {
        RailAxis {
            _name_id: name,
            axis_index: axis_id,
            bogie_index: bogie_id,
            ..Default::default()
        }
    }

    // Entspricht der Variable M_Axle_N_{b}_{a}
    pub fn tractionforce(&self) -> f32 {
        todo!()
    }

    // Entspricht der Variable M_Axle_N_{b}_{a}
    pub fn set_tractionforce(&mut self, value: f32) {
        todo!()
    }

    // Entspricht der Variable MBrake_Axle_N_{b}_{a}
    pub fn brakeforce(&self) -> f32 {
        todo!()
    }

    // Entspricht der Variable MBrake_Axle_N_{b}_{a}
    pub fn set_brakeforce(&mut self, value: f32) {
        todo!()
    }

    // Entspricht der Variable sanding_{b}_{a}
    pub fn sanding(&self) -> f32 {
        todo!()
    }

    // Entspricht der Variable sanding_{b}_{a}
    pub fn set_sanding(value: f32) {
        todo!()
    }

    // Entspricht der Variable v_Axle_mps_{b}_{a}
    pub fn speed_mps(&self) -> f32 {
        todo!()
    }

    // Entspricht der Variable alpha_Axle_deg_{b}_{a}
    pub fn spring_axle_deg(&self) -> f32 {
        todo!()
    }

    // Entspricht der Variable spring_Axle_m_{b}_{a}
    pub fn spring_axle_m(&self) -> f32 {
        todo!()
    }

    // Entspricht der Variable loadforce_Axle_N_{b}_{a}
    pub fn loadforce_axle(&self) -> f32 {
        todo!()
    }

    // Entspricht der invradius_{b}_{a}
    pub fn invradius(&self) -> f32 {
        todo!()
    }

    // Entspricht der Variable railquality_{b}_{a}
    pub fn railquality(&self) -> Railquality {
        todo!()
    }

    // Entspricht der Variable surfacetype_{b}_{a}
    pub fn surfacetype(&self) -> Surfacetype {
        todo!()
    }
}

#[derive(Default, Debug)]
pub struct StreetAxis {
    _name_id: String,
}

impl StreetAxis {
    pub fn new(name: String) -> Self {
        StreetAxis {
            _name_id: name,
            ..Default::default()
        }
    }

    // Entspricht der Variable M_Axle_N_{a}
    pub fn tractionforce() -> f32 {
        todo!()
    }

    // Entspricht der Variable MBrake_Wheel_N_{a}_{s}
    pub fn set_tractionforce(&mut self, value: f32) {
        todo!()
    }

    // Entspricht der Variable MBrake_Wheel_N_{a}_{s}
    pub fn brakeforce(&self) -> f32 {
        todo!()
    }

    // Entspricht der Variable MBrake_Wheel_N_{a}_{s}
    pub fn set_brakeforce(&mut self, value: f32) {
        todo!()
    }

    // Entspricht der Variable v_Wheel_mps_{a}_{s}
    pub fn speed_mps(&self) -> f32 {
        todo!()
    }

    // Entspricht der Variable alpha_Wheel_deg_{a}_{s}
    pub fn spring_wheel_deg(&self) -> f32 {
        todo!()
    }

    // Entspricht der Variable spring_Wheel_m_{a}_{s}
    pub fn spring_wheel_m(&self) -> f32 {
        todo!()
    }

    // Entspricht der Variable steering_Wheel_m_{a}_{s}
    pub fn steering_wheel_m(&self) -> f32 {
        todo!()
    }
}
