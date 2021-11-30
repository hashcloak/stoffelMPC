mod arithmetic;
mod boolean;

#[derive(Debug, Clone, Copy)]
pub struct Wire(usize);

trait Circuit {
    type Gate;
    fn gates(&self) -> &[Self::Gate];
    fn mut_gates(&mut self) -> &mut [Self::Gate];
    fn nth_gate(&self, n: usize) -> &Self::Gate;
    fn mut_nth_gate(&mut self, n: usize) -> &mut Self::Gate;
    fn execute(&self) -> Self::Gate;
    fn size(&self) -> usize;
}
