//! Advertisement content ID generation utilities.
//!
//! This module provides functions for generating randomized advertisement content IDs
//! for different billboard formats (horizontal and vertical). The content IDs are used
//! to select appropriate advertisements from predefined ranges.

use lotus_script::{
    content::ContentId,
    rand::{gen_f64, gen_u64},
};

/// Generates a random content ID for horizontal billboard advertisements.
///
/// This function randomly selects between two different user pools and their
/// associated sub-ID ranges to provide variety in horizontal ad content.
///
/// # Returns
///
/// A [`ContentId`] struct containing:
/// - For 50% of calls: `user_id` 5749281 with `sub_id` in range 300100-300106
/// - For 50% of calls: `user_id` 1000 with `sub_id` in range 300100-300107
///
/// # Examples
///
/// ```rust
/// use your_crate::get_horizontal_ad;
///
/// let ad_id = get_horizontal_ad();
/// println!("Horizontal ad ID: user {} sub {}", ad_id.user_id, ad_id.sub_id);
/// ```
///
/// # Note
///
/// The selection is based on a random float comparison (> 0.5), providing
/// approximately equal distribution between the two user pools over many calls.
#[must_use]
pub fn get_horizontal_ad() -> ContentId {
    // Horizontal billboards
    if gen_f64() > 0.5 {
        ContentId {
            user_id: 5749281,
            sub_id: gen_u64(300100..=300106) as i32,
        }
    } else {
        ContentId {
            user_id: 1000,
            sub_id: gen_u64(300100..=300107) as i32,
        }
    }
}

/// Generates a random content ID for vertical billboard advertisements.
///
/// This function generates content IDs exclusively from user 1000's vertical
/// advertisement pool, providing consistent sourcing for vertical ad formats.
///
/// # Returns
///
/// A [`ContentId`] struct containing:
/// - `user_id`: Always 1000
/// - `sub_id`: Random value in range 300000-300009 (inclusive)
///
/// # Examples
///
/// ```rust
/// use your_crate::get_vertical_ad;
///
/// let ad_id = get_vertical_ad();
/// assert_eq!(ad_id.user_id, 1000);
/// assert!(ad_id.sub_id >= 300000 && ad_id.sub_id <= 300009);
/// ```
///
/// # Note
///
/// The sub-ID range (300000-300009) provides 10 different vertical advertisement
/// options, ensuring variety while maintaining consistent user sourcing.
#[must_use]
pub fn get_vertical_ad() -> ContentId {
    // Vertical billboards
    ContentId {
        user_id: 1000,
        sub_id: gen_u64(300000..=300009) as i32,
    }
}
