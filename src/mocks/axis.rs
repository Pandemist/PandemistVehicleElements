use crate::structs::enums::Side;
use lotus_script::var::VariableType;

use super::mock_enums::{Railquality, Surfacetype};

#[derive(Default, Debug)]
pub struct RailAxis {
    axis_index: usize,
    bogie_index: usize,
}

impl RailAxis {
    pub fn new(axis_id: usize, bogie_id: usize) -> Self {
        RailAxis {
            axis_index: axis_id,
            bogie_index: bogie_id,
            ..Default::default()
        }
    }

    // Entspricht der Variable M_Axle_N_{b}_{a}
    pub fn set_tractionforce(&self, value: f32) {
        value.set(&format!(
            "M_Axle_N_{}_{}",
            self.bogie_index, self.axis_index
        ));
    }

    // Entspricht der Variable MBrake_Axle_N_{b}_{a}
    pub fn set_brakeforce(&self, value: f32) {
        value.set(&format!(
            "MBrake_Axle_N_{}_{}",
            self.bogie_index, self.axis_index
        ));
    }

    // Entspricht der Variable sanding_{b}_{a}
    pub fn set_sanding(&self, value: f32) {
        value.set(&format!("Variable{}_{}", self.bogie_index, self.axis_index));
    }

    // Entspricht der Variable v_Axle_mps_{b}_{a}
    pub fn speed_mps(&self) -> f32 {
        f32::get(&format!(
            "v_Axle_mps_{}_{}",
            self.bogie_index, self.axis_index
        ))
    }

    // Entspricht der Variable alpha_Axle_deg_{b}_{a}
    pub fn spring_axle_deg(&self) -> f32 {
        f32::get(&format!(
            "alpha_Axle_deg_{}_{}",
            self.bogie_index, self.axis_index
        ))
    }

    // Entspricht der Variable spring_Axle_m_{b}_{a}
    pub fn spring_axle_m(&self) -> f32 {
        f32::get(&format!(
            "spring_Axle_m_{}_{}",
            self.bogie_index, self.axis_index
        ))
    }

    // Entspricht der Variable loadforce_Axle_N_{b}_{a}
    pub fn loadforce_axle(&self) -> f32 {
        f32::get(&format!(
            "loadforce_Axle_N_{}_{}",
            self.bogie_index, self.axis_index
        ))
    }

    // Entspricht der invradius_{b}_{a}
    pub fn invradius(&self) -> f32 {
        f32::get(&format!(
            "invradius_{}_{}",
            self.bogie_index, self.axis_index
        ))
    }

    // Entspricht der Variable railquality_{b}_{a}
    pub fn railquality(&self) -> Railquality {
        match u32::get(&format!(
            "railquality_{}_{}",
            self.bogie_index, self.axis_index
        )) {
            7 => Railquality::DisortedUneven,
            6 => Railquality::DisortedEven,
            5 => Railquality::VeryEven,
            4 => Railquality::Flat,
            3 => Railquality::UnevenWithCenterPiece,
            2 => Railquality::EvenWithCenterPiece,
            1 => Railquality::Uneven,
            _ => Railquality::Even,
        }
    }

    // Entspricht der Variable surfacetype_{b}_{a}
    pub fn surfacetype(&self) -> Surfacetype {
        match u32::get(&format!(
            "surfacetype_{}_{}",
            self.bogie_index, self.axis_index
        )) {
            2 => Surfacetype::Grass,
            1 => Surfacetype::Road,
            _ => Surfacetype::Ballast,
        }
    }
}

#[derive(Debug)]
pub struct StreetAxis {
    id: usize,
}

impl StreetAxis {
    pub fn new(new_id: usize) -> Self {
        StreetAxis { id: new_id }
    }

    // Entspricht der Variable M_Axle_N_{a}_{s}
    pub fn set_tractionforce(&self, value: f32, s: Side) {
        value.set(&format!("MBrake_Wheel_N_{}_{}", self.id, s));
    }

    // Entspricht der Variable MBrake_Wheel_N_{a}_{s}
    pub fn set_brakeforce(&self, value: f32, s: Side) {
        value.set(&format!("MBrake_Wheel_N_{}_{}", self.id, s));
    }

    // Entspricht der Variable v_Wheel_mps_{a}_{s}
    pub fn speed_mps(&self, s: Side) -> f32 {
        f32::get(&format!("v_Wheel_mps_{}_{}", self.id, s))
    }

    // Entspricht der Variable alpha_Wheel_deg_{a}_{s}
    pub fn spring_wheel_deg(&self, s: Side) -> f32 {
        f32::get(&format!("alpha_Wheel_deg_{}_{}", self.id, s))
    }

    // Entspricht der Variable spring_Wheel_m_{a}_{s}
    pub fn spring_wheel_m(&self, s: Side) -> f32 {
        f32::get(&format!("spring_Wheel_m_{}_{}", self.id, s))
    }

    // Entspricht der Variable steering_Wheel_m_{a}_{s}
    pub fn steering_wheel_m(&self, s: Side) -> f32 {
        f32::get(&format!("steering_Wheel_m_{}_{}", self.id, s))
    }
}
