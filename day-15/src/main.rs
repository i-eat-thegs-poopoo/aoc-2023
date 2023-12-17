use std::iter;

fn main() {
    let (one, two) = utils::setup();
    one(|input| {
        let mut parser = utils::Parser::new(input);
        let mut sum = 0;

        parser.sep_by(",", |parser| {
            let str = parser.str_while(|c| *c != ',');
            sum += hash(str);
        });

        sum
    });
    two(|input| {
        let mut parser = utils::Parser::new(input);

        let mut map = HashMap {
            boxes: iter::repeat_with(|| Vec::new()).take(256).collect(),
        };

        parser.sep_by(",", |parser| {
            let label = parser.ident();
            let op = match parser.next().unwrap() {
                '=' => Operation::Insert(parser.int()),
                '-' => Operation::Remove,
                _ => panic!(),
            };

            process_step(label, op, &mut map);
        });

        let mut sum = 0;

        for (b, bucket) in map.boxes.into_iter().enumerate() {
            for (s, (_, focus)) in bucket.into_iter().enumerate() {
                sum += (b as u64 + 1) * (s as u64 + 1) * focus;
            }
        }

        sum
    });
}

struct HashMap<'a> {
    boxes: Vec<Vec<(&'a str, u64)>>,
}

enum Operation {
    Insert(u64),
    Remove,
}

fn hash(str: &str) -> u64 {
    let mut curr = 0;

    for char in str.chars() {
        let value = char as u64;
        curr += value;
        curr *= 17;
        curr %= 256;
    }

    curr
}

fn process_step<'a>(label: &'a str, op: Operation, map: &mut HashMap<'a>) {
    let bucket = hash(label) as usize;
    let bucket = &mut map.boxes[bucket];

    let lens = bucket.iter().position(|lens| lens.0 == label);

    match op {
        Operation::Insert(focus) => match lens {
            Some(lens) => bucket[lens].1 = focus,
            None => bucket.push((label, focus)),
        },
        Operation::Remove => {
            if let Some(lens) = lens {
                bucket.remove(lens);
            }
        }
    }
}
