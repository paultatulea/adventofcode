use std::collections::HashMap;
use std::collections::VecDeque;

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
    for (input_modname, module) in &map {
        let targets: Vec<String>;
        match module {
            Module::FlipFlop(_, dest) | Module::Conjunction(_, dest) | Module::Broadcast(dest) => targets = dest.clone(),
        }
        for target in targets {
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

    let mut q = VecDeque::new();
    let mut low = 0;
    let mut high = 0;
    for i in 0..1000 {
        q.push_back(("button".to_string(), "broadcaster".to_string(), false));

        while let Some((fromname, modname, pulse)) = q.pop_front() {
            println!("{fromname} -{pulse}-> {modname}");
            if pulse {
                high += 1;
            } else {
                low += 1;
            }

            map.entry(modname.clone()).and_modify(|module| {
                match module {
                    Module::Broadcast(targets) => {
                        for target in targets {
                            q.push_back((modname.clone(), target.to_string(), pulse));
                        }
                    },
                    Module::FlipFlop(is_on, targets) => {
                        if !pulse {
                            for target in targets {
                                q.push_back((modname.clone(), target.to_string(), !*is_on));
                            }
                            *is_on = !*is_on;
                        }
                    },
                    Module::Conjunction(input_map,targets) => {
                        input_map.entry(fromname.to_string()).and_modify(|e| *e = pulse);
                        let all_on = input_map.values().all(|&x| x);
                        for target in targets {
                            q.push_back((modname.clone(), target.clone(), !all_on));
                        }
                    }
                };
            });
        }
    }

    let ans = low * high;
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
