use lotus_script::time::delta;

/// A time-based delay component that outputs a value only after it has been stable for a specified duration.
///
/// The `Delay` struct is useful for debouncing input values, ensuring that rapid changes
/// don't immediately propagate to the output. The output only changes when the input
/// has remained constant for at least the specified delay time.
///
/// # Type Parameters
///
/// * `T` - The type of values being delayed. Must implement `PartialEq` for comparison
///   and `Clone` for value copying.
///
/// # Examples
///
/// ```rust
/// use pandemist_vehicle_elements::Delay;
///
/// // Create a delay with 1.0 second delay time, initial value false
/// let mut delay = Delay::new(1.0, false);
///
/// // Initially, output matches the initial value
/// assert_eq!(delay.output, false);
///
/// // Change input to true, but output won't change immediately
/// delay.tick(true);
/// assert_eq!(delay.output, false); // Still false until delay expires
///
/// // After sufficient time passes (>= 1.0 seconds), output will change
/// // Note: You need to call tick() repeatedly in your game loop
/// ```
pub struct Delay<T: PartialEq + Clone> {
    /// The delay time in seconds before changes propagate to output
    time: f32,
    /// Current countdown timer in seconds
    timer: f32,
    /// The current input value being delayed
    input: T,
    /// The current output value (delayed input)
    pub output: T,
}

impl<T: PartialEq + Clone> Delay<T> {
    /// Creates a new `Delay` instance.
    ///
    /// # Parameters
    ///
    /// * `time` - The delay time in seconds. Must be positive.
    /// * `init_val` - The initial value for both input and output.
    ///
    /// # Returns
    ///
    /// A new `Delay` instance with the specified delay time and initial value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pandemist_vehicle_elements::Delay;
    ///
    /// // Create a 0.5 second delay for boolean values
    /// let delay = Delay::new(0.5, false);
    ///
    /// // Create a 2.0 second delay for string values
    /// let string_delay = Delay::new(2.0, "initial".to_string());
    /// ```
    pub fn new(time: f32, init_val: T) -> Self {
        Self {
            time,
            timer: 0.0,
            input: init_val.clone(),
            output: init_val,
        }
    }

    /// Updates the delay with a new target value and advances the internal timer.
    ///
    /// This method should be called once per frame/update cycle. It compares the
    /// target value with the current input, and if they differ, starts a new delay
    /// timer. If the timer expires (reaches zero or below), the output is updated
    /// to match the input.
    ///
    /// The timer is decremented by the frame delta time obtained from `lotus_script::time::delta()`.
    ///
    /// # Parameters
    ///
    /// * `target` - The new target value to potentially delay.
    ///
    /// # Behavior
    ///
    /// - If `target` differs from the current input, the delay timer is reset
    /// - If the timer has expired (≤ 0.0), the output is updated to match the input
    /// - Otherwise, the timer is decremented by the frame delta time
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pandemist_vehicle_elements::Delay;
    ///
    /// let mut delay = Delay::new(1.0, 0);
    ///
    /// // Update with new value
    /// delay.tick(42);
    /// // Output is still 0 immediately after the change
    ///
    /// // Continue calling tick() in your game loop...
    /// // After 1.0 seconds of delta time accumulation:
    /// // delay.output will become 42
    /// ```
    pub fn tick(&mut self, target: T) {
        if self.input != target {
            self.timer = self.time;
            self.input = target;
        }

        if self.timer <= 0.0 {
            self.output = self.input.clone();
        } else {
            self.timer -= delta();
        }
    }
}
