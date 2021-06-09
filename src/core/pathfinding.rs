use crate::world::Node;

/// Given a arbitrary node, returns all siblings of that node.
/// # Returns
/// A vector of all siblings. This could be a empty vector if the node has no siblings.
pub fn [;get_siblings(n: &Node) -> Vec<(Node, u64)> {
    n.siblings().iter().map(|sibling| {
        let pos = sibling.position();
        (Node::new(pos.x, pos.y, pos.z, Some(sibling.cost())), sibling.cost())
    }).collect::<Vec<_>>()
}

pub fn heuristic(n: &Node, dest: &Node) -> f32 {
    let start = n.position();
    let end = dest.position();

    start.distance_to(end)
}

#[cfg(test)]
mod test_pathfinding {
    use super::*;
    use crate::world::Node;
    use pathfinding::directed::astar;

    #[test]
    fn pathfinding_zero_to_zero_path() {
        let start = Node::new(0.0, 0.0, 0.0, Some(0));
        let end = Node::new(0.0, 0.0, 0.0, Some(0));

        let path = astar::astar(
            &start,
            |n| get_siblings(n),
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
            |n| get_siblings(n),
            |n| heuristic(n, &end) as u64,
            |n| *n == end,
        );

        assert_eq!(path, None);
    }

    #[test]
    fn pathfinding_zero_to_ten_path() {
        // start by initializing nodes
        let mut start = Node::new(0.0, 0.0, 0.0, None);
        let mut end = Node::new(10.0, 0.0, 0.0, None);

        // now create two intermediate nodes that are both siblings of each other
        // one of them is siblings with the start, and the other is siblings with the end
        let mut node_1 = Node::new(3.3, 0.0, 0.0, None);
        let mut node_2 = Node::new(6.6, 0.0, 0.0, None);

        // start <-> n1     n2     end
        start.add_sibling(&mut node_1);
        // start <-> n1 <-> n2     end
        node_1.add_sibling(&mut node_2);
        // start <-> n1 <-> n2 <-> end
        node_2.add_sibling(&mut end);
        // we now have each node being siblings with at least one other
        // this would of course be automated in a real game

        let path = astar::astar(
            &start,
            |n| get_siblings(n),
            |n| heuristic(n, &end) as u64,
            |n| *n == end,
        );

        dbg!(path);
    }
}
