use super::{Circuit, Gate};

/// A boolean circuit
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
    pub fn and(wires: &[usize]) -> BooleanGate<T> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boolean_circuit_simple() {
        let gate_and = BooleanGate::and(&[0, 1]);
        let gate_or = BooleanGate::or(&[0, 1]);

        let circuit_and = BooleanCircuit {
            inputs: vec![true, false],
            gates: vec![gate_and],
        };
        let circuit_or = BooleanCircuit {
            inputs: vec![true, false],
            gates: vec![gate_or],
        };
        assert_eq!(circuit_and.execute(), false);
        assert_eq!(circuit_or.execute(), true);
    }

    #[test]
    fn boolean_circuit_complicated() {
        let gate_not = BooleanGate::not(0);
        let gate_and = BooleanGate::and(&[1, 2]);

        let circuit = BooleanCircuit {
            inputs: vec![false, true],
            gates: vec![gate_not, gate_and],
        };

        assert_eq!(circuit.execute(), true);
    }
}
