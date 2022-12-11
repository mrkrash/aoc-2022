pub fn part_one(input: &str) -> Option<u32> {
    let mut h_actual_position: (u32, u32) = (500,500);
    let mut t_actual_position: (u32, u32) = (500,500);
    let mut visited_point: Vec<(u32, u32)> = vec![(500,500)];

    input
        .lines()
        .for_each(
            |line| {
                let _line = line.split(" ").collect::<Vec<&str>>();
                let direction: String = _line[0].parse().unwrap();
                let steps: u32 = _line[1].parse().unwrap();

                println!("-----------");
                for _i in 0 .. steps {
                    match direction.trim() {
                        "R" => {
                            h_actual_position = (h_actual_position.0 + 1, h_actual_position.1);
                            if not_surroundings(h_actual_position, t_actual_position) {
                                t_actual_position = (t_actual_position.0 + 1, h_actual_position.1);
                            }
                        },
                        "L" => {
                            h_actual_position = (h_actual_position.0 - 1, h_actual_position.1);
                            if not_surroundings(h_actual_position, t_actual_position) {
                                t_actual_position = (t_actual_position.0 - 1, h_actual_position.1);
                            }
                        },
                        "U" => {
                            h_actual_position = (h_actual_position.0, h_actual_position.1 + 1);
                            if not_surroundings(h_actual_position, t_actual_position) {
                                t_actual_position = (h_actual_position.0, t_actual_position.1 + 1);
                            }
                        },
                        "D" => {
                            h_actual_position = (h_actual_position.0, h_actual_position.1 - 1);
                            if not_surroundings(h_actual_position, t_actual_position) {
                                t_actual_position = (h_actual_position.0, t_actual_position.1 - 1);
                            }
                        },
                        _ => {}
                    }
                    println!("{} # H({},{}) T({},{})", line, h_actual_position.0, h_actual_position.1, t_actual_position.0, t_actual_position.1);
                    if !visited_point.contains(&t_actual_position) {
                        visited_point.push(t_actual_position);
                    }
                }
            }
    );

    Some(visited_point.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn not_surroundings(h_position:(u32, u32), t_position:(u32, u32)) -> bool {
    if h_position.0.abs_diff(t_position.0) == 1_u32 && h_position.1.abs_diff(t_position.1) == 1_u32 {
        return false;
    }
    if h_position.0.abs_diff(t_position.0) == 1_u32 && h_position.1.abs_diff(t_position.1) == 0_u32 {
        return false;
    }
    if h_position.0.abs_diff(t_position.0) == 0_u32 && h_position.1.abs_diff(t_position.1) == 1_u32 {
        return false;
    }
    if h_position.0.abs_diff(t_position.0) == 0_u32 && h_position.1.abs_diff(t_position.1) == 0_u32 {
        return false;
    }
    return true;
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
