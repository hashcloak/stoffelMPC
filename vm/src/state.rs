use types::vm::MpcType;

// TODO: Define the correct type for the type T. A register can contain a
// Share<Domain> or a Domain.
#[derive(Clone, Debug)]
pub struct Register<T: MpcType, const N: usize = 8>([T; N]);

impl<T: MpcType, const N: usize> Default for Register<T, N> {
    fn default() -> Self {
        Self([T::default(); N])
    }
}

impl<T: MpcType, const N: usize> Register<T, N> {
    pub fn read(&self, i: usize) -> T {
        self.0[i]
    }

    pub fn write(&mut self, i: usize, element: T) {
        self.0[i] = element;
    }
}

// TODO: Define the correct type for the stack register, it can be Share<Domain>
// or Domain.
#[derive(Clone, Debug, Default)]
pub struct StackRegister<T: MpcType>(Vec<T>);

impl<T: MpcType> StackRegister<T> {
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

// TODO: Define the correct type for T. It can be a Share<Domain> or Domain.
#[derive(Clone, Debug, Default)]
pub struct Memory<T: MpcType>(Vec<T>);

// TODO: Define the correct type for T
impl<T: MpcType> Memory<T> {
    pub fn new() -> Self {
        Self(Vec::new())
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
