use lotus_extra::math::PiecewiseLinearFunction;
use lotus_script::var::set_var;

/// An animation controller that manages animation state and position.
///
/// The `Animation` struct provides a simple interface for controlling animations
/// by tracking their current position and optionally syncing with script variables
/// through the lotus_script system.
///
/// # Examples
///
/// ```
/// use your_crate::Animation;
///
/// // Create an animation with a name
/// let mut anim = Animation::new(Some("fade_in"));
/// anim.set(0.5); // Set animation to 50% progress
/// ```
#[derive(Debug)]
pub struct Animation {
    /// Optional name identifier for the animation.
    /// When present, position changes are synchronized with script variables.
    pub name: Option<String>,
    /// Current position/progress of the animation (typically 0.0 to 1.0).
    pub pos: f32,
}

impl Animation {
    /// Creates a new `Animation` instance.
    ///
    /// # Arguments
    ///
    /// * `name` - Optional name for the animation. If provided, the animation will sync its position with a script variable of the same name.
    ///
    /// # Returns
    ///
    /// A new `Animation` instance with position initialized to 0.0.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate::Animation;
    ///
    /// // Named animation
    /// let anim = Animation::new(Some("my_switch"));
    /// ```
    pub fn new(name: Option<&str>) -> Self {
        Self {
            name: name.map(|s| s.into()),
            pos: 0.0,
        }
    }

    /// Sets the animation position and updates the associated script variable.
    ///
    /// If the animation has a name, this method will update the corresponding
    /// script variable through `lotus_script::var::set_var`. Anonymous animations
    /// (those without a name) will only update their internal position.
    ///
    /// # Arguments
    ///
    /// * `pos` - The new position value for the animation
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate::Animation;
    ///
    /// let mut anim = Animation::new(Some("door_wing"));
    /// anim.set(0.75); // Sets animation to 75% and updates script variable
    /// assert_eq!(anim.pos, 0.75);
    /// ```
    pub fn set(&mut self, pos: f32) {
        if let Some(anim) = &self.name {
            set_var(anim, pos);
            self.pos = pos;
        }
    }
}

//========================================================================

/// The `MappedAnimation` struct provides a complexer interface for controlling animations
/// by tracking their current position and optionally syncing with script variables
/// through the lotus_script system, after mapping it to a given path.
///
/// # Examples
///
/// ```
/// use your_crate::MappedAnimation;
///
/// // Create an animation with a name
/// let mut anim = MappedAnimation::new(
///         Some("my_mapped_switch"),
///         Some(PiecewiseLinearFunction::new(vec![
///             (-17.0, -154.0),
///             (0.0, 0.0),
///             (1.0, 30.0),
///         ])),
///     );
/// anim.set(0.5); // Set animation to 50% progress
/// ```
#[derive(Debug)]
pub struct MappedAnimation {
    /// Optional name identifier for the animation.
    /// When present, position changes are synchronized with script variables.
    name: Option<String>,
    /// Optional path the animation should follow.
    path: Option<PiecewiseLinearFunction>,
    /// Current position/progress of the animation (typically 0.0 to 1.0).
    pub pos: f32,
}

impl MappedAnimation {
    /// Creates a new `MappedAnimation` instance.
    ///
    /// # Arguments
    ///
    /// * `name` - Optional name for the animation. If provided, the animation will sync its position with a script variable of the same name.
    /// * `path` - Optional path that the animation should follow.
    ///
    /// # Returns
    ///
    /// A new `MappedAnimation` instance with position initialized to 0.0.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::{
    ///     api::animation::MappedAnimation,
    ///     elements::std::piecewise_linear_function::PiecewiseLinearFunction,
    /// };
    ///
    /// // Named mapped animation
    /// let anim = MappedAnimation::new(
    ///         Some("my_mapped_switch"),
    ///         Some(PiecewiseLinearFunction::new(vec![
    ///             (-17.0, -154.0),
    ///             (0.0, 0.0),
    ///             (1.0, 30.0),
    ///         ])),
    ///     );
    /// ```
    pub fn new(name: Option<&str>, path: Option<PiecewiseLinearFunction>) -> Self {
        Self {
            name: name.map(|s| s.into()),
            path,
            pos: 0.0,
        }
    }

    /// Updates the animation to the given position.
    ///
    /// If the animation has a name, the value will be mapped through the path
    /// (if one exists) and then applied using [`set_var`].
    ///
    /// The internal `pos` field is updated to reflect the effective position value.
    ///
    /// # Arguments
    ///
    /// * `pos` - The new position to apply. This is either used directly or mapped
    ///   through the `path`.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate::MappedAnimation;
    ///
    /// let mut anim = MappedAnimation::new(Some("switch"));
    /// anim.set(0.75); // Sets animation to 75% and updates script variable
    /// assert_eq!(anim.pos, 0.75);
    /// ```
    pub fn set(&mut self, pos: f32) {
        if let Some(anim) = &self.name {
            let pos = if let Some(path) = &self.path {
                path.get_value_or_default(pos)
            } else {
                pos
            };
            set_var(anim, pos);
            self.pos = pos;
        }
    }
}
