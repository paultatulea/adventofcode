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
const EXPECTED: &'static str = "405";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn is_palindrome(i: usize, grid: &Vec<Vec<u8>>, rowwise: bool) -> bool {
    let mut first = i;
    let mut second = i+1;
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

fn solution(s: &str) -> String {
    let grids: Vec<Vec<Vec<u8>>> = s.split("\n\n").map(|m| m.lines().map(|line| line.to_string().into_bytes()).collect()).collect();

    let ans: usize = grids.iter().map(|grid| {
        // Horizontal scan
        for i in 0..grid.len() - 1 {
            if grid[i] == grid[i+1] {
                if is_palindrome(i, &grid, true) {
                    println!("Found horizontal match at {i}");
                    return 100 * (i + 1);

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
                    println!("Found vertical match at {i}");
                    return i + 1;
                }
            }
        }
        unreachable!()
    }).sum();

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
