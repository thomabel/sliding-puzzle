/*
    Sliding Block Puzzle Solver
    Thomas Abel
    AI class
    2022-10-18
*/
mod puzzle;
mod agent;
mod vector;
mod test;

use puzzle::*;
use agent::*;
use vector::Vector2;

fn main() {
    // Welcome message.
    println!("\n<---------- Starting the session. ---------->\n");
    /* 
    for _i in 0..10 {
        let puzzle = Puzzle::new(dimension);
        println!("{}", puzzle.read_map());
    }
    */
    
    let dimension = Vector2::new(3, 3);
    let goal = Puzzle::new_from_vec(dimension, vec![
        1, 2, 3, 
        4, 5, 6, 
        7, 8, 0]);

    let puzzle_random = Puzzle::new(dimension);

    let puzzle_hw = Puzzle::new_from_vec(dimension, vec![
        4, 5, 0, 
        6, 1, 8, 
        7, 3, 2]);
    
    let puzzle_trivial = Puzzle::new_from_vec(dimension, vec![
        4, 1, 3, 
        0, 2, 6, 
        7, 5, 8]);
    
    //let puzzle = &puzzle_trivial;
    //let puzzle = &puzzle_random;
    let puzzle = &puzzle_hw;
    
    goal.print("Goal");
    puzzle.print("Puzzle");
    
    let heuristic = Heuristic::OrthoDistance;
    let loop_count = u32::max_value(); // u32::pow(2, 16);
    let count = true;

    let mut agent = Agent::new((*puzzle).clone(), goal, heuristic);
    let solution = agent.uniform_cost_search(heuristic, loop_count, count);
    match solution {
        None => println!("No Solution found."),
        Some(t) => t.print(),
    }

    // Ending message.
    println!("\n<----------  Ending the session.  ---------->\n");
}
