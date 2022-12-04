
pub fn part_one(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .fold(0, |acc, line| acc + full_overlap(line));
    Some(score as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .fold(0, |acc, line| acc + pair_overlap(line));
    Some(score as u32)
}

pub fn full_overlap(pairs: &str) -> i32 {
    let (pair_one, pair_two) = get_pair(pairs);
    if (pair_one[0] >= pair_two[0] && pair_one[1] <= pair_two[1]) ||
        (pair_two[0] >= pair_one[0] && pair_two[1] <= pair_one[1]) {
        return 1 as i32;
    }
    0
}
pub fn pair_overlap(pairs: &str) -> i32 {
    let (pair_one, pair_two) = get_pair(pairs);
    if (pair_one[0] >= pair_two[0] && pair_one[0] <= pair_two[1]) ||
        (pair_one[1] >= pair_two[0] && pair_one[1] <= pair_two[1]) ||
        (pair_one[0] >= pair_two[0] && pair_one[1] <= pair_two[1]) ||
        (pair_two[0] >= pair_one[0] && pair_two[1] <= pair_one[1]) {
        return 1 as i32;
    }
    0
}

pub fn get_pair(pairs: &str) -> (Vec<i8>, Vec<i8>) {
    let pair = pairs.split_terminator(',').collect::<Vec<&str>>();
    let pair_one = pair[0].split_terminator('-')
        .map(|limit| limit.parse::<i8>().unwrap())
        .collect::<Vec<i8>>();
    let pair_two = pair[1].split_terminator('-')
        .map(|limit| limit.parse::<i8>().unwrap())
        .collect::<Vec<i8>>();

    return (pair_one, pair_two);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
