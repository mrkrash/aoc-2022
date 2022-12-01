pub fn part_one(input: &str) -> Option<u32> {
    let mut strong_elv = 0;
    let mut this_elv = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            if this_elv > strong_elv {
                strong_elv = this_elv;
            }

            this_elv = 0;
            continue
        } else {
            this_elv += line.parse::<i32>().unwrap();
        }

    }
    Some(strong_elv as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
