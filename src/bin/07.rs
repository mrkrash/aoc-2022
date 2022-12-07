pub struct Folder {
    parent: Option<usize>,
    files: u64,
}

pub fn part_one(input: &str) -> Option<u64> {
    let folders = scan_dir(input);
    let sum = folders
        .iter()
        .filter_map(|f| (f.files < 100000).then_some(f.files))
        .sum::<u64>();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let folders = scan_dir(input);
    let free_space = 70000000 - folders[0].files;
    let space_to_free = 30000000 - free_space;
    let mut candidate = folders
        .iter()
        .filter_map(|f| (f.files >= space_to_free).then_some(f.files))
        .collect::<Vec<u64>>();
    candidate.sort();
    Some(candidate[0])
}

pub fn scan_dir(input: &str) -> Vec<Folder> {
    let mut folders:Vec<Folder> = vec![];
    let mut current = None;
    input
        .lines()
        .for_each(
            |line| {
                let mut l = line.split(' ');
                match l.next().unwrap() {
                    "$" => if l.next().unwrap() == "cd" {
                        match l.next().unwrap() {
                            ".." => {
                                let idx: usize = current.unwrap();
                                current = folders[idx].parent;
                            },
                            _s => {
                                folders.push(Folder {
                                    parent: current,
                                    files: 0
                                });
                                current = Some(folders.len() -1);
                            },
                        }
                    },
                    "dir" => { l.next().unwrap(); },
                    n => {
                        let mut parent = current;
                        while let Some(id) = parent {
                            folders[id].files += n.parse::<u64>().unwrap();
                            parent = folders[id].parent;
                        }
                    },
                }
            });
    folders
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
