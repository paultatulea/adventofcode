#[allow(dead_code)]
const INPUT: &'static str = "\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

#[allow(dead_code)]
const EXPECTED: &'static str = "145";

fn main() {
    let input = include_str!("../input.txt");
    let sol = solution(input);
    dbg!(sol);
}

fn solution(s: &str) -> String {
    let mut boxes: Vec<Vec<(String, usize)>> = vec![Vec::new(); 256];
    for op in s.lines().next().unwrap().split(",") {
        let label: String = op.chars().take_while(|c| c.is_alphabetic()).collect();
        let label_hash = op.chars().take_while(|c| c.is_alphabetic()).fold(0, |mut hash, c| {
            hash += c as usize;
            hash *= 17;
            hash %= 256;
            hash
        });

        let cmd = op.chars().skip(label.len()).next().unwrap();
        match cmd {
            '=' => {
                let focal_lens = op.chars().last().map(|x| x.to_digit(10).unwrap() as usize).unwrap();
                if let Some(i) = boxes[label_hash].iter().position(|x| x.0 == label) {
                    boxes[label_hash][i] = (label, focal_lens);
                } else {
                    boxes[label_hash].push((label, focal_lens));
                }
            },
            '-' => {
                if let Some(i) = boxes[label_hash].iter().position(|x| x.0 == label) {
                    boxes[label_hash].remove(i);
                }
            },
            _ => unreachable!(),
        }
    }

    
    let ans = boxes.iter().enumerate().fold(0, |mut acc, (bi, b)| {
        for (li, l) in b.iter().enumerate() {
            acc += (bi + 1) * (li + 1) * l.1;
        }
        acc
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
