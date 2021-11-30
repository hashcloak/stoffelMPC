mod arithmetic;
mod boolean;

#[derive(Debug, Clone, Copy)]
pub struct Wire(usize);

trait Cicuit {
    type Gate<T>;
    fn gates<T>(&self) -> &[Self::Gate<T>];
    fn mut_gates<T>(&mut self) -> &mut [Self::Gate<T>];
    fn nth_gate<T>(&self, n: usize) -> &Self::Gate<T>;
    fn mut_nth_gate<T>(&mut self, n: usize) -> &mut Self::Gate<T>;
    fn execute<T>(&self) -> Self::Gate<T>;
    fn size<T>(&self) -> usize;
}
