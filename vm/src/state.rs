use types::numbers::secret_sharing::{SecretSharing, shamir::Shamir};
use types::numbers::{
    fixed::{PubFixed, SecFixed},
    gf2::{PubGf2, SecGf2},
    int::{PubInt, SecInt},
    MPCType,
};
use ark_ff::fields::{PrimeField, SquareRootField};

#[derive(Clone, Debug, Default)]
pub struct Register<T: MPCType>(Vec<T>);

#[derive(Clone, Debug, Default)]
pub struct StackRegister<T: MPCType>(Vec<T>);

#[derive(Clone, Debug)]
pub struct Memory<T: MPCType, const N: usize>([T; N]);

#[derive(Clone, Debug, Default)]
pub struct GlobalMemory<const N: usize, const M: usize, Fr: PrimeField + SquareRootField> {
    secret_shared_int_memory: Memory<SecInt<Shamir<Fr>>, N>,
    secret_shared_gf2n_memory: Memory<SecGf2<Shamir<Fr>>, N>,
    public_int_memory: Memory<PubInt, N>,
    public_gf2n_int_memory: Memory<PubGf2<N>, N>,
}

impl<T: MPCType> Register<T> {
    fn read(&self, i: usize) -> T {
        self.0[i]
    }

    fn write(&mut self, i: usize, element: T) {
        self.0[i] = element;
    }
}

impl<T: MPCType> StackRegister<T> {
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

impl<T: MPCType, const N: usize> Memory<T, N> {
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

impl<T: MPCType, const N: usize> Default for Memory<T, N> {
    fn default() -> Self {
        Memory([T::default(); N])
    }
}

impl GlobalMemory {
    fn new() -> Self {
        todo!();
    }
}