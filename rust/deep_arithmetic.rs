/// Source: https://edabit.com/challenge/E8WcotHKRGfYodchW 
///
/// Note: the whole exercise can be made trivial by using std::iter::Iterator::flatten()

use std::error::Error;

/// Extract the sum of the digits contained within a string.
fn extract_partial_sum_from(s: &str) -> isize {
    let mut partial_sum: isize = 0;

    if s.parse::<isize>().is_ok() {
        // the whole string is a number
        partial_sum = s.parse().unwrap();
    } else {
        let mut buffer = String::new();

        for ch in s.chars() {
            if ch.is_ascii_digit() {
                buffer.push(ch);
            } else if ch == '-' {
                // the hyphens will be isolated but the minus signs will be close
                // to the numbers => when we parse successively it will detect the
                // negative numbers and ignore the hyphens!
                buffer.push('-');
            } else {
                buffer.push('.');
            }

        }

        let parts = buffer.split('.').collect::<Vec<&str>>(); // will have some blank entries
        for part in parts.iter() {
            if part.parse::<isize>().is_ok() {
                partial_sum += part.parse::<isize>().unwrap();
            }
        }
    }

    partial_sum
}

/// The function requested by the exercise.
fn sum(mut arr: String) -> isize {
    let mut sum = 0;

    arr = arr.replace("\"", "");
    arr = arr.replace("[", "");
    arr = arr.replace("]", "");

    let words: Vec<&str> = arr.split(',').collect();

    for word in words.iter() {
        sum += extract_partial_sum_from(word);
    }

    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let s = String::from("[\"1\", \"five\", \"2wenty\", \"thr33\"]");
    
    assert!(sum(s) == 36);
    println!("Test #1 passed");

    let s = String::from("[[\"1X2\", \"t3n\"],[\"1024\", \"5\", \"64\"]]");
    
    assert!(sum(s) == 1099);
    println!("Test #2 passed");

    let s = String::from("[[[\"1\"], \"10v3\"], [\"738h\"], [[\"s0\"], [\"1mu4ch3\"],\"-1s0\"]]");
    
    assert!(sum(s) == 759);
    println!("Test #3 passed");

    Ok(())
}
