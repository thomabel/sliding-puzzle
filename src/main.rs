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
    experiment();
}

fn experiment() {
    println!("\n<---------- Starting the session. ---------->\n");

    let dimension = Vector2::new(3, 3);
    let puzzle = puzzles(dimension);
    let heuristic = Heuristic::OrthoDistance;
    let loop_count = 1_000_000;
    let count = true;

    let mut agent = Agent::new(puzzle[3].clone(), puzzle[0].clone());
    let solution = agent.uniform_cost_search(heuristic, loop_count, count);
    println!();
    match solution {
        None => println!("No Solution found."),
        Some(t) => t.print(),
    }

    println!("\n<----------  Ending the session.  ---------->\n");
}

fn puzzles(dimension: Vector2) -> Vec<Puzzle> {
    let puzzle_raw:Vec<Vec<u8>>;
    puzzle_raw = vec![
            // goal
            vec![
                1, 2, 3, 
                4, 5, 6, 
                7, 8, 0
            ],
            // from homework sheet
            vec![
                4, 5, 0, 
                6, 1, 8, 
                7, 3, 2
            ],
            // trivial
            vec![
                4, 1, 3, 
                0, 2, 6, 
                7, 5, 8
            ],
    ];
    let mut puzzle = Vec::new();
    for p in puzzle_raw {
        puzzle.push(Puzzle::from_vec(dimension, p));
    }
    let mut random_puzzle = Puzzle::new(dimension);
    while !random_puzzle.test_solvable() {
        random_puzzle = Puzzle::new(dimension);
    }
    puzzle.push(random_puzzle);
    
    puzzle
}

#[test]
fn solvable() {
    let dimension = Vector2::new(3, 3);
    let puzzle = puzzles(dimension);

    assert!(puzzle[0].test_solvable());
    assert!(!puzzle[1].test_solvable());
    assert!(puzzle[2].test_solvable());
    assert!(puzzle[3].test_solvable());
}