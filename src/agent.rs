use std::collections::HashMap;
use priority_queue::PriorityQueue;
use crate::puzzle::*;
use slab_tree::*;

#[derive(Clone)]
pub struct Node {
    state: Puzzle,
    action: ActionType,
    path_cost: u32,
}
impl Node {
    pub fn root_node(state: &Puzzle) -> Node {
        Node { state: state.clone(), action: ActionType::None, path_cost: 0 }
    }
    pub fn child_node(parent_state: &Puzzle, parent_cost: u32, action: ActionType) -> Node {
        let state = parent_state.act(action);
        let path_cost = parent_cost + 1;
        Node { state, action, path_cost }
    }
}

pub struct Solution {
    state_path: Vec<Puzzle>,
    steps: u32,
}
impl Solution {
    pub fn print(&self) {
        for puzzle in &self.state_path {
            println!("{}", puzzle.read_map());
        }
    }
}

pub struct Agent {
    tree: Tree<Node>,
    frontier: PriorityQueue<NodeId, u32>,
    frontier_hash: HashMap<Puzzle, NodeId>,
    explored: HashMap<Puzzle, NodeId>,
    goal: Puzzle,
}
impl Agent {
    pub fn new(initial: Puzzle, goal: Puzzle) -> Agent {
        let root = Node::root_node(&initial);
        let tree = 
            TreeBuilder::<Node>::new()
            .with_root(root)
            .build();
        
        let mut frontier = PriorityQueue::<NodeId, u32>::new();
        let root_id = tree.root_id().unwrap();
        frontier.push(root_id, 0);

        let mut frontier_hash = HashMap::new();
        frontier_hash.insert(initial, root_id);

        let explored = HashMap::new();

        Agent { tree, frontier, frontier_hash, explored, goal }
    }

    pub fn uniform_cost_search(&mut self) -> Option<Solution> {
        let mut counter = 1_000_000;
        while counter > 0 {
            // Check if the frontier is empty.
            // Returns no solution if true, the cheapest path cost node if false.
            let parent_id = 
                match self.frontier.pop() {
                    None => return None,
                    Some(t) => t.0,
                };
            
            {
                let node = self.tree.get(parent_id)?;
                
                // If the goal state has been reached then return the solution.
                if node.data().state.goal_test(&self.goal) {
                    return self.solution(parent_id);
                }
                
                // Add the nodes state to explored to show we've now reached that state.
                self.explored.insert(node.data().state.clone(), node.node_id());
            }
                
            // Iterate through all action types.
            let parent = self.tree.get(parent_id)?.data().clone();
            //parent.data().state.print("");
            for action in [ActionType::Up, ActionType::Down, ActionType::Left, ActionType::Right].iter() {
                let child = 
                    Node::child_node(
                        &parent.state.clone(), 
                        parent.path_cost, 
                        *action);

                // Search to see if new child's state is already there.
                if !self.explored.contains_key(&child.state) 
                || !self.frontier_hash.contains_key(&child.state) {
                    // Insert the child node into the frontier since it's not there yet.
                    self.insert_frontier(parent_id, child);
                }
                // If child's state is in the frontier with a higher path cost then replace it.
                else if self.frontier_hash.contains_key(&child.state) {
                    let id = *self.frontier_hash.get(&child.state)?;
                    let path_cost = self.tree.get(id)?.data().path_cost;

                    if child.path_cost < path_cost {
                        self.remove_frontier(id, &child.state);
                        self.insert_frontier(parent_id, child);
                    }
                }
            }
            counter -= 1;
        }
        None
    }

    fn insert_frontier(&mut self, parent: NodeId, child: Node) {
        let mut branch = match self.tree.get_mut(parent) {
            None => return,
            Some(t) => t
        };
        branch.append(child);
                    
        let id = branch.node_id();
        let data = branch.data();
        //let priority = data.path_cost;
        let priority = data.state.heuristic_misplaced(&self.goal) as u32;
        
        self.frontier.push(id, priority);
        self.frontier_hash.insert(data.state.clone(), id);
    }

    fn remove_frontier(&mut self, node_id: NodeId, state: &Puzzle) {
        self.tree.remove(node_id, RemoveBehavior::DropChildren);
        self.frontier.remove(&node_id);
        self.frontier_hash.remove(state);
    }

    fn solution(&self, start: NodeId) -> Option<Solution> {
        let node = self.tree.get(start)?;
        let root = self.tree.root()?;
        
        let mut steps = 0;
        let mut state_path = vec![node.data().state.clone()];
        //let mut state_path = Vec::<Puzzle>::new();
        //state_path.push(node.data().state.clone());

        for parent in node.ancestors() {
            if parent.node_id() == root.node_id() {
                break;
            }
            state_path.push(parent.data().state.clone());
            steps += 1;
        }

        Some(Solution { state_path, steps })
    }

}
