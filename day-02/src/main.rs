use std::str::Chars;

mod one;
mod two;

fn main() {
    let input = utils::read_input();
    one::run(&input);
    two::run(&input);
}

fn parse_line(line: &str) -> (u32, Vec<(u32, u32, u32)>) {
    let mut chars = line.chars();

    chars.by_ref().take(5).for_each(drop);
    let id = parse_int(&mut chars);

    chars.next(); // space

    let mut sets = Vec::new();

    loop {
        let mut set = (0, 0, 0);

        loop {
            let count = parse_int(&mut chars);
            let color = match chars.next().unwrap() {
                'r' => &mut set.0,
                'g' => &mut set.1,
                'b' => &mut set.2,
                _ => panic!(),
            };

            *color = count;

            match chars.find(|&c| c == ',' || c == ';') {
                Some(',') => {
                    chars.next(); // space
                }
                Some(';') => break,
                _ => {
                    sets.push(set);
                    return (id, sets);
                }
            }
        }

        sets.push(set);
        chars.next(); // space
    }
}

fn parse_int(chars: &mut Chars) -> u32 {
    chars
        .by_ref()
        .take_while(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .unwrap()
}