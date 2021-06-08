use crate::world::Node;

pub fn successors(n: &Node) -> Vec<(Node, u64)> {
    let mut result = Vec::new();
    for sibling in n.siblings() {
        let pos = sibling.position();
        let node = Node::new(pos.x, pos.y, pos.z, Some(sibling.cost()));
        result.push((node, sibling.cost()))
    }
    result
}

pub fn heuristic(n: &Node, dest: &Node) -> f32 {
    let start = n.position();
    let end = dest.position();

    start.distance_to(end)
}

#[cfg(test)]
mod test_pathfinding {
    use crate::core::{heuristic, successors};
    use crate::world::Node;
    use pathfinding::directed::astar;

    #[test]
    fn pathfinding_zero_to_zero_path() {
        let start = Node::new(0.0, 0.0, 0.0, Some(0));
        let end = Node::new(0.0, 0.0, 0.0, Some(0));

        let path = astar::astar(
            &start,
            |n| successors(n),
            |n| heuristic(n, &end) as u64,
            |n| n == &end,
        );

        assert_eq!(path, Some((vec![Node::new(0.0, 0.0, 0.0, Some(0))], 0)))
    }

    #[test]
    fn pathfinding_no_path() {
        let start = Node::new(0.0, 0.0, 0.0, Some(0));
        let end = Node::new(0.0, 0.0, 10.0, Some(0));

        let path = astar::astar(
            &start,
            |n| successors(n),
            |n| heuristic(n, &end) as u64,
            |n| *n == end,
        );

        assert_eq!(path, None);
    }
}
