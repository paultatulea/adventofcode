use std::collections::HashMap;

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
const EXPECTED: &'static str = "19114";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

#[derive(Debug)]
struct Rule
{
    part: Option<String>,
    op: Option<char>,
    value: Option<u32>,
    dest: String,
}

fn solution(s: &str) -> String {
    let mut sections = s.split("\n\n");
    let workflows = sections.next().unwrap();
    let workflow_map = workflows.lines().fold(HashMap::new(), |mut acc, line| {
        let name: String = line.chars().take_while(|c| c.is_alphabetic()).collect();
        let r: String = line.chars().skip(name.len() + 1).take_while(|&c| c != '}').collect();
        let rules: Vec<_> = r.split(',').map(|rule| {
            let mut chars = rule.chars();
            if rule.contains(':') {
                let part = chars.next().unwrap().to_string();
                let op = chars.next().unwrap();
                let valuestr: String = rule.chars().skip(2).take_while(|&c| c.is_numeric()).collect();
                let value = valuestr.parse::<u32>().unwrap();
                let dest: String = rule.chars().skip_while(|&c| c != ':').skip(1).collect();
                let rule = Rule { part: Some(part), op: Some(op), value: Some(value), dest };
                rule
            } else {
                let dest: String = chars.collect();
                let rule = Rule { part: None, op: None, value: None, dest };
                rule
            }
        }).collect();
        acc.insert(name, rules);
        acc
    });

    let parts = sections.next().unwrap();
    let ans = parts.lines().fold(0, |mut score, line| {
        let clean = &line[1..line.len()-1];
        let ratings: HashMap<String, u32> = clean.split(",").fold(HashMap::new(), |mut acc, x| {
            let mut z = x.split('=');
            let name = z.next().unwrap().to_string();
            let value = z.next().unwrap().parse::<u32>().unwrap();
            acc.insert(name, value);
            acc
        });

        let mut next_workflow = String::from("in");
        let mut accepted = false;
        loop {
            let rules = workflow_map.get(&next_workflow).unwrap();
            for rule in rules {
                if let Some(rule_rating) = &rule.part {
                    let op = rule.op.unwrap();
                    let rule_value = rule.value.unwrap();
                    let &part_rating = ratings.get(rule_rating).unwrap();
                    let mut success = false;
                    if op == '<' {
                        success = part_rating < rule_value;
                    } else {
                        success = part_rating > rule_value;
                    }
                    if success {
                        next_workflow = rule.dest.clone();
                        break;
                    }
                } else {
                    next_workflow = rule.dest.clone();
                    break;
                }
            }
            if next_workflow == String::from("A") {
                accepted = true;
                break;
            } else if next_workflow == String::from("R") {
                break;
            }
        }
        if accepted {
            score += ratings.values().sum::<u32>();
        }
        score 
    });

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
