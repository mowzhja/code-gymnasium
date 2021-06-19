use std::io;

const HALT: usize = 99;

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

    /// Executes the program
    pub fn exec(&mut self) {
        loop {
            let (opcode, _, _, _) = self.get_modes();
            match opcode {
                1 => {
                    let dest = self.instructions[self.ip + 3] as usize;

                    let (_, mode1, mode2, _) = self.get_modes();
                    self.perform_addition(mode1, mode2, dest);
                }
                2 => {
                    let dest = self.instructions[self.ip + 3] as usize;

                    let (_, mode1, mode2, _) = self.get_modes();
                    self.perform_multiplication(mode1, mode2, dest);
                }
                3 => {
                    let input = self.input();
                    let dest = self.instructions[self.ip + 1] as usize;

                    self.instructions[dest] = input;
                    self.ip += 2;
                }
                4 => {
                    let (_, mode, _, _) = self.get_modes(); // rest are 0
                    self.output(mode);
                    self.ip += 2;
                }
                5 => {
                    let (_, mode1, mode2, _) = self.get_modes(); // mode3 is 0
                    self.perform_jump_if(true, mode1, mode2);
                }
                6 => {
                    let (_, mode1, mode2, _) = self.get_modes(); // mode3 is 0
                    self.perform_jump_if(false, mode1, mode2);
                }
                7 => {
                    let dest = self.instructions[self.ip + 3] as usize;

                    let (_, mode1, mode2, _) = self.get_modes();
                    self.perform_less_than(mode1, mode2, dest);
                }
                8 => {
                    let dest = self.instructions[self.ip + 3] as usize;

                    let (_, mode1, mode2, _) = self.get_modes();
                    self.perform_equals(mode1, mode2, dest);
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
    fn get_modes(&self) -> (usize, usize, usize, usize) {
        let (opcode, mode1, mode2, mode3);
        let mut s = self.instructions[self.ip].to_string();

        // TODO this if is kinda ugly...
        if s.len() == 1 || s == "99" {
            return (s.parse::<usize>().unwrap(), 0, 0, 0);
        }

        if s.len() == 4 {
            // either a 1 or a 2 with a missing leading 0
            s = format!("0{}", s);

            opcode = s[3..].parse::<usize>().unwrap();
            // they appear in reverse order
            mode1 = s.remove(2).to_digit(10).unwrap() as usize;
            mode2 = s.remove(1).to_digit(10).unwrap() as usize;
            mode3 = s.remove(0).to_digit(10).unwrap() as usize;
        } else {
            // we got a 4 (output) instruction
            opcode = s[1..].parse::<usize>().unwrap();
            mode1 = s.remove(0).to_digit(10).unwrap() as usize;
            mode2 = 0;
            mode3 = 0;
        }

        (opcode, mode1, mode2, mode3)
    }

    /// Performs the addition operation (opcode 1)
    fn perform_addition(&mut self, mode1: usize, mode2: usize, dest: usize) {
        if mode1 == 0 {
            // address mode
            if mode2 == 0 {
                // address mode
                self.instructions[dest] = self.instructions
                    [self.instructions[self.ip + 1] as usize]
                    + self.instructions[self.instructions[self.ip + 2] as usize];
            } else {
                // immediate mode
                self.instructions[dest] = self.instructions
                    [self.instructions[self.ip + 1] as usize]
                    + self.instructions[self.ip + 2];
            }
        } else {
            // immediate mode
            if mode2 == 0 {
                // address mode
                self.instructions[dest] = self.instructions[self.ip + 1]
                    + self.instructions[self.instructions[self.ip + 2] as usize];
            } else {
                // immediate mode
                self.instructions[dest] =
                    self.instructions[self.ip + 1] + self.instructions[self.ip + 2];
            }
        }

        self.ip += 4;
    }

    /// Performs the multiplication operation (opcode 2)
    fn perform_multiplication(&mut self, mode1: usize, mode2: usize, dest: usize) {
        if mode1 == 0 {
            // address mode
            if mode2 == 0 {
                // address mode
                self.instructions[dest] = self.instructions
                    [self.instructions[self.ip + 1] as usize]
                    * self.instructions[self.instructions[self.ip + 2] as usize];
            } else {
                // immediate mode
                self.instructions[dest] = self.instructions
                    [self.instructions[self.ip + 1] as usize]
                    * self.instructions[self.ip + 2];
            }
        } else {
            // immediate mode
            if mode2 == 0 {
                // address mode
                self.instructions[dest] = self.instructions[self.ip + 1]
                    * self.instructions[self.instructions[self.ip + 2] as usize];
            } else {
                // immediate mode
                self.instructions[dest] =
                    self.instructions[self.ip + 1] * self.instructions[self.ip + 2];
            }
        }

        self.ip += 4;
    }

    /// Takes input from user and returns it for further processing (opcode 3)
    fn input(&self) -> isize {
        let mut s = String::new();

        println!("INPUT:");
        io::stdin().read_line(&mut s).expect("failed reading input");

        if let Ok(n) = s.trim().parse::<isize>() {
            println!(); // for better formatting
            return n;
        } else {
            panic!("expected an integer as input");
        }
    }

    /// Prints output to the screen (opcode 4)
    fn output(&self, mode1: usize) {
        if mode1 == 0 {
            // address mode
            println!(
                "OUTPUT: {}",
                self.instructions[self.instructions[self.ip + 1] as usize]
            );
        } else {
            // immediate mode
            println!("OUTPUT: {}", self.instructions[self.ip + 1]);
        }
    }

    /// Performs the jump operations, both jump_true and jump_false (opcodes 5 and 6)
    fn perform_jump_if(&mut self, choice: bool, mode1: usize, mode2: usize) {
        let first = if mode1 == 0 {
            // address mode
            self.instructions[self.instructions[self.ip + 1] as usize]
        } else {
            // immediate mode
            self.instructions[self.ip + 1]
        };

        match choice {
            true => {
                if first != 0 {
                    if mode2 == 0 {
                        // address mode
                        self.ip =
                            self.instructions[self.instructions[self.ip + 2] as usize] as usize;
                    } else {
                        // immediate mode
                        self.ip = self.instructions[self.ip + 2] as usize;
                    }
                }
            }
            false => {
                if first == 0 {
                    if mode2 == 0 {
                        // address mode
                        self.ip =
                            self.instructions[self.instructions[self.ip + 2] as usize] as usize;
                    } else {
                        // immediate mode
                        self.ip = self.instructions[self.ip + 2] as usize;
                    }
                }
            }
        }
    }

    /// Performs the less_than operation (opcode 7)
    fn perform_less_than(&mut self, mode1: usize, mode2: usize, dest: usize) {
        if mode1 == 0 {
            // address mode
            if mode2 == 0 {
                // address mode
                self.instructions[dest] = if self.instructions
                    [self.instructions[self.ip + 1] as usize]
                    < self.instructions[self.instructions[self.ip + 2] as usize]
                {
                    1
                } else {
                    0
                };
            } else {
                // immediate mode
                self.instructions[dest] = if self.instructions
                    [self.instructions[self.ip + 1] as usize]
                    < self.instructions[self.ip + 2]
                {
                    1
                } else {
                    0
                };
            }
        } else {
            // immediate mode
            if mode2 == 0 {
                // address mode
                self.instructions[dest] = if self.instructions[self.ip + 1]
                    < self.instructions[self.instructions[self.ip + 2] as usize]
                {
                    1
                } else {
                    0
                };
            } else {
                // immediate mode
                self.instructions[dest] =
                    if self.instructions[self.ip + 1] < self.instructions[self.ip + 2] {
                        1
                    } else {
                        0
                    };
            }
        }

        self.ip += 4;
    }

    /// Performs the equals operation (opcode 8)
    fn perform_equals(&mut self, mode1: usize, mode2: usize, dest: usize) {
        if mode1 == 0 {
            // address mode
            if mode2 == 0 {
                // address mode
                self.instructions[dest] = if self.instructions
                    [self.instructions[self.ip + 1] as usize]
                    == self.instructions[self.instructions[self.ip + 2] as usize]
                {
                    1
                } else {
                    0
                };
            } else {
                // immediate mode
                self.instructions[dest] = if self.instructions
                    [self.instructions[self.ip + 1] as usize]
                    == self.instructions[self.ip + 2]
                {
                    1
                } else {
                    0
                };
            }
        } else {
            // immediate mode
            if mode2 == 0 {
                // address mode
                self.instructions[dest] = if self.instructions[self.ip + 1]
                    == self.instructions[self.instructions[self.ip + 2] as usize]
                {
                    1
                } else {
                    0
                };
            } else {
                // immediate mode
                self.instructions[dest] =
                    if self.instructions[self.ip + 1] == self.instructions[self.ip + 2] {
                        1
                    } else {
                        0
                    };
            }
        }

        self.ip += 4;
    }
}
