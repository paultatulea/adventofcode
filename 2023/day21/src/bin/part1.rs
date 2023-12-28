use std::collections::BinaryHeap;
use std::collections::HashSet;

#[allow(dead_code)]
const INPUT: &'static str = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

#[allow(dead_code)]
const EXPECTED: &'static str = "16";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn solution(s: &str) -> String {
    let (mut startrow, mut startcol) = (0, 0);
    let grid: Vec<Vec<_>> = s
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .inspect(|(col, c)| {
                    if *c == 'S' {
                        startrow = row as i32;
                        startcol = *col as i32;
                    }
                })
                .map(|(_, c)| c)
                .collect::<Vec<char>>()
        })
        .collect();

    let nrows = grid.len();
    let ncols = grid[0].len();
    let goal = 64;
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut seen = vec![vec![0; ncols]; nrows]; 
    heap.push((0, startrow, startcol));

    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    while let Some((steps, row, col)) = heap.pop() {
        println!("{steps} ({row}, {col})");
        if steps < -goal {
            break;
        }
        if visited.contains(&(steps, row, col)) {
            continue;
        }
        visited.insert((steps, row, col));
        if steps == -goal {
            seen[row as usize][col as usize] = 1;
        }

        for (dr, dc) in directions {
            let nr = row + dr;
            let nc = col + dc;
            if nr < 0 || nc < 0 || nr as usize >= nrows || col as usize >= ncols {
                continue;
            }
            if grid[nr as usize][nc as usize] == '#' {
                continue;
            }
            heap.push((steps - 1, nr, nc));
        }
    }

    let ans: usize = seen.iter().fold(0, |acc, row| {
        acc + row.iter().sum::<usize>()
    });
    println!("{ans}");
    return ans.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert!(solution(INPUT) == EXPECTED.to_string());
    }
}
