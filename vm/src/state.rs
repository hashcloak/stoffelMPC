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

#[derive(Clone, Debug, Default)]
pub struct Memory<T: MPCType>(Vec<T>);

impl<T: MPCType> Register<T> {

}

impl<T: MPCType> StackRegister<T> {

    fn push(&mut self, element: T) {
        self.0.push(element);
    }

    fn pop<'a>(&mut self, mut element: &'a mut T) {
        //let tmp = self.0.pop().unwrap();
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

impl<T: MPCType> Memory<T> {

}