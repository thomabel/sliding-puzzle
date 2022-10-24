use std::cmp::Reverse;
use priority_queue::PriorityQueue;
use slab_tree::{NodeId, TreeBuilder};
use crate::{puzzle::*, agent::Node};

#[test]
fn goal() {
    let dimension = Vector2::new(3, 3);
    let goal = Puzzle::new_from_vec(dimension, vec![1, 2, 3, 4, 5, 6, 7, 8, 0]);
    let puzzle = Puzzle::new_from_vec(dimension, vec![4, 5, 0, 6, 1, 8, 7, 3, 2]);
    assert!(goal.goal_test(&goal));
    assert!(!goal.goal_test(&puzzle));
}

#[test]
fn heuristics() {
    let dimension = Vector2::new(3, 3);
    let goal = Puzzle::new_from_vec(dimension, vec![1, 2, 3, 4, 5, 6, 7, 8, 0]);
    let puzzle = Puzzle::new_from_vec(dimension, vec![4, 5, 0, 6, 1, 8, 7, 3, 2]);
    assert_eq!(goal.heuristic_misplaced(&goal), 0);
    assert_eq!(goal.heuristic_distances(&goal), 0);
    assert_eq!(puzzle.heuristic_misplaced(&goal), 7);
    assert_eq!(puzzle.heuristic_distances(&goal), 14);
}

#[test]
fn act() {
    let dimension = Vector2::new(3, 3);
    let goal = Puzzle::new_from_vec(dimension, vec![
        1, 2, 3, 
        4, 5, 6, 
        7, 8, 0]);

    let new = Puzzle::new_from_vec(dimension, vec![
        1, 2, 3, 
        4, 5, 0, 
        7, 8, 6]);
    let act = goal.act(ActionType::Up);
    assert!(act.goal_test(&new));
    assert_eq!(act, new);

    let new2 = Puzzle::new_from_vec(dimension, vec![
        1, 2, 3, 
        4, 0, 5, 
        7, 8, 6]);
    let act2 = act.act(ActionType::Left);
    assert!(new2.goal_test(&act2));
    assert_eq!(new2, act2);

}

#[test]
fn priority() {
    let dimension = Vector2::new(3, 3);
    let goal = Puzzle::new_from_vec(dimension, vec![
        1, 2, 3, 
        4, 5, 6, 
        7, 8, 0]);
    let puzzle = Puzzle::new_from_vec(dimension, vec![
        4, 5, 0, 
        6, 1, 8, 
        7, 3, 2]);
    let puzzle2 = Puzzle::new_from_vec(dimension, vec![
        1, 2, 3, 
        4, 5, 0, 
        7, 8, 6]);

    let root = Node::new(goal.clone(), ActionType::None, 0);
    let mut tree = 
        TreeBuilder::<Node>::new()
        .with_root(root)
        .build();
    
    let mut frontier = PriorityQueue::<NodeId, Reverse<u32>>::new();
    let root_id = tree.root_id().unwrap();
    frontier.push(root_id, Reverse(puzzle.heuristic_distances(&goal)));
    
    let distance = puzzle2.heuristic_distances(&goal);
    let mut root = tree.root_mut().expect("msg");
    let node = root.append(Node::new(puzzle2.clone(), ActionType::None, puzzle2.heuristic_distances(&goal)));
    frontier.push(node.node_id(), Reverse(distance));

    let pop1 = frontier.pop().unwrap();
    assert_eq!(pop1.1, Reverse(1));
    //assert!(frontier);
    assert_eq!(frontier.pop().unwrap().1, Reverse(14));

}