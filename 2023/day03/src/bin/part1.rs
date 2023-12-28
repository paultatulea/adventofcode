use std::collections::HashSet;

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
const EXPECTED: &'static str = "4361";

#[derive(Debug)]
struct Number {
    pub value: u32,
    pub start: (usize, usize),
    pub span: usize,
}

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn solution(s: &str) -> String {
    let mut symbols: HashSet<(i32, i32)> = HashSet::new();
    let mut nums: Vec<Number> = Vec::new();

    s.lines().enumerate().for_each(|(lineno, line)| {
        let mut curr: String = String::new();
        let mut start: usize = usize::MAX;
        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                curr.push(c);
                if start == usize::MAX {
                    start = i;
                }
            } else {
                if c!= '.' {
                    symbols.insert((lineno as i32, i as i32));
                }
                // Push any consumed number into vec
                if !curr.is_empty() {
                    nums.push(Number { value: curr.parse::<u32>().unwrap(), start: (lineno, start), span: curr.len() }); 
                }
                // Reset number tracker
                curr = String::new();
                start = usize::MAX;
            }
        }
        // Final check in case number is on the boundary
        if !curr.is_empty() {
            nums.push(Number { value: curr.parse::<u32>().unwrap(), start: (lineno, start), span: curr.len() }); 
        }
    });

    let mut ans = 0;
    for num in nums.iter() {
        let mut yes = false;
        let up = num.start.0 as i32 - 1;
        let down = num.start.0 as i32 + 1;
        let left = num.start.1 as i32 - 1;
        let right = (num.start.1 + num.span) as i32;
        println!("{}, start={}, span={}, up={}, down={}, left={}, right={}", num.value, num.start.1, num.span, up, down, left, right);
        for col in (left)..=(right) {
            if symbols.contains(&(up, col)) || symbols.contains(&(down, col)) {
                yes = true;
                break;
            }
        }
        if !yes {
            if symbols.contains(&(num.start.0 as i32, left)) || symbols.contains(&(num.start.0 as i32, right)) {
                yes = true;
            }
        }

        println!("{} {yes}", num.value);
        if yes {
            ans += num.value;
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
