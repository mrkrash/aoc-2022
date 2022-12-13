pub fn part_one(input: &str) -> Option<u32> {
    let first_cycle = 20;
    let interval = 40;
    let cycle_to_check = 6;

    let mut cycle = 0;
    let mut cycle_checked = 0;
    let mut register_value:i32 = 1;
    let mut signal_strength: i32 = 0;
    let lines = input.lines();

    for line in lines {
        let (op, value) = line.split_at(4);
        cycle += 1;

        if cycle == first_cycle + interval * cycle_checked {
            signal_strength += register_value * (first_cycle + interval * cycle_checked);
            cycle_checked += 1;
            if cycle_to_check == cycle_checked {
                break;
            }
        }
        if op == "addx" {
            cycle += 1;
            if cycle == first_cycle + interval * cycle_checked {
                signal_strength += register_value * (first_cycle + interval * cycle_checked);
                cycle_checked += 1;
                if cycle_to_check == cycle_checked {
                    break;
                }
            }
            register_value += value.trim().parse::<i32>().unwrap();
        }

    }
    Some(signal_strength as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let interval = 40;
    let cycle_to_check = 6;

    let mut cycle = 0;
    let mut actual_cycle = 1;
    let mut sprite:i32 = 1;
    let mut row: String = "".to_owned();
    let lines = input.lines();

    for line in lines {
        let (op, value) = line.split_at(4);
        cycle += 1;

        //print!("{cycle} {actual_cycle} {sprite}");
        if cycle >= sprite && cycle <= sprite +2 {
            row.push('#');
            //println!(" #");
        } else {
            row.push('.');
            //println!(" .");
        }
        if cycle == interval * actual_cycle {
            println!("{row}");
            row = "".to_owned();
            sprite += interval;
            if cycle_to_check == actual_cycle {
                break;
            }
            actual_cycle += 1;
        }

        if op == "addx" {
            cycle += 1;

            //print!("{cycle} {actual_cycle} {sprite}");
            if cycle >= sprite && cycle <= sprite +2 {
                row.push('#');
                //println!(" #");
            } else {
                row.push('.');
                //println!(" .");
            }
            sprite += value.trim().parse::<i32>().unwrap();

            if cycle == interval * actual_cycle {
                println!("{row}");
                row = "".to_owned();
                sprite += interval;
                if cycle_to_check == actual_cycle {
                    break;
                }
                actual_cycle += 1;
            }
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
