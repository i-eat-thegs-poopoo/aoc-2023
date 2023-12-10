use std::{collections::HashMap, iter};

pub fn run(input: &str) {
    let mut parser = utils::Parser::new(input);

    let instrs = parser.str_while(char::is_ascii_alphabetic).chars();
    let instrs = iter::repeat(instrs).flatten();

    parser.expect("\n\n");

    let mut map = HashMap::new();
    parser.sep_by("\n", |parser| {
        let node = parser.alphanumeric();
        parser.expect(" = (");
        let left = parser.alphanumeric();
        parser.expect(", ");
        let right = parser.alphanumeric();
        parser.expect(")");

        map.insert(node, (left, right));
    });

    let mut curr = "AAA";
    let mut steps = 0;

    for instr in instrs {
        let (left, right) = map.get(curr).unwrap();
        match instr {
            'L' => curr = *left,
            'R' => curr = *right,
            _ => panic!(),
        }

        steps += 1;

        if curr == "ZZZ" {
            break;
        }
    }

    println!("One: {steps}");
}
