#[allow(dead_code)]
const INPUT: &'static str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

#[allow(dead_code)]
const EXPECTED: &'static str = "136";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn solution(s: &str) -> String {
    let mut grid: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
    let nrows = grid.len();
    let ncols = grid[0].len();

    loop {
        let mut movements: usize = 0;
        for i in 0..nrows {
            for j in 0..ncols {
                if grid[i][j] != 'O' {
                    continue;
                }
                if i == 0 {
                    continue;
                }
                if grid[i-1][j] == '.' {
                    grid[i-1][j] = 'O';
                    grid[i][j] = '.';
                    movements += 1;
                }
            }
        }
        if movements == 0 {
            break;
        }
    }
    
    let mut ans = 0;
    for i in 0..nrows {
        for j in 0..ncols {
            if grid[i][j] == 'O' {
                ans += nrows - i;
            }
        }
    }
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
