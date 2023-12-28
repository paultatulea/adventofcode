use std::collections::HashMap;

#[allow(dead_code)]
const INPUT: &'static str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

#[allow(dead_code)]
const EXPECTED: &'static str = "525152";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn f(springs: &[char], groups: &[usize], running_count: usize, memo: &mut HashMap<(Vec<char>, Vec<usize>, usize), usize>) -> usize {
    // Base cases
    if springs.len() == 0 {
        if groups.len() == 0 && running_count == 0 {
            return 1;
        } else if groups.len() == 1 && running_count == groups[0] {
            return 1;
        } else {
            return 0;
        }
    }

    if let Some(&cached) = memo.get(&(springs.to_vec(), groups.to_vec(), running_count)) {
        return cached;
    }

    let mut successes = 0usize;

    if let Some(spring) = springs.iter().next() {
        match spring {
            '?' => {
                // Try setting to '#'
                successes += f(&springs[1..], &groups, running_count + 1, memo);
                // Try setting to '.'
                if running_count > 0 {
                    if let Some(next_group) = groups.iter().next() {
                        if *next_group == running_count {
                            successes += f(&springs[1..], &groups[1..], 0, memo);
                        }
                    }
                } else {
                    successes += f(&springs[1..], &groups, 0, memo);
                }
            },
            '#' => {
                successes += f(&springs[1..], &groups, running_count + 1, memo);
            },
            '.' => {
                if running_count > 0 {
                    if let Some(next_group) = groups.iter().next() {
                        if *next_group == running_count {
                            successes += f(&springs[1..], &groups[1..], 0, memo);
                        }
                    }
                } else {
                    successes += f(&springs[1..], &groups, 0, memo);
                }
            },
            _ => { },
        }
    }
    memo.insert((springs.to_vec(), groups.to_vec(), running_count), successes.clone());
    successes
}

fn solution(s: &str) -> String {
    let ans = s.lines().fold(0, |acc, line| {
        let mut iter = line.split_whitespace();
        let parse_springs: Vec<_> = iter.next().unwrap().chars().collect();
        let parse_groups: Vec<_> = iter.next().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        let mut springs: Vec<char> = parse_springs.clone();
        let mut groups: Vec<usize> = parse_groups.clone();
        for _ in 0..4 {
            springs.push('?');
            springs.extend(parse_springs.clone());
            groups.extend(parse_groups.clone());

        }
        let mut memo = HashMap::new();
        let combinations = f(&springs, &groups, 0, &mut memo);
        println!("{}, {:?} -> {}", springs.iter().collect::<String>(), groups, combinations);
        acc + combinations
    });

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
