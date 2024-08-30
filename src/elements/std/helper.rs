use lotus_script::{
    delta,
    rand::{gen_f64, gen_u64},
};
use std::ops::{Bound, RangeBounds};

pub fn bool_to_int_pos_neg(b: bool) -> i8 {
    b as i8 * 2 - 1
}

pub fn plus_minus_null(a: bool, b: bool) -> i8 {
    a as i8 - b as i8
}

pub fn a_b_0(a: bool, b: bool) -> i8 {
    a as i8 - b as i8
}

pub fn exponential_approach(value: f32, exponent: f32, target: f32) -> f32 {
    1.0 - (delta() * -exponent).exp() * (target - value) + value
}

pub fn b_to_f(b: bool) -> f32 {
    if b {
        1.0
    } else {
        0.0
    }
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

pub fn enhance_string(mut s: String, length: usize, filler: char) -> String {
    while s.len() < length {
        s.push(filler);
    }
    s
}

pub fn get_random_element<T>(vec: &Vec<T>) -> &T {
    let index = gen_u64(0..=(vec.len() as u64 - 1)) as usize;
    &vec[index]
}

pub fn gen_bool() -> bool {
    gen_f64() > 0.5
}

pub fn gen_f64_range(range: impl RangeBounds<f64>) -> f64 {
    let min = match range.start_bound() {
        Bound::Included(min) => *min,
        Bound::Excluded(min) => min + 1.0,
        Bound::Unbounded => 0.0,
    };

    let max = match range.end_bound() {
        Bound::Included(max) => *max,
        Bound::Excluded(max) => max - 1.0,
        Bound::Unbounded => f64::MAX,
    };

    min + (gen_f64() * (max - min))
}

pub fn gen_f32_range(range: impl RangeBounds<f32>) -> f32 {
    let min = match range.start_bound() {
        Bound::Included(min) => *min,
        Bound::Excluded(min) => min + 1.0,
        Bound::Unbounded => 0.0,
    };

    let max = match range.end_bound() {
        Bound::Included(max) => *max,
        Bound::Excluded(max) => max - 1.0,
        Bound::Unbounded => f32::MAX,
    };

    min + ((gen_f64() as f32) * (max - min))
}
