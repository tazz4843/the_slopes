use raylib::math::Vector3;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};
use std::sync::Arc;

#[derive(Debug)]
pub struct Node {
    /// The world position of this node.
    position: Vector3,
    /// All siblings of this node.
    siblings: HashSet<Arc<Node>>,
    /// The cost of pathfinding across this node.
    cost: AtomicU64,
}

impl Node {
    pub fn new(x: f32, y: f32, z: f32, cost: Option<u64>) -> Self {
        Self {
            position: Vector3::new(x, y, z),
            siblings: HashSet::new(),
            cost: AtomicU64::new(match cost {
                Some(c) => c,
                None => 1,
            }),
        }
    }

    pub fn siblings(&self) -> HashSet<Arc<Node>> {
        self.siblings.clone()
    }

    pub fn cost(&self) -> u64 {
        self.cost.load(AtomicOrdering::Relaxed)
    }

    pub fn position(&self) -> Vector3 {
        self.position.clone()
    }

    pub fn add_sibling(&mut self, other: &mut Self) {
        self.siblings.insert(Arc::new(other.clone()));
        other.siblings.insert(Arc::new(self.clone()));
    }

    pub fn remove_sibling(&mut self, other: &mut Self) {
        self.siblings.remove(other);
        other.siblings.remove(self);
    }
}

impl Eq for Node {}

impl Ord for Node {
    // same inline comments as for PartialOrd

    #[inline(always)]
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.partial_cmp(rhs)
            .expect("comparison of two `Node`s failed")
    }
}

impl PartialOrd for Node {
    // this will most likely get called millions (if not billions, or possibly trillions) of times
    // during a game so all of them are inlined

    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.eq(other) {
            Some(Ordering::Equal)
        } else if self.gt(other) {
            Some(Ordering::Greater)
        } else if self.lt(other) {
            Some(Ordering::Less)
        } else {
            None
        }
    }

    #[inline(always)]
    fn lt(&self, other: &Self) -> bool {
        self.position.x > other.position.x && self.position.y > other.position.y
    }

    #[inline(always)]
    fn le(&self, other: &Self) -> bool {
        self.position.x >= other.position.x && self.position.y >= other.position.y
    }

    #[inline(always)]
    fn gt(&self, other: &Self) -> bool {
        self.position.x < other.position.x && self.position.y < other.position.y
    }

    #[inline(always)]
    fn ge(&self, other: &Self) -> bool {
        self.position.x <= other.position.x && self.position.y <= other.position.y
    }
}

impl PartialEq for Node {
    // same inline comments as for PartialOrd

    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.position.x == other.position.x
            && self.position.y == other.position.y
            && self.position.z == other.position.z
    }

    #[inline(always)]
    fn ne(&self, other: &Self) -> bool {
        self.position.x != other.position.x
            && self.position.y != other.position.y
            && self.position.z != other.position.z
    }
}

// we need to implement it ourselves because the Vector3 doesn't implement Hash
impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.position.x.trunc() as u64);
        state.write_u64(self.position.x.fract() as u64);

        state.write_u64(self.position.y.trunc() as u64);
        state.write_u64(self.position.y.fract() as u64);

        state.write_u64(self.position.z.trunc() as u64);
        state.write_u64(self.position.z.fract() as u64);

        // commented out because self.eq doesn't check if cost is the same
        // state.write_u64(self.cost());
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
            siblings: self.siblings.clone(),
            cost: AtomicU64::new(self.cost()),
        }
    }
}
