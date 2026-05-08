use std::collections::BinaryHeap;
use std::collections::HashMap;
use crate::common::heuristics::Heuristic;
use crate::space::Space;
use crate::common::min_heap::MinHeapNode;

pub struct AStar<H> {
    heuristic: H,
}


impl<H> AStar<H> {
    pub fn new(heuristic: H) -> Self {
        Self {
            heuristic,
        }
    }

    pub fn find_path<S>(
        &self,
        space: &S,
        source: &S::Config,
        goal: &S::Config,
    ) -> Option<Vec<S::Config>>
    where
        S: Space,
        H: Heuristic<S::Config>,
    {
        // Check that source and goal are in the bounds of the space
        if !space.in_bounds(source) || !space.in_bounds(goal) {
            return None;
        }

        // check that source and goal are in free space
        if space.is_occupied(source) || space.is_occupied(goal) {
            return None;
        }

        let mut frontier: BinaryHeap<MinHeapNode<S::Config>> = BinaryHeap::new();
        let mut came_from: HashMap<S::Config, S::Config> = HashMap::new();
        let mut cost_so_far: HashMap<S::Config, f64> = HashMap::new();

        frontier.push(MinHeapNode { cost: 0.0, config: source.clone() });
        cost_so_far.insert(source.clone(), 0.0);

        // Loop while the frontier is not empty
        while let Some(MinHeapNode { config: current, .. }) = frontier.pop() {
            // If current is the goal we return the path
            if &current == goal {
                let mut path = vec![current.clone()];
                let mut cur = current;

                // Construct the path from came_from and reverse it
                while let Some(prev) = came_from.get(&cur) {
                    path.push(prev.clone());
                    cur = prev.clone();
                }
                path.reverse();
                return Some(path);
            }

            for next in space.get_neighbors(&current) {
                let new_cost = cost_so_far[&current] + space.get_cost(&current, &next);
                if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next] {
                    cost_so_far.insert(next.clone(), new_cost);
                    let priority = new_cost + self.heuristic.estimate(&next, goal);
        
                    frontier.push(MinHeapNode { cost: priority, config: next.clone() });
                    came_from.insert(next, current.clone());
                }
            }
        }

        None
    }
}