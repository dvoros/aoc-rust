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
        let rules = rest[0..rest.len() - 1]
            .split(",")
            .map(|rule| {
                if let Some((check, res)) = rule.split_once(":") {
                    if let Some((category, num)) = check.split_once("<") {
                        Rule {
                            category: category.chars().next().unwrap(),
                            gt: false,
                            num: num.parse().unwrap(),
                            res: Result::from(res),
                        }
                    } else if let Some((category, num)) = check.split_once(">") {
                        Rule {
                            category: category.chars().next().unwrap(),
                            gt: true,
                            num: num.parse().unwrap(),
                            res: Result::from(res),
                        }
                    } else {
                        panic!("unexpected operator");
                    }
                } else {
                    Rule {
                        category: '_',
                        gt: false,
                        num: 0,
                        res: Result::from(rule),
                    }
                }
            })
            .collect();
        (name.to_string(), Workflow { rules: rules })
    }
}

fn val(count: &HashMap<char, (usize, usize)>) -> usize {
    count.values().map(|v| v.1 - v.0 + 1).product()
}

fn count_accepted(
    workflows: &HashMap<String, Workflow>,
    flow: &str,
    mut count: HashMap<char, (usize, usize)>,
) -> usize {
    let mut sum = 0;

    for r in &workflows[flow].rules {
        match r.category {
            '_' => match &r.res {
                Result::Reject => {}
                Result::Accept => sum += val(&count),
                Result::Redirect(next) => sum += count_accepted(workflows, &next, count.clone()),
            },
            c => {
                let mut matching_count = count.clone();
                if r.gt {
                    // x>123
                    if count[&c].1 < r.num {
                        continue;
                    }
                    matching_count.insert(c, (count[&c].0.max(r.num + 1), count[&c].1));
                    count.insert(c, (count[&c].0, count[&c].1.min(r.num)));
                } else {
                    // x<123
                    if count[&c].0 > r.num {
                        continue;
                    }
                    matching_count.insert(c, (count[&c].0, count[&c].1.min(r.num - 1)));
                    count.insert(c, (count[&c].0.max(r.num), count[&c].1));
                }
                match &r.res {
                    Result::Reject => {}
                    Result::Accept => sum += val(&matching_count),
                    Result::Redirect(next) => {
                        sum += count_accepted(workflows, &next, matching_count)
                    }
                }
            }
        }
    }

    sum
}

fn main() {
    let (workflows, _) = include_str!("../input").trim().split_once("\n\n").unwrap();
    let workflows: HashMap<String, Workflow> = workflows
        .lines()
        .map(|line| Workflow::parse(line))
        .collect();

    let res = count_accepted(
        &workflows,
        "in",
        "xmas".chars().map(|c| (c, (1, 4000))).collect(),
    );
    println!("res: {res}");
}
