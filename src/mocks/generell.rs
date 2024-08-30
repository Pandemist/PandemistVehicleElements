use glam::{vec2, Vec2};
use lotus_script::var::VariableType;

// Entspricht der Varialbe NightTex
pub fn night_tex() -> i32 {
    i32::get("NightTex")
}

// Entspricht der Varialbe EnvirBrightness
pub fn env_brightness() -> f32 {
    f32::get("EnvirBrightness")
}

// Entspricht der Varialbe EnvirBrightnessSurface
pub fn surface_brightness() -> f32 {
    f32::get("EnvirBrightnessSurface")
}

// Entspricht der Varialbe DistrictLight
pub fn district_light() -> f32 {
    f32::get("DistrictLight")
}

// Entspricht der Varialbe TimeOfDay
pub fn time_of_day() -> f32 {
    f32::get("TimeOfDay")
}

// Entspricht der Varialbe Date
pub fn date() -> f32 {
    f32::get("Date")
}

// Entspricht der Varialbe Hint
pub fn set_hint(hint: String) {
    hint.set("Hint");
}

// Entspricht den Varialben Mouse_X & Mouse_Y
pub fn mouse_move() -> Vec2 {
    vec2(f32::get("Mouse_X"), f32::get("Mouse_Y"))
}

// Wandelt eine userid und subid in den identifier einer Textur um
// Entspricht: GetTextureIndex(userID, contentSubID: integer)
pub fn texture_by_id(user_id: u32, sub_id: u32) -> u32 {
    todo!()
}

// Entspricht der Variable Signalstate
pub fn signalstate() -> u32 {
    u32::get("Signalstate")
}
