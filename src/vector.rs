use std::ops;


#[derive(Clone, Copy, PartialEq, Hash, Debug)]
pub struct Vector2 {
    pub x: i32, pub y: i32
}
#[allow(unused)]
impl Vector2 {
    pub fn new(x: i32, y: i32) -> Vector2 {
        Vector2 { x, y }
    }
    pub fn dim(&self) -> ndarray::Dim<[usize; 2]> {
        ndarray::Dim((self.x as usize, self.y as usize))
    }
    pub fn distance_ortho(&self, other: &Vector2) -> u32 {
        (self.x - other.x).abs() as u32 + (self.y - other.y).abs() as u32
    }
    pub fn index(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}
