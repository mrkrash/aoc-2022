pub fn part_one(input: &str) -> Option<u32> {
    for i in 0..input.len() -2 {
        let crane = input.get(i..(i+4)).unwrap().to_string();
        if !&crane[0..3].contains(&crane[3..4]) &&
            !&crane[0..2].contains(&crane[2..3]) &&
            crane[0..1] != crane[1..2] {
            println!("{:?}", crane);
            return Some((i + 4) as u32);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    for i in 0..input.len() -12 {
        let crane = input.get(i..(i+14)).unwrap().to_string();
        if !&crane[0..13].contains(&crane[13..14]) &&
            !&crane[0..12].contains(&crane[12..13]) &&
            !&crane[0..11].contains(&crane[11..12]) &&
            !&crane[0..10].contains(&crane[10..11]) &&
            !&crane[0..9].contains(&crane[9..10]) &&
            !&crane[0..8].contains(&crane[8..9]) &&
            !&crane[0..7].contains(&crane[7..8]) &&
            !&crane[0..6].contains(&crane[6..7]) &&
            !&crane[0..5].contains(&crane[5..6]) &&
            !&crane[0..4].contains(&crane[4..5]) &&
            !&crane[0..3].contains(&crane[3..4]) &&
            !&crane[0..2].contains(&crane[2..3]) &&
            crane[0..1] != crane[1..2] {
            println!("{:?}", crane);
            return Some((i + 14) as u32);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(10));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(29));
    }
}
