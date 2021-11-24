struct Wire(usize);

enum Circuit<T> {
    Arithmetic(ArithmeticCircuit<T>),
    Boolean(BooleanCircuit<T>),
}

struct ArithmeticGate<T> {
    first: Wire,
    second: Wire,
    operation: fn(x: T, y: T) -> T,
}

impl<T> ArithmeticGate<T> {
    pub fn new(first: Wire, second: Wire, operation: fn(x: T, y: T) -> T) -> ArithmeticGate<T> {
        ArithmeticGate {
            first,
            second,
            operation,
        }
    }

    pub fn execute(&self, x: T, y: T) -> T {
        (self.operation)(x, y)
    }
}

impl<T: std::ops::Add<Output = T>> ArithmeticGate<T> {
    pub fn add(first: Wire, second: Wire) -> ArithmeticGate<T> {
        ArithmeticGate {
            first,
            second,
            operation: |first: T, second: T| -> T { first + second },
        }
    }
}

impl<T: std::ops::Mul<Output = T>> ArithmeticGate<T> {
    pub fn mul(first: Wire, second: Wire) -> ArithmeticGate<T> {
        ArithmeticGate {
            first,
            second,
            operation: |first: T, second: T| -> T { first * second },
        }
    }
}

// WIP
struct BooleanCircuit<T> {marker: std::marker::PhantomData<T>}

struct ArithmeticCircuit<T> {
    inputs: Vec<ArithmeticGate<T>>,
}

impl<T: Clone> ArithmeticCircuit<T> {
    pub fn execute(&self) -> T {
        let outputs: Vec<Option<T>> = vec![None; self.inputs.len()];
        todo!();
    }
}
