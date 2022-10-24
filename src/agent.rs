use std::{collections::HashMap, cmp::Reverse};
use priority_queue::PriorityQueue;
use crate::puzzle::*;
use slab_tree::*;

#[derive(Clone)]
pub struct Node {
    state: Puzzle,
    _action: ActionType,
    path_cost: u32,
}
impl Node {
    pub fn new(state: Puzzle, action: ActionType, path_cost: u32) -> Node {
        Node { state, _action: action, path_cost }
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
            println!("{}", puzzle.read_map());
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
    tree: Tree<Node>,
    frontier: PriorityQueue<NodeId, Reverse<u32>>,
    frontier_hash: HashMap<Puzzle, NodeId>,
    explored: HashMap<Puzzle, NodeId>,
    goal: Puzzle,
}
impl Agent {
    pub fn new(initial: Puzzle, goal: Puzzle, heuristic: Heuristic) -> Agent {
        //let path_cost = get_heuristic(&initial, &goal, heuristic);
        let path_cost = 0;
        let root = Node::new(initial.clone(), ActionType::None, path_cost);
        let tree = TreeBuilder::<Node>::new().with_root(root).build();
        
        let mut frontier = PriorityQueue::<NodeId, Reverse<u32>>::new();
        let root_id = tree.root_id().unwrap();
        frontier.push(root_id, Reverse(path_cost));

        let mut frontier_hash = HashMap::new();
        frontier_hash.insert(initial, root_id);

        let explored = HashMap::new();

        Agent { tree, frontier, frontier_hash, explored, goal }
    }

    pub fn uniform_cost_search(&mut self, heuristic: Heuristic, loop_count: u32, count: bool) -> Option<Solution> {
        let mut counter = loop_count;
        while counter > 0 {
            if count { counter -= 1; }
            
            // Check if the frontier is empty.
            // Returns no solution if true, the cheapest path cost node if false.
            let parent_id = match self.frontier.pop() {
                    Some(t) => t.0,
                    None => return None,
            };
            let parent = self.tree.get(parent_id)?.data().clone();
            self.frontier_hash.remove(&parent.state);
            //parent.state.print("parent");
                
            // If the goal state has been reached then return the solution.
            if parent.state.goal_test(&self.goal) {
                return self.solution(parent_id);
            }
                
            // Add the nodes state to explored to show we've now reached that state.
            self.explored.insert(parent.state.clone(), parent_id);
                
            
            // Iterate through all action types.
            for action in [ActionType::Up, ActionType::Down, ActionType::Left, ActionType::Right].iter() {
                let state = parent.state.act(*action);
                let priority = get_heuristic(&state, &self.goal, heuristic);
                let path_cost = parent.path_cost + priority;
                let child = Node::new(state, *action, path_cost);
                    //child.state.print("child");

                // Search to see if new child's state is already there.
                if !self.explored.contains_key(&child.state) 
                || !self.frontier_hash.contains_key(&child.state) {
                    // Insert the child node into the frontier since it's not there yet.
                    //println!("^ Does not exist priority: {} ^ \n", priority);
                    self.insert_frontier(parent_id, child, priority);
                }
                // If child's state is in the frontier with a higher path cost then replace it.
                else if self.frontier_hash.contains_key(&child.state) {
                    let id = *self.frontier_hash.get(&child.state)?;
                    let path_cost_existing = self.tree.get(id)?.data().path_cost;

                    if child.path_cost < path_cost_existing {
                        self.remove_frontier(id, &child.state);
                        //println!("Better rate ^priority: {}\n", priority);
                        self.insert_frontier(parent_id, child, priority);
                    }
                }
            }
        }
        None
    }

    fn insert_frontier(&mut self, parent: NodeId, child: Node, priority: u32) {
        let mut p_node = match self.tree.get_mut(parent) {
            None => return,
            Some(t) => t
        };
        let mut c_node = p_node.append(child);
                    
        let id = c_node.node_id();
        self.frontier.push(id, Reverse(priority));
        self.frontier_hash.insert(c_node.data().state.clone(), id);
    }

    fn remove_frontier(&mut self, node_id: NodeId, state: &Puzzle) {
        self.tree.remove(node_id, RemoveBehavior::DropChildren);
        self.frontier.remove(&node_id);
        self.frontier_hash.remove(state);
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
