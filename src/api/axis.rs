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

use crate::management::enums::general_enums::Side;

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

//=========================================================================

/// Interface for managing street vehicle axles.
///
/// `StreetAxis` provides control and monitoring capabilities for individual
/// axles on street vehicles (cars, buses, trucks, etc.). Unlike railway axles,
/// street vehicle axles have separate left and right wheels that can be
/// controlled independently.
///
/// # Examples
///
/// ```rust
/// use your_crate::axis::StreetAxis;
/// use your_crate::management::enums::general_enums::Side;
///
/// // Create an axle interface for the front axle
/// let axle = StreetAxis::new(0);
///
/// // Apply different brake forces to left and right wheels
/// axle.set_brakeforce(2000.0, &Side::Left);
/// axle.set_brakeforce(2200.0, &Side::Right);
///
/// // Check wheel speeds
/// let left_speed = axle.speed_mps(&Side::Left);
/// let right_speed = axle.speed_mps(&Side::Right);
/// ```
#[derive(Debug)]
pub struct StreetAxis {
    /// Unique identifier for this axle
    id: usize,
}

impl StreetAxis {
    /// Creates a new street vehicle axle interface.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the axle (typically 0 for front, 1 for rear, etc.)
    ///
    /// # Returns
    ///
    /// A new `StreetAxis` instance for the specified axle.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let front_axle = StreetAxis::new(0);
    /// let rear_axle = StreetAxis::new(1);
    /// ```
    #[must_use]
    pub fn new(id: usize) -> Self {
        StreetAxis { id }
    }

    /// Sets the traction force for a specific wheel on this axle.
    ///
    /// **Note**: There appears to be a bug in the original implementation - this method
    /// currently sets the brake force variable instead of the traction force variable.
    /// The variable name suggests it should correspond to `M_Axle_N_{a}_{s}` but the
    /// implementation uses `MBrake_Wheel_N_{a}_{s}`.
    ///
    /// # Arguments
    ///
    /// * `value` - Traction force in Newtons
    /// * `s` - Which side/wheel to apply the force to
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_crate::management::enums::general_enums::Side;
    ///
    /// let axle = StreetAxis::new(0);
    /// axle.set_tractionforce(1500.0, &Side::Left);
    /// axle.set_tractionforce(1500.0, &Side::Right);
    /// ```
    pub fn set_tractionforce(&self, value: f32, s: &Side) {
        set_var(&format!("MBrake_Wheel_N_{}_{}", self.id, s), value);
    }

    /// Sets the brake force for a specific wheel on this axle.
    ///
    /// Corresponds to the simulation variable: `MBrake_Wheel_N_{a}_{s}`
    /// where `{a}` is the axle ID and `{s}` is the side (Left/Right).
    ///
    /// # Arguments
    ///
    /// * `value` - Brake force in Newtons. Should typically be positive.
    /// * `s` - Which side/wheel to apply the brake force to
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_crate::management::enums::general_enums::Side;
    ///
    /// let axle = StreetAxis::new(0);
    /// // Apply stronger braking to the right wheel (e.g., for turning assistance)
    /// axle.set_brakeforce(3000.0, &Side::Left);
    /// axle.set_brakeforce(3500.0, &Side::Right);
    /// ```
    pub fn set_brakeforce(&self, value: f32, s: &Side) {
        set_var(&format!("MBrake_Wheel_N_{}_{}", self.id, s), value);
    }

    /// Returns the current speed of a specific wheel in meters per second.
    ///
    /// Corresponds to the simulation variable: `v_Wheel_mps_{a}_{s}`
    /// where `{a}` is the axle ID and `{s}` is the side (Left/Right).
    ///
    /// # Arguments
    ///
    /// * `s` - Which side/wheel to get the speed for
    ///
    /// # Returns
    ///
    /// Current wheel speed in m/s. Positive values indicate forward motion.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_crate::management::enums::general_enums::Side;
    ///
    /// let axle = StreetAxis::new(0);
    /// let left_speed = axle.speed_mps(&Side::Left);
    /// let right_speed = axle.speed_mps(&Side::Right);
    ///
    /// // Check for wheel slip
    /// let speed_difference = (left_speed - right_speed).abs();
    /// if speed_difference > 2.0 {
    ///     println!("Warning: Significant wheel speed difference detected");
    /// }
    /// ```
    #[must_use]
    pub fn speed_mps(&self, s: &Side) -> f32 {
        get_var::<f32>(&format!("v_Wheel_mps_{}_{}", self.id, s))
    }

    /// Returns the spring deflection angle for a specific wheel in degrees.
    ///
    /// Corresponds to the simulation variable: `alpha_Wheel_deg_{a}_{s}`
    /// where `{a}` is the axle ID and `{s}` is the side (Left/Right).
    ///
    /// # Arguments
    ///
    /// * `s` - Which side/wheel to get the angle for
    ///
    /// # Returns
    ///
    /// Spring deflection angle in degrees.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_crate::management::enums::general_enums::Side;
    ///
    /// let axle = StreetAxis::new(0);
    /// let left_angle = axle.spring_wheel_deg(&Side::Left);
    /// let right_angle = axle.spring_wheel_deg(&Side::Right);
    /// println!("Suspension angles - Left: {}°, Right: {}°", left_angle, right_angle);
    /// ```
    #[must_use]
    pub fn spring_wheel_deg(&self, s: &Side) -> f32 {
        get_var::<f32>(&format!("alpha_Wheel_deg_{}_{}", self.id, s))
    }

    /// Returns the spring compression distance for a specific wheel in meters.
    ///
    /// Corresponds to the simulation variable: `spring_Wheel_m_{a}_{s}`
    /// where `{a}` is the axle ID and `{s}` is the side (Left/Right).
    ///
    /// # Arguments
    ///
    /// * `s` - Which side/wheel to get the compression for
    ///
    /// # Returns
    ///
    /// Spring compression distance in meters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_crate::management::enums::general_enums::Side;
    ///
    /// let axle = StreetAxis::new(0);
    /// let compression = axle.spring_wheel_m(&Side::Left);
    /// if compression > 0.05 {
    ///     println!("Heavy compression detected: {} m", compression);
    /// }
    /// ```
    #[must_use]
    pub fn spring_wheel_m(&self, s: &Side) -> f32 {
        get_var::<f32>(&format!("spring_Wheel_m_{}_{}", self.id, s))
    }

    /// Returns the steering angle for a specific wheel in meters.
    ///
    /// **Note**: The return type suggests this might be a steering displacement
    /// rather than an angle, despite the method name. The unit appears to be meters
    /// rather than degrees or radians.
    ///
    /// Corresponds to the simulation variable: `steering_Wheel_m_{a}_{s}`
    /// where `{a}` is the axle ID and `{s}` is the side (Left/Right).
    ///
    /// # Arguments
    ///
    /// * `s` - Which side/wheel to get the steering value for
    ///
    /// # Returns
    ///
    /// Steering value in meters (possibly steering linkage displacement).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use your_crate::management::enums::general_enums::Side;
    ///
    /// let axle = StreetAxis::new(0);
    /// let left_steering = axle.steering_wheel_m(&Side::Left);
    /// let right_steering = axle.steering_wheel_m(&Side::Right);
    /// println!("Steering positions - Left: {} m, Right: {} m",
    ///          left_steering, right_steering);
    /// ```
    #[must_use]
    pub fn steering_wheel_m(&self, s: &Side) -> f32 {
        get_var::<f32>(&format!("steering_Wheel_m_{}_{}", self.id, s))
    }
}
