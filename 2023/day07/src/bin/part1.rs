use std::{collections::HashMap, cmp::Ordering};

#[allow(dead_code)]
const INPUT: &'static str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

#[allow(dead_code)]
const EXPECTED: &'static str = "6440";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn solution(s: &str) -> String {
    let order = "123456789TJQKA";
    let mut hands: Vec<(&str, usize, i32)> = s.lines().map(|line| {
        let parts: Vec<_> = line.split(" ").collect();
        let cards = parts[0];
        let bid = parts[1].parse::<usize>().unwrap();
        let map = cards.chars().fold(HashMap::new(), |mut acc, c| {
            acc.entry(c).and_modify(|e| *e += 1 ).or_insert(1);
            acc
        });
        let mut freqs: Vec<_> = map.values().collect();
        freqs.sort();
        let score = match freqs[..] {
            [5] => 6,
            [1, 4] => 5,
            [2, 3] => 4,
            [1, 1, 3] => 3,
            [1, 2, 2] => 2,
            [1, 1, 1, 2] => 1,
            _ => 0,
        };
        (cards, bid, score)
    }).collect();

    let sort_fn = |a: &(&str, usize, i32), b: &(&str, usize, i32)| {
        if a.2 != b.2 {
            return a.2.cmp(&b.2);
        }
        for (ac, bc) in a.0.chars().zip(b.0.chars()) {
            if ac != bc {
                return order.chars().position(|x| x == ac).unwrap().cmp(&order.chars().position(|x| x == bc).unwrap());
            }
        }
        Ordering::Equal
    };

    hands.sort_by(|a, b| sort_fn(b, a));
    let total_hands = hands.len();
    let ans: usize = hands.iter().enumerate().map(|(i, hand)| {
        (total_hands - i) * hand.1
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
