#[allow(dead_code)]
const INPUT: &'static str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

#[allow(dead_code)]
const EXPECTED: &'static str = "142";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn solution(s: &str) -> String {
    let ans: u32 = s.lines().map(|line| {
        let mut chars = line.chars().filter(|c| c.is_numeric());
        let first = chars.next().map(|c| c.to_digit(10).unwrap()).unwrap();
        let last = if let Some(x) = chars.last() {
            x.to_digit(10).unwrap()
        } else {
            first
        };
        let sum = first * 10 + last;
        sum
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
