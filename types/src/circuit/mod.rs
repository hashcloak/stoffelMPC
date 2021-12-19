mod arithmetic;
mod boolean;

/// Trait for different circuit types
///
/// A circuit reperesents an amount of gates connected by wires.
/// This allows to implement arbitrary operations.
trait Circuit<T: Copy + Default> {
    /// The gate type used in this circuits
    type CircuitGate: Gate<T>;

    /// Get reference to all inputs
    fn inputs(&self) -> &[T];

    /// Get mutable reference to all inputs
    fn inputs_mut(&mut self) -> &mut [T];

    /// Get a reference to the nth input
    fn nth_input(&self, n: usize) -> &T {
        &self.inputs()[n]
    }

    /// Get a mutable reference to the nth input
    fn nth_input_mut(&mut self, n: usize) -> &mut T {
        &mut self.inputs_mut()[n]
    }

    /// Get a reference to all gates
    fn gates(&self) -> &[Self::CircuitGate];

    /// Get a mutable reference to all gates
    fn gates_mut(&mut self) -> &mut [Self::CircuitGate];

    /// Get a reference to the nth gate
    fn nth_gate(&self, n: usize) -> &Self::CircuitGate {
        &self.gates()[n]
    }

    /// Get a mutable reference to the nth gate
    fn nth_gate_mut(&mut self, n: usize) -> &mut Self::CircuitGate {
        &mut self.gates_mut()[n]
    }

    /// Execute the whole circuit, returning a result
    fn execute(&self) -> T {
        let inputs_len = self.inputs().len();
        let gates_len = self.gates().len();
        let mut outputs: Vec<T> = vec![T::default(); inputs_len + gates_len];
        outputs[..inputs_len].copy_from_slice(&self.inputs());
        for i in inputs_len..inputs_len + gates_len {
            let gate = self.nth_gate(i - inputs_len);
            let inputs = {
                let mut tmp: Vec<T> = Vec::with_capacity(gate.wires().len());
                gate.wires()
                    .iter()
                    .copied()
                    .for_each(|x| tmp.push(outputs[x]));
                tmp
            };
            outputs[i] = self.nth_gate(i - inputs_len).compute(inputs.as_ref());
        }
        outputs.remove(outputs.len() - 1)
    }

    /// Obtain some information about the circuit
    fn stats(&self) {
        todo!()
    }
}

/// Trait for different gate types
///
/// A gate represents a small computation unit
pub trait Gate<T: Copy + Default> {
    /// Get the input wires to this circuit
    fn wires(&self) -> &[usize];

    /// Perform the computation of this gate with the provided values
    fn compute(&self, values: &[T]) -> T;
}
