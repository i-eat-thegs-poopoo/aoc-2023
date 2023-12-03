use std::collections::HashSet;

struct Number {
    value: u32,
    x: (usize, usize),
    y: usize,
}

pub fn run(input: &str) {
    let (numbers, symbols) = parse_nums_symbols(input);
    let mut sum = 0;

    for num in numbers {
        for x in num.x.0 - 1..=num.x.1 + 1 {
            for y in num.y - 1..=num.y + 1 {
                if symbols.contains(&(x, y)) {
                    sum += num.value;
                }
            }
        }
    }

    println!("One: {sum}");
}

fn parse_nums_symbols(input: &str) -> (Vec<Number>, HashSet<(usize, usize)>) {
    let mut buffer = String::new();

    let mut numbers = Vec::new();
    let mut symbols = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        let mut chars = line.chars().enumerate().peekable();

        loop {
            match chars.peek() {
                Some((_, '.')) => {
                    chars.next();
                }
                Some((start, '0'..='9')) => {
                    let start = *start;
                    let mut end = start;
                    buffer.clear();
                    
                    while let Some((x, c)) = chars.next_if(|(_, c)| c.is_ascii_digit()) {
                        end = x;
                        buffer.push(c);
                    }

                    let value = buffer.parse().unwrap();
                    numbers.push(Number {
                        value,
                        x: (start + 1, end + 1),
                        y: y + 1,
                    });
                }
                Some((x, _)) => {
                    symbols.insert((x + 1, y + 1));
                    chars.next();
                }
                None => break,
            }
        }
    }

    (numbers, symbols)
}