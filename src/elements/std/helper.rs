//! Helper utilities for common operations in game development and mathematical calculations.
//!
//! This module provides utility functions for:
//! - Mathematical operations with dual-speed exponential approaches
//! - String manipulation and padding
//! - Random number generation and element selection
//! - Text measurement for bitmap fonts

use lotus_script::content::ContentId;
use lotus_script::font::BitmapFont;
use lotus_script::math::exponential_approach;
use lotus_script::rand::{gen_f64, gen_u64};
use std::ops::{Bound, RangeBounds};

/// Performs exponential approach with different speeds for increasing and decreasing values.
///
/// This function applies exponential smoothing to approach a target value, but uses different
/// exponents depending on whether the target is above or below the current value. This is
/// useful for animations or smoothing operations where you want different response times
/// for upward vs downward movements.
///
/// # Arguments
///
/// * `value` - The current value
/// * `exponent_up` - The exponent to use when target > value (approaching upward)
/// * `exponent_down` - The exponent to use when target < value (approaching downward)
/// * `target` - The target value to approach
///
/// # Returns
///
/// The new value after applying the appropriate exponential approach
///
/// # Examples
///
/// ```
/// # use your_crate::helper::exponential_approach_two_speed;
/// // Smooth movement with faster upward motion
/// let current = 10.0;
/// let new_value = exponential_approach_two_speed(current, 0.1, 0.05, 20.0);
/// assert!(new_value > current && new_value < 20.0);
/// ```
#[must_use]
pub fn exponential_approach_two_speed(
    value: f32,
    exponent_up: f32,
    exponent_down: f32,
    target: f32,
) -> f32 {
    if target > value {
        exponential_approach(value, exponent_up, target)
        //    1.0 - (delta() * -exponent_up).exp() * (target - value) + value
    } else {
        exponential_approach(value, exponent_down, target)
        //    1.0 - (delta() * -exponent_down).exp() * (target - value) + value
    }
}

/// Pads a string to a specified length with a given character.
///
/// Extends the input string by appending the specified filler character until
/// the string reaches the desired length. If the string is already longer than
/// or equal to the specified length, it remains unchanged.
///
/// # Arguments
///
/// * `s` - The string to enhance/pad
/// * `length` - The desired minimum length of the string
/// * `filler` - The character to use for padding
///
/// # Returns
///
/// The padded string
///
/// # Examples
///
/// ```
/// # use your_crate::helper::enhance_string;
/// let result = enhance_string("hello".to_string(), 10, '*');
/// assert_eq!(result, "hello*****");
///
/// let unchanged = enhance_string("already_long".to_string(), 5, '-');
/// assert_eq!(unchanged, "already_long");
/// ```
#[must_use]
pub fn enhance_string(mut s: String, length: usize, filler: char) -> String {
    while s.len() < length {
        s.push(filler);
    }
    s
}

/// Returns a random element from a slice.
///
/// Selects a random element from the provided slice using uniform distribution.
/// The slice must not be empty.
///
/// # Arguments
///
/// * `vec` - A non-empty slice to select from
///
/// # Returns
///
/// A reference to a randomly selected element
///
/// # Panics
///
/// Panics if the slice is empty (when `vec.len()` is 0).
///
/// # Examples
///
/// ```
/// # use your_crate::helper::get_random_element;
/// let items = vec!["apple", "banana", "cherry"];
/// let random_fruit = get_random_element(&items);
/// assert!(items.contains(random_fruit));
/// ```
pub fn get_random_element<T>(vec: &[T]) -> &T {
    let index = gen_u64(0..(vec.len() as u64)) as usize;
    &vec[index]
}

/// Generates a random boolean value.
///
/// Returns `true` or `false` with equal probability (50% each).
///
/// # Returns
///
/// A randomly generated boolean value
///
/// # Examples
///
/// ```
/// # use your_crate::helper::gen_bool;
/// let random_decision = gen_bool();
/// // random_decision is either true or false
/// ```
#[must_use]
pub fn gen_bool() -> bool {
    gen_f64() > 0.5
}

/// Generates a random f32 value within the specified range.
///
/// Generates a uniformly distributed random floating-point number within the given range.
/// Supports inclusive and exclusive bounds, as well as unbounded ranges.
///
/// # Arguments
///
/// * `range` - A range specification (e.g., `0.0..1.0`, `0.0..=1.0`, `..`, etc.)
///
/// # Returns
///
/// A random f32 value within the specified range
///
/// # Examples
///
/// ```
/// # use your_crate::helper::gen_f32;
/// // Generate between 0.0 and 1.0 (exclusive)
/// let value1 = gen_f32(0.0..1.0);
/// assert!(value1 >= 0.0 && value1 < 1.0);
///
/// // Generate between -5.0 and 5.0 (inclusive)
/// let value2 = gen_f32(-5.0..=5.0);
/// assert!(value2 >= -5.0 && value2 <= 5.0);
/// ```
pub fn gen_f32(range: impl RangeBounds<f32>) -> f32 {
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

/// Calculates the rendered length of text using a bitmap font.
///
/// Measures the pixel width that the given text would occupy when rendered
/// with the specified bitmap font and character spacing.
///
/// # Arguments
///
/// * `font` - The content ID of the bitmap font to use for measurement
/// * `text` - The text string to measure
/// * `spacing` - Additional spacing between characters in pixels
///
/// # Returns
///
/// The total width in pixels that the text would occupy, or 0 if the font
/// cannot be loaded
///
/// # Examples
///
/// ```
/// # use your_crate::helper::get_text_len;
/// # use lotus_script::content::ContentId;
/// // Measure text width with a specific font
/// let font_id = ContentId::from("my_font");
/// let width = get_text_len(font_id, "Hello World", 2);
/// // width contains the pixel width of "Hello World" with 2px character spacing
/// ```
pub fn get_text_len(font: ContentId, text: &str, spacing: i32) -> i32 {
    if let Some(f) = BitmapFont::try_load(font) {
        //
        f.text_len(text, spacing) as i32
    } else {
        0
    }
}
