use std::collections::HashMap;

struct Number {
    value: u64,
    x: (usize, usize),
    y: usize,
}

pub fn run(input: &str) {
    let (numbers, mut gears) = parse_nums_gears(input);

    for num in numbers {
        for x in num.x.0 - 1..=num.x.1 + 1 {
            for y in num.y - 1..=num.y + 1 {
                if let Some(count) = gears.get_mut(&(x, y)) {
                    count.push(num.value);
                }
            }
        }
    }

    let mut sum = 0;

    for nums in gears.values() {
        if nums.len() == 2 {
            sum += nums[0] * nums[1];
        }
    }

    println!("Two: {sum}");
}

fn parse_nums_gears(input: &str) -> (Vec<Number>, HashMap<(usize, usize), Vec<u64>>) {
    let mut buffer = String::new();

    let mut numbers = Vec::new();
    let mut gears = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        let mut chars = line.chars().enumerate().peekable();

        loop {
            match chars.peek() {
                Some((x, '*')) => {
                    gears.insert((x + 1, y + 1), Vec::new());
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
                Some(_) => {
                    chars.next();
                }
                None => break,
            }
        }
    }

    (numbers, gears)
}
