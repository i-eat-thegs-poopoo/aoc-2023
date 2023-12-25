use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    iter,
};

fn main() {
    let (one, two) = utils::setup();
    one(|input| {
        let mut circuit = parse(input);

        for _ in 0..1000 {
            push_button(&mut circuit, |_, _| {});
        }

        circuit.tot_high * circuit.tot_low
    });
    two(|input| {
        let mut circuit = parse(input);
        find_rx(&mut circuit)
    });
}

#[derive(Clone, Copy, Debug)]
enum Pulse {
    High,
    Low,
}

enum Kind {
    Broadcaster,
    Flipflop {
        on: bool,
    },
    Conjunction {
        // Assuming less than 64 modules
        inputs: u64,
        highs: u64,
    },
    NoOp,
}

struct Module {
    kind: Kind,
    to: Vec<usize>,
}

struct Circuit {
    mods: Vec<Module>,
    broadcaster: usize,
    rx: usize,
    tot_high: u64,
    tot_low: u64,
}

fn parse(input: &str) -> Circuit {
    let mut parser = utils::Parser::new(input);
    let mut circuit = Circuit {
        mods: Vec::new(),
        broadcaster: 0,
        rx: 0,
        tot_high: 0,
        tot_low: 0,
    };
    let mut ids = HashMap::new();

    fn get_id<'a>(
        name: &'a str,
        circuit: &mut Circuit,
        ids: &mut HashMap<&'a str, usize>,
    ) -> usize {
        *ids.entry(name).or_insert_with(|| {
            let id = circuit.mods.len();

            circuit.mods.push(Module {
                kind: Kind::NoOp,
                to: Vec::new(),
            });

            match name {
                "broadcaster" => circuit.broadcaster = id,
                "rx" => circuit.rx = id,
                _ => {}
            }

            id
        })
    }

    parser.sep_by("\n", |parser| {
        let kind = match parser.peek().unwrap() {
            '%' => {
                parser.next();
                Kind::Flipflop { on: false }
            }
            '&' => {
                parser.next();
                Kind::Conjunction {
                    inputs: 0,
                    highs: 0,
                }
            }
            _ => Kind::Broadcaster,
        };

        let id = get_id(parser.ident(), &mut circuit, &mut ids);
        parser.expect(" -> ");
        parser.sep_by(", ", |parser| {
            let to = get_id(parser.ident(), &mut circuit, &mut ids);
            circuit.mods[id].to.push(to);
        });

        circuit.mods[id].kind = kind;
    });

    for id in 0..circuit.mods.len() {
        let flag = id_to_flag(id);

        let (left, right) = circuit.mods.split_at_mut(id);
        let (module, right) = right.split_first_mut().unwrap();

        for &to in &module.to {
            let to = match to.cmp(&id) {
                Ordering::Less => &mut left[to],
                Ordering::Greater => &mut right[to - id - 1],
                Ordering::Equal => panic!(),
            };

            if let Kind::Conjunction { inputs, .. } = &mut to.kind {
                *inputs |= flag;
            }
        }
    }

    circuit
}

fn id_to_flag(id: usize) -> u64 {
    1 << id
}

fn push_button(circuit: &mut Circuit, mut visitor: impl FnMut(usize, &Module)) {
    let mut queue = VecDeque::from([(Pulse::Low, circuit.broadcaster, 0)]);

    while let Some((recv, id, from)) = queue.pop_front() {
        let module = circuit.mods.get_mut(id).unwrap();

        match recv {
            Pulse::High => circuit.tot_high += 1,
            Pulse::Low => circuit.tot_low += 1,
        }

        let send = match &mut module.kind {
            Kind::Broadcaster => Some(recv),
            Kind::Flipflop { on } => match recv {
                Pulse::High => None,
                Pulse::Low => {
                    let send = match on {
                        true => Pulse::Low,
                        false => Pulse::High,
                    };

                    *on = !*on;
                    Some(send)
                }
            },
            Kind::Conjunction { inputs, highs } => {
                let flag = id_to_flag(from);

                match recv {
                    Pulse::High => *highs |= flag,
                    Pulse::Low => *highs &= !flag,
                }

                Some(if highs == inputs {
                    Pulse::Low
                } else {
                    Pulse::High
                })
            }
            Kind::NoOp => None,
        };

        if let Some(send) = send {
            for to in &module.to {
                queue.push_back((send, *to, id));
            }
        }

        visitor(id, module);
    }
}

fn find_rx(circuit: &mut Circuit) -> u64 {
    // Assumed to be conjunction
    let to_rx = circuit
        .mods
        .iter()
        .position(|module| module.to.contains(&circuit.rx))
        .unwrap();

    let Kind::Conjunction { inputs, highs } = circuit.mods[to_rx].kind else {
        panic!();
    };

    let mut prev = highs;

    // Assumed to have regular periods
    let mut periods = iter::repeat(None)
        .take(inputs.count_ones() as usize)
        .collect::<Vec<_>>();

    for n in 1.. {
        let mut all_seen = false;

        push_button(circuit, |id, module| {
            if id == to_rx {
                let Kind::Conjunction { highs, .. } = module.kind else {
                    panic!();
                };

                if highs != 0 && highs != prev {
                    prev = highs;

                    let (mut inputs, mut highs) = (inputs, highs);
                    all_seen = true;

                    for period in &mut periods {
                        while inputs & 1 == 0 {
                            highs >>= 1;
                            inputs >>= 1;
                        }

                        if period.is_none() {
                            if highs & 1 != 0 {
                                *period = Some(n);
                            } else {
                                all_seen = false;
                            }
                        }

                        highs >>= 1;
                        inputs >>= 1;
                    }
                }
            }
        });

        if all_seen {
            break;
        }
    }

    periods.into_iter().map(Option::unwrap).reduce(lcm).unwrap()
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        (a, b) = (b, a % b);
    }

    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}
