pub fn part_one(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .fold(0,|acc: i32, fight| acc + combat(fight));
    Some(score as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .fold(0,|acc: i32, fight| acc + manipulated_combat(fight));
    Some(score as u32)
}

pub fn combat(combat: &str) -> i32 {
    let splitted = combat.split_whitespace().collect::<Vec<&str>>();
    let mut score = 1;
    if splitted[1] == "Y" {
        score += 1;
    } else if splitted[1] == "Z" {
        score += 2;
    }

    let _combat = splitted.join("");
    if _combat == "AX" || _combat == "BY" || _combat == "CZ" {
        score += 3;
    } else if _combat == "CX" || _combat == "AY" || _combat == "BZ" {
        score += 6;
    }

    score
}

pub fn manipulated_combat(combat: &str) -> i32 {
    match combat {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        &_ => 0
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
