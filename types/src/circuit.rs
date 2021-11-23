
type Wire = u64;

pub trait Gate {

    pub fn set_inputs();
    pub fn set_outputs();
    pub fn n_inputs();
    pub fn n_outputs();
    pub fn get_inputs();
    pub fn get_outputs()
}

pub trait Circuit {

    pub fn set_inputs();
    pub fn set_outputs();
    pub fn get_inputs();
    pub fn get_outputs();


    pub fn size();
    pub fn encode();
    pub fn decode();

    pub fn execute();
}

pub enum CircuitType {
    ArithmeticCircuit,
    BooleanCircuit
}

pub struct Circuit {
    pub circuit_type: CircuitType,
    pub gate_type: GateType
    pub inputs: Vec<Gate>,
    pub outputs: Vec<Gate>,
}

pub enum GateType {
    ArithmeticGate,
    BooleanGate
}

pub enum ArithmeticGate {
    Add,
    Mul
}

pub enum BooleanGate {
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

impl Circuit for Circuit {
    pub fn set_inputs() {
        todo!();
    }

    pub fn set_outputs() {
        todo!();
    }

    pub fn get_inputs() {
        todo!();
    }

    pub fn get_outputs() {
        todo!();
    }

    pub fn size() {
        todo!();
    }

    pub fn encode() {
        todo!();
    }

    pub fn decode() {
        todo!();
    }

    pub fn execute() {
        todo!();
    }
}

#[#[test]
fn name() {
    unimplemented!();
}]