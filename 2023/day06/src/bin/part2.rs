#[allow(dead_code)]
const INPUT: &'static str = "\
Time:      7  15   30
Distance:  9  40  200";

#[allow(dead_code)]
const EXPECTED: &'static str = "71503";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn solution(s: &str) -> String {
    let mut lines = s.lines();
    let time: u64 = lines.next().unwrap().chars().filter(|c| c.is_numeric()).map(|c| c.to_digit(10).unwrap() as u64).fold(0, |acc, x| { acc * 10 + x });
    let distance: u64 = lines.next().unwrap().chars().filter(|c| c.is_numeric()).map(|c| c.to_digit(10).unwrap() as u64).fold(0, |acc, x| { acc * 10 + x });

    let mut ans = 0;
    for i in 1..time {
        let travel_time = time - i;
        let distance_travelled = i * travel_time;
        if distance_travelled > distance {
            ans += 1;
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
