pub fn run(input: &str) {
    let mut parser = utils::Parser::new(input);
    let mut histories = Vec::new();

    parser.sep_by("\n", |parser| {
        let mut values = Vec::new();
        parser.sep_by(" ", |parser| {
            let sign = parser.consume_match("-");
            let val = parser.int() as i64;

            values.push(if sign { -val } else { val });
        });
        histories.push(values);
    });

    let mut sum = 0;

    for history in histories {
        sum += next_in_row(history);
    }

    println!("One: {sum}");
}

fn next_in_row(row: Vec<i64>) -> i64 {
    if row.iter().all(|d| *d == 0) {
        return 0;
    }

    let diffs = row.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    let diff = next_in_row(diffs);
    *row.last().unwrap() + diff
}
