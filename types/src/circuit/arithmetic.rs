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
        for i in inputs_len..inputs_len + gates_len {
            let current_gate = self.nth_gate(i);
            let wire_1 = current_gate.first;
            let wire_2 = current_gate.second;
            outputs[inputs_len + i] = current_gate.execute(outputs[wire_1], outputs[wire_2])
        }
        outputs.remove(outputs.len() - 1)
    }
}

/// A gate type for arithmetic operations
pub struct ArithmeticGate<T> {
    pub first: usize,
    pub second: usize,
    operation: fn(x: T, y: T) -> T,
}

impl<T> ArithmeticGate<T> {
    // Execute the gate, yielding a result of the operation
    pub fn execute(&self, x: T, y: T) -> T {
        (self.operation)(x, y)
    }
}

impl<T: std::ops::Add<Output = T>> ArithmeticGate<T> {
    /// Create a gate for addition
    pub fn add(first: usize, second: usize) -> ArithmeticGate<T> {
        ArithmeticGate {
            first,
            second,
            operation: |x: T, y: T| -> T { x + y },
        }
    }
}

impl<T: std::ops::Mul<Output = T>> ArithmeticGate<T> {
    /// Create a gate for multiplication
    pub fn mul(first: usize, second: usize) -> ArithmeticGate<T> {
        ArithmeticGate {
            first,
            second,
            operation: |x: T, y: T| -> T { x * y },
        }
    }
}

impl<T: std::ops::Div<Output = T>> ArithmeticGate<T> {
    /// Create a gate for division
    pub fn div(first: usize, second: usize) -> ArithmeticGate<T> {
        ArithmeticGate {
            first,
            second,
            operation: |x: T, y: T| -> T { x / y },
        }
    }
}

impl<T: std::ops::Sub<Output = T>> ArithmeticGate<T> {
    /// Create a gate for subtraction
    pub fn sub(first: usize, second: usize) -> ArithmeticGate<T> {
        ArithmeticGate {
            first,
            second,
            operation: |x: T, y: T| -> T { x - y },
        }
    }
}
