
trait Gate {
    pub fn set_inputs();
    pub fn set_outputs();
    pub fn n_inputs();
    pub fn n_outputs();
    pub fn get_inputs();
    pub fn get_outputs()
}

enum ArithmeticGate {
    Add,
    Mul
}

enum BooleanGate {
    AND,
    OR,
    NOT,
    XOR,
    NAND
}

impl Gate for ArithmeticGate {
    fn set_inputs() {
        todo!();
    }

    fn set_outputs() {
        todo!();
    }

    fn n_inputs() {
        todo!();
    }

    fn n_outputs() {
        todo!();
    }

    fn get_inputs() {
        todo!();
    }

    fn get_outputs() {
        todo!()
    }
}

impl Gate for BooleanGate {
    fn set_inputs() {
        todo!();
    }

    fn set_outputs() {
        todo!();
    }

    fn n_inputs() {
        todo!();
    }

    fn n_outputs() {
        todo!();
    }

    fn get_inputs() {
        todo!();
    }

    fn get_outputs() {
        todo!()
    }
}
