pub fn part_one(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .fold(0, |acc: i32, line| acc + priorities(line));

    Some(score as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score = 0;
    let mut lines = input.lines();
    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        score += group_badge(line1, line2, line3);
    }
    Some(score as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

pub fn group_badge(line1: &str, line2: &str, line3: &str) -> i32 {
    let l:Vec<char> = line1.chars().collect();
    for i in 0 .. l.len() {
        if line2.contains(&line1[i..i+1]) && line3.contains(&line1[i..i+1]) {
            return calculate_priorities(&line1[i..i+1]);
        }
    }
    0
}

pub fn priorities(rucksacks: &str) -> i32 {
    let (left, right) = rucksacks.split_at(rucksacks.len() /2);
    let l:Vec<char> = left.chars().collect();
    let r:Vec<char> = right.chars().collect();
    for item in l {
        if r.contains(&item) {
            let s: String = item.into();
            return calculate_priorities(&s);
        }
    }
    0
}

pub fn calculate_priorities(item: &str) -> i32 {
    match item {
        "a" => 1,
        "b" => 2,
        "c" => 3,
        "d" => 4,
        "e" => 5,
        "f" => 6,
        "g" => 7,
        "h" => 8,
        "i" => 9,
        "j" => 10,
        "k" => 11,
        "l" => 12,
        "m" => 13,
        "n" => 14,
        "o" => 15,
        "p" => 16,
        "q" => 17,
        "r" => 18,
        "s" => 19,
        "t" => 20,
        "u" => 21,
        "v" => 22,
        "w" => 23,
        "x" => 24,
        "y" => 25,
        "z" => 26,
        "A" => 27,
        "B" => 28,
        "C" => 29,
        "D" => 30,
        "E" => 31,
        "F" => 32,
        "G" => 33,
        "H" => 34,
        "I" => 35,
        "J" => 36,
        "K" => 37,
        "L" => 38,
        "M" => 39,
        "N" => 40,
        "O" => 41,
        "P" => 42,
        "Q" => 43,
        "R" => 44,
        "S" => 45,
        "T" => 46,
        "U" => 47,
        "V" => 48,
        "W" => 49,
        "X" => 50,
        "Y" => 51,
        "Z" => 52,
        _ => 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
