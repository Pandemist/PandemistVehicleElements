//! Axis management for railway and street vehicles.
//!
//! This module provides structures and methods for managing vehicle axes,
//! including both rail and street vehicle configurations. It handles force
//! application, monitoring of physical properties, and interaction with
//! the underlying simulation system.

use lotus_script::{
    var::{get_var, set_var},
    vehicle::{Axle, RailQuality, SurfaceType, VehicleError},
};

/// API interface for managing railway vehicle axles.
///
/// `ApiRailAxis` provides a high-level interface for controlling and monitoring
/// individual axles on railway vehicles. Each axle is identified by its position
/// within a bogie and the bogie's position on the vehicle.
///
/// # Examples
///
/// ```rust
/// use your_crate::axis::ApiRailAxis;
///
/// // Create an axle interface for the first axle on the first bogie
/// let axle = ApiRailAxis::new(0, 0);
///
/// // Apply traction force
/// axle.set_tractionforce(5000.0);
///
/// // Check current speed
/// let speed = axle.speed_mps();
/// println!("Current axle speed: {} m/s", speed);
/// ```
pub struct ApiRailAxis {
    /// Index of the axle within its bogie
    pub axle_index: usize,
    /// Index of the bogie within the vehicle
    pub bogie_index: usize,
    /// Handle to the underlying axle object
    pub axle: Result<Axle, VehicleError>,
}

impl ApiRailAxis {
    /// Creates a new railway axle interface.
    ///
    /// # Arguments
    ///
    /// * `axle_index` - The index of the axle within its bogie (0-based)
    /// * `bogie_index` - The index of the bogie within the vehicle (0-based)
    ///
    /// # Returns
    ///
    /// A new `ApiRailAxis` instance that interfaces with the specified axle.
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Create interface for second axle on third bogie
    /// let axle = ApiRailAxis::new(1, 2);
    /// ```
    #[must_use]
    pub fn new(axle_index: usize, bogie_index: usize) -> Self {
        Self {
            axle_index,
            bogie_index,
            axle: Axle::get(bogie_index, axle_index),
        }
    }

    /// Sets the traction force applied to this axle.
    ///
    /// Corresponds to the simulation variable: `M_Axle_N_{b}_{a}`
    /// where `{b}` is the bogie index and `{a}` is the axle index.
    ///
    /// # Arguments
    ///
    /// * `value` - Traction force in Newtons. Positive values indicate forward thrust,
    ///   negative values indicate reverse thrust.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let axle = ApiRailAxis::new(0, 0);
    /// axle.set_tractionforce(2500.0); // Apply 2500N forward thrust
    /// axle.set_tractionforce(-1000.0); // Apply 1000N reverse thrust
    /// ```
    pub fn set_tractionforce(&self, value: f32) {
        if let Ok(b) = self.axle {
            b.set_traction_force_newton(value);
        }
    }

    /// Sets the brake force applied to this axle.
    ///
    /// Corresponds to the simulation variable: `MBrake_Axle_N_{b}_{a}`
    /// where `{b}` is the bogie index and `{a}` is the axle index.
    ///
    /// # Arguments
    ///
    /// * `value` - Brake force in Newtons. Should typically be a positive value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let axle = ApiRailAxis::new(0, 0);
    /// axle.set_brakeforce(5000.0); // Apply 5000N brake force
    /// ```
    pub fn set_brakeforce(&self, value: f32) {
        if let Ok(b) = self.axle {
            b.set_brake_force_newton(value);
        }
    }

    /// Enables or disables sanding for this axle.
    ///
    /// Sanding improves wheel-rail adhesion by applying sand to the rail surface.
    /// Corresponds to the simulation variable: `sanding_{b}_{a}`
    /// where `{b}` is the bogie index and `{a}` is the axle index.
    ///
    /// # Arguments
    ///
    /// * `value` - `true` to enable sanding, `false` to disable
    ///
    /// # Examples
    ///
    /// ```rust
    /// let axle = ApiRailAxis::new(0, 0);
    /// axle.set_sanding(true);  // Enable sanding for better traction
    /// axle.set_sanding(false); // Disable sanding
    /// ```
    pub fn set_sanding(&self, value: bool) {
        set_var(
            &format!("sanding_{}_{}", self.bogie_index, self.axle_index),
            value,
        );
    }

    /// Returns the current speed of this axle in meters per second.
    ///
    /// Corresponds to the simulation variable: `v_Axle_mps_{b}_{a}`
    /// where `{b}` is the bogie index and `{a}` is the axle index.
    ///
    /// # Returns
    ///
    /// Current axle speed in m/s. Positive values indicate forward motion,
    /// negative values indicate reverse motion.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let axle = ApiRailAxis::new(0, 0);
    /// let speed = axle.speed_mps();
    /// if speed > 0.0 {
    ///     println!("Moving forward at {} m/s", speed);
    /// }
    /// ```
    #[must_use]
    pub fn speed_mps(&self) -> f32 {
        get_var::<f32>(&format!(
            "v_Axle_mps_{}_{}",
            self.bogie_index, self.axle_index
        ))
    }

