//! # Seals Module
//!
//! This module provides implementations for various types of sealed and covered control elements
//! commonly found in simulation systems, particularly aviation or industrial control panels.
//!
//! The module includes three main components:
//! - [`SimpleSeal`]: A basic seal that can be removed with a key
//! - [`SealedSwitch`]: A switch that is protected by a removable seal
//! - [`CoveredKey`]: A key switch with an animated cover that can be opened and closed
//!
//! # Examples
//!
//! ```rust
//! use your_crate::seals::{SimpleSeal, SealedSwitch, CoveredKey};
//! use your_crate::api::key_event::CockpitSide;
//! use your_crate::switches::Switch;
//! use your_crate::elements::tech::key_switch::KeySwitch;
//!
//! // Create a simple seal
//! let mut seal = SimpleSeal::new(Some(CockpitSide::ACab), "seal_visibility", "remove_seal_key");
//!
//! // Create a sealed switch
//! let switch = Switch::new(); // See switch.rs
//! let mut sealed_switch = SealedSwitch::new(
//!     Some(CockpitSide::ACab),
//!     "switch_seal_vis",
//!     "seal_removal_key",
//!     switch
//! );
//!
//! // Create a covered key with sounds
//! let key_switch = KeySwitch::new(); // See key_switch.rs
//! let mut covered_key = CoveredKey::builder(
//!     "cover_animation",
//!     "toggle_cover_key",
//!     key_switch,
//!     Some(CockpitSide::ACab)
//! )
//! .snd_on("cover_open_sound")
//! .snd_off("cover_close_sound")
//! .build();
//! ```

use lotus_extra::vehicle::CockpitSide;

use crate::{
    api::{animation::Animation, key_event::KeyEvent, sound::Sound, visible_flag::Visiblility},
    elements::tech::{key_switch::KeySwitch, switches::StepSwitch},
};

use super::switches::Switch;

//=================================================================
// SimpleSeal
//=================================================================

/// A simple seal that can be removed using a key event.
///
/// `SimpleSeal` represents a basic protective seal that becomes invisible (removed)
/// when the associated key is pressed, but only if the seal is currently visible.
/// Once removed, the seal cannot be restored.
///
/// # Examples
///
/// ```rust
/// use your_crate::seals::SimpleSeal;
/// use your_crate::api::key_event::CockpitSide;
///
/// let mut seal = SimpleSeal::new(
///     Some(CockpitSide::ACab),
///     "emergency_seal_visibility",
///     "emergency_seal_key"
/// );
///
/// // Check if seal was just removed
/// if seal.tick() {
///     println!("Emergency seal has been removed!");
/// }
/// ```
pub struct SimpleSeal {
    /// The key event that triggers seal removal
    pub key_off: KeyEvent,
    /// Controls the visibility state of the seal
    visibility: Visiblility,
}

impl SimpleSeal {
    /// Creates a new `SimpleSeal` instance.
    ///
    /// # Arguments
    ///
    /// * `cab_side` - Optional specification of which cab side the key event belongs to
    /// * `vis` - Name/identifier for the visibility flag
    /// * `key_off` - Name/identifier for the key event that removes the seal
    ///
    /// # Returns
    ///
    /// A new `SimpleSeal` instance with the seal initially visible.
    pub fn new(cab_side: Option<CockpitSide>, vis: &str, key_off: &str) -> Self {
        let mut s = Self {
            key_off: KeyEvent::new(Some(key_off), cab_side),
            visibility: Visiblility::new(vis),
        };
        s.visibility.make_visible();
        s
    }

    /// Updates the seal state and handles removal logic.
    ///
    /// This method should be called every frame/tick to process input and update
    /// the seal's state. The seal will be removed (made invisible) if the removal
    /// key is pressed while the seal is currently visible.
    ///
    /// # Returns
    ///
    /// `true` if the seal was just removed this tick, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use your_crate::seals::SimpleSeal;
    /// # let mut seal = SimpleSeal::new(None, "test_vis", "test_key");
    /// if seal.tick() {
    ///     // Seal was just removed - trigger any necessary side effects
    ///     activate_emergency_systems();
    /// }
    /// ```
    pub fn tick(&mut self) -> bool {
        if self.key_off.is_just_pressed() && !self.visibility.check() {
            self.visibility.make_invisible();
            true
        } else {
            false
        }
    }
}

//=================================================================
// SealedSwitch
//=================================================================

