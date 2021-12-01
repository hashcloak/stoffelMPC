mod arithmetic;
mod boolean;

#[derive(Debug, Clone, Copy)]
pub struct Wire(usize);

trait Circuit<T> {
    type Gate;
    fn gates(&self) -> &[Self::Gate];
    fn gates_mut(&mut self) -> &mut [Self::Gate];

    fn nth_gate(&self, n: usize) -> &Self::Gate {
        &self.gates()[n]
    }

    fn nth_gate_mut(&mut self, n: usize) -> &mut Self::Gate {
        &mut self.gates_mut()[n]
    }

    fn execute(&self) -> T {
        todo!()
    }

    fn size(&self) -> usize {
        self.gates().len()
    }
}
