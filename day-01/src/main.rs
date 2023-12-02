use std::fs;

fn main() {
    let input = utils::read_input();
    one(&input);
    two(&input);
}

fn one(input: &str) {
    let mut sum = 0_u32;

    for line in input.lines() {
        let mut iter = line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10))
            .flatten();

        let first = iter.next().unwrap();
        let last = iter.last().unwrap_or(first);

        sum += first * 10 + last;
    }

    println!("One: {sum}");
}

fn two(input: &str) {
    const PATTERNS: [(&str, u32); 10] = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut sum = 0_u32;

    for line in input.lines() {
        let patterns = PATTERNS
            .into_iter()
            .map(|(pat, val)| (pat.chars(), val))
            .collect::<Vec<_>>();
        let first = find(line.chars(), &patterns);

        let patterns = PATTERNS
            .into_iter()
            .map(|(pat, val)| (pat.chars().rev(), val))
            .collect::<Vec<_>>();
        let last = find(line.chars().rev(), &patterns);

        sum += first * 10 + last;
    }

    println!("Two: {sum}");
}

fn find(
    mut input: impl Iterator<Item = char> + Clone,
    patterns: &[(impl Iterator<Item = char> + Clone, u32)],
) -> u32 {
    loop {
        let digit = input.clone().next().and_then(|c| c.to_digit(10));

        if let Some(digit) = digit {
            return digit;
        }

        for (pat, val) in patterns {
            let iter = input.clone();
            if iter.zip(pat.clone()).all(|(i, p)| i == p) {
                return *val;
            }
        }

        input.next();
    }
}
