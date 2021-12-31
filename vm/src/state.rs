use types::numbers::secret_sharing::SecretSharing;
use types::numbers::{
    MPCType,
    fixed::{PubFixed, SecFixed},
    gf2::{PubGf2, SecGf2},
    int::{PubInt, SecInt},
};

#[derive(Clone, Debug, Default)]
pub struct Register<T: MPCType>(Vec<T>);

#[derive(Clone, Debug, Default)]
pub struct StackRegister<T: MPCType>(Vec<T>);

#[derive(Clone, Debug)]
pub struct Memory<T: MPCType, const N: usize>([T; N]);

impl<T: MPCType> Register<T> {

    fn read(&mut self, i: usize) -> T {
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

    fn pop<'a>(&mut self, element: &'a mut T) {
        *element = self.0.pop().unwrap();
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

}

impl<T: MPCType, const N: usize> Default for Memory<T, N> {
    fn default() -> Self {
        Memory([T::default(); N])
    }
}