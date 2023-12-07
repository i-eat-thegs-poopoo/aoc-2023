mod one;
mod two;

fn main() {
    let input = utils::read_input();
    one::run(&input);
    two::run(&input);
}

fn parse_line(line: &str) -> (u64, Vec<(u64, u64, u64)>) {
    let mut parser = utils::Parser::new(line);

    parser.expect("Game ");
    let id = parser.int();
    parser.expect(": ");

    let mut sets = Vec::new();
    parser.sep_by("; ", |parser| {
        let mut colors = (0, 0, 0);
        parser.sep_by(", ", |parser| {
            let count = parser.int();
            parser.expect(" ");

            match parser.ident() {
                "red" => colors.0 = count,
                "green" => colors.1 = count,
                "blue" => colors.2 = count,
                _ => panic!(),
            }
        });
        sets.push(colors);
    });

    (id, sets)
}
