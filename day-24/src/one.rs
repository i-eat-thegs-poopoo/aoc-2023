use super::*;

enum Line {
    Si { m: Fraction, b: Fraction },
    Vert { x: Fraction },
}

struct Hailstone {
    trajectory: Line,
    x_bound: (Ordering, Fraction),
    y_bound: (Ordering, Fraction),
}

pub fn count_intersect(hailstones: &[([i128; 3], [i128; 3])]) -> u64 {
    let hailstones = hailstones
        .iter()
        .map(|&(pos, velo)| to_slope_int(pos, velo))
        .collect::<Vec<_>>();

    let mut count = 0;

    for a in 0..hailstones.len() - 1 {
        for b in a + 1..hailstones.len() {
            if intersect(&hailstones[a], &hailstones[b]) {
                count += 1;
            }
        }
    }

    count
}

fn to_slope_int(pos: [i128; 3], velo: [i128; 3]) -> Hailstone {
    let zero = Fraction::int(0);
    let [px, py, _pz] = pos.map(Fraction::int);
    let [vx, vy, _vz] = velo.map(Fraction::int);

    let trajectory = if vx == zero {
        Line::Vert { x: px }
    } else {
        let slope = vy.mul(vx.flip());
        let y_int = py.add(slope.mul(px).neg());

        Line::Si { m: slope, b: y_int }
    };

    Hailstone {
        trajectory,
        x_bound: (vx.cmp(&zero), px),
        y_bound: (vy.cmp(&zero), py),
    }
}

fn in_bounds(coord: Fraction, (ord, against): (Ordering, Fraction)) -> bool {
    coord.cmp(&against) == ord
}

// x = (b2 - b1) / (m1 - m2)
fn intersect(a: &Hailstone, b: &Hailstone) -> bool {
    let min = Fraction::int(200_000_000_000_000);
    let max = Fraction::int(400_000_000_000_000);

    let (x, y) = match (&a.trajectory, &b.trajectory) {
        (Line::Si { m: m1, b: b1 }, Line::Si { m: m2, b: b2 }) => {
            let slopes = m1.add(m2.neg());

            if slopes == Fraction::int(0) {
                return false;
            }

            let x = b2.add(b1.neg()).mul(slopes.flip());
            let y = m1.mul(x).add(*b1);

            (x, y)
        }
        (Line::Si { m, b }, Line::Vert { x }) | (Line::Vert { x }, Line::Si { m, b }) => {
            let y = m.mul(*x).add(*b);
            (*x, y)
        }
        (Line::Vert { x: x1 }, Line::Vert { x: x2 }) => {
            if x1 == x2 {
                panic!("I dunno what to do for this");
            } else {
                return false;
            }
        }
    };

    x >= min
        && x <= max
        && y >= min
        && y <= max
        && in_bounds(x, a.x_bound)
        && in_bounds(x, b.x_bound)
        && in_bounds(y, a.y_bound)
        && in_bounds(y, b.y_bound)
}
