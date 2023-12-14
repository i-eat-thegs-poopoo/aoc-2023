use std::collections::HashMap;

fn main() {
    let (one, two) = utils::setup();
    one(|input| {
        let mut cache = HashMap::new();

        parse(input)
            .into_iter()
            .map(|rec| {
                cache.clear();

                let state = State {
                    next_spring: 0,
                    next_group: 0,
                    remaining: None,
                };

                find_combos(&rec.springs, &rec.groups, state, &mut cache)
            })
            .sum::<u64>()
    });

    two(|input| {
        let mut cache = HashMap::new();

        parse(input)
            .into_iter()
            .map(|rec| {
                cache.clear();

                let mut springs = rec.springs;
                springs.push(Spring::Unknown);
                let mut springs = springs.repeat(5);
                springs.pop();

                let groups = rec.groups.repeat(5);

                let state = State {
                    next_spring: 0,
                    next_group: 0,
                    remaining: None,
                };

                find_combos(&springs, &groups, state, &mut cache)
            })
            .sum::<u64>()
    });
}

struct Record {
    springs: Vec<Spring>,
    groups: Vec<u64>,
}

#[derive(Clone, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

fn parse(input: &str) -> Vec<Record> {
    let mut parser = utils::Parser::new(input);
    let mut records = Vec::new();

    parser.sep_by("\n", |parser| {
        let mut record = Record {
            springs: Vec::new(),
            groups: Vec::new(),
        };

        loop {
            let spring = match parser.inner.peek().unwrap().1 {
                '.' => Spring::Operational,
                '#' => Spring::Damaged,
                '?' => Spring::Unknown,
                _ => break,
            };

            parser.inner.next();
            record.springs.push(spring);
        }

        parser.expect(" ");

        parser.sep_by(",", |parser| {
            let len = parser.int();
            record.groups.push(len);
        });

        records.push(record);
    });

    records
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    next_spring: usize,
    next_group: usize,
    remaining: Option<u64>,
}

fn find_combos(
    springs: &[Spring],
    groups: &[u64],
    mut state: State,
    cache: &mut HashMap<State, u64>,
) -> u64 {
    if let Some(combos) = cache.get(&state) {
        return *combos;
    }

    while state.next_spring < springs.len() {
        state.remaining = match springs[state.next_spring] {
            Spring::Operational => match state.remaining {
                Some(0) | None => None,
                Some(_) => return 0,
            },
            Spring::Damaged => match state.remaining {
                Some(0) => return 0,
                Some(len) => Some(len - 1),
                None => match groups.get(state.next_group) {
                    Some(len) => {
                        state.next_group += 1;
                        Some(len - 1)
                    }
                    None => return 0,
                },
            },
            Spring::Unknown => match state.remaining {
                Some(0) => None,
                Some(len) => Some(len - 1),
                None => {
                    let if_operational = {
                        let state = State {
                            next_spring: state.next_spring + 1,
                            next_group: state.next_group,
                            remaining: None,
                        };

                        find_combos(springs, groups, state, cache)
                    };

                    let if_damaged = groups
                        .get(state.next_group)
                        .map(|len| {
                            let state = State {
                                next_spring: state.next_spring + 1,
                                next_group: state.next_group + 1,
                                remaining: Some(len - 1),
                            };

                            find_combos(springs, groups, state, cache)
                        })
                        .unwrap_or(0);

                    let combos = if_operational + if_damaged;
                    cache.insert(state, combos);

                    return combos;
                }
            },
        };

        state.next_spring += 1;
    }

    (state.next_group == groups.len() && state.remaining.unwrap_or(0) == 0) as u64
}
