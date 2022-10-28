use std::{collections::HashMap, cmp::Reverse};
use chronometer::Chronometer;
use priority_queue::PriorityQueue;
use crate::puzzle::*;
use slab_tree::*;

#[derive(Clone)]
pub struct Path {
    state: Puzzle,
    _action: ActionType,
    path_cost: u32,
}
impl Path {
    pub fn new(state: Puzzle, action: ActionType, path_cost: u32) -> Path {
        Path { state, _action: action, path_cost }
    }
}

pub struct Solution {
    state_path: Vec<Puzzle>,
    steps: u32,
}
impl Solution {
    pub fn print(&self) {
        println!("Solution: ");
        for puzzle in &self.state_path {
            println!("{}", puzzle.to_string());
        }
        println!("Steps: {}", self.steps);
    }
}

#[derive(Clone, Copy)]
pub enum Heuristic {
    Misplaced,
    OrthoDistance,
}

pub struct Agent {
    tree: Tree<Path>,
    frontier_prique: PriorityQueue<NodeId, Reverse<u32>>,
    frontier_hmap: HashMap<Puzzle, NodeId>,
    explored_hmap: HashMap<Puzzle, NodeId>,
    goal: Puzzle,
}
impl Agent {
    pub fn new(initial: Puzzle, goal: Puzzle) -> Agent {
        // Create root node, then tree.
        let path_cost = 0;
        let root = Path::new(initial.clone(), ActionType::None, path_cost);
        let tree = TreeBuilder::<Path>::new().with_root(root).build();
        let root_id = tree.root_id().unwrap();
        
        let mut frontier = PriorityQueue::<NodeId, Reverse<u32>>::new();
        let mut frontier_hash = HashMap::new();
        let explored = HashMap::new();

        // Add the root node to the frontier.
        frontier.push(root_id, Reverse(path_cost));
        frontier_hash.insert(initial, root_id);

        Agent { tree, frontier_prique: frontier, frontier_hmap: frontier_hash, explored_hmap: explored, goal }
    }

    pub fn uniform_cost_search(&mut self, heuristic: Heuristic, loop_count: u32, count: bool) -> Option<Solution> {
        let mut watch = Chronometer::new();
        watch.start();
        
        let mut counter = loop_count;
        while counter > 0 {
            if count { counter -= 1; }
            
            // Timer
            println!("{:6} {:.6} s", counter, watch.duration().unwrap().as_secs_f32());
            //println!("{}" counter);

            // Check if the frontier is empty.
            // Returns no solution if true, the cheapest path cost node if false.
            let parent_id = match self.frontier_prique.pop() {
                    Some(t) => t.0,
                    None => return None,
            };
            let parent = self.tree.get(parent_id)?.data().clone();
            self.frontier_hmap.remove(&parent.state);
            //parent.state.print("parent");
                
            // If the goal state has been reached then return the solution.
            if parent.state == self.goal {
                return self.solution(parent_id);
            }
                
            // Add the node's state to explored to show we've now reached that state.
            self.explored_hmap.insert(parent.state.clone(), parent_id);
                
            
            // Iterate through all action types.
            for action in [ActionType::Up, ActionType::Down, ActionType::Left, ActionType::Right].iter() {
                let state = parent.state.act(*action);
                let path_cost = parent.path_cost + get_heuristic(&state, &self.goal, heuristic) + 1;
                let child = Path::new(state, *action, path_cost);
                //child.state.print("child");

                // Search to see if new child's state is already in the frontier or explored.
                let child_in_frontier = self.frontier_hmap.contains_key(&child.state);
                let child_in_explored = self.explored_hmap.contains_key(&child.state);
                if !child_in_explored || !child_in_frontier {
                    // Insert the child node into the frontier since it's not there yet.
                    self.frontier_insert(parent_id, child);
                }
                // If child's state is in the frontier with a higher path cost then replace it.
                else if child_in_frontier {
                    let id = *self.frontier_hmap.get(&child.state)?;
                    let existing_path_cost = self.tree.get(id)?.data().path_cost;

                    if child.path_cost < existing_path_cost {
                        self.frontier_remove(id, &child.state);
                        self.frontier_insert(parent_id, child);
                    }
                }
            }
        }
        None
    }

    fn frontier_insert(&mut self, parent_id: NodeId, child: Path) {
        let mut parent = 
        match self.tree.get_mut(parent_id) {
            Some(t) => t,
            None => return,
        };
        let mut child_node = parent.append(child);
        let child_id = child_node.node_id();
        let child_data = child_node.data();
        let priority = Reverse(child_data.path_cost);
        self.frontier_prique.push(child_id, priority);
        self.frontier_hmap.insert(child_data.state.clone(), child_id);
    }

    fn frontier_remove(&mut self, node_id: NodeId, state: &Puzzle) {
        self.frontier_prique.remove(&node_id);
        self.frontier_hmap.remove(state);
        self.tree.remove(node_id, RemoveBehavior::DropChildren);
    }

    fn solution(&self, start: NodeId) -> Option<Solution> {
        let node = self.tree.get(start)?;
        let mut steps = 0;
        let mut state_path = vec![node.data().state.clone()];

        for parent in node.ancestors() {
            state_path.push(parent.data().state.clone());
            steps += 1;
        }

        Some(Solution { state_path, steps })
    }

}

fn get_heuristic(state: &Puzzle, goal: &Puzzle, heuristic: Heuristic) -> u32 {
    match heuristic {
        Heuristic::Misplaced => state.heuristic_misplaced(goal) as u32,
        Heuristic::OrthoDistance => state.heuristic_distances(goal),
    }
}
