pub fn part_one(input: &str) -> Option<u32> {
    let mut strong_elv = 0;
    let mut this_elv = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            if this_elv > strong_elv {
                strong_elv = this_elv;
            }
            this_elv = 0;
        } else {
            this_elv += line.parse::<i32>().unwrap();
        }
    }
    Some(strong_elv as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut first_three = [0, 0, 0];
    let mut this_elv = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            first_three.sort();

            if this_elv > first_three[2] {
                first_three[0] = first_three[1];
                first_three[1] = first_three[2];
                first_three[2] = this_elv;
            } else if this_elv > first_three[1] {
                first_three[0] = first_three[1];
                first_three[1] = this_elv;
            } else if this_elv > first_three[0] {
                first_three[0] = this_elv;
            }
            this_elv = 0;
        } else {
            this_elv += line.parse::<i32>().unwrap();
        }

        println!("{} - {} - [{} |{}| {}]", this_elv, line, first_three[0], first_three[1], first_three[2]);
    }

    if this_elv > first_three[2] {
        first_three[0] = first_three[1];
        first_three[1] = first_three[2];
        first_three[2] = this_elv;
    } else if this_elv > first_three[1] {
        first_three[0] = first_three[1];
        first_three[1] = this_elv;
    } else if this_elv > first_three[0] {
        first_three[0] = this_elv;
    }
    Some(first_three.iter().sum::<i32>() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
