use std::collections::HashMap;

#[allow(dead_code)]
const INPUT: &'static str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

#[allow(dead_code)]
const EXPECTED: &'static str = "6";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn solution(s: &str) -> String {
    let mut lines = s.lines();
    let mut instructions = lines.next().unwrap().chars().cycle();
    let _ = lines.next();

    let graph = lines.fold(HashMap::new(), |mut map, line| {
        let node: String = line.chars().take(3).collect();
        let left: String = line.chars().skip(7).take(3).collect();
        let right: String = line.chars().skip(12).take(3).collect();
        let edges = vec![left, right];
        map.insert(node, edges);
        map
    });

    let mut node = "AAA".to_string();
    let goal = "ZZZ".to_string();
    for i in 0.. {
        if node == goal {
            return i.to_string();
        }
        let instruction = instructions.next().unwrap();
        let idx: usize = if instruction == 'L' { 0 } else { 1 };
        node = graph.get(&node).unwrap()[idx].clone();
    }

    unreachable!()
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert!(solution(INPUT) == EXPECTED.to_string());
    }
}
