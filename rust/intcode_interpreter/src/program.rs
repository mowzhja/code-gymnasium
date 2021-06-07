use std::io;

const HALT: isize = 99;

#[derive(Clone)]
pub struct Program {
    ip: usize, // instruction pointer (mostly here for debug purposes)
    instructions: Vec<isize>,
}

impl Program {
    pub fn new(p_set: Vec<isize>) -> Self {
        Program {
            ip: 0,
            instructions: p_set.clone(),
        }
    }

    /// Executes the program
    pub fn exec(&mut self) {
        loop {
            // matches opcode
            match self.instructions[self.ip] {
                1 => {
                    let dest = self.instructions[self.ip + 3] as usize;

                    self.instructions[dest] =
                        self.instructions[self.instructions[self.ip + 1] as usize] + self.instructions[self.instructions[self.ip + 2] as usize];
                    self.ip += 4;
                }
                2 => {
                    let dest = self.instructions[self.ip + 3] as usize;

                    self.instructions[dest] =
                        self.instructions[self.instructions[self.ip + 1] as usize] * self.instructions[self.instructions[self.ip + 2] as usize];
                    self.ip += 4;
                }
                3 => {
                    let input = self.input();
                    let dest = self.instructions[self.ip + 1] as usize;

                    self.instructions[dest] = input;
                    self.ip += 2;
                }
                4 => {
                    self.output();
                    self.ip += 2;
                }
                HALT => {
                    return;
                }
                _ => {
                    panic!(
                        "instruction {} not recognized (at ip: {})",
                        self.instructions[self.ip], self.ip
                    );
                }
            }
        }
    }

    /// Extracts the information about the various modes for opcode and parameters of an
    /// instruction
    fn get_modes(&self) -> (usize, isize, isize, isize) {
        let mut s = self.instructions[self.ip].to_string();
        if s.len() == 4 {
            // either a 1 or a 2 with a missing leading 0
            s = format!("0{}", s);
        }

        let opcode = s[3..].parse::<usize>().unwrap();
        // they appear in reverse order
        let mode1 = s.remove(2).to_digit(10).unwrap() as isize;
        let mode2 = s.remove(1).to_digit(10).unwrap() as isize;
        let mode3 = s.remove(0).to_digit(10).unwrap() as isize;

        (opcode, mode1, mode2, mode3)
    }

    /// Takes input
    fn input(&self) -> isize {
        let mut s = String::new();
        print!("INPUT:");
        io::stdin().read_line(&mut s).expect("failed reading input");

        // TODO input checking should be added
        s.parse::<isize>().unwrap()
    }

    /// Prints output
    fn output(&self) {
        println!("{}", self.instructions[self.instructions[self.ip + 1] as usize]);
    }

    /// Sets the noun to the desired value
    pub fn set_noun(&mut self, value: isize) {
        self.instructions[1] = value;
    }

    /// Sets the verb to the desired value
    pub fn set_verb(&mut self, value: isize) {
        self.instructions[2] = value;
    }

    /// Restores the gravity assist program to 1202 program alarm state
    pub fn restore_1202(&mut self) {
        self.instructions[1] = 12;
        self.instructions[2] = 2;
    }

    /// Gets the the value at address 0 (in our case the desired result for both days 2 and 5)
    pub fn at_zero(&self) -> isize {
        self.instructions[0]
    }
}
