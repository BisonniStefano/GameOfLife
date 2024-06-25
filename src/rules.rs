use crate::states::StandardStates;

pub trait Rule<S> {

    fn apply(&self, state: S, neighbors: Vec<S>) -> S;

}



pub struct StandardRules;

impl Rule<StandardStates> for StandardRules{

    fn apply(&self, state: StandardStates, neighbors: Vec<StandardStates>) -> StandardStates {

        let n_alives = neighbors.iter().filter(|&neighbor| *neighbor == StandardStates::Alive).count();

        match (state, n_alives) {
            (StandardStates::Alive, x) if x < 2 => StandardStates::Dead,
            (StandardStates::Alive, 2..=3) => StandardStates::Alive,
            (StandardStates::Alive, x) if x > 3 => StandardStates::Dead,
            (StandardStates::Dead, 3) => StandardStates::Alive,
            (previous, _) => previous,
        }
    }

}