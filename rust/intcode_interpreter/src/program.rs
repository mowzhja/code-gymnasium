const HALT: usize = 99;

#[derive(Clone)]
pub struct Program {
    ip: usize, // instruction pointer (mostly here for debug purposes)
    instructions: Vec<usize>,
}

impl Program {
    pub fn new(p_set: Vec<usize>) -> Self {
        Program {
            ip: 0,
            instructions: p_set.clone(),
        }
    }

    /// Executes the program
    pub fn exec(&mut self) {
        for i_set in self.instructions.clone().chunks(4) {
            // opcode
            match i_set[0] {
                1 => {
                    self.instructions[i_set[3]] =
                        self.instructions[i_set[1]] + self.instructions[i_set[2]];
                    self.ip += 1;
                }
                2 => {
                    self.instructions[i_set[3]] =
                        self.instructions[i_set[1]] * self.instructions[i_set[2]];
                    self.ip += 1;
                }
                HALT => {
                    return;
                }
                _ => {
                    panic!(
                        "instruction {} not recognized (at ip: {})",
                        i_set[0], self.ip
                    );
                }
            }
        }
    }

    /// Sets the noun to the desired value
    pub fn set_noun(&mut self, value: usize) {
        self.instructions[1] = value;
    }

    /// Sets the verb to the desired value
    pub fn set_verb(&mut self, value: usize) {
        self.instructions[2] = value;
    }

    /// Restores the gravity assist program to 1202 program alarm state
    pub fn restore_1202(&mut self) {
        self.instructions[1] = 12;
        self.instructions[2] = 2;
    }

    /// Gets the desired result (the value at position 0)
    pub fn get_result(&self) -> usize {
        self.instructions[0]
    }
}
