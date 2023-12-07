use super::parse_line;

const MAX: (u64, u64, u64) = (12, 13, 14);

pub fn run(input: &str) {
    let mut sum = 0;

    for line in input.lines() {
        let (id, sets) = parse_line(line);

        if sets
            .iter()
            .all(|(r, g, b)| *r <= MAX.0 && *g <= MAX.1 && *b < MAX.2)
        {
            sum += id;
        }
    }

    println!("One: {sum}");
}
