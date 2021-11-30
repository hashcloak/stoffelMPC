use super::Wire;

pub struct ArithmeticCircuit();

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
