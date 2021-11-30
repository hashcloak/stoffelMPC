use super::{Circuit, Wire};

pub struct ArithmeticCircuit<T>(Vec<ArithmeticGate<T>>);

impl<T> Circuit for ArithmeticCircuit<T> {
    type Gate = ArithmeticGate<T>;

    fn gates(&self) -> &[Self::Gate] {
        todo!()
    }

    fn mut_gates(&mut self) -> &mut [Self::Gate] {
        todo!()
    }

    fn nth_gate(&self, n: usize) -> &Self::Gate {
        todo!()
    }

    fn mut_nth_gate(&mut self, n: usize) -> &mut Self::Gate {
        todo!()
    }

    fn execute(&self) -> Self::Gate {
        todo!()
    }

    fn size(&self) -> usize {
        todo!()
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
            operation: |first: T, second: T| -> T { first + second },
        }
    }
}

impl<T: std::ops::Mul<Output = T>> ArithmeticGate<T> {
    /// Create a gate for multiplication
    pub fn mul(first: Wire, second: Wire) -> ArithmeticGate<T> {
        ArithmeticGate {
            first,
            second,
            operation: |first: T, second: T| -> T { first * second },
        }
    }
}

impl<T: std::ops::Div<Output = T>> ArithmeticGate<T> {
    /// Create a gate for division
    pub fn div(first: Wire, second: Wire) -> ArithmeticGate<T> {
        ArithmeticGate {
            first,
            second,
            operation: |first: T, second: T| -> T { first / second },
        }
    }
}

impl<T: std::ops::Sub<Output = T>> ArithmeticGate<T> {
    /// Create a gate for subtraction
    pub fn sub(first: Wire, second: Wire) -> ArithmeticGate<T> {
        ArithmeticGate {
            first,
            second,
            operation: |first: T, second: T| -> T { first - second },
        }
    }
}
