use std::collections::HashMap;

fn main() {
    let (polymer, rules) = include_str!("../input").split_once("\n\n").unwrap();
    let rules: HashMap<_, _> = rules.lines().map(|rule| {
        rule.split_once(" -> ").unwrap()
    }).collect();

    let steps_to_take = 10;

    let mut polymer = String::from(polymer);
    for step in 1..=steps_to_take {
        println!("doing step {step}");
        let mut new_polymer = String::new();
        for i in 0..polymer.len()-1 {
            // println!("{:?}", &polymer[i..=i+1]);
            new_polymer.push_str(&polymer[i..=i]);
            if let Some(&insert_this) = rules.get(&polymer[i..=i+1]) {
                // println!("{}", insert_this);
                new_polymer.push_str(insert_this);
            }
        }
        new_polymer.push_str(&polymer[polymer.len()-1..=polymer.len()-1]);
        // println!("after {step} step(s): {new_polymer}");
        polymer = new_polymer;
    }

    let mut counts = HashMap::new();
    polymer.chars().for_each(|ch| {
        *counts.entry(ch).or_insert(0) += 1;
    });

    let (min, max) = counts.iter().fold((i32::MAX, 0), |(min, max), n| {
        (min.min(*n.1), max.max(*n.1))
    });
    // println!("{polymer}");
    println!("{} - {} = {}", max, min, max-min);

}
