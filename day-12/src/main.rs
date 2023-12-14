fn main() {
    let (one, two) = utils::setup();
    one(|input| {
        parse(input)
            .into_iter()
            .map(|rec| find_unknowns(&rec.springs, &rec.groups))
            .sum::<usize>()
    });

    two(|input| {
        parse(input)
            .into_iter()
            .map(|rec| {
                let mut springs = rec.springs;
                springs.push(Spring::Unknown);
                let mut springs = springs.repeat(5);
                springs.pop();

                let groups = rec.groups.repeat(5);

                find_unknowns(&springs, &groups)
            })
            .sum::<usize>()
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

struct State {
    next_spring: usize,
    next_group: usize,
    remaining: Option<u64>,
}

fn find_unknowns(springs: &[Spring], groups: &[u64]) -> usize {
    let mut combos = 0;
    let mut state_stack = vec![State {
        next_spring: 0,
        next_group: 0,
        remaining: None,
    }];

    'outer: while let Some(mut state) = state_stack.pop() {
        for spring in state.next_spring..springs.len() {
            state.remaining = match springs[spring] {
                Spring::Operational => match state.remaining {
                    Some(0) | None => None,
                    Some(_) => continue 'outer,
                },
                Spring::Damaged => match state.remaining {
                    Some(0) => continue 'outer,
                    Some(len) => Some(len - 1),
                    None => match groups.get(state.next_group) {
                        Some(len) => {
                            state.next_group += 1;
                            Some(len - 1)
                        }
                        None => continue 'outer,
                    },
                },
                Spring::Unknown => match state.remaining {
                    Some(0) => None,
                    Some(len) => Some(len - 1),
                    None => {
                        // Assume damaged
                        if let Some(len) = groups.get(state.next_group) {
                            state_stack.push(State {
                                next_spring: spring + 1,
                                next_group: state.next_group + 1,
                                remaining: Some(len - 1),
                            });
                        }

                        // Assume operational
                        None
                    }
                },
            };
        }

        if state.next_group == groups.len() && state.remaining.unwrap_or(0) == 0 {
            combos += 1;
        }
    }

    combos
}
