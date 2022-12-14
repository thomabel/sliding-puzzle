use ndarray::prelude::*;
use ndarray_rand::{rand::{seq::SliceRandom, thread_rng}};
use crate::vector::Vector2;

#[derive(Clone, Copy, PartialEq)]
pub enum ActionType {
    None, Up, Down, Left, Right
}

#[derive(Clone, PartialEq, Hash, Debug)]
pub struct Puzzle {
    map: Array2<u8>,  // Starting state, for reference.
    blank: Vector2,     // Location of the blank spot.
    dimension: Vector2, // Size of the grid.
}
impl Eq for Puzzle {}

impl Puzzle {
    // NEW
    pub fn new(dimension: Vector2) -> Puzzle {
        let map = random(dimension);
        let blank = match find_value(&map, 0) {
            Some(vec) => vec,
            None => panic!("Can't find blank."),
        };

        Puzzle { map, blank, dimension }
    } 
    pub fn from_vec(dimension: Vector2, vec: Vec<u8>) -> Puzzle {
        let shape = dimension.dim();
        let map = Array2::<u8>::from_shape_vec(shape, vec).unwrap();
        let blank = find_value(&map, 0).unwrap();
        Puzzle { map, blank, dimension }
    }

    /// Returns a cloned version of the puzzle changed by the given move.
    pub fn act(&self, action: ActionType) -> Puzzle {
        let mut puzzle = self.clone();
        let mut direction = Vector2::new(0, 0);

        match action {
            ActionType::None => return puzzle,
            ActionType::Up => {
                if puzzle.blank.x > 0 {
                    direction.x = -1;
                }
            },
            ActionType::Down => {
                if puzzle.blank.x < puzzle.dimension.x - 1 {
                    direction.x = 1;
                }
            },
            ActionType::Left => {
                if puzzle.blank.y > 0 {
                    direction.y = -1;
                }
            },
            ActionType::Right => {
                if puzzle.blank.y < puzzle.dimension.y - 1 {
                    direction.y = 1;
                }
            },
        };
        puzzle.move_blank(direction);

        puzzle
    }

    /// Updates the position of the blank tile.
    fn move_blank(&mut self, direction: Vector2) {
        if direction.x == 0 && direction.y == 0 {
            return;
        }
        let start = self.blank;
        self.blank.x += direction.x;
        self.blank.y += direction.y;
        let end = self.blank;
        self.swap(start, end);
    }
    
    // Reading and Writing
    /// Swaps the values at the two given positions.
    fn swap(&mut self, pos1: Vector2, pos2: Vector2) {
        let val = self.read_at_pos(pos1);
        self.write_at_pos(pos1, self.read_at_pos(pos2));
        self.write_at_pos(pos2, val)
    }
    fn write_at_pos(&mut self, pos: Vector2, val: u8) {
        self.map[[pos.x as usize, pos.y as usize]] = val;
    }
    pub fn read_at_pos(&self, pos: Vector2) -> u8 {
        self.map[[pos.x as usize, pos.y as usize]]
    }
    
    pub fn print(&self, label: &str) {
        println!("{}\n{}", label, self.to_string());
    }

    /// A heuristic that counts the number of misplaced tiles.
    pub fn heuristic_misplaced(&self, goal: &Puzzle) -> u8 {
        let mut count = 0;
        for spot in self.map.iter().zip(goal.map.iter()) {
            if *spot.0 != *spot.1 && *spot.0 != 0 {
                count += 1;
            }
        }
        count
    }

    /// A heuristic that sums the distances of each tile from its goal.
    pub fn heuristic_distances(&self, goal: &Puzzle) -> u32 {
        let mut count = 0;
        for (i, row) in self.map.rows().into_iter().enumerate() {
            for (j, col) in row.into_iter().enumerate() {
                if *col != 0 {
                    let position = Vector2::new(i as i32, j as i32);
                    let other = find_value(&goal.map, *col).unwrap();
                    count += position.distance_ortho(&other);
                } 
            }
        }
        count
    }

    /// Finds the number of inversions in the puzzle layout.
    pub fn inversions(&self) -> u32 {
        let vec = self.map.clone().into_raw_vec();
        let len = vec.len();

        let mut counter = 0;
        for i in 0..len {
            if vec[i] == 0 {
                continue;
            }
            let k = i + 1;
            for j in k..len {
                if vec[j] != 0 && 
                    vec[i] > vec[j] {
                    counter += 1;
                }
            }
        }
        counter
    }

    /// Uses inversions to test if this puzzle is solvable.
    pub fn test_solvable(&self) -> bool {
        self.inversions() % 2 == 0
    }
}

impl ToString for Puzzle {
    // Produces a string version of the map.
    fn to_string(&self) -> String {
        let mut str = String::new();
        let total = self.dimension.x * self.dimension.y;

        for i in self.map.rows() {
            for j in i {
                let ch = match j {
                    0 => String::from("_"),
                    _ => j.to_string(),
                };
                let st = match total {
                    v if v >=  10 => format!("{:>2} ", ch),
                    v if v >= 100 => format!("{:>3} ", ch),
                    _ => format!("{} ", ch),
                };

                str.push_str(&st);
            }
            str.push('\n');
        }

        str
    }
}

/// Creates a random array with one of each value, from 0..x*y
fn random(dimension: Vector2) -> Array2<u8> {
    let range = 0..(dimension.x.abs() * dimension.y.abs()) as u8;
    let mut vec: Vec<u8> = range.collect();
    vec.shuffle(&mut thread_rng());

    let shape = dimension.dim();
    Array2::<u8>::from_shape_vec(shape, vec)
        .unwrap_or_else(|_|Array2::<u8>::zeros(shape))
}

/// If it exists, finds the position of the given value on the map.
fn find_value(map: &Array2<u8>, value: u8) -> Option<Vector2> {
    for (i, row) in map.rows().into_iter().enumerate() {
        for (j, col) in row.into_iter().enumerate() {
            if *col == value {
                return Some(Vector2::new(i as i32, j as i32));
            }
        }
    }
    None
}
