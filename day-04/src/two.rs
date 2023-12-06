pub fn run(input: &str) {
    let mut parser = utils::Parser::new(input);
    let mut total = 0;

    let mut copies = Vec::new();

    parser.sep_by("\n", |parser| {
        parser.expect("Card");
        parser.str_while(|c| *c == ' ');
        let card = parser.int() as usize;
        parser.expect(":");

        let mut winning = Vec::new();
        let mut matches = 0;

        loop {
            parser.str_while(|c| *c == ' ');

            if parser.inner.peek().unwrap().1 == '|' {
                parser.inner.next();
                break;
            } else {
                let num = parser.int();
                winning.push(num);
            }
        }

        loop {
            if parser.str_while(|c| *c == ' ').is_empty() {
                break;
            }

            let num = parser.int();

            if winning.contains(&num) {
                matches += 1;
            }
        }

        while copies.len() < card + matches {
            copies.push(1);
        }

        for offset in 0..matches {
            copies[card + offset] += copies[card - 1];
        }

        total += copies[card - 1];
    });

    println!("Two: {total}");
}
