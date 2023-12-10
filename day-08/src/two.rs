use std::{collections::HashMap, str::Chars};

#[derive(Debug)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
    gen: u64,
    past: Vec<(usize, u64, usize)>,
}

#[derive(Debug)]
struct Path<'a> {
    init_node: &'a str,
    leading: Vec<u64>,
    cycle: Vec<u64>,
    cycle_start: u64,
    cycle_len: u64,
}

pub fn run(input: &str) {
    let time = std::time::Instant::now();
    let step = calc(input);
    println!("Two: {step}");
    println!("{:?}", time.elapsed());
}

fn calc(input: &str) -> u64 {
    let (instrs, mut map, mut paths) = parse(input);

    let mut gen = 0;
    for path in paths.iter_mut() {
        find_z_steps(path, &mut gen, &mut map, instrs.clone());
    }

    // Check leading acyclic
    let leading_and_cycle = paths
        .iter()
        .flat_map(|path| path.leading.iter().chain(path.cycle.iter()));

    let mut acyclic_overlap = HashMap::new();
    for z_step in leading_and_cycle {
        let count = acyclic_overlap
            .entry(*z_step)
            .and_modify(|z_step| *z_step += 1)
            .or_insert(1);

        if *count == paths.len() {
            return *z_step;
        }
    }

    // Check cyclic
    let mut paths = paths.iter().map(|path| {
        path.cycle
            .iter()
            .map(|z_step| (path.cycle_len, *z_step))
            .collect::<Vec<_>>()
    });
    let mut intersect = paths.next().unwrap();
    let mut buffer = Vec::new();

    for path in paths {
        buffer.clear();

        for (int_per, int_shift) in intersect.drain(..) {
            for &(path_per, path_shift) in path.iter() {
                let (gcd, s, _) = extended_gcd(int_per, path_per);
                let rel_shift = int_shift.abs_diff(path_shift);

                if rel_shift % gcd != 0 {
                    continue;
                }

                let factor = rel_shift / gcd;
                let intersect = s * factor * int_per + int_shift;
                let period = int_per * path_per / gcd;

                buffer.push((period, intersect % period));
            }
        }

        intersect.extend(buffer.drain(..));
    }

    *intersect
        .iter()
        .map(|(per, shift)| if *shift == 0 { per } else { shift })
        .min()
        .unwrap()
}

fn parse<'a>(input: &'a str) -> (Chars<'a>, HashMap<&'a str, Node<'a>>, Vec<Path<'a>>) {
    let mut parser = utils::Parser::new(input);

    let instrs = parser.str_while(char::is_ascii_alphabetic).chars();
    parser.expect("\n\n");

    let mut map = HashMap::new();
    let mut paths = Vec::new();

    parser.sep_by("\n", |parser| {
        let node = parser.alphanumeric();
        parser.expect(" = (");
        let left = parser.alphanumeric();
        parser.expect(", ");
        let right = parser.alphanumeric();
        parser.expect(")");

        map.insert(
            node,
            Node {
                left,
                right,
                gen: 0,
                past: Vec::new(),
            },
        );

        if node.chars().last().unwrap() == 'A' {
            paths.push(Path {
                init_node: node,
                leading: Vec::new(),
                cycle: Vec::new(),
                cycle_start: 0,
                cycle_len: 0,
            });
        }
    });

    (instrs, map, paths)
}

fn find_z_steps<'a>(
    path: &mut Path<'a>,
    curr_gen: &mut u64,
    map: &mut HashMap<&'a str, Node<'a>>,
    instrs: impl Iterator<Item = char> + Clone,
) {
    *curr_gen += 1;
    let curr_gen = *curr_gen;

    let mut curr = path.init_node;
    let mut step = 0;

    for (idx, instr) in instrs.clone().enumerate().cycle() {
        let node = map.get_mut(curr).unwrap();

        if node.gen != curr_gen {
            node.gen = curr_gen;
            node.past.clear();
        }

        if let Some((_, cycle_start, split_at)) = node.past.iter().find(|(i, ..)| *i == idx) {
            path.cycle = path.leading.split_off(*split_at);
            path.cycle_start = *cycle_start;
            path.cycle_len = step - path.cycle_start;

            return;
        }

        node.past.push((idx, step, path.leading.len()));

        if curr.chars().last().unwrap() == 'Z' {
            path.leading.push(step);
        }

        match instr {
            'L' => curr = node.left,
            'R' => curr = node.right,
            _ => panic!(),
        }

        step += 1;
    }
}

/*

Find ints a, b such that:
a * Ta + xa = b * Tb + xb

a * Ta - b * Tb = xb - xa
s * Ta - t * Tb = gcd(Ta, Tb)
z = |xb - xa|/gcd(Ta, Tb) where z must be an integer
zs * Ta - zt * Tb = xb - xa
a = zs
b = zt

zs * Ta + xa = intersection
lcm(Ta, Tb) = Ta * Tb / gcd(Ta, Tb) = combined period

*/

fn extended_gcd(a: u64, b: u64) -> (u64, u64, u64) {
    let (mut old_r, mut r) = (a as i64, b as i64);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }

    (
        old_r.unsigned_abs(),
        old_s.unsigned_abs(),
        old_t.unsigned_abs(),
    )
}
