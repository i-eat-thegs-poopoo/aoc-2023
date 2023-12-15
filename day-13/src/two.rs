use super::*;
use std::collections::HashMap;

pub fn find_axis_fixable(
    AxisParams {
        lines,
        max,
        get_tile,
        terrain,
    }: AxisParams,
) -> Option<usize> {
    let axis = find_axis_asymmetry(lines, max, get_tile, terrain);

    for (pt, unmatched) in axis {
        if check_line_fixable(pt, get_tile, terrain, unmatched).is_some() {
            return Some(pt);
        }
    }

    None
}

fn find_axis_asymmetry(
    lines: usize,
    max: usize,
    get_tile: fn(usize, &Terrain, usize) -> Option<Tile>,
    terrain: &Terrain,
) -> Vec<(usize, usize)> {
    let mut symmetry_counter = HashMap::new();

    for line in 0..lines {
        let pts = find_line_symmetry(get_tile, terrain, line);

        for pt in pts.into_iter().filter(|pt| *pt < max) {
            symmetry_counter
                .entry(pt)
                .and_modify(|lines: &mut Vec<usize>| lines.push(line))
                .or_insert_with(|| vec![line]);
        }
    }

    let mut symmetry_pts = Vec::new();

    for (pt, matched_lines) in symmetry_counter {
        if matched_lines.len() == lines - 1 {
            let unmatched_line = (0..lines)
                .zip(matched_lines.iter().copied())
                .find(|(line, matched)| line != matched)
                .map(|(line, _)| line)
                .unwrap_or(lines - 1);

            symmetry_pts.push((pt, unmatched_line));
        }
    }

    symmetry_pts
}

fn check_line_fixable(
    symmetry_pt: usize,
    get_tile: fn(usize, &Terrain, usize) -> Option<Tile>,
    terrain: &Terrain,
    parent: usize,
) -> Option<()> {
    let (mut left, mut right) = (symmetry_pt - 1, symmetry_pt);

    loop {
        let left_tile = get_tile(left, terrain, parent)?;
        let right_tile = get_tile(right, terrain, parent)?;

        if left_tile != right_tile {
            return Some(());
        }

        left = left.checked_sub(1)?;
        right += 1;
    }
}
