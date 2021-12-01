use super::Wire;

pub struct BooleanCircuit<T>(Vec<BooleanGate<T>>);

/// A gate type for boolean operations
pub struct BooleanGate<T> {
    first: Wire,
    second: Wire,
    operation: fn(x: T, y: T) -> T,
}

impl<T: std::ops::BitOr<Output = T>> BooleanGate<T> {
    /// Create a gate for or
    pub fn or(first: Wire, second: Wire) -> BooleanGate<T> {
        BooleanGate {
            first,
            second,
            operation: |first: T, second: T| -> T { first | second },
        }
    }
}

impl<T: std::ops::BitAnd<Output = T>> BooleanGate<T> {
    /// Create a gate for and
    pub fn add(first: Wire, second: Wire) -> BooleanGate<T> {
        BooleanGate {
            first,
            second,
            operation: |first: T, second: T| -> T { first & second },
        }
    }
}

impl<T: std::ops::Not<Output = T>> BooleanGate<T> {
    /// Create a gate for not
    pub fn not(only: Wire) -> BooleanGate<T> {
        BooleanGate {
            first: only,
            second: only,
            operation: |first: T, _second: T| -> T { !first },
        }
    }
}
