/*
    Sliding Block Puzzle Solver
    Thomas Abel
    AI class
    2022-10-18
*/
mod puzzle;
mod agent;

use puzzle::*;
use agent::*;

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
    let puzzle = Puzzle::new(dimension);
    //let puzzle = Puzzle::new_from_vec(dimension, vec![4, 5, 0, 6, 1, 8, 7, 3, 2]);
    let goal = Puzzle::new_from_vec(dimension, vec![1, 2, 3, 5, 5, 6, 7, 8, 0]);
    
    goal.print("Goal");
    puzzle.print("Puzzle");
    
    let mut agent = Agent::new(puzzle, goal);
    let solution = agent.uniform_cost_search();
    match solution {
        None => println!("No Solution found."),
        Some(t) => t.print(),
    }

    // Ending message.
    println!("\n<----------  Ending the session.  ---------->\n");
}
