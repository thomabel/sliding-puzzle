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
    let mut solutions = Vec::new();
    let loop_count = 1_000_000;

    // Run a trial for each strategy and heuristic combination.
    for search_strategy in [SearchStrategy::BestFirst, SearchStrategy::AStar] {
        for heuristic in [Heuristic::Misplaced, Heuristic::OrthoDistance, Heuristic::Inversions] {
            // Run the trial 5 times using the same set of 5 initial states.
            for i in 0..5 {
                let mut agent = Agent::new(puzzle[3 + i].clone(), puzzle[0].clone());
                let label = format!("{} + {}", search_strategy.to_string(), heuristic.to_string());
                match agent.uniform_cost_search(search_strategy, heuristic, loop_count) {
                    None => {
                        solutions.push((label, Err("\nNo Solution found.")));
                    },
                    Some(sol) => {
                        solutions.push((label, Ok(sol)));
                    },
                }

            }
        }
    }

    analyze_solutions(solutions, 6, 5);

    println!("\n<----------  Ending the session.  ---------->\n");
}

fn analyze_solutions(solutions: Vec<(String, Result<Solution, &str>)>, categories: usize, trials: usize) {
    for i in 0..categories as usize {
        // Track average steps for each category.
        let mut steps = 0;
        let mut count = 0;
        let mut least_steps = u32::max_value();
        let mut index = 0;

        let mut k = i * trials;
        println!("{}", &solutions[i * trials].0);
        for j in 0.. trials as usize {
            k = i * trials + j;
            let sol = &solutions[k];
        
            match &sol.1 {
                Ok(sol) => {
                    count += 1;
                    steps += sol.steps;
                    if sol.steps < least_steps {
                        least_steps = sol.steps;
                        index = k;
                    }
                },
                Err(message) => {
                    println!("{}", message);
                },
            }
        }
        println!("Best Solution: ");
        solutions[index].1.as_ref().unwrap().print();
        let avg_steps = steps as f32 / count as f32;
        println!("Average Steps: {}", avg_steps);
    }

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

    // Add random puzzles
    for i in 0..5 {
        let mut random_puzzle = Puzzle::new(dimension);
        while !random_puzzle.test_solvable() {
            random_puzzle = Puzzle::new(dimension);
        }
        puzzle.push(random_puzzle);
    }
    
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
