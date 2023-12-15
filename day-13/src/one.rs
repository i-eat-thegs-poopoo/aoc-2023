use super::*;

pub fn find_axis_symmetry(
    AxisParams {
        lines,
        max,
        get_tile,
        terrain,
    }: AxisParams,
) -> Option<usize> {
    let mut symmetry_pts = None::<Vec<usize>>;

    for line in 0..lines {
        let pts = find_line_symmetry(get_tile, terrain, line);

        if let Some(ref mut symmetry_pts) = symmetry_pts {
            symmetry_pts.retain(|pt| pts.contains(pt) && *pt < max);
        } else {
            symmetry_pts = Some(pts);
        }
    }

    symmetry_pts.unwrap().get(0).copied()
}
