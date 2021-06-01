use std::fs;

mod program;
use program::Program;

use intcode_interpreter::parse_program;

fn main() {
    let text = fs::read_to_string("code.txt").expect("error reading file");
    let p_vec = parse_program(&text);
    let mut program = Program::new(p_vec);

    program.restore_1202();
    program.exec();

    println!("Value at 0 => {}", program.get_result());
}
