use std::collections::HashSet;

type Point = (i32, i32);

pub fn part_one(input: &str) -> Option<u32> {
    let mut tail: Vec<Point> = vec![(500,500); 2];
    let mut visited_point: Vec<Point> = vec![(500,500)];

    input
        .lines()
        .for_each(
            |line| {
                let _line = line.split(" ").collect::<Vec<&str>>();
                let direction: String = _line[0].parse().unwrap();
                let steps: u32 = _line[1].parse().unwrap();

                for _ in 0 .. steps {
                    moves(direction.trim(), &mut tail[0]);

                    for i in 1 .. tail.len() {
                        follow(tail[i - 1], &mut tail[i]);
                    }

                    if !visited_point.contains(&tail.last().unwrap()) {
                        visited_point.push(*tail.last().unwrap());
                    }
                }
            }
    );

    Some(visited_point.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut tail: Vec<Point> = vec![(500,500); 10];
    let mut visited_point = HashSet::from([(500, 500)]);

    input
        .lines()
        .for_each(
            |line| {
                let _line = line.split(" ").collect::<Vec<&str>>();
                let direction: String = _line[0].parse().unwrap();
                let steps: u32 = _line[1].parse().unwrap();

                for _ in 0 .. steps {
                    moves(direction.trim(), &mut tail[0]);

                    for i in 1 .. tail.len() {
                        follow(tail[i - 1], &mut tail[i]);
                    }

                    visited_point.insert(*tail.last().unwrap());
                }
            }
        );

    Some(visited_point.len() as u32)
}

fn moves(direction: &str, head: &mut Point) {
    match direction {
        "R" => head.0 += 1,
        "L" => head.0 -= 1,
        "D" => head.1 += 1,
        "U" => head.1 -= 1,
        _ => panic!("{direction}"),
    }
}

fn follow(leader: Point, follower: &mut Point) {
    let delta = (leader.0 - follower.0, leader.1 - follower.1);
    if delta.0.abs() == 2 || delta.1.abs() == 2 {
        follower.0 += delta.0.signum();
        follower.1 += delta.1.signum();
    }
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
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
