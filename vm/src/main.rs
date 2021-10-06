fn main() {}

pub struct StoffelVM {
    stack: Vec<u8>,
    memory: Vec<u8>,
    code: Vec<u8>,
    sp: u32,
    fp: u32,
    ip: u32,
    log_level: (),
}

impl StoffelVM {
    pub fn new() -> Self {
        todo!()
    }

    pub fn run(&mut self) {
        loop {
            todo!()
        }
    }

    fn load_code(&mut self) {}
}
