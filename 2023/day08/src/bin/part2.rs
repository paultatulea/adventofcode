use std::collections::HashMap;
use std::cmp::max;

#[allow(dead_code)]
const INPUT: &'static str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

#[allow(dead_code)]
const EXPECTED: &'static str = "6";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn lcm(nums: Vec<usize>) -> usize {
    // Calculate the prime factors of each number
    // Determine the LCM from the prime factors
    // Could also be implemented with greated common divisor, likely simpler too.
    let mut factors: Vec<HashMap<usize, usize>> = vec![HashMap::new(); nums.len()];
    for (i, num) in nums.iter().enumerate() {
        let mut n = num.clone();
        let mut divisor: usize = 2;
        while n > 1 {
            while n % divisor == 0 {
                factors[i].entry(divisor).and_modify(|e| *e += 1).or_insert(1);
                n = n / divisor;
            }
            divisor += 1;
        }
    }

    let mut highest_power = HashMap::<usize, usize>::new();
    for factor_map in &factors {
        for (factor, power) in factor_map.iter() {
            highest_power.entry(*factor).and_modify(|e| *e = max(*e, *power)).or_insert(*power);
        }
    }

    let lcm = highest_power.iter().fold(1, |acc, (factor, power)| acc *factor.pow(*power as u32));
    lcm
}

fn solution(s: &str) -> String {
    let mut lines = s.lines();
    let mut instructions = lines.next().unwrap().chars().cycle();
    let mut nodes: Vec<String> = Vec::new();
    let _ = lines.next();

    let graph = lines.fold(HashMap::new(), |mut map, line| {
        let node: String = line.chars().take(3).collect();
        let left: String = line.chars().skip(7).take(3).collect();
        let right: String = line.chars().skip(12).take(3).collect();
        let edges = vec![left, right];
        if node.ends_with('A') {
            nodes.push(node.clone())
        }
        map.insert(node, edges);
        map
    });


    // Assumptions made after investigating the input.
    // The cycle from A -> Z is the same length as from Z -> Z
    // for each starting node A. Also, the cycle always starts
    // on the same instruction, otherwise it is not a true cycle.
    // Each path only has one Z node.
    let mut found = vec![false; nodes.len()];
    let mut cycle_lengths = vec![0; nodes.len()];
    for i in 0.. {
        if found.iter().all(|&x| x) {
            break;
        }
        let instruction = instructions.next().unwrap();
        let idx: usize = if instruction == 'L' { 0 } else { 1 };
        for n in 0..nodes.len() {
            let next_node = graph.get(&nodes[n]).unwrap()[idx].clone();
            if next_node.ends_with('Z') && !found[n] {
                cycle_lengths[n] = i + 1;
                found[n] = true;
            }
            nodes[n] = next_node;
        }
    }
    let ans = lcm(cycle_lengths);
    format!("{}", ans)
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert!(solution(INPUT) == EXPECTED.to_string());
    }

    #[test]
    fn test_lcm() {
        assert!(lcm(vec![12, 15, 20]) == 60);
    }
}
