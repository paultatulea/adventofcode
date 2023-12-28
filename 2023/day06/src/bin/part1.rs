#[allow(dead_code)]
const INPUT: &'static str = "\
Time:      7  15   30
Distance:  9  40  200";

#[allow(dead_code)]
const EXPECTED: &'static str = "288";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn solution(s: &str) -> String {
    let mut lines = s.lines();
    let times: Vec<_> = lines.next().unwrap().split_once(":").unwrap().1.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
    let distances: Vec<_> = lines.next().unwrap().split_once(":").unwrap().1.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();

    let ans = times.iter().zip(distances.iter()).map(|(&t, &d)| {
        let mut successes = 0;
        for i in 1..t {
            let travel_time = t - i;
            let distance_travelled = i * travel_time;
            if distance_travelled > d {
                successes += 1;
            }
        }
        successes
    }).fold(1, |acc, x| acc * x);
    
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
