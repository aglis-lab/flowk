pub mod bezier;
pub mod elbow;
pub mod positions;
pub mod smooth_step;
pub mod straight;

use crate::{EdgePathResult, EdgePosition, EdgeType};

pub fn get_edge_path_result(pos: &EdgePosition, edge_type: EdgeType) -> EdgePathResult {
    match edge_type {
        EdgeType::Bezier | EdgeType::SimpleBezier => bezier::get_bezier_path(pos, None),
        EdgeType::SmoothStep => smooth_step::get_smooth_step_path(pos, None, None),
        EdgeType::Elbow => elbow::get_elbow_path(pos, None),
        EdgeType::Straight => straight::get_straight_path(pos),
        EdgeType::Step => smooth_step::get_step_path(pos, None),
    }
}
