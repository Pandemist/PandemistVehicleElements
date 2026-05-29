use std::ops::{Bound, RangeBounds};

use lotus_script::rand::gen_f64;

pub fn gen_i32_with_variance(normal: i32, variance: i32) -> i32 {
    let min = normal - variance;
    let max = normal + variance;

    let range = min..=max;

    let min = match range.start_bound() {
        Bound::Included(min) => *min,
        Bound::Excluded(min) => min + 1,
        Bound::Unbounded => i32::MIN,
    };

    let max = match range.end_bound() {
        Bound::Included(max) => *max,
        Bound::Excluded(max) => max - 1,
        Bound::Unbounded => i32::MAX,
    };

    assert!(min <= max, "min must be less than or equal to max");

    (min as f32 + ((gen_f64() as f32) * (max as f32 - min as f32))) as i32
}
