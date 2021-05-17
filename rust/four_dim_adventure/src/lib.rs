use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Reads a the file with filename and returns an Iterator to go over each line in the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file).lines())
}

/// Parses the lines of the input file and transforms them in coordinates with which we can
/// operate.
pub fn parse_coordinates(line: &str) -> Vec<isize> {
    let mut coords: Vec<isize> = Vec::new();

    let nums = line.split(",").collect::<Vec<_>>();
    for n in nums.iter() {
        if let Ok(xi) = n.parse::<isize>() {
            coords.push(xi);
        }
    }

    coords
}

/// Calculates the Manhattan distance (https://en.wikipedia.org/wiki/Manhattan_distance) given two points in spacetime.
pub fn manhattan_distance(x: &Vec<isize>, y: &Vec<isize>) -> isize {
    // ain't it elegant as fuck?
    x.iter()
        .zip(y.iter())
        .map(|(xi, yi)| (*xi - *yi).abs())
        .sum()
}

/// Returns the points in spacetime that are in the same constellation as x.
pub fn in_same_constellation(x: &Vec<isize>, spacetime: &Vec<Vec<isize>>) -> Vec<Vec<isize>> {
    let mut in_same: Vec<Vec<isize>> = Vec::new();

    for point in spacetime.iter() {
        if point != x {
            if manhattan_distance(point, x) <= 3 {
                in_same.push(point.clone());
            }
        }
    }

    in_same
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(manhattan_distance(&vec![0, 0, 0], &vec![0, 0, 0]), 0);
        assert_eq!(manhattan_distance(&vec![3, 0, 0], &vec![3, 0, 0]), 0);

        assert_eq!(manhattan_distance(&vec![0, 0, 0], &vec![3, 0, 0]), 3);
        assert_eq!(manhattan_distance(&vec![0, 0, 0], &vec![-3, 0, 0]), 3);

        assert_eq!(manhattan_distance(&vec![0, 0, 0], &vec![0, 4, 0]), 4);
        assert_eq!(manhattan_distance(&vec![0, 0, 0], &vec![0, 0, 5]), 5);
    }
}
