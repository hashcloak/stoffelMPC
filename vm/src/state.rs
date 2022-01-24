use ark_ff::fields::{PrimeField, SquareRootField};
use types::numbers::secret_sharing::{shamir::Shamir, SecretSharing};
use types::numbers::{
    fixed::{PubFixed, SecFixed},
    gf2::{PubGf2, SecGf2},
    int::{PubInt, SecInt},
    Number,
};

#[derive(Clone, Debug, Default)]
pub struct Register<T: Number>(Vec<T>);

#[derive(Clone, Debug, Default)]
pub struct StackRegister<T: Number>(Vec<T>);

#[derive(Clone, Debug)]
pub struct Memory<T: Number, const N: usize>([T; N]);

#[derive(Clone, Debug, Default)]
pub struct GlobalMemory<Fr: PrimeField + SquareRootField, const N: usize, const M: usize> {
    secret_shared_int_memory: Memory<SecInt<Shamir<Fr>>, N>,
    secret_shared_gf2n_memory: Memory<SecGf2<Shamir<Fr>>, N>,
    public_int_memory: Memory<PubInt, N>,
    public_gf2n_int_memory: Memory<PubGf2<M>, N>,
}

impl<T: Number> Register<T> {
    fn read(&self, i: usize) -> T {
        self.0[i]
    }

    fn write(&mut self, i: usize, element: T) {
        self.0[i] = element;
    }
}

impl<T: Number> StackRegister<T> {
    fn push(&mut self, element: T) {
        self.0.push(element);
    }

    fn pop(&mut self) -> T {
        self.0.pop().unwrap()
    }

    fn peek<'a>(&'a mut self, location: usize, mut element: &'a mut T) {
        if location > self.0.len() {
            panic!("location is out of range");
        }
        element = &mut self.0[location];
    }

    fn poke(&mut self, location: usize, element: T) {
        if location > self.0.len() {
            panic!("location is out of range");
        }
        self.0[location] = element;
    }
}

impl<T: Number, const N: usize> Memory<T, N> {
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

impl<T: Number, const N: usize> Default for Memory<T, N> {
    fn default() -> Self {
        Memory([T::default(); N])
    }
}

impl<Fr: PrimeField + SquareRootField, const N: usize, const M: usize> GlobalMemory<Fr, N, M> {
    fn new() -> Self {
        todo!();
    }
}
