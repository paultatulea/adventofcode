use std::collections::HashMap;
use std::collections::VecDeque;
use std::cmp;

#[allow(dead_code)]
const INPUT: &'static str = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

#[allow(dead_code)]
const EXPECTED: &'static str = "167409079868000";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

#[derive(Debug)]
struct Rule
{
    part: Option<String>,
    start: u128,
    end: u128,
    dest: String,
}

fn solution(s: &str) -> String {
    let lb: u128 = 1;
    let ub: u128 = 4001;
    let mut sections = s.split("\n\n");
    let workflows = sections.next().unwrap();
    let workflow_map = workflows.lines().fold(HashMap::new(), |mut acc, line| {
        let name: String = line.chars().take_while(|c| c.is_alphabetic()).collect();
        let r: String = line.chars().skip(name.len() + 1).take_while(|&c| c != '}').collect();
        let rules: Vec<_> = r.split(',').map(|rule| {
            let mut chars = rule.chars();
            let part: Option<String>;
            let start: u128;
            let end: u128;
            let dest: String;
            if rule.contains(":") {
                part = Some(chars.next().unwrap().to_string());
                let op = chars.next().unwrap();
                let valuestr: String = rule.chars().skip(2).take_while(|&c| c.is_numeric()).collect();
                let value = valuestr.parse::<u128>().unwrap();
                start = if op == '<' { lb } else { value + 1 };
                end = if op == '>' { ub } else { value };
                dest = rule.chars().skip_while(|&c| c != ':').skip(1).collect();
            } else {
                part = None;
                start = 0;
                end = 4001;
                dest = rule.to_string();
            }
            let rule = Rule { part, start, end, dest };
            rule
        }).collect();
        acc.insert(name, rules);
        acc
    });

    let mut processing = VecDeque::new();
    let mut initial = HashMap::new();
    initial.insert("x".to_string(), (lb, ub));
    initial.insert("m".to_string(), (lb, ub));
    initial.insert("a".to_string(), (lb, ub));
    initial.insert("s".to_string(), (lb, ub));
    processing.push_back((initial, String::from("in")));
    let mut ans = 0;
    while let Some((ranges, target_workflow)) = processing.pop_front() {
        if target_workflow == String::from("A") {
            // Accept all combinations
            ans += ranges.values().fold(1, |acc, (start, end)| {
                acc * (end - start)
            });
            continue;
        } else if target_workflow == String::from("R") {
            // Reject all
            continue;
        }

        let workflow = workflow_map.get(&target_workflow).unwrap();
        let mut current_rule_processing = VecDeque::new();
        let mut next_rule = Vec::new();
        current_rule_processing.push_back(ranges);
        for rule in workflow.iter().take(workflow.len() - 1) {
            let rule_part = rule.part.clone().unwrap();
            while let Some(curr_ranges) = current_rule_processing.pop_front() {
                let &(start, end) = curr_ranges.get(&rule_part).unwrap();
                let overlap_start = cmp::max(start, rule.start);
                let overlap_end = cmp::min(end, rule.end);
                if overlap_start < overlap_end {
                    let mut next_ranges = curr_ranges.clone();
                    next_ranges.insert(rule_part.clone(), (overlap_start, overlap_end));
                    processing.push_back((next_ranges, rule.dest.clone()));
                }
                if start < overlap_start {
                    let mut next_ranges = curr_ranges.clone();
                    next_ranges.insert(rule_part.clone(), (start, overlap_start));
                    next_rule.push(next_ranges);
                }
                if end > overlap_end {
                    let mut next_ranges = curr_ranges.clone();
                    next_ranges.insert(rule_part.clone(), (overlap_end, end));
                    next_rule.push(next_ranges);
                }
            }
            current_rule_processing = next_rule.drain(..).collect();
        }

        let last_rule = workflow.last().unwrap();
        for ranges in current_rule_processing {
            processing.push_back((ranges, last_rule.dest.clone()));
        }
    }

    println!("{ans}");
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
