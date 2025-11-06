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

use lotus_script::vehicle::{Pantograph, VehicleError};

use crate::{
    api::{mock_enums::ThirdRailState, variable::get_var},
    management::enums::general_enums::Side,
};

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
/// use pandemist_vehicle_elements::electrical_supply::ApiPantograph;
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

//=========================================================================

/// A third rail power collector for subway and metro systems.
///
/// Third rail systems provide power through a separate rail running alongside or between
/// the running rails. This struct manages the connection state and voltage detection
/// for a specific collector shoe on either the left or right side of the vehicle.
///
/// # Examples
///
/// ```rust
/// use your_crate::electrical_supply::ApiThirdRailCollector;
/// use your_crate::management::enums::general_enums::Side;
///
/// let collector = ApiThirdRailCollector::new(1, Side::Left);
/// let has_voltage = collector.voltage();
/// let connection_state = collector.value();
/// ```
#[derive(Debug)]
pub struct ApiThirdRailCollector {
    /// Unique identifier for this collector
    id: usize,
    /// Side of the vehicle (Left or Right) where this collector is located
    side: Side,
}

impl ApiThirdRailCollector {
    /// Creates a new `ApiThirdRailCollector` instance.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the collector
    /// * `side` - The side of the vehicle where this collector is mounted
    ///
    /// # Returns
    ///
    /// A new `ApiThirdRailCollector` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_crate::electrical_supply::ApiThirdRailCollector;
    /// use your_crate::management::enums::general_enums::Side;
    ///
    /// let left_collector = ApiThirdRailCollector::new(1, Side::Left);
    /// let right_collector = ApiThirdRailCollector::new(2, Side::Right);
    /// ```
    #[must_use]
    pub fn new(id: usize, side: Side) -> Self {
        Self { id, side }
    }

    /// Returns the connection state of the third rail collector.
    ///
    /// This corresponds to the lotus_script variable: `V_ThirdRailCollector_{id}_{side}`
    /// The variable can have values: -1 (disconnected), -0.5 (partially connected), 0/1 (connected)
    ///
    /// # Returns
    ///
    /// * [`ThirdRailState::Disconnnected`] - Collector is not in contact with the rail
    /// * [`ThirdRailState::PartwiseConnected`] - Collector has partial contact
    /// * [`ThirdRailState::Connected`] - Collector is fully connected
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_crate::electrical_supply::ApiThirdRailCollector;
    /// use your_crate::mock_enums::ThirdRailState;
    ///
    /// let collector = ApiThirdRailCollector::new(1, Side::Left);
    /// match collector.value() {
    ///     ThirdRailState::Connected => println!("Fully connected to third rail"),
    ///     ThirdRailState::PartwiseConnected => println!("Partial connection"),
    ///     ThirdRailState::Disconnnected => println!("No connection"),
    /// }
    /// ```
    #[must_use]
    pub fn value(&self) -> ThirdRailState {
        match get_var::<f32>(&format!("V_ThirdRailCollector_{}_{}", self.id, self.side)) {
            -1.0 => ThirdRailState::Disconnnected,
            -0.5 => ThirdRailState::PartwiseConnected,
            _ => ThirdRailState::Connected,
        }
    }

    /// Checks if the collector is receiving voltage from the third rail.
    ///
    /// This method returns `true` when the lotus_script variable `V_ThirdRailCollector_{id}_{side}` equals 1.0,
    /// indicating that the collector is not only connected but also receiving power.
    ///
    /// # Returns
    ///
    /// * `true` - If voltage is detected (variable value = 1.0)
    /// * `false` - If no voltage is detected
    ///
    /// # Examples
    ///
    /// ```rust
    /// let collector = ApiThirdRailCollector::new(1, Side::Left);
    /// if collector.voltage() {
    ///     println!("Power available from third rail");
    /// } else {
    ///     println!("No power from third rail");
    /// }
    /// ```
    #[must_use]
    pub fn voltage(&self) -> bool {
        matches!(
            get_var::<f32>(&format!("V_ThirdRailCollector_{}_{}", self.id, self.side)),
            1.0
        )
    }
}
