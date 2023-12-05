pub fn run(input: &str) {
    let mut parser = utils::Parser::new(input);
    let mut sum = 0;

    parser.sep_by("\n", |parser| {
        parser.expect("Card");
        parser.str_while(|c| *c == ' ');
        parser.int();
        parser.expect(":");

        let mut winning = Vec::new();
        let mut points = 0;

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
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }

        sum += points;
    });

    println!("One: {sum}");
}