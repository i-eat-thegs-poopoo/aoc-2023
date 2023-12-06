use std::collections::HashMap;

pub fn run(input: &str) {
    let (seeds, maps) = parse(input);

    let mut path = Vec::new();
    find_path("seed", "location", &maps, &mut path);

    let mut values = seeds;
    let mut buffer = Vec::new();

    for window in path.windows(2) {
        let [from, to] = window else {
            panic!();
        };

        buffer.clone_from(&values);

        for (dest, src, len) in maps.get(&(from, to)).unwrap().iter() {
            for (idx, value) in values.iter().enumerate() {
                if value >= src && *value < src + len {
                    buffer[idx] += dest - src;
                }
            }
        }

        values.clone_from(&buffer);
    }

    let mut lowest = i64::MAX;

    for location in values {
        if location < lowest {
            lowest = location;
        }
    }

    println!("One: {lowest}");
}

fn parse<'a>(input: &'a str) -> (Vec<i64>, HashMap<(&'a str, &'a str), Vec<(i64, i64, i64)>>) {
    let mut parser = utils::Parser::new(input);

    parser.expect("seeds: ");

    let mut seeds = Vec::new();
    parser.sep_by(" ", |parser| {
        let seed = parser.int() as i64;
        seeds.push(seed);
    });
    parser.expect("\n\n");

    let mut maps = HashMap::new();
    'outer: loop {
        let from = parser.ident();
        parser.expect("-to-");
        let to = parser.ident();
        parser.expect(" map:\n");

        let mut ranges = Vec::new();
        loop {
            if parser.consume_match("\n") {
                break;
            }

            let dest = parser.int() as i64;
            parser.expect(" ");
            let src = parser.int() as i64;
            parser.expect(" ");
            let len = parser.int() as i64;

            ranges.push((dest, src, len));

            if !parser.consume_match("\n") {
                maps.insert((from, to), ranges);
                break 'outer;
            }
        }

        maps.insert((from, to), ranges);
    }

    (seeds, maps)
}

// Returns true if the path is correct.
fn find_path<'a>(
    curr: &'a str,
    find: &'a str,
    maps: &HashMap<(&'a str, &'a str), Vec<(i64, i64, i64)>>,
    path: &mut Vec<&'a str>,
) -> bool {
    path.push(curr);

    if curr == find {
        return true;
    }

    for (from, to) in maps.keys() {
        if *from == curr && !path.contains(to) {
            if find_path(*to, find, maps, path) {
                return true;
            }
        }
    }

    assert_eq!(path.pop(), Some(curr));
    false
}
