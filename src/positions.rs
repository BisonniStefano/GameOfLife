use rocket::serde::{Deserialize, Serialize};

pub trait Position {
    fn neighbors(&self) -> Vec<Self> where Self: Sized;
}





#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point2D {
    x: i32,
    y: i32,
}

impl Point2D {
    pub fn new(x: i32, y: i32) -> Self { Point2D { x, y } }
}

impl Position for Point2D {

    fn neighbors(&self) -> Vec<Self> {
        vec![
            Point2D { x: self.x-1, y: self.y+1 },
            Point2D { x: self.x, y: self.y+1 },
            Point2D { x: self.x+1, y: self.y+1 },

            Point2D { x: self.x-1, y: self.y },
            Point2D { x: self.x+1, y: self.y },

            Point2D { x: self.x-1, y: self.y-1 },
            Point2D { x: self.x, y: self.y-1 },
            Point2D { x: self.x+1, y: self.y-1 },
        ]
    }

}