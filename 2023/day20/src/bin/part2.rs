use std::collections::HashMap;
use std::collections::VecDeque;
use std::cmp::max;

#[allow(dead_code)]
const INPUT: &'static str = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

#[allow(dead_code)]
const EXPECTED: &'static str = "11687500";

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


#[derive(Debug, Eq, PartialEq, Clone)]
enum Module {
    FlipFlop(bool, Vec<String>), // ison, targets
    Conjunction(HashMap<String, bool>, Vec<String>),  // targets
    Broadcast(Vec<String>),
}


fn solution(s: &str) -> String {
    let mut map = s.lines().fold(HashMap::new(), |mut acc, line| {
        let parts: Vec<_> = line.split(" -> ").collect();
        let dest: Vec<_> = parts[1].split(", ").map(|x| x.to_string()).collect();
        let first = parts[0];
        let name: String;
        let module: Module;
        if first.starts_with("%") {
            name = first.chars().skip(1).collect();
            module = Module::FlipFlop(false, dest);
        } else if first.starts_with("&") {
            name = first.chars().skip(1).collect();
            module = Module::Conjunction(HashMap::new(), dest);
        } else {
            name = first.to_string();
            module = Module::Broadcast(dest);
        }
        acc.insert(name, module);
        acc
    });


    let mut temp: HashMap<String, Vec<String>> = HashMap::new();
    let mut last_conjunction = String::new();
    for (input_modname, module) in &map {
        let targets: Vec<String>;
        match module {
            Module::FlipFlop(_, dest) | Module::Conjunction(_, dest) | Module::Broadcast(dest) => targets = dest.clone(),
        }
        for target in targets {
            if target == String::from("rx") {
                last_conjunction = input_modname.to_string();
            }
            if let Some(target_mod) = map.get(&target) {
                match target_mod {
                    Module::Conjunction(_, _) => {
                        temp.entry(target).and_modify(|e| e.push(input_modname.to_string())).or_insert(vec![input_modname.to_string()]);
                    },
                    _ => {},
                };
            }
        }
    }
    for (target, inputs) in temp {
        map.entry(target).and_modify(|e| {
            match e {
                Module::Conjunction(input_map, _) => {
                    for input in inputs {
                        input_map.insert(input, false);
                    }
                },
                _ => {},
            };
        });
    }

    // Find feeders to last conjunction
    let mut last_conjunction_feeders: HashMap<String, usize> = HashMap::new();
    for (input_modname, module) in &map {
        match module {
            Module::FlipFlop(_, dest) | Module::Conjunction(_, dest) | Module::Broadcast(dest) => {
                if dest.contains(&last_conjunction) {
                    last_conjunction_feeders.insert(input_modname.clone(), 0);
                }
            }
        }
    }

    let mut should_break = false;
    let mut q = VecDeque::new();
    for i in 1.. {
        if should_break {
            break;
        }
        q.push_back(("button".to_string(), "broadcaster".to_string(), false));

        while let Some((source, dest, pulse)) = q.pop_front() {
            last_conjunction_feeders.entry(source.clone()).and_modify(|e| {
                if dest == last_conjunction && pulse && *e == 0 {
                    *e = i;
                }
            });
            if last_conjunction_feeders.values().all(|&x| x > 0) {
                should_break = true;
                break;
            }

            map.entry(dest.clone()).and_modify(|module| {
                match module {
                    Module::Broadcast(targets) => {
                        for target in targets {
                            q.push_back((dest.clone(), target.to_string(), pulse));
                        }
                    },
                    Module::FlipFlop(is_on, targets) => {
                        if !pulse {
                            for target in targets {
                                q.push_back((dest.clone(), target.to_string(), !*is_on));
                            }
                            *is_on = !*is_on;
                        }
                    },
                    Module::Conjunction(input_map,targets) => {
                        input_map.entry(source.to_string()).and_modify(|e| *e = pulse);
                        let all_on = input_map.values().all(|&x| x);
                        for target in targets {
                            q.push_back((dest.clone(), target.clone(), !all_on));
                        }
                    }
                };
            });
        }
    }
    let factors: Vec<_> = last_conjunction_feeders.values().cloned().collect();
    println!("{factors:?}");

    let ans = lcm(factors);
    println!("{ans}");
    return ans.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert!(solution(INPUT) == EXPECTED.to_string());
    }
}
