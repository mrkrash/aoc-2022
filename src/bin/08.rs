struct Tree {
    tall: u8,
    visible: bool,
    score: u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let forest= populate_forest(input);
    let visible_tree = forest.iter().fold(0, |acc, row| {
        acc + row.iter().fold(0, |mut acc2, tree| {
            if tree.visible {
                acc2 += 1;
            }
            acc2
        })
    });
    Some(visible_tree as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let forest= populate_forest(input);
    let score = forest.iter().fold(0, |mut acc, row| {
        let acc2 = row.iter().fold(0, |mut acc2, tree| {
            if acc2 < tree.score {
                acc2 = tree.score;
            }
            acc2
        });
        if acc < acc2 {
            acc = acc2;
        }
        acc
    });
    Some(score as u32)
}

fn populate_forest(input: &str) -> Vec<Vec<Tree>> {
    let mut forest: Vec<Vec<Tree>> = input
        .lines()
        .fold(
            vec![vec![]],
            |mut acc, line| {
                // println!("acc {}", acc.len());
                // println!("{}", line);
                let l = line
                    .split("")
                    .filter(|f| !f.is_empty())
                    .fold(
                        vec![],
                        |mut row, r| {
                            // println!("{}", r);
                            row.push(Tree {
                                tall: r.parse().unwrap(),
                                visible: true,
                                score: 1
                            });
                            // println!("{}", row.len());
                            row
                        }
                    );
                acc.push(l);
                acc
            });

    forest.reverse();
    forest.pop();
    forest.reverse();
    for i in 1..forest.len() -1 {
        for j in 1..forest[0].len() -1 {
            let mut horizontal_left_invisible = false;
            let mut horizontal_right_invisible = false;
            let mut vertical_top_invisible = false;
            let mut vertical_down_invisible = false;
            let mut hl_score = 0;
            let mut hr_score = 0;
            let mut vt_score = 0;
            let mut vd_score = 0;
            for r in (0..i).rev() {
                hl_score += 1;
                if forest[j][r].tall >= forest[j][i].tall {
                    horizontal_left_invisible = true;
                    break;
                }
            }
            for r in i+1..forest[0].len() {
                hr_score += 1;
                if forest[j][r].tall >= forest[j][i].tall {
                    horizontal_right_invisible = true;
                    break;
                }
            }
            for r in (0..j).rev() {
                vt_score += 1;
                if forest[r][i].tall >= forest[j][i].tall {
                    vertical_top_invisible = true;
                    break;
                }
            }
            for r in j+1..forest.len() {
                vd_score += 1;
                if forest[r][i].tall >= forest[j][i].tall {
                    vertical_down_invisible = true;
                    break;
                }
            }
            if horizontal_left_invisible &&
                horizontal_right_invisible &&
                vertical_top_invisible &&
                vertical_down_invisible {
                forest[i][j].visible = false;
            }
            forest[i][j].score = hl_score * hr_score * vt_score * vd_score;
        }
    }
    forest
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
