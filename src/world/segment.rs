use smallvec::SmallVec;
use crate::world::Node;
use raylib::math::Vector2;

pub struct Segment {
    bend_position: Vector2,
    nodes: SmallVec<Node>
}