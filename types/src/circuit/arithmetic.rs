use super::{Circuit, Gate};

pub struct ArithmeticCircuit<T: Copy + Default> {
    inputs: Vec<T>,
    gates: Vec<ArithmeticGate<T>>,
}

impl<T: Copy + Default> Circuit<T> for ArithmeticCircuit<T> {
    type CircuitGate = ArithmeticGate<T>;

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

/// A gate type for arithmetic operations
pub struct ArithmeticGate<T: Copy + Default> {
    pub wires: Vec<usize>,
    operation: fn(values: &[T]) -> T,
}

impl<T: Copy + Default> Gate<T> for ArithmeticGate<T> {
    // Execute the gate, yielding a result of the operation
    fn compute(&self, values: &[T]) -> T {
        (self.operation)(values)
    }

    fn wires(&self) -> &[usize] {
        &self.wires
    }
}

impl<T: Copy + Default + std::ops::Add<Output = T>> ArithmeticGate<T> {
    /// Create a gate for addition
    pub fn add(wires: &[usize]) -> ArithmeticGate<T> {
        ArithmeticGate {
            wires: wires.to_vec(),
            operation: |values: &[T]| -> T {
                let mut out = values[0];
                for i in 1..values.len() {
                    out = out + values[i]
                }
                out
            },
        }
    }
}

impl<T: Copy + Default + std::ops::Mul<Output = T>> ArithmeticGate<T> {
    /// Create a gate for multiplication
    pub fn mul(wires: &[usize]) -> ArithmeticGate<T> {
        ArithmeticGate {
            wires: wires.to_vec(),
            operation: |values: &[T]| -> T {
                let mut out = values[0];
                for i in 1..values.len() {
                    out = out * values[i]
                }
                out
            },
        }
    }
}
