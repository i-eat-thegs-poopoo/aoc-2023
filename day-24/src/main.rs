use std::cmp::Ordering;

mod one;
mod two;

fn main() {
    let (one, two) = utils::setup();
    one(|input| {
        let hailstones = parse(input);
        one::count_intersect(&hailstones)
    });
    two(|input| {
        let hailstones = parse(input);
        two::find_rock(&hailstones)
    });
}

fn parse(input: &str) -> Vec<([i128; 3], [i128; 3])> {
    let mut parser = utils::Parser::new(input);
    let mut hailstones = Vec::new();

    parser.sep_by("\n", |parser| {
        let px = parser.signed_int() as i128;
        parser.expect(", ");
        let py = parser.signed_int() as i128;
        parser.expect(", ");
        let pz = parser.signed_int() as i128;

        parser.expect(" @ ");

        parser.consume_match(" ");
        let vx = parser.signed_int() as i128;
        parser.expect(", ");
        parser.consume_match(" ");
        let vy = parser.signed_int() as i128;
        parser.expect(", ");
        parser.consume_match(" ");
        let vz = parser.signed_int() as i128;

        hailstones.push(([px, py, pz], [vx, vy, vz]));
    });

    hailstones
}

// Always simplified and non-zero denom
// Denom is always 1 when num is 0
#[derive(Clone, Copy, PartialEq, Eq)]
struct Fraction {
    num: i128,
    denom: u128,
}

impl Fraction {
    fn int(num: i128) -> Self {
        Self { num, denom: 1 }
    }

    fn gcd(mut a: u128, mut b: u128) -> u128 {
        while b != 0 {
            (a, b) = (b, a % b);
        }

        a
    }

    fn neg(self) -> Self {
        Self {
            num: -self.num,
            denom: self.denom,
        }
    }

    fn abs(self) -> Self {
        Self {
            num: self.num.abs(),
            denom: self.denom,
        }
    }

    fn flip(self) -> Self {
        assert_ne!(self.num, 0);

        Self {
            num: self.denom as i128 * self.num.signum(),
            denom: self.num.unsigned_abs(),
        }
    }

    fn add(self, other: Self) -> Self {
        let gcd = Self::gcd(self.denom, other.denom);

        let s_num = self.num * (other.denom / gcd) as i128;
        let o_num = other.num * (self.denom / gcd) as i128;

        let num = s_num + o_num;
        let denom = self.denom / gcd * other.denom;

        let gcd = Self::gcd(num.unsigned_abs(), denom);

        Self {
            num: num / gcd as i128,
            denom: denom / gcd,
        }
    }

    fn sub(self, other: Self) -> Self {
        other.neg().add(self)
    }

    fn mul(self, other: Self) -> Self {
        let (s_num, o_denom) = {
            let gcd = Self::gcd(self.num.unsigned_abs(), other.denom);
            (self.num / gcd as i128, other.denom / gcd)
        };

        let (o_num, s_denom) = {
            let gcd = Self::gcd(other.num.unsigned_abs(), self.denom);
            (other.num / gcd as i128, self.denom / gcd)
        };

        Self {
            num: s_num * o_num,
            denom: s_denom * o_denom,
        }
    }

    fn div(self, other: Self) -> Self {
        other.flip().mul(self)
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.add(other.neg()).num.cmp(&0))
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
