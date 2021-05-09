/// Source: https://edabit.com/challenge/eiwM33xiRvWwpRZua

use std::error::Error;

/// Gets max value of the array vs (supposed unsorted).
fn get_max(vs: &[u32]) -> u32 {
    let mut max = 0;

    // yes, i could use builtin Rust functionality, but
    // classic is best sometimes
    for i in 0..vs.len() {
        if max < vs[i] {
            max = vs[i]
        }
    }

    max
}

/// The required function.
fn window_maxes(A: Vec<u32>, wl: usize) -> Vec<u32> {
    let mut M: Vec<u32> = Vec::new();
    let iters = (A.len() - wl) + 1;

    for i in 0..iters {
        M.push(get_max(&A[i..(i + wl)]))
    }

    M
}

fn main() -> Result<(), Box<dyn Error>> {
    let A = Vec::from([1, 2, 3, 4, 3, 2, 1, 2, 5]);

    assert!(window_maxes(A, 3) == [3, 4, 4, 4, 3, 2, 5]);
    println!("Test #1 passed");

    Ok(())
}
