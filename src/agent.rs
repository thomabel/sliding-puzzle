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
    pub state_path: Vec<Puzzle>,
    pub steps: u32,
}
impl Solution {
    pub fn print(&self) {
        println!("Solution: ");
        for puzzle in &self.state_path {
            println!("{}", puzzle.to_string());
        }
        println!("Steps: {}\n", self.steps);
    }
}

#[derive(Clone, Copy)]
pub enum SearchStrategy {
    BestFirst,
    AStar,
}
impl ToString for SearchStrategy {
    fn to_string(&self) -> String {
        match self {
            SearchStrategy::BestFirst => "Best First",
            SearchStrategy::AStar => "AStar",
        }.to_string()
    }
}

#[derive(Clone, Copy)]
pub enum Heuristic {
    Misplaced,
    OrthoDistance,
    Inversions,
}
impl ToString for Heuristic {
    fn to_string(&self) -> String {
        match self {
            Heuristic::Misplaced => "Misplaced",
            Heuristic::OrthoDistance => "Orthogonal Distance",
            Heuristic::Inversions => "Inversions",
        }.to_string()
    }
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

    pub fn uniform_cost_search(&mut self, search_strategy: SearchStrategy, heuristic: Heuristic, loop_count: u32) -> Option<Solution> {
        let mut watch = Chronometer::new();
        let mut counter = loop_count;
        
        watch.start();
        self.tree.root().unwrap().data().state.print("Initial");

        while counter > 0 {
            counter -= 1;
            // Check if the frontier is empty.
            // Returns no solution if true, the cheapest path cost node if false.
            let parent_id = match self.frontier_prique.pop() {
                    Some(t) => t.0,
                    None => {
                        timer(counter, loop_count, &watch);
                        return None
                    },
            };
            let parent = self.tree.get(parent_id)?.data().clone();
            self.frontier_hmap.remove(&parent.state);
                
            // If the goal state has been reached then return the solution.
            if parent.state == self.goal {
                timer(counter, loop_count, &watch);
                return self.solution(parent_id);
            }
                
            // Add the node's state to explored to show we've now reached that state.
            self.explored_hmap.insert(parent.state.clone(), parent_id);
                
            
            // Iterate through all action types to add to the frontier.
            for action in [ActionType::Up, ActionType::Down, ActionType::Left, ActionType::Right].iter() {
                // First, create a new child node.
                let state = parent.state.act(*action);
                let h = get_heuristic(&state, &self.goal, heuristic);
                let g = 1;
                let path_cost = parent.path_cost + 
                    match search_strategy {
                        SearchStrategy::BestFirst => h,
                        SearchStrategy::AStar => h + g,
                    };
                let child = Path::new(state, *action, path_cost);

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
        Heuristic::Inversions => state.inversions(),
    }
}

fn timer(counter: u32, loop_count: u32, watch: &Chronometer) {
    println!("{:6} {:.6} s", loop_count - counter, watch.duration().unwrap().as_secs_f32());
}
