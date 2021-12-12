use super::{Circuit, Gate};

pub struct BooleanCircuit<T: Copy + Default> {
    inputs: Vec<T>,
    gates: Vec<BooleanGate<T>>,
}

impl<T: Copy + Default> Circuit<T> for BooleanCircuit<T> {
    type CircuitGate = BooleanGate<T>;

    fn inputs(&self) -> &[T] {
        &self.inputs
    }

    fn inputs_mut(&mut self) -> &mut [T] {
        &mut self.inputs
    }

    fn gates(&self) -> &[Self::CircuitGate] {
        &self.gates
    }

    fn gates_mut(&mut self) -> &mut [Self::CircuitGate] {
        &mut self.gates
    }
}

/// A gate type for boolean operations
pub struct BooleanGate<T: Copy + Default> {
    wires: Vec<usize>,
    operation: fn(values: &[T]) -> T,
}

impl<T: Copy + Default> Gate<T> for BooleanGate<T> {
    // Execute the gate, yielding a result of the operation
    fn compute(&self, values: &[T]) -> T {
        (self.operation)(values)
    }

    fn wires(&self) -> &[usize] {
        &self.wires
    }
}

impl<T: Copy + Default + std::ops::BitOr<Output = T>> BooleanGate<T> {
    /// Create a gate for or
    pub fn or(wires: &[usize]) -> BooleanGate<T> {
        BooleanGate {
            wires: wires.to_vec(),
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
        BooleanGate {
            wires: wires.to_vec(),
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
        BooleanGate {
            wires: vec![wire],
            operation: |values: &[T]| -> T { !values[0] },
        }
    }
}
