#[allow(dead_code)]
const INPUT: &'static str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

#[allow(dead_code)]
const EXPECTED: &'static str = "114";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn solution(s: &str) -> String {
    let ans: i32 = s.lines().map(|line| {
        let mut nums: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let mut stack: Vec<i32> = vec![*nums.last().unwrap()];
        while nums.iter().any(|x| *x != 0) {
            // Inefficient use of vectors, probably better to do recursively
            let mut latest = Vec::with_capacity(nums.len() - 1);
            for i in 1..nums.len() {
                latest.push(nums[i] - nums[i - 1]);
            }
            stack.push(*latest.last().unwrap());
            nums = latest;
        }

        let mut extrap = 0;
        while !stack.is_empty() {
            extrap += stack.pop().unwrap();
        }
        extrap
    }).sum();
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
