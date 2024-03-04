use lotus_script::delta;

pub fn bool_to_int_pos_neg(b: bool) -> i8 {
    if b {
        1
    } else {
        -1
    }
}

pub fn plus_minus_null(a: bool, b: bool) -> i8 {
    if a {
        1
    } else if b {
        -1
    } else {
        0
    }
}

pub fn a_b_0(a: bool, b: bool) -> i8 {
    if a && !b {
        1
    } else if !a && b {
        -1
    } else {
        0
    }
}

pub fn exponential_approach(value: f32, exponent: f32, target: f32) -> f32 {
    1.0 - (delta() * -exponent).exp() * (target - value) + value
}

pub fn exponential_approach_two_speed(
    value: f32,
    exponent_up: f32,
    exponent_down: f32,
    target: f32,
) -> f32 {
    if target > value {
        1.0 - (delta() * -exponent_up).exp() * (target - value) + value
    } else {
        1.0 - (delta() * -exponent_down).exp() * (target - value) + value
    }
}