/// A switch that is protected by a removable seal.
///
/// `SealedSwitch` combines a `Switch` with a protective seal mechanism. The switch
/// can only be operated after the seal has been removed using the designated key.
/// The seal removal is permanent for the lifetime of the instance.
///
/// # Examples
///
/// ```rust
/// use your_crate::seals::SealedSwitch;
/// use your_crate::switches::Switch;
/// use your_crate::api::key_event::CockpitSide;
///
/// let switch = Switch::new(); // See switch.rs
/// let mut sealed_switch = SealedSwitch::new(
///     Some(CockpitSide::ACab),
///     "fire_suppression_seal",
///     "break_seal_key",
///     switch
/// );
///
/// // This should be called every frame
/// sealed_switch.tick();
/// ```
pub struct SealedSwitch {
    /// The key event that removes the protective seal
    pub key_off: KeyEvent,
    /// Controls the visibility/presence of the seal
    visibility: Visiblility,
    /// The underlying switch that becomes operational after seal removal
    pub switch: Switch,
}

impl SealedSwitch {
    /// Creates a new `SealedSwitch` instance.
    ///
    /// # Arguments
    ///
    /// * `cab_side` - Optional specification of which cab side the key event belongs to
    /// * `vis` - Name/identifier for the seal's visibility flag
    /// * `key_off` - Name/identifier for the key event that removes the seal
    /// * `switch` - The underlying switch to be protected by the seal
    ///
    /// # Returns
    ///
    /// A new `SealedSwitch` instance with the seal initially present (visible).
    pub fn new(cab_side: Option<CockpitSide>, vis: &str, key_off: &str, switch: Switch) -> Self {
        let mut s = Self {
            key_off: KeyEvent::new(Some(key_off), cab_side),
            visibility: Visiblility::new(vis),
            switch,
        };
        s.visibility.make_visible();
        s
    }

    /// Updates the sealed switch state and processes input.
    ///
    /// This method handles both seal removal and switch operation. When the seal
    /// removal key is pressed, the seal becomes invisible. Once the seal is removed,
    /// the underlying switch's `tick()` method is called to handle switch operations.
    ///
    /// # Behavior
    ///
    /// - If the removal key is pressed, the seal is permanently removed
    /// - If the seal is not present (removed), the switch becomes operational
    /// - The switch only responds to input after seal removal
    pub fn tick(&mut self) {
        if self.key_off.is_just_pressed() {
            self.visibility.make_invisible();
        }
        if !self.visibility.check() {
            self.switch.tick();
        }
    }
}

//=================================================================
// SealedStepSwitch
//=================================================================

pub struct SealedStepSwitch {
    pub key_off: KeyEvent,
    visibility: Visiblility,
    pub switch: StepSwitch,
}

impl SealedStepSwitch {
    pub fn new(
        cab_side: Option<CockpitSide>,
        vis: &str,
        key_off: &str,
        switch: StepSwitch,
    ) -> Self {
        let mut s = Self {
            key_off: KeyEvent::new(Some(key_off), cab_side),
            visibility: Visiblility::new(vis),
            switch,
        };
        s.visibility.make_visible();
        s
    }

    pub fn tick(&mut self) {
        if self.key_off.is_just_pressed() {
            self.visibility.make_invisible();
        }
        if !self.visibility.check() {
            self.switch.tick();
        }
    }
}

//=================================================================
// CoveredKey
//=================================================================

/// Builder for creating `CoveredKey` instances with optional sound effects.
///
/// This builder pattern allows for flexible configuration of sound effects
/// for the cover opening and closing actions.
pub struct CoveredKeyBuilder {
    key_toggle: KeyEvent,
    anim: Animation,
    key_switch: KeySwitch,
    snd_on: Sound,
    snd_off: Sound,
}

