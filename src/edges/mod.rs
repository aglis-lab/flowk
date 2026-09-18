//! Edge path construction and endpoint-position helpers.
//!
//! The module provides path builders for the supported [`crate::EdgeType`]
//! variants, including Bezier, straight, smooth-step, and elbow routing.

pub mod bezier;
pub mod elbow;
pub mod positions;
pub mod smooth_step;
pub mod straight;

use crate::{EdgePathResult, EdgePosition, EdgeType};

/// Build the path for an edge using the requested [`EdgeType`].
///
/// The returned points are in screen coordinates and include the source and
/// target endpoints. The result also contains the position used for an edge
/// label and the path center used by the renderer.
pub fn get_edge_path_result(pos: &EdgePosition, edge_type: EdgeType) -> EdgePathResult {
    match edge_type {
        EdgeType::Bezier | EdgeType::SimpleBezier => bezier::get_bezier_path(pos, None),
        EdgeType::SmoothStep => smooth_step::get_smooth_step_path(pos, None, None),
        EdgeType::Elbow => elbow::get_elbow_path(pos, None),
        EdgeType::Straight => straight::get_straight_path(pos),
        EdgeType::Step => smooth_step::get_step_path(pos, None),
    }
}
