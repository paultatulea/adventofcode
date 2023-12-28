use std::collections::HashSet;

#[allow(dead_code)]
const INPUT: &'static str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

#[allow(dead_code)]
const EXPECTED: &'static str = "30";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn solution(s: &str) -> String {
    let mut q: Vec<usize> = vec![1];
    for (i, line) in s.lines().enumerate() { 
        if i >= q.len() {
            q.push(1);
        }
        let mut cards = line.split_once(": ").unwrap().1.split(" | ");
        let winning: HashSet<u32> = HashSet::from_iter(cards.next().unwrap().split(" ").filter(|x| !x.is_empty()).map(|x| x.trim().parse::<u32>().unwrap()));
        let have = HashSet::from_iter(cards.next().unwrap().split(" ").filter(|x| !x.is_empty()).map(|x| x.trim().parse::<u32>().unwrap()));
        let intersect = winning.intersection(&have).count();

        for j in 1..=intersect {
            let idx = i + j;
            if idx >= q.len() {
                q.push(q[i] + 1);
            } else {
                q[idx] += q[i];
            }
        }
    }
    let ans: usize = q.iter().sum();
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
