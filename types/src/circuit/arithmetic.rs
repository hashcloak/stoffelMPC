use super::{Circuit, Gate};

/// An arithmetic circuit
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
    wires: Vec<usize>,
    operation: fn(values: &[T]) -> T,
}

impl<T: Copy + Default> Gate<T> for ArithmeticGate<T> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arithmetic_circuit_simple() {
        let gate_add: ArithmeticGate<i32> = ArithmeticGate::add(&[0, 1]);
        let gate_mul: ArithmeticGate<i32> = ArithmeticGate::mul(&[0, 1]);

        let circuit_add = ArithmeticCircuit {
            inputs: vec![5, 7],
            gates: vec![gate_add],
        };
        let circuit_mul = ArithmeticCircuit {
            inputs: vec![5, 7],
            gates: vec![gate_mul],
        };
        assert_eq!(circuit_add.execute(), 12_i32);
        assert_eq!(circuit_mul.execute(), 35_i32);
    }

    #[test]
    fn arithmetic_circuit_complicated() {
        let gate_add: ArithmeticGate<i32> = ArithmeticGate::add(&[0, 1]);
        let gate_mul: ArithmeticGate<i32> = ArithmeticGate::mul(&[2, 3]);

        let circuit = ArithmeticCircuit {
            inputs: vec![2, 3, 5],
            gates: vec![gate_add, gate_mul],
        };

        assert_eq!(circuit.execute(), 25_i32);
    }
}
