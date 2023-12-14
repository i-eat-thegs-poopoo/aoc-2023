use std::iter;

fn main() {
    let (one, two) = utils::setup();
    one(|input| {
        let records = parse(input);

        records
            .into_iter()
            .map(|rec| find_unknowns(rec.springs.into_iter(), rec.groups.iter().copied()))
            .sum::<usize>()
    });

    two(|input| {
        let records = parse(input);

        records
            .into_iter()
            .map(|rec| {
                let mut springs = iter::repeat(
                        iter::once(Spring::Unknown)
                            .chain(rec.springs.iter().copied())
                    )
                        .take(5)
                        .flatten();
                springs.next();
                find_unknowns(springs, iter::repeat(rec.groups.iter().copied()).take(5).flatten())
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

fn find_unknowns(
    springs: impl Iterator<Item = Spring>,
    groups: impl Iterator<Item = u64> + Clone,
) -> usize {
    let mut combos = vec![(groups, None)];
    let mut buffer = Vec::new();

    for spring in springs {
        buffer.clear();

        for (mut groups, remaining) in combos.drain(..) {
            let remaining = match spring {
                Spring::Operational => match remaining {
                    Some(0) | None => None,
                    Some(_) => continue,
                },
                Spring::Damaged => match remaining.or_else(|| groups.next()) {
                    Some(0) | None => continue,
                    Some(len) => Some(len - 1),
                },
                Spring::Unknown => match remaining {
                    Some(0) => None,
                    Some(len) => Some(len - 1),
                    None => {
                        // Assume operational
                        buffer.push((groups.clone(), None));

                        // Assume damaged
                        if let Some(len) = groups.next() {
                            buffer.push((groups, Some(len - 1)));
                        }

                        continue;
                    }
                },
            };

            buffer.push((groups, remaining));
        }

        combos.extend(buffer.drain(..));
    }

    combos
        .into_iter()
        .map(|(mut groups, remaining)| (groups.next(), remaining))
        .filter(|(next, remaining)| next.is_none() && remaining.unwrap_or(0) == 0)
        .count()
}
