#![allow(dead_code)]
use crate::world::Node;
use raylib::math::Vector2;
use smallvec::SmallVec;

pub struct Segment {
    bend_position: Vector2,
    nodes: SmallVec<[Node; 2]>,
}
