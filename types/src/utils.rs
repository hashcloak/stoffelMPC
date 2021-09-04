use ark_bls12_381::Fr;
use ark_ff::BigInteger256;
use ark_ff::Field;
use ark_poly::univariate::DensePolynomial;
use ark_poly::UVPolynomial;
use rand::{thread_rng, Rng};

pub struct Polynomial<T: Field>(DensePolynomial<T>);

impl<T: Field> Polynomial<T> {
    pub fn random(degree: usize) -> Self {
        let mut rng = thread_rng();
        Polynomial(DensePolynomial::rand(degree, &mut rng))
    }

    pub fn set_coeff(&mut self, nth: usize, coefficient: T) {
        self.0.coeffs[nth] = coefficient;
    }
}
