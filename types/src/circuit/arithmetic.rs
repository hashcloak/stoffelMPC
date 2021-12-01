use super::{Circuit, Wire};

pub struct ArithmeticCircuit<T>(Vec<ArithmeticGate<T>>);

impl<T> Circuit<T> for ArithmeticCircuit<T> {
    type Gate = ArithmeticGate<T>;

    fn gates(&self) -> &[Self::Gate] {
        &self.0
    }

    fn gates_mut(&mut self) -> &mut [Self::Gate] {
        &mut self.0
    }
}

/// A gate type for arithmetic operations
pub struct ArithmeticGate<T> {
    first: Wire,
    second: Wire,
    operation: fn(x: T, y: T) -> T,
}

impl<T: std::ops::Add<Output = T>> ArithmeticGate<T> {
    /// Create a gate for addition
    pub fn add(first: Wire, second: Wire) -> ArithmeticGate<T> {
        ArithmeticGate {
            first,
            second,
            operation: |x: T, y: T| -> T { x + y },
        }
    }
}

impl<T: std::ops::Mul<Output = T>> ArithmeticGate<T> {
    /// Create a gate for multiplication
    pub fn mul(first: Wire, second: Wire) -> ArithmeticGate<T> {
        ArithmeticGate {
            first,
            second,
            operation: |x: T, y: T| -> T { x * y },
        }
    }
}

impl<T: std::ops::Div<Output = T>> ArithmeticGate<T> {
    /// Create a gate for division
    pub fn div(first: Wire, second: Wire) -> ArithmeticGate<T> {
        ArithmeticGate {
            first,
            second,
            operation: |x: T, y: T| -> T { x / y },
        }
    }
}

impl<T: std::ops::Sub<Output = T>> ArithmeticGate<T> {
    /// Create a gate for subtraction
    pub fn sub(first: Wire, second: Wire) -> ArithmeticGate<T> {
        ArithmeticGate {
            first,
            second,
            operation: |x: T, y: T| -> T { x - y },
        }
    }
}
