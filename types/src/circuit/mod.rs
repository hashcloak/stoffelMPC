mod arithmetic;
mod boolean;

trait Circuit<T: Copy + Default> {
    type CircuitGate;
    fn inputs(&self) -> &[T];
    fn inputs_mut(&mut self) -> &mut [T];

    fn nth_input(&self, n: usize) -> &T {
        &self.inputs()[n]
    }

    fn nth_input_mut(&mut self, n: usize) -> &mut T {
        &mut self.inputs_mut()[n]
    }

    fn gates(&self) -> &[Self::CircuitGate];
    fn gates_mut(&mut self) -> &mut [Self::CircuitGate];

    fn nth_gate(&self, n: usize) -> &Self::CircuitGate {
        &self.gates()[n]
    }

    fn nth_gate_mut(&mut self, n: usize) -> &mut Self::CircuitGate {
        &mut self.gates_mut()[n]
    }

    fn execute(&self) -> T;
}
