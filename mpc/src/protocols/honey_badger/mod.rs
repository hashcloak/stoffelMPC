use super::{Channel, Open};
use crate::types::{PubInt, SecInt};
use std::io::{Read, Write};
use std::marker::PhantomData;

const FFT_THRESHOLD: usize = 1000;

pub struct HoneyBadger<T> {
    parties: Vec<Box<dyn Channel>>,
    threshold: usize,
    secret_type: PhantomData<T>,
}

impl<T> HoneyBadger<T> {
    fn batch_recieve(&mut self, secrets: &[SecInt<T>]) -> Vec<PubInt> {
        if self.parties.len() < FFT_THRESHOLD {
            self.vandermonde_interpolate(secrets)
        } else {
            self.fft_interpolate(secrets)
        }
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

    fn vandermonde_interpolate(&mut self, secrets: &[SecInt<T>]) -> Vec<PubInt> {
        todo!();
    }

    fn fft_interpolate(&mut self, secrets: &[SecInt<T>]) -> Vec<PubInt> {
        todo!()
    }
}

impl<T> Open<T> for HoneyBadger<T> {
    fn open<U: Read + Write>(channel: &mut U, secret: SecInt<T>) -> PubInt {
        todo!()
    }
}
