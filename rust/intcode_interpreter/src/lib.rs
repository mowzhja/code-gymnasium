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

#[cfg(tests)]
mod test {
    use super::*;
}
