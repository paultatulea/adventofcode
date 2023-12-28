#[allow(dead_code)]
const INPUT: &'static str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

#[allow(dead_code)]
const EXPECTED: &'static str = "467835";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn solution(s: &str) -> String {
    let mut rows: Vec<Vec<usize>> = Vec::new();
    let mut nums: Vec<u32> = Vec::new();

    for line in s.lines() { 
        let mut curr: String = String::new();
        let mut start: usize = usize::MAX;
        let mut row_vec: Vec<usize> = Vec::new();
        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                curr.push(c);
                if start == usize::MAX {
                    start = i;
                }
                row_vec.push(nums.len() + 1);
            } else {
                if c == '*' {
                    row_vec.push(usize::MAX);
                } else {
                    row_vec.push(0);
                }
                // Push any consumed number into vec
                if !curr.is_empty() {
                    nums.push(curr.parse::<u32>().unwrap());
                }
                // Reset number tracker
                curr = String::new();
                start = usize::MAX;
            }
        }
        // Final check in case number is on the boundary
        if !curr.is_empty() {
            nums.push(curr.parse::<u32>().unwrap());
        }
        
        rows.push(row_vec);
    }

    let mut ans = 0;
    let ROW_LEN = rows[0].len();
    let COL_LEN = rows.len();
    for (nrow, row) in rows.iter().enumerate() {
        for (ncol, col) in row.iter().enumerate() {
            if *col != usize::MAX {
                continue;
            }
            let mut seen: Vec<usize> = Vec::new();
            let up = if nrow == 0 { 0 } else { nrow - 1 };
            let left = if ncol == 0 { 0 } else { ncol - 1 };
            for i in up..=(nrow + 1) {
                for j in left..=(ncol + 1) {
                    if i < ROW_LEN && j < COL_LEN {
                        let x = rows[i][j];
                        if x != 0 && x != usize::MAX && !seen.contains(&x) {
                            seen.push(x);
                        }
                    }
                }
            }
            if seen.len() == 2 {
                let gearing = seen.iter().fold(1, |acc, x| acc * nums[x - 1]); 
                ans += gearing;
            }
        }
    }
    format!("{}", ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert!(solution(INPUT) == EXPECTED.to_string());
    }
}
