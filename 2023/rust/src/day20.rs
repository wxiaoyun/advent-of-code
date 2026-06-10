use std::{
    any::Any,
    collections::{HashMap, HashSet, VecDeque},
    convert::identity,
    fmt::Debug,
};

use faststr::FastStr;
use tap::{Pipe, Tap};

const BUTTON: &str = "button";
const BC: &str = "broadcaster";
const INV: &str = "inv";
const FLIP_FLOP: char = '%';
const CONJUNCTION: char = '&';
const HIGH: bool = true;
const LOW: bool = false;

trait Module: Debug {
    fn pulse(&mut self, src: &FastStr, pulse: bool) -> Option<bool>;

    fn as_any(&mut self) -> &mut dyn Any;
}

#[derive(Debug, Clone)]
struct FlipFlop {
    on: bool,
}

impl FlipFlop {
    fn new() -> Self {
        Self { on: false }
    }
}

impl Module for FlipFlop {
    fn pulse(&mut self, _: &FastStr, pulse: bool) -> Option<bool> {
        if pulse == HIGH {
            return None;
        }

        let on_state = self.on;
        self.on = !self.on;
        match on_state {
            false => Some(HIGH),
            true => Some(LOW),
        }
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
struct Conjunction {
    recent_input_dict: HashMap<FastStr, bool>,
}

impl Conjunction {
    fn new() -> Self {
        Self {
            recent_input_dict: Default::default(),
        }
    }

    fn set_inputs(&mut self, inputs: impl Iterator<Item = FastStr>) {
        self.recent_input_dict = HashMap::with_capacity(inputs.size_hint().0)
            .tap_borrow_mut(|hm| hm.extend(inputs.map(|i| (i, LOW))));
    }
}

impl Module for Conjunction {
    fn pulse(&mut self, src: &FastStr, pulse: bool) -> Option<bool> {
        self.recent_input_dict.insert(src.clone(), pulse);

        let all_high = self.recent_input_dict.values().copied().all(identity);
        match all_high {
            true => LOW,
            false => HIGH,
        }
        .pipe(Some)
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

type Broadcast = ();

impl Module for Broadcast {
    fn pulse(&mut self, _: &FastStr, pulse: bool) -> Option<bool> {
        Some(pulse)
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let (mut modules, adj_list) = parse_input(input);

    let mut n_pulse_low = 0;
    let mut n_pulse_high = 0;
    let mut event_queue = VecDeque::new();

    for _ in 0..1000 {
        event_queue.push_back((
            BUTTON.to_string().pipe(FastStr::from),
            BC.to_string().pipe(FastStr::from),
            LOW,
        ));

        while !event_queue.is_empty() {
            let len = event_queue.len();

            for _ in 0..len {
                let (src, dst, pulse) = event_queue.pop_front().unwrap();

                match pulse {
                    LOW => n_pulse_low += 1,
                    HIGH => n_pulse_high += 1,
                };

                let Some(module) = modules.get_mut(&dst) else {
                    println!("{dst} not found, skipping");
                    continue;
                };
                let Some(result_pulse) = module.pulse(&src, pulse) else {
                    continue;
                };

                for node in adj_list.get(&dst).unwrap() {
                    event_queue.push_back((dst.clone(), node.clone(), result_pulse));
                }
            }
        }
    }

    n_pulse_low * n_pulse_high
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    0
}

fn parse_input(
    input: impl AsRef<str>,
) -> (
    HashMap<FastStr, Box<dyn Module>>,
    HashMap<FastStr, Vec<FastStr>>,
) {
    let modules: HashMap<_, Box<dyn Module>> = HashMap::new();
    let adj_list = HashMap::new();
    let rev_adj_list = HashMap::new();

    let (mut modules, adj_list, rev_adj_list) = input.as_ref().lines().fold(
        (modules, adj_list, rev_adj_list),
        |(mut modules, mut adj_list, mut rev_adj_list), l| {
            let (mut mod_str, mut dest_str) = l.split_once("->").unwrap();
            mod_str = mod_str.trim();
            dest_str = dest_str.trim();

            let adj = dest_str
                .split(',')
                .map(|s| s.trim().to_string().into())
                .collect::<Vec<FastStr>>();

            let (mod_name, module): (FastStr, Box<dyn Module>) = match mod_str {
                BC => (mod_str.to_string().into(), Box::new(())),
                _ => {
                    let mut mod_chars = mod_str.chars();
                    let ty = mod_chars.next();
                    let name = mod_chars.as_str().to_string().into();
                    match ty {
                        Some(FLIP_FLOP) => (name, Box::new(FlipFlop::new())),
                        Some(CONJUNCTION) => (name, Box::new(Conjunction::new())),
                        _ => unreachable!(),
                    }
                }
            };

            modules.insert(mod_name.clone(), module);
            adj_list.insert(mod_name.clone(), adj.clone());

            for adj_node in adj {
                let list = rev_adj_list
                    .entry(adj_node.clone())
                    .or_insert(HashSet::new());
                list.insert(mod_name.clone());
            }

            (modules, adj_list, rev_adj_list)
        },
    );

    for (mod_name, mut module) in modules.iter_mut() {
        if let Some(m) = module.as_any().downcast_mut::<Conjunction>() {
            m.set_inputs(rev_adj_list.get(mod_name).unwrap().iter().cloned());
        };
    }

    (modules, adj_list)
}