    /// Returns the spring deflection angle of this axle in degrees.
    ///
    /// Corresponds to the simulation variable: `alpha_Axle_deg_{b}_{a}`
    /// where `{b}` is the bogie index and `{a}` is the axle index.
    ///
    /// # Returns
    ///
    /// Spring deflection angle in degrees.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let axle = ApiRailAxis::new(0, 0);
    /// let angle = axle.spring_axle_deg();
    /// println!("Spring deflection: {} degrees", angle);
    /// ```
    #[must_use]
    pub fn spring_axle_deg(&self) -> f32 {
        get_var::<f32>(&format!(
            "alpha_Axle_deg_{}_{}",
            self.bogie_index, self.axle_index
        ))
    }

    /// Returns the spring deflection distance of this axle in meters.
    ///
    /// Corresponds to the simulation variable: `spring_Axle_m_{b}_{a}`
    /// where `{b}` is the bogie index and `{a}` is the axle index.
    ///
    /// # Returns
    ///
    /// Spring deflection distance in meters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let axle = ApiRailAxis::new(0, 0);
    /// let deflection = axle.spring_axle_m();
    /// println!("Spring compression: {} meters", deflection);
    /// ```
    #[must_use]
    pub fn spring_axle_m(&self) -> f32 {
        get_var::<f32>(&format!(
            "spring_Axle_m_{}_{}",
            self.bogie_index, self.axle_index
        ))
    }

    /// Returns the load force applied to this axle in Newtons.
    ///
    /// This represents the vertical force pressing the axle against the rail,
    /// typically from the weight of the vehicle and its cargo.
    /// Corresponds to the simulation variable: `loadforce_Axle_N_{b}_{a}`
    /// where `{b}` is the bogie index and `{a}` is the axle index.
    ///
    /// # Returns
    ///
    /// Load force in Newtons.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let axle = ApiRailAxis::new(0, 0);
    /// let load = axle.loadforce_axle();
    /// println!("Axle load: {} N", load);
    /// ```
    #[must_use]
    pub fn loadforce_axle(&self) -> f32 {
        get_var::<f32>(&format!(
            "loadforce_Axle_N_{}_{}",
            self.bogie_index, self.axle_index
        ))
    }

    /// Returns the inverse radius of the track curve at this axle's position.
    ///
    /// A higher value indicates a tighter curve. A value of 0 indicates straight track.
    /// Corresponds to the simulation variable: `invradius_{b}_{a}`
    /// where `{b}` is the bogie index and `{a}` is the axle index.
    ///
    /// # Returns
    ///
    /// Inverse radius value (1/meters). Returns 0.0 if axle access fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let axle = ApiRailAxis::new(0, 0);
    /// let inv_radius = axle.invradius();
    /// if inv_radius > 0.0 {
    ///     let radius = 1.0 / inv_radius;
    ///     println!("Curve radius: {} meters", radius);
    /// } else {
    ///     println!("Straight track");
    /// }
    /// ```
    #[must_use]
    pub fn invradius(&self) -> f32 {
        if let Ok(b) = self.axle {
            b.inverse_radius()
        } else {
            0.0
        }
    }

    /// Returns the quality of the rail at this axle's position.
    ///
    /// Rail quality affects ride comfort, wear, and maximum safe speeds.
    /// Corresponds to the simulation variable: `railquality_{b}_{a}`
    /// where `{b}` is the bogie index and `{a}` is the axle index.
    ///
    /// # Returns
    ///
    /// [`RailQuality`] enum value. Returns [`RailQuality::Smooth`] if axle access fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lotus_script::vehicle::RailQuality;
    ///
    /// let axle = ApiRailAxis::new(0, 0);
    /// match axle.railquality() {
    ///     RailQuality::Smooth => println!("High-quality track"),
    ///     RailQuality::Rough => println!("Poor-quality track - reduce speed"),
    ///     _ => println!("Unknown rail quality"),
    /// }
    /// ```
    #[must_use]
    pub fn railquality(&self) -> RailQuality {
        if let Ok(b) = self.axle {
            b.rail_quality()
        } else {
            RailQuality::Smooth
        }
    }

    /// Returns the surface type of the track at this axle's position.
    ///
    /// Different surface types affect traction, braking performance, and noise.
    /// Corresponds to the simulation variable: `surfacetype_{b}_{a}`
    /// where `{b}` is the bogie index and `{a}` is the axle index.
    ///
    /// # Returns
    ///
    /// [`SurfaceType`] enum value. Returns [`SurfaceType::Gravel`] if axle access fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lotus_script::vehicle::SurfaceType;
    ///
    /// let axle = ApiRailAxis::new(0, 0);
    /// match axle.surfacetype() {
    ///     SurfaceType::Steel => println!("Steel rails - normal operation"),
    ///     SurfaceType::Gravel => println!("Gravel surface - reduced traction"),
    ///     _ => println!("Other surface type"),
    /// }
    /// ```
    #[must_use]
    pub fn surfacetype(&self) -> SurfaceType {
        if let Ok(b) = self.axle {
            b.surface_type()
        } else {
            SurfaceType::Gravel
        }
    }
}
