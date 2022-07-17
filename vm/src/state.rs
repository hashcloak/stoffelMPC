use types::vm::Number;

#[derive(Clone, Debug)]
pub struct Register<T: Number, const N: usize = 8>([T; N]);

impl<T: Number, const N: usize> Default for Register<T, N> {
    fn default() -> Self {
        Self([T::default(); N])
    }
}

impl<T: Number, const N: usize> Register<T, N> {
    pub fn read(&self, i: usize) -> T {
        self.0[i]
    }

    pub fn write(&mut self, i: usize, element: T) {
        self.0[i] = element;
    }
}

#[derive(Clone, Debug, Default)]
pub struct StackRegister<T: Number>(Vec<T>);

impl<T: Number> StackRegister<T> {
    pub fn push(&mut self, element: T) {
        self.0.push(element);
    }

    pub fn pop(&mut self) -> T {
        self.0.pop().unwrap()
    }

    pub fn peek(&self, location: usize) -> Option<&T> {
        self.0.get(location)
    }

    pub fn poke(&mut self, location: usize, element: T) {
        if location > self.0.len() {
            panic!("location is out of range");
        }
        self.0[location] = element;
    }
}

#[derive(Clone, Debug, Default)]
pub struct Memory<T: Number>(Vec<T>);

impl<T: Number> Memory<T> {
    fn new() -> Self {
        todo!();
    }

    pub fn read() -> T {
        todo!();
    }

    pub fn write() {
        todo!();
    }

    pub fn allocate() {
        todo!();
    }

    pub fn deallocate() {
        todo!();
    }

    pub fn resize() {
        todo!();
    }
}
