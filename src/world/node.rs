use raylib::math::Vector3;
use smallvec::SmallVec;
use crate::world::Segment;

pub struct Node {
    position: Vector3,
    segments: SmallVec<Segment>
}