use super::{Channel, Open};
use crate::types::{PubInt, SecInt};
use std::io::{Read, Write};
use std::marker::PhantomData;

pub struct HoneyBadger<T> {
    parties: Vec<Box<dyn Channel>>,
    threshold: usize,
    secret_type: PhantomData<T>,
}

impl<T> HoneyBadger<T> {
    fn batch_recieve(secrets: &[SecInt<T>]) -> Vec<PubInt> {
        todo!();
    }

    fn robust_interpolate(secrets: &[SecInt<T>]) -> Vec<PubInt> {
        todo!();
    }

    fn rs_decode() {
        todo!();
    }

    fn ran_dou_sha() {
        todo!();
    }

    fn vandermonde_interpolate() {
        todo!();
    }

    fn fft_interpolate() {
        todo!()
    }
}

impl<T> Open<T> for HoneyBadger<T> {
    fn open<U: Read + Write>(channel: &mut U, secret: SecInt<T>) -> PubInt {
        todo!()
    }
}
