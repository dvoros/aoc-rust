use std::collections::HashMap;

#[derive(Debug)]
enum Result {
    Accept,
    Reject,
    Redirect(String),
}

impl Result {
    fn from(s: &str) -> Result {
        match s {
            "A" => Result::Accept,
            "R" => Result::Reject,
            _ => Result::Redirect(s.to_string()),
        }
    }
}

#[derive(Debug)]
struct Rule {
    category: char,
    gt: bool,
    num: usize,
    res: Result,
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn parse(line: &str) -> (String, Workflow) {
        let (name, rest) = line.split_once("{").unwrap();
        let rules = rest[0..rest.len()-1].split(",").map(|rule| {
            if let Some((check, res)) = rule.split_once(":") {
                if let Some((category, num)) = check.split_once("<") {
                    Rule{
                        category: category.chars().next().unwrap(),
                        gt: false,
                        num: num.parse().unwrap(),
                        res: Result::from(res),
                    }
                } else if let Some((category, num)) = check.split_once(">") {
                    Rule{
                        category: category.chars().next().unwrap(),
                        gt: true,
                        num: num.parse().unwrap(),
                        res: Result::from(res),
                    }
                } else {
                    panic!("unexpected operator");
                }
            } else {
                Rule{
                    category: '_',
                    gt: false,
                    num: 0,
                    res: Result::from(rule),
                }
            }
        }).collect();
        (
            name.to_string(),
            Workflow {
                rules: rules,
            },
        )
    }

    fn run_on(&self, part: &HashMap<char, usize>) -> &Result {
        for r in &self.rules {
            match r.category {
                '_' => return &r.res,
                c => {
                    if r.gt && part[&c] > r.num {
                        return &r.res
                    }
                    if !r.gt && part[&c] < r.num {
                        return &r.res
                    }
                }
            }
        }
        panic!("no result");
    }
}

fn main() {
    let (workflows, parts) = include_str!("../input").trim().split_once("\n\n").unwrap();
    let workflows: HashMap<String, Workflow> = workflows.lines().map(|line| Workflow::parse(line)).collect();
    let parts: Vec<HashMap<char, usize>> = parts.lines().map(|line| {
        line[1..line.len()-1].split(",").map(|p| {
            let (category, num) = p.split_once("=").unwrap();
            (category.chars().next().unwrap(), num.parse().unwrap())
        }).collect()

    }).collect();

    let mut sum = 0;
    for part in parts {
        let mut res = &Result::Redirect(String::from("in"));
        loop {
            match res {
                Result::Redirect(next) => res = workflows[next.as_str()].run_on(&part),
                Result::Accept =>  {
                    sum += part.values().sum::<usize>();
                    break;
                }
                Result::Reject => {
                    break;
                }
            }
        }
    }

    println!("res: {sum}");

}
