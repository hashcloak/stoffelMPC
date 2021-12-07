pub struct BooleanCircuit<T: Copy + Default>(Vec<BooleanGate<T>>);

/// A gate type for boolean operations
pub struct BooleanGate<T: Copy + Default> {
    inputs: Vec<usize>,
    operation: fn(values: &[T]) -> T,
}

impl<T: Copy + Default + std::ops::BitOr<Output = T>> BooleanGate<T> {
    /// Create a gate for or
    pub fn or(wires: &[usize]) -> BooleanGate<T> {
        let inputs = wires.to_vec();
        BooleanGate {
            inputs,
            operation: |values: &[T]| -> T {
                let mut out = values[0];
                for i in 1..values.len() {
                    out = out | values[i]
                }
                out
            },
        }
    }
}

impl<T: Copy + Default + std::ops::BitAnd<Output = T>> BooleanGate<T> {
    /// Create a gate for and
    pub fn add(wires: &[usize]) -> BooleanGate<T> {
        let inputs = wires.to_vec();
        BooleanGate {
            inputs,
            operation: |values: &[T]| -> T {
                let mut out = values[0];
                for i in 1..values.len() {
                    out = out & values[i]
                }
                out
            },
        }
    }
}

impl<T: Copy + Default + std::ops::Not<Output = T>> BooleanGate<T> {
    /// Create a gate for not
    pub fn not(wire: usize) -> BooleanGate<T> {
        let inputs = vec![wire];
        BooleanGate {
            inputs,
            operation: |values: &[T]| -> T { !values[0] },
        }
    }
}
