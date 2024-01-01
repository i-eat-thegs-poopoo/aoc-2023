use super::*;

/*

rock_x(t) = r_x0 + v_rx * t
rock_y(t) = r_y0 + v_ry * t
hail_x(t) = h_x0 + v_hx * t
hail_y(t) = h_y0 + v_hy * t

... Rewrite ...

l r_x0 + m r_y0 + n v_rx + o v_ry = p
... where ...
l = v_ay - v_by
m = -v_ax + v_bx
n = -a_y0 + b_y0
o = a_x0 - b_x0
p = a_x0 v_ay - a_y0 v_ax - b_x0 v_by + b_y0 v_bx

... Solve for x and y ...

[ l1 m1 n1 o1 ] [ r_x0 ] = [ p1 ]
[ l2 m2 n2 o2 ] [ r_y0 ] = [ p2 ]
[ l3 m3 n3 o3 ] [ v_rx ] = [ p3 ]
[ l4 m4 n4 o4 ] [ v_ry ] = [ p4 ]

... Solve for z ...

rock_z(t) = r_z0 + v_rz * t
hail_z(t) = h_z0 + v_hz * t

l r_x0 + m r_z0 + n v_rx + o v_rz = p
m r_z0 + o v_rz = q
... where ...
q = p - l r_x0 - n v_rx

[ m1 o1 ] [ r_z0 ] = [ q1 ]
[ m2 o2 ] [ v_rz ] = [ q2 ]

*/

pub fn find_rock(hailstones: &[([i128; 3], [i128; 3])]) -> i128 {
    let hailstones = hailstones
        .iter()
        .take(5)
        .map(|&(pos, velo)| {
            let [px, py, pz] = pos.map(Fraction::int);
            let [vx, vy, vz] = velo.map(Fraction::int);

            [
                Axis { v: vx, p: px },
                Axis { v: vy, p: py },
                Axis { v: vz, p: pz },
            ]
        })
        .collect::<Vec<_>>();

    let [px, py, vx, _] = {
        let mut coeffs = [[Fraction::int(0); 4]; 4];
        let mut consts = [Fraction::int(0); 4];

        for (i, window) in hailstones.windows(2).take(4).enumerate() {
            let &[[ax, ay, _], [bx, by, _]] = window else {
                panic!();
            };

            let (lhs, rhs) = make_xy_eq(ax, ay, bx, by);
            coeffs[i] = lhs;
            consts[i] = rhs;
        }

        gauss_elim(&mut coeffs, &mut consts)
    };

    let [pz, _] = {
        let mut coeffs = [[Fraction::int(0); 2]; 2];
        let mut consts = [Fraction::int(0); 2];
        let x = Axis { v: vx, p: px };

        for (i, window) in hailstones.windows(2).take(2).enumerate() {
            let &[[ax, _, az], [bx, _, bz]] = window else {
                panic!();
            };

            let (lhs, rhs) = make_z_eq(ax, az, bx, bz, x);
            coeffs[i] = lhs;
            consts[i] = rhs;
        }

        gauss_elim(&mut coeffs, &mut consts)
    };

    assert_eq!(px.denom, 1);
    assert_eq!(py.denom, 1);
    assert_eq!(pz.denom, 1);

    px.add(py).add(pz).num
}

#[derive(Clone, Copy)]
struct Axis {
    v: Fraction,
    p: Fraction,
}

fn make_xy_eq(ax: Axis, ay: Axis, bx: Axis, by: Axis) -> ([Fraction; 4], Fraction) {
    let p = ay.v.sub(by.v);
    let q = bx.v.sub(ax.v);
    let r = by.p.sub(ay.p);
    let s = ax.p.sub(bx.p);
    let k =
        ax.p.mul(ay.v)
            .sub(ay.p.mul(ax.v))
            .sub(bx.p.mul(by.v))
            .add(by.p.mul(bx.v));

    ([p, q, r, s], k)
}

fn make_z_eq(ax: Axis, az: Axis, bx: Axis, bz: Axis, x: Axis) -> ([Fraction; 2], Fraction) {
    let ([p, q, r, s], k) = make_xy_eq(ax, az, bx, bz);
    ([q, s], k.sub(x.p.mul(p)).sub(x.v.mul(r)))
}

fn gauss_elim<const N: usize>(
    coeffs: &mut [[Fraction; N]; N],
    consts: &mut [Fraction; N],
) -> [Fraction; N] {
    let zero = Fraction::int(0);

    for k in 0..N {
        let (i_max, i_val) = (k..N)
            .map(|i| (i, coeffs[i][k].abs()))
            .max_by_key(|(_, val)| *val)
            .unwrap();

        if i_val == zero {
            panic!("matrix not invertible");
        }

        coeffs.swap(k, i_max);
        consts.swap(k, i_max);

        for i in k + 1..N {
            let factor = coeffs[i][k].div(coeffs[k][k]);
            coeffs[i][k] = zero;

            for j in k + 1..N {
                coeffs[i][j] = coeffs[i][j].sub(coeffs[k][j].mul(factor));
            }

            consts[i] = consts[i].sub(consts[k].mul(factor));
        }
    }

    let mut sols = [zero; N];

    for i in (0..N).rev() {
        let mut sum = zero;

        for j in i + 1..N {
            sum = sols[j].mul(coeffs[i][j]).add(sum);
        }

        sols[i] = consts[i].sub(sum).div(coeffs[i][i]);
    }

    sols
}
