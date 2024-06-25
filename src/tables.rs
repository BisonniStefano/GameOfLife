use std::collections::HashMap;
use crate::positions::{Position, Point2D};
use crate::rules::Rule;
use crate::states::StandardStates;


pub trait Table<P, S> where P: Position {

    fn tick<'a>(&'a self, rule: &Box<dyn Rule<S>>) -> Box<dyn Table<P, S> + 'a>;

    fn get(&self, position: P) -> S;

    fn get_active_cells(&self) -> Vec<(P, S)>;

}




#[derive(Clone)]
pub struct UnlimitedGrid<P, S> where P: Position {
    cells: HashMap<P, S>
}

impl UnlimitedGrid<Point2D, StandardStates> {

    pub fn new() -> Self {
        UnlimitedGrid {
            cells: HashMap::new()
        }
    }

    pub fn insert(&mut self, position: Point2D) {
        self.cells.insert(position, StandardStates::Alive);
    }

}

impl Table<Point2D, StandardStates> for UnlimitedGrid<Point2D, StandardStates> {

    fn tick<'a>(&'a self, rule: &Box<dyn Rule<StandardStates>>) -> Box<dyn Table<Point2D, StandardStates> + 'a> {
        let mut interesting_positions = Vec::new();
        for (p, _) in &self.cells {
            interesting_positions.push(*p);
            let neighbors = p.neighbors();
            for n in neighbors {
                interesting_positions.push(n);
            }
        }

        let mut result = UnlimitedGrid::new();

        for position in interesting_positions {
            let mut near_states = Vec::new();
            let neighbors = position.neighbors();
            for n in neighbors {
                near_states.push(self.get(n));
            }

            let current_state = self.get(position);
            let next_state = rule.apply(current_state, near_states);
            result.insert(position);
        }

        Box::new(result)
    }


    fn get(&self, position: Point2D) -> StandardStates {
        *self.cells.get(&position).unwrap_or(&StandardStates::Dead)
    }

    fn get_active_cells(&self) -> Vec<(Point2D, StandardStates)> {
        self.cells.iter().map(|(p, s)| (*p, *s)).collect()
    }

}