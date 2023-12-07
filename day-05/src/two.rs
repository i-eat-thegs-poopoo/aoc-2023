use std::collections::HashMap;

pub fn run(input: &str) {
    let (seeds, maps) = parse(input);

    let mut path = Vec::new();
    find_path("seed", "location", &maps, &mut path);

    let mut ranges = seeds
        .chunks(2)
        .filter(|chunk| chunk[1] != 0)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1] - 1))
        .collect::<Vec<_>>();
    let mut untouched_buffer = Vec::new();
    let mut result_buffer = Vec::new();

    for window in path.windows(2) {
        let [from, to] = window else {
            panic!();
        };

        result_buffer.clear();

        for (dest, src, len) in maps.get(&(from, to)).unwrap().iter() {
            untouched_buffer.clear();

            for &(vmin, vmax) in ranges.iter() {
                let (smin, smax) = (*src, src + len - 1);
                let offset = dest - src;

                if vmax < smin || vmin > smax {
                    // no intersection between range and src
                    untouched_buffer.push((vmin, vmax));
                } else if vmin >= smin && vmax <= smax {
                    // range within src; 1 resultant range
                    result_buffer.push((vmin + offset, vmax + offset));
                } else if vmin < smin && vmax > smax {
                    // range contains src; 3 resultant ranges
                    untouched_buffer.push((vmin, smin - 1));
                    result_buffer.push((smin + offset, smax + offset));
                    untouched_buffer.push((smax + 1, vmax));
                } else if vmin < smin && vmax <= smax {
                    // range left-intersects src; 2 resultant ranges
                    untouched_buffer.push((vmin, smin - 1));
                    result_buffer.push((smin + offset, vmax + offset));
                } else if vmin >= smin && vmax > smax {
                    // range right-intersects src; 2 resultant ranges
                    result_buffer.push((vmin + offset, smax + offset));
                    untouched_buffer.push((smax + 1, vmax));
                } else {
                    unreachable!();
                }
            }

            ranges.clone_from(&untouched_buffer);
        }

        ranges.extend_from_slice(&result_buffer);
    }

    let mut lowest = i64::MAX;

    for (location, _) in ranges {
        if location < lowest {
            lowest = location;
        }
    }

    println!("Two: {lowest}");
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
