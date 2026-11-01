use lotus_script::traffic::TrafficLightPhase;

use crate::api::variable::get_var;

/// Displays the current traffic light phase.
///
/// # Returns
///
/// Returns the Trafficphase for this object, returns 'TrafficLightPhase::OffPrimary' if there is no Phase
///
/// # Examples
///
/// ```
/// use pandemist_vehicle_elements::world_options::trafficlight_phase;
///
/// let my_phase = trafficlight_phase();
/// ```
///
/// # Corresponds to
///
/// Script variable: `"trafficlight_phase"`
#[must_use]
pub fn trafficlight_phase() -> TrafficLightPhase {
    get_var::<TrafficLightPhase>("trafficlight_phase").unwrap_or(TrafficLightPhase::OffPrimary)
}

#[must_use]
pub fn trafficlight_phase_or(default: TrafficLightPhase) -> TrafficLightPhase {
    get_var::<TrafficLightPhase>("trafficlight_phase").unwrap_or(default)
}

#[must_use]
pub fn trafficlight_phase_is_green() -> bool {
    if let Some(phase) = get_var::<TrafficLightPhase>("trafficlight_phase") {
        phase == TrafficLightPhase::Green1
            || phase == TrafficLightPhase::Green2
            || phase == TrafficLightPhase::Green3
            || phase == TrafficLightPhase::YellowB3
    } else {
        false
    }
}

#[must_use]
pub fn trafficlight_phase_is_yellow() -> bool {
    if let Some(phase) = get_var::<TrafficLightPhase>("trafficlight_phase") {
        phase == TrafficLightPhase::YellowA1
            || phase == TrafficLightPhase::YellowA2
            || phase == TrafficLightPhase::YellowA3
            || phase == TrafficLightPhase::YellowB1
            || phase == TrafficLightPhase::YellowB2
            || phase == TrafficLightPhase::YellowB3
    } else {
        false
    }
}

#[must_use]
pub fn trafficlight_phase_is_red() -> bool {
    if let Some(phase) = get_var::<TrafficLightPhase>("trafficlight_phase") {
        phase == TrafficLightPhase::YellowA1
            || phase == TrafficLightPhase::YellowA2
            || phase == TrafficLightPhase::YellowA3
            || phase == TrafficLightPhase::Red1
            || phase == TrafficLightPhase::Red2
            || phase == TrafficLightPhase::Red3
    } else {
        false
    }
}

#[must_use]
pub fn trafficlight_phase_is_user() -> bool {
    if let Some(phase) = get_var::<TrafficLightPhase>("trafficlight_phase") {
        phase == TrafficLightPhase::User1
            || phase == TrafficLightPhase::User2
            || phase == TrafficLightPhase::User3
    } else {
        false
    }
}
