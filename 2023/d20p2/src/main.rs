use std::collections::{HashMap, VecDeque};

const LOW: bool = false;
const HIGH: bool = true;

#[derive(Debug, PartialEq, Eq)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug)]
struct Module {
    tipe: ModuleType,
    flip_flop_state: bool,
    conjunction_inputs: HashMap<&'static str, bool>,
    outputs: Vec<&'static str>,
}

#[derive(Debug)]
struct Machine {
    counts: (usize, usize),
    modules: HashMap<&'static str, Module>,
    queue: VecDeque<(bool, &'static str, &'static str)>,
}

impl Machine {
    fn from(modules: HashMap<&'static str, (char, Vec<&'static str>)>) -> Self {
        let mut mods = HashMap::new();
        // println!("digraph g {{");
        for (&name, v) in modules.iter() {
            let tipe = match v.0 {
                '_' => ModuleType::Broadcaster,
                '%' => ModuleType::FlipFlop,
                '&' => ModuleType::Conjunction,
                _ => panic!("unknown type"),
            };
            
            let outputs = &v.1;

            // dot output
            // let color = match tipe {
            //     ModuleType::Broadcaster => "blue",
            //     ModuleType::Conjunction => "green",
            //     ModuleType::FlipFlop => "orange",
            // };
            // println!("{name} [style=filled, fillcolor={color}]");
            // print!("{name} -> ");
            // println!("{}", outputs.join(","));

            mods.insert(
                name,
                Module {
                    tipe,
                    flip_flop_state: false,
                    conjunction_inputs: HashMap::new(),
                    outputs: outputs.clone(),
                },
            );
        }
        // println!("}}");

        for (&name, v) in modules.iter() {
            let outputs = &v.1;
            for &output in outputs {
                if let Some(e) = mods.get_mut(output) {
                    if e.tipe == ModuleType::Conjunction {
                        e.conjunction_inputs.insert(name, false);
                    }
                }
            }
        }
        Machine {
            counts: (0, 0),
            modules: mods,
            queue: VecDeque::new(),
        }
    }

    fn push_button(&mut self, n: usize) {
        self.queue.push_back((LOW, "button", "broadcaster"));

        while !self.queue.is_empty() {
            let pulse = self.queue.pop_front().unwrap();
            let (pulse_level, from, to) = pulse;

            if matches!(from, "jc" | "qq" | "fj" | "vm") && pulse_level == LOW {
                println!("{n}: LOW from {from}");

            }

            match pulse_level {
                LOW => self.counts.0 += 1,
                HIGH => self.counts.1 += 1,
            }

            if let Some(module) = self.modules.get_mut(to) {
                match module.tipe {
                    ModuleType::Broadcaster => {
                        for &output in &module.outputs {
                            self.queue.push_back((pulse_level, to, output));
                        }
                    }
                    ModuleType::FlipFlop => match pulse_level {
                        HIGH => {}
                        LOW => {
                            module.flip_flop_state = !module.flip_flop_state;
                            let level_to_send = module.flip_flop_state;
                            for &output in &module.outputs {
                                self.queue.push_back((level_to_send, to, output));
                            }
                        }
                    },
                    ModuleType::Conjunction => {
                        module
                            .conjunction_inputs
                            .entry(from)
                            .and_modify(|x| *x = pulse_level);
                        let level_to_send = !module.conjunction_inputs.iter().all(|i| *i.1);
                        for &output in &module.outputs {
                            self.queue.push_back((level_to_send, to, output));
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let modules: HashMap<_, _> = include_str!("../input")
        .trim()
        .lines()
        .map(|line| {
            let (fr, to) = line.split_once(" -> ").unwrap();
            let to: Vec<_> = to.split(",").map(|x| x.trim()).collect();
            let from;
            if fr == "broadcaster" {
                from = ('_', "broadcaster");
            } else {
                from = (fr.chars().next().unwrap(), &fr[1..])
            };
            (from.1, (from.0, to))
        })
        .collect();

    let mut machine = Machine::from(modules);
    (1..10000).for_each(|n| machine.push_button(n));
}
