use ark_ff::Field;
use ark_poly::univariate::DensePolynomial;
use ark_poly::{Polynomial as ArkPolynomial, UVPolynomial};
use rand::thread_rng;

pub struct Polynomial<T: Field>(DensePolynomial<T>);

impl<T: Field> Polynomial<T> {
    pub fn new(coeffs: &[T]) -> Self {
        Polynomial(DensePolynomial::from_coefficients_slice(coeffs))
    }

    pub fn random(degree: usize) -> Self {
        let mut rng = thread_rng();
        Polynomial(DensePolynomial::rand(degree, &mut rng))
    }

    pub fn set_coeff(&mut self, nth: usize, coefficient: T) {
        self.0.coeffs[nth] = coefficient;
    }

    pub fn evaluate(&self, value: &T) -> T {
        self.0.evaluate(value)
    }
}
