pub use polynomial::Polynomial;

mod polynomial {

    use ark_ff::fields::Field;
    use ark_poly::univariate::DensePolynomial;
    use ark_poly::{DenseUVPolynomial, Polynomial as ArkPolynomial};
    use rand::thread_rng;

    /// A struct representing a polynomial
    ///
    /// This struct wraps the `DensePolynomial` to provide
    /// some utility functions.
    pub struct Polynomial<T: Field>(DensePolynomial<T>);

    impl<T: Field> Polynomial<T> {
        /// Create a new polynomial from field elements.
        ///
        /// Uses a slice of field elements `coeffs` for creating
        /// the polynomial, i.e. the coefficients. First coefficient
        /// is for degree 0, second for degree 1, and so on.
        pub fn new(coeffs: &[T]) -> Self {
            Polynomial(DensePolynomial::from_coefficients_slice(coeffs))
        }

        /// Creates a random polynomial of `degree`
        pub fn random(degree: usize) -> Self {
            let mut rng = thread_rng();
            Polynomial(DensePolynomial::rand(degree, &mut rng))
        }

        /// Sets the nth coefficient to `value`
        pub fn set_coeff(&mut self, nth: usize, value: T) {
            self.0.coeffs[nth] = value;
        }

        /// Evaluates the polynomial at field element `value`
        pub fn evaluate(&self, value: &T) -> T {
            self.0.evaluate(value)
        }
    }
}
