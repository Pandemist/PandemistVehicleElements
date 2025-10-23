use lotus_script::content::{preload, ContentId};

/// A roller blind animation system that smoothly transitions between texture frames.
///
/// `Rollerblind` manages the animation of textures by dynamically loading and transitioning
/// between different texture frames based on a display ID. It provides smooth interpolation
/// between frames using a shift value and preloads upcoming textures for optimal performance.
///
/// The system works by:
/// - Maintaining references to upper and lower texture frames
/// - Calculating texture indices based on the current display ID
/// - Preloading the next texture frame to ensure smooth playback
/// - Providing fractional shift values for smooth interpolation between frames
///
/// # Examples
///
/// ```rust
/// use rollerblind::Rollerblind;
///
/// // Create a new roller blind animation
/// let mut blind = Rollerblind::new(
///     "upper_texture",
///     "lower_texture",
///     123,  // user_id
///     100   // base_sub_id
/// );
///
/// // Update animation with display_id 2.5
/// blind.tick(2.5);
///
/// // The blind will now show frame 102 as upper texture,
/// // frame 103 as lower texture, with 0.5 shift for interpolation
/// ```
#[derive(Debug)]
pub struct Rollerblind {
    /// Name identifier for the upper texture
    tex_name_upper: String,
    /// Name identifier for the lower texture  
    tex_name_lower: String,

    /// User identifier for content access
    user_id: i32,
    /// Base texture sub-identifier used as starting point for frame calculation
    base_sub_id: i32,

    /// Last calculated texture sub-identifier to avoid redundant updates
    tex_sub_id_last: i32,

    /// Content ID for the currently active upper texture frame
    upper_tex: ContentId,
    /// Content ID for the currently active lower texture frame
    lower_tex: ContentId,
    /// Fractional shift value between 0.0 and 1.0 for smooth interpolation
    shift: f32,
}

impl Rollerblind {
    /// Creates a new `Rollerblind` instance.
    ///
    /// # Arguments
    ///
    /// * `upper_tex` - Name or identifier for the upper texture (converted to String)
    /// * `lower_tex` - Name or identifier for the lower texture (converted to String)  
    /// * `user_id` - User identifier for content system access
    /// * `base_sub_id` - Base sub-identifier used as starting point for texture frame calculation
    ///
    /// # Returns
    ///
    /// A new `Rollerblind` instance initialized with the provided parameters.
    /// Both upper and lower textures start with the same `ContentId` using the base_sub_id.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let blind = Rollerblind::new("sky_texture", "ground_texture", 456, 200);
    /// ```
    #[must_use]
    pub fn new(
        upper_tex: impl Into<String>,
        lower_tex: impl Into<String>,
        user_id: i32,
        base_sub_id: i32,
    ) -> Self {
        Self {
            tex_name_upper: upper_tex.into(),
            tex_name_lower: lower_tex.into(),

            user_id,
            base_sub_id,

            tex_sub_id_last: -1,

            upper_tex: ContentId {
                user_id,
                sub_id: base_sub_id,
            },
            lower_tex: ContentId {
                user_id,
                sub_id: base_sub_id,
            },
            shift: 0.0,
        }
    }

    /// Updates the roller blind animation state based on the provided display ID.
    ///
    /// This method calculates which texture frames should be active and updates
    /// the internal state accordingly. It also handles preloading of upcoming
    /// textures for smooth playback.
    ///
    /// # Arguments
    ///
    /// * `display_id` - A floating-point value that determines the current animation frame.
    ///   The integer part determines the base frame index, while the fractional part
    ///   is used for interpolation between frames.
    ///
    /// # Behavior
    ///
    /// - Calculates the current texture sub-identifier by adding the integer part of
    ///   `display_id` to the base_sub_id
    /// - Sets the shift value to the fractional part of `display_id` (0.0 to 1.0)
    /// - Updates upper_tex to the current frame and lower_tex to the next frame
    /// - Preloads the frame after next for optimal performance
    /// - Only updates texture references if the frame has actually changed
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut blind = Rollerblind::new("tex_a", "tex_b", 1000, 50);
    ///
    /// // Update to texture 52.25
    /// blind.tick(2.25);
    /// // upper_tex will be texture 52, lower_tex texture 53, shift = 0.25
    ///
    /// // Update to texture 53.75  
    /// blind.tick(3.75);
    /// // upper_tex will be texture 53, lower_tex texture 54, shift = 0.75
    /// ```
    pub fn tick(&mut self, display_id: f32) {
        let tex_sub_id = self.base_sub_id + (display_id as i32);
        self.shift = display_id.rem_euclid(1.0);

        if tex_sub_id != self.tex_sub_id_last {
            self.upper_tex = ContentId {
                user_id: self.user_id,
                sub_id: tex_sub_id,
            };
            self.lower_tex = ContentId {
                user_id: self.user_id,
                sub_id: (tex_sub_id + 1).min(tex_sub_id),
            };
            preload(ContentId {
                user_id: self.user_id,
                sub_id: (tex_sub_id + 2).min(tex_sub_id),
            });
        }

        self.tex_sub_id_last = tex_sub_id;
    }
}
