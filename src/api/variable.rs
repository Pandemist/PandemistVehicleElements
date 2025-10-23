//! Variable management module for lotus_script.
//!
//! This module provides a type-safe way to handle variables with different types
//! through the `Variable` struct and associated utility functions.

use lotus_script::var::VariableType;

/// A type-safe variable container that provides access to variables by name.
///
/// The `Variable` struct acts as a handle to a named variable of a specific type `T`.
/// It uses the `VariableType` trait to provide type-safe getting and setting operations.
///
/// # Type Parameters
///
/// * `T` - The type of the variable, which must implement `VariableType`
///
/// # Examples
///
/// ```rust
/// use variable::Variable;
///
/// // Create a string variable
/// let my_var: Variable<String> = Variable::new("my_string");
///
/// // Set a value
/// my_var.set("Hello, World!".to_string());
///
/// // Get the value
/// let value = my_var.get();
/// ```
#[derive(Debug)]
pub struct Variable<T> {
    /// The name identifier for this variable
    name: String,
    /// Phantom data to maintain type information at compile time
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Variable<T> {
    /// Creates a new variable handle with the specified name.
    ///
    /// This constructor creates a variable handle that can be used to access
    /// a variable by name. The actual variable storage is managed by the
    /// `VariableType` implementation.
    ///
    /// # Parameters
    ///
    /// * `name` - The name identifier for the variable (can be any type that converts to `String`)
    ///
    /// # Returns
    ///
    /// A new `Variable` instance that can be used to access the named variable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use variable::Variable;
    ///
    /// let var1: Variable<i32> = Variable::new("counter");
    /// let var2: Variable<String> = Variable::new(String::from("message"));
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: VariableType> Variable<T> {
    /// Retrieves the current value of the variable.
    ///
    /// This method uses the `VariableType` trait to get the variable's value
    /// in a type-safe manner. The return type is determined by the associated
    /// `Output` type of the `VariableType` implementation.
    ///
    /// # Returns
    ///
    /// The current value of the variable as defined by `T::Output`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use variable::Variable;
    ///
    /// let counter: Variable<i32> = Variable::new("my_counter");
    /// let current_value = counter.get();
    /// ```
    ///
    /// # Note
    ///
    /// This method is marked with `#[must_use]` to encourage handling of the
    /// returned value, as getting a variable value without using it is often
    /// a programming error.
    #[must_use]
    pub fn get(&self) -> T::Output {
        T::get_var(&self.name)
    }

    /// Sets the variable to a new value.
    ///
    /// This method uses the `VariableType` trait to set the variable's value
    /// in a type-safe manner.
    ///
    /// # Parameters
    ///
    /// * `value` - The new value to assign to the variable
    ///
    /// # Examples
    ///
    /// ```rust
    /// use variable::Variable;
    ///
    /// let counter: Variable<i32> = Variable::new("my_counter");
    /// counter.set(42);
    ///
    /// let message: Variable<String> = Variable::new("greeting");
    /// message.set("Hello, Rust!".to_string());
    /// ```
    pub fn set(&self, value: T) {
        T::set_var(&self.name, value);
    }
}

/// Retrieves a variable value by name using the specified type.
///
/// This is a convenience function that provides direct access to variable values
/// without needing to create a `Variable` instance first.
///
/// # Type Parameters
///
/// * `T` - The type of the variable, which must implement `VariableType`
///
/// # Parameters
///
/// * `name` - The name of the variable to retrieve
///
/// # Returns
///
/// The current value of the variable as defined by `T::Output`.
///
/// # Examples
///
/// ```rust
/// use variable::get_var;
///
/// let value: i32 = get_var::<i32>("counter");
/// let message: String = get_var::<String>("greeting");
/// ```
pub fn get_var<T: VariableType>(name: &str) -> T::Output {
    T::get_var(name)
}

/// Sets a variable value by name using the specified type.
///
/// This is a convenience function that provides direct access to variable setting
/// without needing to create a `Variable` instance first.
///
/// # Type Parameters
///
/// * `T` - The type of the variable, which must implement `VariableType`
///
/// # Parameters
///
/// * `name` - The name of the variable to set
/// * `var` - The value to assign to the variable
///
/// # Examples
///
/// ```rust
/// use variable::set_var;
///
/// set_var("counter", 42i32);
/// set_var("greeting", "Hello, World!".to_string());
/// ```
pub fn set_var<T: VariableType>(name: &str, var: T) {
    T::set_var(name, var);
}
