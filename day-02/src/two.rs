use super::parse_line;

pub fn run(input: &str) {
    let mut sum = 0;

    for line in input.lines() {
        let mut min = (0, 0, 0);
        let (_, sets) = parse_line(line);

        for (r, g, b) in sets {
            min.0 = min.0.max(r);
            min.1 = min.1.max(g);
            min.2 = min.2.max(b);
        }

        sum += min.0 * min.1 * min.2;
    }

    println!("Two: {sum}");
}
