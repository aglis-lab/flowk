use egui::{Pos2, pos2};
use smallvec::{SmallVec, smallvec};

use crate::{EdgePathResult, EdgePosition, Position};

/// How far a path travels straight out from a node before it's allowed to
/// bend. Keeps the connector from turning immediately at the node's edge.
const STUB: f32 = 80.0;

pub fn get_elbow_path(pos: &EdgePosition, _curvature: Option<f32>) -> EdgePathResult {
    let source = pos2(pos.source_x, pos.source_y);
    let target = pos2(pos.target_x, pos.target_y);

    let source_horizontal = matches!(pos.source_pos, Position::Left | Position::Right);
    let target_horizontal = matches!(pos.target_pos, Position::Left | Position::Right);

    let dx = target.x - source.x;
    let dy = target.y - source.y;

    let mut points: SmallVec<[Pos2; 8]> = smallvec![source];

    match (source_horizontal, target_horizontal) {
        // Both ports face left/right: bend along a vertical line sitting
        // between the two horizontal stubs.
        (true, true) => {
            if dy.abs() < 1.0 {
                // Same row — a straight line already looks orthogonal.
                points.push(target);
            } else {
                let source_stub_x = source.x + stub_dir(pos.source_pos) * STUB;
                let target_stub_x = target.x + stub_dir(pos.target_pos) * STUB;
                let mid_x = (source_stub_x + target_stub_x) / 2.0;

                points.push(pos2(mid_x, source.y));
                points.push(pos2(mid_x, target.y));
                points.push(target);
            }
        }
        // Both ports face top/bottom: mirror of the above, bending along a
        // horizontal line.
        (false, false) => {
            if dx.abs() < 1.0 {
                points.push(target);
            } else {
                let source_stub_y = source.y + stub_dir_v(pos.source_pos) * STUB;
                let target_stub_y = target.y + stub_dir_v(pos.target_pos) * STUB;
                let mid_y = (source_stub_y + target_stub_y) / 2.0;

                points.push(pos2(source.x, mid_y));
                points.push(pos2(target.x, mid_y));
                points.push(target);
            }
        }
        // Source faces left/right, target faces top/bottom: a single
        // right-angle bend at the corner where the two extensions meet.
        (true, false) => {
            points.push(pos2(target.x, source.y));
            points.push(target);
        }
        // Source faces top/bottom, target faces left/right: same idea,
        // corner taken from the other axis.
        (false, true) => {
            points.push(pos2(source.x, target.y));
            points.push(target);
        }
    }

    let label_pos = match points.as_slice() {
        [a, b] => pos2((a.x + b.x) / 2.0, (a.y + b.y) / 2.0),
        [_, corner, _] => *corner,
        [_, a, b, _] => pos2((a.x + b.x) / 2.0, (a.y + b.y) / 2.0),
        _ => target,
    };

    let (min_x, max_x) = points.iter().fold((f32::MAX, f32::MIN), |(lo, hi), p| {
        (lo.min(p.x), hi.max(p.x))
    });
    let (min_y, max_y) = points.iter().fold((f32::MAX, f32::MIN), |(lo, hi), p| {
        (lo.min(p.y), hi.max(p.y))
    });
    let center_x = (min_x + max_x) / 2.0;
    let center_y = (min_y + max_y) / 2.0;

    EdgePathResult {
        points,
        label_pos,
        center_x,
        center_y,
    }
}

/// +1 for a port that exits rightward, -1 for one that exits leftward.
fn stub_dir(p: Position) -> f32 {
    match p {
        Position::Right => 1.0,
        Position::Left => -1.0,
        _ => 0.0,
    }
}

/// +1 for a port that exits downward, -1 for one that exits upward.
fn stub_dir_v(p: Position) -> f32 {
    match p {
        Position::Bottom => 1.0,
        Position::Top => -1.0,
        _ => 0.0,
    }
}
