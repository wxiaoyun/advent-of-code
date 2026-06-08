use std::{collections::HashMap, str::FromStr};

const ACCEPT: &str = "A";
const REJECT: &str = "R";
const IN: &str = "in";

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let (workflows, parts) = parse_input(input);
    let Some(init_wf) = workflows.get(&IN.to_string()) else {
        unreachable!()
    };

    let mut score = 0;
    'outer: for p in parts {
        let mut cur_workflow = init_wf;

        'inner: loop {
            for r in cur_workflow {
                let outcome = match r {
                    Rule::Test(t) => {
                        let (ok, outcome) = t.test(&p);
                        if !ok {
                            continue;
                        }
                        outcome
                    }
                    Rule::Outcome(j) => j.clone(),
                };

                match outcome {
                    s if s == ACCEPT => {
                        score += p.values().copied().sum::<i64>();
                        continue 'outer;
                    }
                    s if s == REJECT => {
                        continue 'outer;
                    }
                    s => {
                        cur_workflow = workflows.get(&s).unwrap();
                        continue 'inner;
                    }
                }
            }
        }
    }

    score
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    let (workflows, parts) = parse_input(input);

    let mut full_range = HashMap::new();
    full_range.extend("xmas".chars().map(|ch| {
        (
            ch,
            Range {
                start: 1,
                end: 4001,
            },
        )
    }));

    fn search(
        wfs: &HashMap<String, Vec<Rule>>,
        part_ranges: &HashMap<char, Range>,
        wf_name: String,
        wf_index: usize,
    ) -> i64 {
        let rule = wfs.get(&wf_name).and_then(|wf| wf.get(wf_index)).unwrap();

        let t = match rule {
            Rule::Outcome(o) if o == ACCEPT => {
                return part_ranges
                    .values()
                    .fold(1, |acc, r| acc * (r.end - r.start));
            }
            Rule::Outcome(o) if o == REJECT => {
                return 0;
            }
            Rule::Outcome(o) => {
                return search(wfs, part_ranges, o.clone(), 0);
            }
            Rule::Test(t) => t,
        };

        let mut counts = 0;

        // Reject case:
        let mut rej_case_ranges = part_ranges.clone();
        t.shrink(rej_case_ranges.get_mut(&t.attr).unwrap(), true);
        counts += search(wfs, &mut rej_case_ranges, wf_name, wf_index + 1);

        let mut accept_case_ranges = part_ranges.clone();
        t.shrink(accept_case_ranges.get_mut(&t.attr).unwrap(), false);
        counts += match &t.outcome {
            o if o == ACCEPT => accept_case_ranges
                .values()
                .fold(1, |acc, r| acc * (r.end - r.start)),
            o if o == REJECT => 0,
            o => search(wfs, &accept_case_ranges, o.clone(), 0),
        };

        counts
    }

    search(&workflows, &full_range, IN.to_string(), 0)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: i64,
    end: i64,
}

#[derive(Debug)]
struct Test {
    attr: char,
    cmp: char,
    threshold: i64,
    outcome: String,
}

impl Test {
    fn test(self: &Self, part: &HashMap<char, i64>) -> (bool, String) {
        let Some(&val) = part.get(&self.attr) else {
            unreachable!();
        };

        let ok = match self.cmp {
            '<' => val < self.threshold,
            '>' => val > self.threshold,
            _ => unreachable!(),
        };

        (ok, self.outcome.clone())
    }

    fn shrink(&self, r: &mut Range, negate: bool) {
        let ok = match self.cmp {
            '<' => {
                if negate {
                    // >=
                    r.start = self.threshold;
                } else {
                    r.end = self.threshold;
                }
            }
            '>' => {
                if negate {
                    // <=
                    r.end = self.threshold + 1;
                } else {
                    r.start = self.threshold + 1;
                }
            }
            _ => unreachable!(),
        };
    }
}

#[derive(Debug)]
enum Rule {
    Test(Test),
    Outcome(String),
}

impl FromStr for Rule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.contains(':') {
            return Ok(Rule::Outcome(s.to_owned()));
        }

        let Some((test, outcome)) = s.split_once(':') else {
            return Err(());
        };

        let mut test = test.chars();
        let attr = test.next().unwrap();
        let cmp = test.next().unwrap();
        let threshold = test.collect::<String>().parse::<i64>().unwrap();

        Ok(Rule::Test(Test {
            attr,
            cmp,
            threshold,
            outcome: outcome.to_owned(),
        }))
    }
}

fn parse_input(input: impl AsRef<str>) -> (HashMap<String, Vec<Rule>>, Vec<HashMap<char, i64>>) {
    let (workflows, parts) = input.as_ref().split_once("\n\n").unwrap();
    let workflows = workflows
        .lines()
        .map(|l| {
            let (name, rules) = l.trim_end_matches('}').split_once('{').unwrap();
            let rules = rules
                .split(',')
                .map(Rule::from_str)
                .collect::<Result<Vec<_>, _>>()
                .unwrap();

            (name.to_owned(), rules)
        })
        .collect::<HashMap<_, _>>();

    let parts = parts
        .lines()
        .map(|l| {
            l.trim_matches(['{', '}'])
                .split(',')
                .map(|p| {
                    let (attr, val) = p.split_once('=').unwrap();
                    (attr.chars().next().unwrap(), val.parse::<i64>().unwrap())
                })
                .collect::<HashMap<_, _>>()
        })
        .collect::<Vec<_>>();

    (workflows, parts)
}
