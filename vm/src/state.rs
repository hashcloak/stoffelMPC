use types::vm::Number;

#[derive(Clone, Debug)]
pub struct Register<T: Number, const N: usize = 8>([T; N]);

impl<T: Number, const N: usize> Default for Register<T, N> {
    fn default() -> Self {
        Self([T::default(); N])
    }
}

impl<T: Number, const N: usize> Register<T, N> {
    fn read(&self, i: usize) -> T {
        self.0[i]
    }

    fn write(&mut self, i: usize, element: T) {
        self.0[i] = element;
    }
}

#[derive(Clone, Debug, Default)]
pub struct StackRegister<T: Number>(Vec<T>);

impl<T: Number> StackRegister<T> {
    fn push(&mut self, element: T) {
        self.0.push(element);
    }

    fn pop(&mut self) -> T {
        self.0.pop().unwrap()
    }

    fn peek(&self, location: usize) -> Option<&T> {
        self.0.get(location)
    }

    fn poke(&mut self, location: usize, element: T) {
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

    fn read() -> T {
        todo!();
    }

    fn write() {
        todo!();
    }

    fn allocate() {
        todo!();
    }

    fn deallocate() {
        todo!();
    }

    fn resize() {
        todo!();
    }
}
