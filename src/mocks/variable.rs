use lotus_script::var::VariableType;

#[derive(Debug)]
pub struct Variable<T> {
    name: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Variable<T> {
    pub fn new(name: String) -> Self {
        Variable {
            name: name,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: VariableType> Variable<T> {
    pub fn get(&self) -> T {
        T::get(&self.name)
    }

    pub fn set(&self, value: &T) {
        value.set(&self.name);
    }
}
