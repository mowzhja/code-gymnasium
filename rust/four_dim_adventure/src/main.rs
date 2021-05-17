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

    for sp_point in spacetime_points.iter() {
        let in_same = in_same_constellation(sp_point, &spacetime_points);
        // HashSet = in_same.collect() ...
    }
    // TODO look into using HashSets and looking for intersections => intersection 
    // TODO means we have a constellation made up of multiple in_sames (look above)
}
