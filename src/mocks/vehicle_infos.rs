use glam::{f32::Vec3, vec3};
use lotus_script::var::VariableType;

pub fn veh_number() -> String {
    String::get("veh_number")
}

pub fn set_veh_number(value: String) {
    value.set("veh_number");
}

pub fn veh_registration() -> String {
    String::get("veh_registration")
}

pub fn set_veh_registration(value: String) {
    value.set("veh_registration");
}

pub fn v_ground() -> f32 {
    f32::get("v_ground")
}

pub fn a_ground() -> f32 {
    f32::get("a_ground")
}

pub fn acceleration_vec() -> Vec3 {
    vec3(f32::get("acc_x"), f32::get("acc_y"), f32::get("acc_z"))
}
