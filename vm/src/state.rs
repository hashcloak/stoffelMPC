use types::numbers::{
    bit::{PubBit, SecBit},
    fixed::{PubFixed, SecFixed},
    float::{PubFloat, SecFloat},
    gf2::{PubGf2, SecGf2},
    int::{PubInt, SecInt},
    Number, SecretSharing,
};

#[derive(Clone, Debug, Default)]
pub struct StackRegisters<T: Number + SecretSharing, U: Number, const N: usize> {
    secret_int_memory: StackRegister<SecInt<T>>,
    pub_int_memory: StackRegister<PubInt<U>>,

    secret_fixed_memory: StackRegister<SecFixed<T>>,
    pub_fixed_memory: StackRegister<PubFixed<U>>,

    secret_float_memory: StackRegister<SecFloat<T>>,
    pub_float_memory: StackRegister<PubFloat<U>>,

    secret_bit_memory: StackRegister<SecBit<T>>,
    pub_bit_memory: StackRegister<PubBit<U>>,

    secret_gf2_memory: StackRegister<SecGf2<T>>,
    public_gf2_memory: StackRegister<PubGf2<U, N>>,
}

#[derive(Clone, Debug, Default)]
pub struct Registers<T: Number + SecretSharing, U: Number, const M: usize, const N: usize> {
    secret_int_memory: Register<SecInt<T>, N>,
    pub_int_memory: Register<PubInt<U>, N>,

    secret_fixed_memory: Register<SecFixed<T>, N>,
    pub_fixed_memory: Register<PubFixed<U>, N>,

    secret_float_memory: Register<SecFloat<T>, N>,
    pub_float_memory: Register<PubFloat<U>, N>,

    secret_bit_memory: Register<SecBit<T>, N>,
    pub_bit_memory: Register<PubBit<U>, N>,

    secret_gf2_memory: Register<SecGf2<T>, N>,
    public_gf2_memory: Register<PubGf2<U, M>, N>,
}

#[derive(Clone, Debug, Default)]
pub struct GlobalMemory<T: Number + SecretSharing, U: Number, const N: usize> {
    secret_int_memory: Memory<SecInt<T>>,
    pub_int_memory: Memory<PubInt<U>>,

    secret_fixed_memory: Memory<SecFixed<T>>,
    pub_fixed_memory: Memory<PubFixed<U>>,

    secret_float_memory: Memory<SecFloat<T>>,
    pub_float_memory: Memory<PubFloat<U>>,

    secret_bit_memory: Memory<SecBit<T>>,
    pub_bit_memory: Memory<PubBit<U>>,

    secret_gf2_memory: Memory<SecGf2<T>>,
    public_gf2_memory: Memory<PubGf2<U, N>>,
}

#[derive(Clone, Debug)]
struct Register<T: Number, const N: usize>([T; N]);

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
struct StackRegister<T: Number>(Vec<T>);

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
struct Memory<T: Number>(Vec<T>);

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
