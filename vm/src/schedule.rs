use crate::{program, Program};

/// Shedule for the execution.
#[derive(Debug)]
pub struct Schedule {
    /// Programs or tapes that will be executed according to the schedule.
    programs: Vec<Program>,
    /// Actual planning on how the programs will be executed.
    schedule: Vec<Vec<Vec<usize>>>,
}

impl Schedule {
    /// Creates a new schedule.
    pub fn new() -> Self {
        Self {
            programs: vec![],
            schedule: vec![],
        }
    }

    /// Loads the programs from a stream of bytes.
    pub fn load_programs_from_bytes(&mut self, bytes: impl AsRef<[u8]>) {
        todo!()
    }

    /// Creates the schedule for the execution.
    pub fn load_schedule(&mut self) {
        todo!()
    }

    /// Obtains the programs of a schedule.
    pub fn programs(&self) -> &Vec<Program> {
        &self.programs
    }

    /// Obtains the actual schedule.
    pub fn schedule(&self) -> &Vec<Vec<Vec<usize>>> {
        &self.schedule
    }
}
