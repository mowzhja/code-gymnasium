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
            let (opcode, _, _, _) = self.get_modes();
            match opcode as isize {
                1 => {
                    let dest = self.instructions[self.ip + 3] as usize;

                    let (_, mode1, mode2, mode3) = self.get_modes();
                    if mode3 != 0 {
                        panic!("the address should always be in address mode");
                    }

                    self.perform_addition(mode1, mode2, dest);
                }
                2 => {
                    let dest = self.instructions[self.ip + 3] as usize;

                    let (_, mode1, mode2, mode3) = self.get_modes();
                    if mode3 != 0 {
                        panic!("the address should always be in address mode");
                    }

                    self.perform_multiplication(mode1, mode2, dest);
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

    /// Performs the addition operation (opcode 1)
    fn perform_addition(&mut self, mode1: usize, mode2: usize, dest: usize) {
        if mode1 == 0 && mode2 == 0 {
            // both in address mode
            self.instructions[dest] = self.instructions[self.instructions[self.ip + 1] as usize]
                + self.instructions[self.instructions[self.ip + 2] as usize];
        }

        if mode1 == 1 && mode2 == 0 {
            // left operand in immediate, right in address
            self.instructions[dest] = self.instructions[self.ip + 1]
                + self.instructions[self.instructions[self.ip + 2] as usize];
        }

        if mode1 == 0 && mode2 == 1 {
            // left operand in address, right in immediate
            self.instructions[dest] = self.instructions[self.instructions[self.ip + 1] as usize]
                + self.instructions[self.ip + 2];
        }

        if mode1 == 1 && mode2 == 1 {
            // both in immediate mode
            self.instructions[dest] =
                self.instructions[self.ip + 1] + self.instructions[self.ip + 2];
        }

        self.ip += 4;
    }

    /// Performs the multiplication operation (opcode 2)
    fn perform_multiplication(&mut self, mode1: usize, mode2: usize, dest: usize) {
        if mode1 == 0 && mode2 == 0 {
            // both in address mode
            self.instructions[dest] = self.instructions[self.instructions[self.ip + 1] as usize]
                * self.instructions[self.instructions[self.ip + 2] as usize];
        }

        if mode1 == 1 && mode2 == 0 {
            // left operand in immediate, right in address
            self.instructions[dest] = self.instructions[self.ip + 1]
                * self.instructions[self.instructions[self.ip + 2] as usize];
        }

        if mode1 == 0 && mode2 == 1 {
            // left operand in address, right in immediate
            self.instructions[dest] = self.instructions[self.instructions[self.ip + 1] as usize]
                * self.instructions[self.ip + 2];
        }

        if mode1 == 1 && mode2 == 1 {
            // both in immediate mode
            self.instructions[dest] =
                self.instructions[self.ip + 1] * self.instructions[self.ip + 2];
        }

        self.ip += 4;
    }

    /// Extracts the information about the various modes for opcode and parameters of an
    /// instruction
    fn get_modes(&self) -> (usize, usize, usize, usize) {
        let mut s = self.instructions[self.ip].to_string();
        // TODO this if is kind ugly...
        if s == "3" || s == "4" {
            // return immediately if opcode is 3 or 4
            return (s.parse::<usize>().unwrap(), 0, 0, 0);
        }

        if s.len() == 4 {
            // either a 1 or a 2 with a missing leading 0
            s = format!("0{}", s);
        }

        let opcode = s[3..].parse::<usize>().unwrap();
        // they appear in reverse order
        let mode1 = s.remove(2).to_digit(10).unwrap() as usize;
        let mode2 = s.remove(1).to_digit(10).unwrap() as usize;
        let mode3 = s.remove(0).to_digit(10).unwrap() as usize;

        (opcode, mode1, mode2, mode3)
    }

    /// Takes input from user and returns it for further processing
    fn input(&self) -> isize {
        let mut s = String::new();

        println!("INPUT:");
        io::stdin().read_line(&mut s).expect("failed reading input");

        if let Ok(n) = s.trim().parse::<isize>() {
            return n;
        } else {
            panic!("expected an integer as input");
        }
    }

    /// Prints output to the screen
    fn output(&self) {
        println!(
            "{}",
            self.instructions[self.instructions[self.ip + 1] as usize]
        );
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
