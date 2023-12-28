#[allow(dead_code)]
const INPUT: &'static str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

#[allow(dead_code)]
const EXPECTED: &'static str = "281";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn solution(s: &str) -> String {
    let nums = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let ans: u32 = s.lines().map(|line| {
        let mut matches: Vec<u32> = Vec::new();
        for (i, c) in line.chars().enumerate() {
            if let Some(x) = c.to_digit(10) {
                matches.push(x);
            } else {
                for (j, num) in nums.iter().enumerate() {
                    if line[i..].starts_with(num) {
                        matches.push((j + 1) as u32);
                        break;
                    }
                }
            }
        }
        matches.first().unwrap() * 10 + matches.last().unwrap()
    }).sum();
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
