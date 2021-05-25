use std::collections::{HashSet, HashMap};
use four_dim_adventure::*;

fn main() {
    let mut spacetime_points: Vec<Vec<isize>> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                spacetime_points.push(parse_coordinates(&ip));
            }
        }
    }

    // XXX
    // use this: https://en.wikipedia.org/wiki/DBSCAN
    // XXX
}
