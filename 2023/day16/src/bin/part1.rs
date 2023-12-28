use std::collections::VecDeque;
use std::collections::HashSet;

#[allow(dead_code)]
const INPUT: &'static str = "\
.|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

#[allow(dead_code)]
const EXPECTED: &'static str = "46";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn solution(s: &str) -> String {
    let grid: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
    let nrows = grid.len() as i32;
    let ncols = grid[0].len() as i32;
    let mut energized = vec![vec![false; ncols as usize]; nrows as usize];
    let mut q: VecDeque<(i32, i32, i32, i32)> = VecDeque::new();
    let mut seen: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    q.push_back((0, 0, 0, 1));

    while !q.is_empty() {
        let node = q.pop_front().unwrap();
        let (x, y, dx, dy) = node;
        if x < 0 || y < 0 || x >= nrows || y >= ncols {
            continue;
        }
        if seen.contains(&node) {
            continue;
        }
        seen.insert(node.clone());
        energized[x as usize][y as usize] = true;
        let curr = grid[x as usize][y as usize];
        match curr {
            '.' => q.push_back((x + dx, y + dy, dx, dy)),
            '/' => {
                if dy == 1 {
                    q.push_back((x - 1, y, -1, 0));
                } else if dy == -1 {
                    q.push_back((x + 1, y, 1, 0));
                } else if dx == -1 {
                    q.push_back((x, y + 1, 0, 1));
                } else {
                    q.push_back((x, y - 1, 0, -1));
                }
            },
            '\\' => {
                if dy == 1 {
                    q.push_back((x + 1, y, 1, 0));
                } else if dy == -1 {
                    q.push_back((x - 1, y, -1, 0));
                } else if dx == -1 {
                    q.push_back((x, y - 1, 0, -1));
                } else {
                    q.push_back((x, y + 1, 0, 1));
                }
            },
            '-' => {
                if dx == 0 {
                    q.push_back((x + dx, y + dy, dx, dy));
                } else {
                    q.push_back((x, y - 1, 0, -1));
                    q.push_back((x, y + 1, 0, 1));
                }
            }
            '|' => {
                if dy == 0 {
                    q.push_back((x + dx, y + dy, dx, dy));
                } else {
                    q.push_back((x - 1 , y, -1, 0));
                    q.push_back((x + 1, y, 1, 0));
                }
            }
            _ => panic!(),
        };
    }

    let ans = energized.iter().fold(0, |acc, row| {
        acc + row.iter().filter(|&&value| value).count()
    });
    println!("{ans}");
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert!(solution(INPUT) == EXPECTED.to_string());
    }
}
