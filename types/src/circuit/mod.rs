mod arithmetic;
mod boolean;

trait Circuit<T: Copy + Default> {
    type CircuitGate: Gate<T>;

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

    fn stats(&self) {
        todo!()
    }
}

pub trait Gate<T: Copy + Default> {
    fn wires(&self) -> &[usize];
    fn compute(&self, values: &[T]) -> T;
}
