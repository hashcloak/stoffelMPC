use super::Circuit;

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

    fn execute(&self) -> T {
        let inputs_len = self.inputs().len();
        let gates_len = self.gates().len();
        let mut outputs: Vec<T> = vec![T::default(); inputs_len + gates_len];
        outputs[..inputs_len].copy_from_slice(&self.inputs);
        for i in inputs_len..inputs_len + gates_len {}
        outputs.remove(outputs.len() - 1);
        todo!()
    }

    fn stats(&self) {
        todo!()
    }
}

/// A gate type for arithmetic operations
pub struct ArithmeticGate<T: Copy + Default> {
    pub inputs: Vec<usize>,
    operation: fn(values: &[T]) -> T,
}

impl<T: Copy + Default> ArithmeticGate<T> {
    // Execute the gate, yielding a result of the operation
    pub fn execute(&self, values: &[T]) -> T {
        (self.operation)(values)
    }
}

impl<T: Copy + Default + std::ops::Add<Output = T>> ArithmeticGate<T> {
    /// Create a gate for addition
    pub fn add(wires: &[usize]) -> ArithmeticGate<T> {
        let inputs = wires.to_vec();
        ArithmeticGate {
            inputs,
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
        let inputs = wires.to_vec();
        ArithmeticGate {
            inputs,
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
