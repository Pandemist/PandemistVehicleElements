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

//=========================================================================

/// A trolley pantograph system for trams and trolleybuses.
///
/// Trolley pantographs are used on trams and trolleybuses to collect power from overhead
/// wires. Unlike train pantographs, they typically have articulated arms that can move
/// both horizontally and vertically to maintain contact with the wire.
///
/// # Examples
///
/// ```rust
/// use your_crate::electrical_supply::ApiTrolleyPantograph;
///
/// let mut trolley = ApiTrolleyPantograph::new(1);
/// let has_voltage = trolley.voltage();
/// trolley.set_angle_hor(15.0); // Adjust horizontal angle
/// ```
#[derive(Debug)]
pub struct ApiTrolleyPantograph {
    /// Unique identifier for this trolley pantograph
    id: usize,
}

impl ApiTrolleyPantograph {
    /// Creates a new `ApiTrolleyPantograph` instance.
    ///
    /// # Arguments
    ///
    /// * `new_id` - Unique identifier for the trolley pantograph
    ///
    /// # Returns
    ///
    /// A new `ApiTrolleyPantograph` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let trolley = ApiTrolleyPantograph::new(1);
    /// ```
    #[must_use]
    pub fn new(new_id: usize) -> Self {
        Self { id: new_id }
    }

    /// Checks if the trolley pantograph is receiving voltage.
    ///
    /// This corresponds to the lotus_script variable: `panto_voltage_{id}`
    ///
    /// # Returns
    ///
    /// * `true` - If voltage is detected
    /// * `false` - If no voltage is detected
    ///
    /// # Examples
    ///
    /// ```rust
    /// let trolley = ApiTrolleyPantograph::new(1);
    /// if trolley.voltage() {
    ///     println!("Trolley has power");
    /// }
    /// ```
    #[must_use]
    pub fn voltage(&self) -> bool {
        get_var::<bool>(&format!("panto_voltage_{}", self.id))
    }

    /// Returns the current pantograph position/height.
    ///
    /// This corresponds to the lotus_script variable: `panto_{id}`
    ///
    /// # Returns
    ///
    /// The current pantograph position as a float value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let trolley = ApiTrolleyPantograph::new(1);
    /// let position = trolley.panto();
    /// println!("Pantograph position: {}", position);
    /// ```
    #[must_use]
    pub fn panto(&self) -> f32 {
        get_var::<f32>(&format!("panto_{}", self.id))
    }

    /// Returns the current horizontal angle of the trolley arm.
    ///
    /// This corresponds to the lotus_script variable: `trolley_angle_{id}_hori`
    ///
    /// # Returns
    ///
    /// The horizontal angle in degrees.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let trolley = ApiTrolleyPantograph::new(1);
    /// let angle = trolley.angle_hor();
    /// println!("Horizontal angle: {}°", angle);
    /// ```
    #[must_use]
    pub fn angle_hor(&self) -> f32 {
        get_var::<f32>(&format!("trolley_angle_{}_hori", self.id))
    }

    /// Sets the horizontal angle of the trolley arm.
    ///
    /// This modifies the lotus_script variable: `trolley_angle_{id}_hori`
    ///
    /// # Arguments
    ///
    /// * `value` - The desired horizontal angle in degrees
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut trolley = ApiTrolleyPantograph::new(1);
    /// trolley.set_angle_hor(15.0); // Set to 15 degrees
    /// ```
    pub fn set_angle_hor(&mut self, value: f32) {
        set_var(&format!("trolley_angle_{}_hori", self.id), value);
    }

    /// Returns the current vertical angle of the trolley arm.
    ///
    /// This corresponds to the lotus_script variable: `trolley_angle_{id}_vert`
    ///
    /// # Returns
    ///
    /// The vertical angle in degrees.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let trolley = ApiTrolleyPantograph::new(1);
    /// let angle = trolley.angle_vert();
    /// println!("Vertical angle: {}°", angle);
    /// ```
    #[must_use]
    pub fn angle_vert(&self) -> f32 {
        get_var::<f32>(&format!("trolley_angle_{}_vert", self.id))
    }

    /// Sets the vertical angle of the trolley arm.
    ///
    /// This modifies the lotus_script variable: `trolley_angle_{id}_vert`
    ///
    /// # Arguments
    ///
    /// * `value` - The desired vertical angle in degrees
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut trolley = ApiTrolleyPantograph::new(1);
    /// trolley.set_angle_vert(45.0); // Set to 45 degrees
    /// ```
    pub fn set_angle_vert(&mut self, value: f32) {
        set_var(&format!("trolley_angle_{}_vert", self.id), value);
    }

    /// Checks if the trolley pantograph is in free movement mode.
    ///
    /// This corresponds to the lotus_script variable: `trolley_free_{id}`
    ///
    /// # Returns
    ///
    /// * `true` - If the trolley can move freely
    /// * `false` - If the trolley movement is constrained
    ///
    /// # Examples
    ///
    /// ```rust
    /// let trolley = ApiTrolleyPantograph::new(1);
    /// if trolley.free() {
    ///     println!("Trolley can move freely");
    /// }
    /// ```
    #[must_use]
    pub fn free(&self) -> bool {
        get_var::<bool>(&format!("trolley_free_{}", self.id))
    }

    /// Checks if the trolley pantograph is online and operational.
    ///
    /// This corresponds to the lotus_script variable: `trolley_online_{id}`
    ///
    /// # Returns
    ///
    /// * `true` - If the trolley system is online
    /// * `false` - If the trolley system is offline
    ///
    /// # Examples
    ///
    /// ```rust
    /// let trolley = ApiTrolleyPantograph::new(1);
    /// if trolley.online() {
    ///     println!("Trolley system is operational");
    /// }
    /// ```
    #[must_use]
    pub fn online(&self) -> bool {
        get_var::<bool>(&format!("trolley_online_{}", self.id))
    }
}
