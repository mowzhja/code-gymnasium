use std::fs;
use std::process::exit;

use intcode_interpreter::*;

fn main() {
    // let text = fs::read_to_string("code.txt").expect("error reading file");
    // let p_vec = parse_program(&text);
    // let program = program::Program::new(p_vec);

    // Part 1:
    // program.restore_1202();
    // program.exec();

    // println!("Value at 0 => {}", program.get_result());

    // Part 2:
    // let target = 19690720;
    // let (noun, verb) = get_noun_verb_for(target, &program);
    // if (noun, verb) != (0, 0) {
    //     println!("Result (part 2) => {}", 100 * noun + verb);
    //     exit(0);
    // }
    // println!("Error bruteforcing target!");

    // Day 5, Part 1:
    let text = fs::read_to_string("tests.txt").expect("error reading file");
    let p_vec = parse_program(&text);
    let mut program = program::Program::new(p_vec);

    // remember to feed it 1 as first input
    program.exec();
}
