extern crate core;

use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<String> {
    let puzzle = input.split_terminator("\n\n").collect::<Vec<&str>>();
    let mut stack = get_stack(puzzle[0]);

    puzzle[1].lines().for_each(|line|{
        let param:Vec<&str> = line.split_whitespace().collect();
        rearrange9000(&mut stack, param[1], param[3], param[5]);
    });

    let message = get_message(&mut stack);

    Some(message)
}

pub fn part_two(input: &str) -> Option<String> {
    let puzzle = input.split_terminator("\n\n").collect::<Vec<&str>>();
    let mut stack = get_stack(puzzle[0]);

    puzzle[1].lines().for_each(|line|{
        let param:Vec<&str> = line.split_whitespace().collect();
        rearrange9001(&mut stack, param[1], param[3], param[5]);
    });

    let message = get_message(&mut stack);

    Some(message)
}

pub fn rearrange9000(stack: &mut [VecDeque<&str>], qty: &str, from: &str, to: &str) {
    for _i in 0.. qty.parse::<u8>().unwrap() {
        let rack = stack[from.parse::<usize>().unwrap() -1].pop_back();
        stack[to.parse::<usize>().unwrap() -1].push_back(rack.unwrap());
    }
}
pub fn rearrange9001(stack: &mut [VecDeque<&str>], qty: &str, from: &str, to: &str) {
    let mut temp: VecDeque<&str> = VecDeque::new();
    for _i in 0.. qty.parse::<u8>().unwrap() {
        let rack = stack[from.parse::<usize>().unwrap() -1].pop_back();
        temp.push_front(rack.unwrap());
    }
    for _i in 0.. qty.parse::<u8>().unwrap() {
        let rack = temp.pop_front();
        stack[to.parse::<usize>().unwrap() -1].push_back(rack.unwrap());
    }
}

pub fn get_message<'a>(stack: &'a mut Vec<VecDeque<&'a str>>) -> String {
    let mut message = String::new();
    for item in stack {
        let aa = item.pop_back();
        if aa.is_some() {
            message.push_str(aa.unwrap());
        }
    }
    message
}

pub fn get_stack(puzzle: &str) -> Vec<VecDeque<&str>> {
    puzzle.lines().fold(
        vec![VecDeque::new(); 9],
        |mut rack, line| {
            for i in 0..9 {
                let crane = line.get((i*4+1)..(i*4+2));
                if let Some(crane) = crane {
                    if !crane.trim().is_empty() && crane.trim().parse::<u8>().is_err() {
                        rack[i].push_front(crane);
                    }
                }
            }
            rack
        }
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