impl CoveredKeyBuilder {
    /// Sets the sound effect for when the cover is opened.
    ///
    /// # Arguments
    ///
    /// * `name` - The name/identifier of the sound to play when opening the cover
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining.
    pub fn snd_on(mut self, name: impl Into<String>) -> Self {
        self.snd_on = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Sets the sound effect for when the cover is closed.
    ///
    /// # Arguments
    ///
    /// * `name` - The name/identifier of the sound to play when closing the cover
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining.
    pub fn snd_off(mut self, name: impl Into<String>) -> Self {
        self.snd_off = Sound::new_simple(Some(&name.into()));
        self
    }

    /// Builds the final `CoveredKey` instance.
    ///
    /// # Returns
    ///
    /// A configured `CoveredKey` instance ready for use.
    pub fn build(self) -> CoveredKey {
        CoveredKey {
            key_toggle: self.key_toggle,
            anim: self.anim,
            key_switch: self.key_switch,
            snd_on: self.snd_on,
            snd_off: self.snd_off,
        }
    }
}

/// A key switch with an animated protective cover.
///
/// `CoveredKey` represents a key switch that is protected by a cover which can be
/// opened and closed. The cover has smooth animation transitions and optional sound
/// effects. The key switch only becomes operational when the cover is open.
///
/// # Animation States
///
/// - `0.0`: Cover is fully closed
/// - `0.5`: Cover is halfway open (transition point)
/// - `1.0`: Cover is fully open
///
/// # Examples
///
/// ```rust
/// use your_crate::seals::CoveredKey;
/// use your_crate::elements::tech::key_switch::KeySwitch;
/// use your_crate::api::key_event::CockpitSide;
///
/// let key_switch = KeySwitch::new(); // See key_switch.rs
/// let mut covered_key = CoveredKey::builder(
///     "master_arm_cover_anim",
///     "master_arm_cover_toggle",
///     key_switch,
///     Some(CockpitSide::ACab)
/// )
/// .snd_on("cover_lift_sound")
/// .snd_off("cover_close_sound")
/// .build();
///
/// // Update every frame
/// covered_key.tick();
/// ```
pub struct CoveredKey {
    /// The key event that toggles the cover open/closed
    pub key_toggle: KeyEvent,
    /// Animation controller for the cover movement
    anim: Animation,
    /// The underlying key switch that becomes available when cover is open
    pub key_switch: KeySwitch,
    /// Sound effect played when cover opens
    snd_on: Sound,
    /// Sound effect played when cover closes
    snd_off: Sound,
}

impl CoveredKey {
    /// Creates a new builder for constructing a `CoveredKey`.
    ///
    /// # Arguments
    ///
    /// * `animation_name` - Name/identifier for the cover animation
    /// * `key_event_name` - Name/identifier for the key event that toggles the cover
    /// * `key_switch` - The underlying key switch to be protected by the cover
    /// * `cab_side` - Optional specification of which cab side the key event belongs to
    ///
    /// # Returns
    ///
    /// A `CoveredKeyBuilder` instance for further configuration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use your_crate::seals::CoveredKey;
    /// # use your_crate::elements::tech::key_switch::KeySwitch;
    /// # use your_crate::api::key_event::CockpitSide;
    /// # let key_switch = KeySwitch::new(); // See key_switch.rs
    /// let covered_key = CoveredKey::builder(
    ///     "emergency_cover_animation",
    ///     "emergency_cover_key",
    ///     key_switch,
    ///     Some(CockpitSide::ACab)
    /// )
    /// .snd_on("emergency_cover_open")
    /// .build();
    /// ```
    pub fn builder(
        animation_name: impl Into<String>,
        key_event_name: impl Into<String>,
        key_switch: KeySwitch,
        cab_side: Option<CockpitSide>,
    ) -> CoveredKeyBuilder {
        CoveredKeyBuilder {
            key_toggle: KeyEvent::new(Some(&key_event_name.into()), cab_side),
            anim: Animation::new(Some(&animation_name.into())),
            key_switch,
            snd_on: Sound::new_simple(None),
            snd_off: Sound::new_simple(None),
        }
    }

    /// Updates the covered key state, handling cover animation and key switch operation.
    ///
    /// This method processes cover toggle input, manages the cover animation, plays
    /// appropriate sound effects, and enables key switch operation when the cover
    /// is sufficiently open.
    ///
    /// # Behavior
    ///
    /// - **Opening**: When toggle key is pressed and cover is closed (pos < 0.5),
    ///   the cover opens fully and the opening sound plays
    /// - **Closing**: When toggle key is pressed and cover is open (pos >= 0.5),
    ///   the cover closes only if no key is inserted in the switch, and the
    ///   closing sound plays
    /// - **Key Operation**: The underlying key switch only responds to input when
    ///   the cover is more than halfway open (pos > 0.5)
    ///
    /// # Safety Feature
    ///
    /// The cover cannot be closed while a key is inserted in the switch, preventing
    /// accidental closure during key switch operation.
    pub fn tick(&mut self) {
        if self.key_toggle.is_just_pressed() {
            if self.anim.pos < 0.5 {
                self.anim.set(1.0);
                self.snd_on.start();
            } else if !self.key_switch.is_inserted() {
                self.anim.set(0.0);
                self.snd_off.start();
            }
        }

        if self.anim.pos > 0.5 {
            self.key_switch.tick();
        }
    }
}
