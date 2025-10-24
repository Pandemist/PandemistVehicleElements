//! # Electrical Supply System
//!
//! This module provides abstractions for various electrical supply systems used in rail vehicles,
//! including pantographs, third rail collectors, and trolley pantographs. It interfaces with
//! the lotus_script library to manage vehicle electrical systems.
//!
//! ## Components
//!
//! - [`ApiPantograph`] - Standard overhead line pantograph systems
//! - [`ApiThirdRailCollector`] - Third rail power collection systems
//! - [`ApiTrolleyPantograph`] - Trolley/tram pantograph systems with angle control

use lotus_script::{
    var::{get_var, set_var},
    vehicle::{Pantograph, VehicleError},
};

use crate::management::enums::general_enums::Side;

use super::mock_enums::ThirdRailState;

//=========================================================================

/// A wrapper around the lotus_script `Pantograph` for overhead line power collection.
///
/// This struct provides a safe interface to pantograph operations, handling potential
/// errors from the underlying vehicle system. Pantographs are used to collect power
/// from overhead electrical lines in trains and trams.
///
/// # Examples
///
/// ```rust
/// use your_crate::electrical_supply::ApiPantograph;
///
/// let pantograph = ApiPantograph::new(1);
/// let voltage = pantograph.voltage();
/// let height = pantograph.height();
/// ```
#[derive(Debug)]
pub struct ApiPantograph {
    /// Unique identifier for this pantograph
    id: usize,
    /// The underlying pantograph instance, which may fail to initialize
    panto: Result<Pantograph, VehicleError>,
}

impl ApiPantograph {
    /// Creates a new `ApiPantograph` instance with the specified ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the pantograph
    ///
    /// # Returns
    ///
    /// A new `ApiPantograph` instance. The underlying pantograph may fail to initialize,
    /// in which case voltage will return 0.0 and height will return None.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let pantograph = ApiPantograph::new(1);
    /// ```
    #[must_use]
    pub fn new(id: usize) -> Self {
        Self {
            id,
            panto: Pantograph::get(id),
        }
    }

    /// Returns the current voltage reading from the pantograph.
    ///
    /// This corresponds to the lotus_script variable: `panto_voltage_{id}`
    ///
    /// # Returns
    ///
    /// * `f32` - The voltage reading in volts. Returns 0.0 if the pantograph failed to initialize.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let pantograph = ApiPantograph::new(1);
    /// let voltage = pantograph.voltage();
    /// println!("Current voltage: {} V", voltage);
    /// ```
    #[must_use]
    pub fn voltage(&self) -> f32 {
        if let Ok(p) = self.panto {
            p.voltage()
        } else {
            0.0
        }
    }

    /// Returns the current height of the pantograph.
    ///
    /// This corresponds to the lotus_script variable: `panto_{id}`
    ///
    /// # Returns
    ///
    /// * `Some(f32)` - The height of the pantograph in meters if successfully initialized
    /// * `None` - If the pantograph failed to initialize
    ///
    /// # Examples
    ///
    /// ```rust
    /// let pantograph = ApiPantograph::new(1);
    /// match pantograph.height() {
    ///     Some(height) => println!("Pantograph height: {} m", height),
    ///     None => println!("Pantograph not available"),
    /// }
    /// ```
    #[must_use]
    pub fn height(&self) -> Option<f32> {
        if let Ok(p) = self.panto {
            Some(p.height())
        } else {
            None
        }
    }
}
