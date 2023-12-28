#[allow(dead_code)]
const INPUT: &'static str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

#[allow(dead_code)]
const EXPECTED: &'static str = "400";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn is_palindrome(i: usize, grid: &Vec<Vec<u8>>, rowwise: bool) -> bool {
    let mut first = i;
    let mut second = i + 1;
    if rowwise {
        let nrows = grid.len() - 1;
        loop {
            if grid[first] != grid[second] {
                return false;
            }
            if first == 0 || second == nrows {
                break;
            }
            first -= 1;
            second += 1;
        }
    } else {
        let ncols = grid[0].len();
        loop {
            for row in grid {
                if row[first] != row[second] {
                    return false;
                }
            }
            if first == 0 || second == ncols - 1 {
                break;
            }
            first -= 1;
            second += 1;
        }
    }
    return true;
}

fn scan_grid(grid: &Vec<Vec<u8>>) -> Vec<(usize, bool)> {
    // Horizontal scan
    let mut rv = Vec::new();
    for i in 0..grid.len() - 1 {
        if grid[i] == grid[i + 1] {
            if is_palindrome(i, &grid, true) {
                rv.push((i, true));
            }
        }
    }
    // Vertical scan
    for i in 0..grid[0].len() - 1 {
        let mut matches = true;
        for row in 0..grid.len() {
            if grid[row][i] != grid[row][i + 1] {
                matches = false;
                break;
            }
        }
        if matches {
            if is_palindrome(i, &grid, false) {
                rv.push((i, false));
            }
        }
    }
    return rv;
}

fn solution(s: &str) -> String {
    let grids: Vec<Vec<Vec<u8>>> = s
        .split("\n\n")
        .map(|m| {
            m.lines()
                .map(|line| line.to_string().into_bytes())
                .collect()
        })
        .collect();

    let ans: usize = grids
        .iter()
        .map(|grid| {
            let first_line = scan_grid(&grid)[0];

            let nrows = grid.len();
            let ncols = grid[0].len();

            for row1 in 1..nrows {
                for row2 in 0..row1 {
                    let mut diffs = Vec::new();
                    for col in 0..ncols {
                        if grid[row1][col] != grid[row2][col] {
                            diffs.push(col);
                        }
                    }
                    if diffs.len() == 1 {
                        let mut grid2 = grid.clone();
                        grid2[row1][diffs[0]] = grid[row2][diffs[0]];
                        if let Some(new_line) = scan_grid(&grid2).iter().filter(|line| **line != first_line).next() {
                            let factor = if new_line.1 { 100 } else { 1 };
                            return factor * (new_line.0 + 1);
                        }
                    }
                }
            }
            for col1 in 1..ncols {
                for col2 in 0..col1 {
                    let mut diffs = Vec::new();
                    for row in 0..nrows {
                        if grid[row][col1] != grid[row][col2] {
                            diffs.push(row);
                        }
                    }
                    if diffs.len() == 1 {
                        let mut grid2 = grid.clone();
                        grid2[diffs[0]][col1] = grid[diffs[0]][col2];
                        if let Some(new_line) = scan_grid(&grid2).iter().filter(|line| **line != first_line).next() {
                            let factor = if new_line.1 { 100 } else { 1 };
                            return factor * (new_line.0 + 1);
                        }
                    }
                }
            }
            unreachable!();
        })
        .sum();

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
