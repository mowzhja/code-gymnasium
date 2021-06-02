pub mod program; // pub to expose it to main.rs as well

use program::Program;

/// Turns the string data into an instruction format we can use to work.
pub fn parse_program(program_text: &str) -> Vec<usize> {
    program_text
        .trim() // so that we don't have to remember to put it in main every time
        .split(',')
        .collect::<Vec<_>>()
        .iter()
        .map(|&s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

/// Brute force noun and verb values until we get the desired target at address 0.
pub fn get_noun_verb_for(target: usize, original_p: &Program) -> (usize, usize) {
    let (mut noun, mut verb): (usize, usize) = (0, 0);

    'out: for n in 0..=99 {
        for v in 0..=99 {
            let mut copy = original_p.clone();
            copy.set_noun(n);
            copy.set_verb(v);

            copy.exec();
            if copy.get_result() == target {
                noun = n;
                verb = v;
                break 'out;
            }
        }
    }

    (noun, verb)
}

#[cfg(tests)]
mod test {
    use super::*;
}
